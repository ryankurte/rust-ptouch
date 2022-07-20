//! Rust PTouch Driver / Utility
//
// https://github.com/ryankurte/rust-ptouch
// Copyright 2021 Ryan Kurte

use std::time::Duration;

use commands::Commands;
use device::Status;
use image::ImageError;
use log::{trace, debug, error};

#[cfg(feature = "structopt")]
use structopt::StructOpt;

#[cfg(feature = "strum")]
use strum::VariantNames;

use rusb::{Context, Device, DeviceDescriptor, DeviceHandle, Direction, TransferType, UsbContext};

pub mod device;
use device::*;

pub mod commands;

pub mod bitmap;

pub mod tiff;

pub mod render;

/// PTouch device instance
pub struct PTouch {
    _device: Device<Context>,
    handle: DeviceHandle<Context>,
    descriptor: DeviceDescriptor,
    //endpoints: Endpoints,
    timeout: Duration,

    cmd_ep: u8,
    stat_ep: u8,
}

/// Brother USB Vendor ID
pub const BROTHER_VID: u16 = 0x04F9;

/// Options for connecting to a PTouch device
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "structopt", derive(StructOpt))]
pub struct Options {
    #[cfg_attr(feature = "structopt", structopt(long, possible_values = &device::PTouchDevice::VARIANTS, default_value = "pt-p710bt"))]
    /// Label maker device kind
    pub device: device::PTouchDevice,

    #[cfg_attr(feature = "structopt", structopt(long, default_value = "0"))]
    /// Index (if multiple devices are connected)
    pub index: usize,

    #[cfg_attr(feature = "structopt", structopt(long, default_value = "500"))]
    /// Timeout to pass to the read_bulk and write_bulk methods
    pub timeout_milliseconds: u64,

    #[cfg_attr(feature = "structopt", structopt(long, hidden = true))]
    /// Do not reset the device on connect
    pub no_reset: bool,

    #[cfg_attr(feature = "structopt", structopt(long, hidden = true))]
    /// (DEBUG) Do not claim USB interface on connect
    pub usb_no_claim: bool,

    #[cfg_attr(feature = "structopt", structopt(long, hidden = true))]
    /// (DEBUG) Do not detach from kernel drivers on connect
    pub usb_no_detach: bool,
}

// Lazy initialised libusb context
lazy_static::lazy_static! {
    static ref CONTEXT: Context = {
        Context::new().unwrap()
    };
}

/// PTouch API errors
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("USB error: {:?}", 0)]
    Usb(rusb::Error),

    #[error("IO error: {:?}", 0)]
    Io(std::io::Error),

    #[error("Image error: {:?}", 0)]
    Image(ImageError),

    #[error("Invalid device index")]
    InvalidIndex,

    #[error("No supported languages")]
    NoLanguages,

    #[error("Unable to locate expected endpoints")]
    InvalidEndpoints,

    #[error("Renderer error")]
    Render,

    #[error("Operation timeout")]
    Timeout,

    #[error("PTouch Error ({:?} {:?})", 0, 1)]
    PTouch(Error1, Error2),
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}

impl From<rusb::Error> for Error {
    fn from(e: rusb::Error) -> Self {
        Error::Usb(e)
    }
}

impl From<ImageError> for Error {
    fn from(e: ImageError) -> Self {
        Error::Image(e)
    }
}

/// PTouch device information
#[derive(Clone, Debug, PartialEq)]
pub struct Info {
    pub manufacturer: String,
    pub product: String,
    pub serial: String,
}

impl PTouch {
    /// Create a new PTouch driver with the provided USB options
    pub fn new(o: &Options) -> Result<Self, Error> {
        Self::new_with_context(o, &CONTEXT)
    }

    /// Create a new PTouch driver with the provided USB options and an existing rusb::Context
    pub fn new_with_context(o: &Options, context: &Context) -> Result<Self, Error> {
        // List available devices
        let devices = context.devices()?;

        // Find matching VID/PIDs
        let mut matches: Vec<_> = devices
            .iter()
            .filter_map(|d| {
                // Fetch device descriptor
                let desc = match d.device_descriptor() {
                    Ok(d) => d,
                    Err(e) => {
                        debug!("Could not fetch descriptor for device {:?}: {:?}", d, e);
                        return None;
                    }
                };

                // Return devices matching vid/pid filters
                if desc.vendor_id() == BROTHER_VID && desc.product_id() == o.device as u16 {
                    Some((d, desc))
                } else {
                    None
                }
            })
            .collect();

        // Check index is valid
        if matches.len() < o.index || matches.len() == 0 {
            debug!(
                "Device index ({}) exceeds number of discovered devices ({})",
                o.index,
                matches.len()
            );
            return Err(Error::InvalidIndex);
        }

        debug!("Found matching devices: {:?}", matches);

        // Fetch matching device
        let (device, descriptor) = matches.remove(o.index);

        // Open device handle
        let mut handle = match device.open() {
            Ok(v) => v,
            Err(e) => {
                debug!("Error opening device");
                return Err(e.into());
            }
        };

        // Reset device
        if let Err(e) = handle.reset() {
            debug!("Error resetting device handle");
            return Err(e.into())
        }

        // Locate endpoints
        let config_desc = match device.config_descriptor(0) {
            Ok(v) => v,
            Err(e) => {
                debug!("Failed to fetch config descriptor");
                return Err(e.into());
            }
        };

        let interface = match config_desc.interfaces().next() {
            Some(i) => i,
            None => {
                debug!("No interfaces found");
                return Err(Error::InvalidEndpoints);
            }
        };

        // EP1 is a bulk IN (printer -> PC) endpoint for status messages
        // EP2 is a bulk OUT (PC -> printer) endpoint for print commands
        // TODO: is this worth it, could we just, hard-code the endpoints?
        let (mut cmd_ep, mut stat_ep) = (None, None);

        for interface_desc in interface.descriptors() {
            for endpoint_desc in interface_desc.endpoint_descriptors() {
                // Find the relevant endpoints
                match (endpoint_desc.transfer_type(), endpoint_desc.direction()) {
                    (TransferType::Bulk, Direction::In) => stat_ep = Some(endpoint_desc.address()),
                    (TransferType::Bulk, Direction::Out) => cmd_ep = Some(endpoint_desc.address()),
                    (_, _) => continue,
                }
            }
        }

        let (cmd_ep, stat_ep) = match (cmd_ep, stat_ep) {
            (Some(cmd), Some(stat)) => (cmd, stat),
            _ => {
                debug!("Failed to locate command and status endpoints");
                return Err(Error::InvalidEndpoints);
            }
        };

        // Detach kernel driver
        // TODO: this is usually not supported on all libusb platforms
        // for now this is enabled through hidden config options...
        // needs testing and a cfg guard as appropriate
        debug!("Checking for active kernel driver");
        match handle.kernel_driver_active(interface.number())? {
            true => {
                if !o.usb_no_detach {
                    debug!("Detaching kernel driver");
                    handle.detach_kernel_driver(interface.number())?;
                } else {
                    debug!("Kernel driver detach disabled");
                }
            },
            false => {
                debug!("Kernel driver inactive");
            },
        }

        // Claim interface for driver
        // TODO: this is usually not supported on all libusb platforms
        // for now this is enabled through hidden config options...
        // needs testing and a cfg guard as appropriate
        if !o.usb_no_claim {
            debug!("Claiming interface");
            handle.claim_interface(interface.number())?;
        } else {
            debug!("Claim interface disabled");
        }

        // Create device object
        let mut s = Self {
            _device: device,
            handle,
            descriptor,
            cmd_ep,
            stat_ep,
            timeout: Duration::from_millis(o.timeout_milliseconds),
        };

        // Unless we're skipping reset
        if !o.no_reset {
            // Send invalidate to reset device
            s.invalidate()?;
            // Initialise device
            s.init()?;
        } else {
            debug!("Skipping device reset");
        }

        Ok(s)
    }

    /// Fetch device information
    pub fn info(&mut self) -> Result<Info, Error> {
        let timeout = Duration::from_millis(200);

        // Fetch base configuration
        let languages = self.handle.read_languages(timeout)?;
        let active_config = self.handle.active_configuration()?;

        trace!("Active configuration: {}", active_config);
        trace!("Languages: {:?}", languages);

        // Check a language is available
        if languages.len() == 0 {
            return Err(Error::NoLanguages);
        }

        // Fetch information
        let language = languages[0];
        let manufacturer =
            self.handle
                .read_manufacturer_string(language, &self.descriptor, timeout)?;
        let product = self
            .handle
            .read_product_string(language, &self.descriptor, timeout)?;
        let serial = self
            .handle
            .read_serial_number_string(language, &self.descriptor, timeout)?;

        Ok(Info {
            manufacturer,
            product,
            serial,
        })
    }

    /// Fetch the device status
    pub fn status(&mut self) -> Result<Status, Error> {
        // Issue status request
        self.status_req()?;

        // Read status response
        let d = self.read(self.timeout)?;

        // Convert to status object
        let s = Status::from(d);

        debug!("Status: {:02x?}", s);

        Ok(s)
    }

    /// Setup the printer and print using raw raster data.
    /// Print output must be shifted and in the correct bit-order for this function.
    /// 
    /// TODO: this is too low level of an interface, should be replaced with higher-level apis
    pub fn print_raw(&mut self, data: Vec<[u8; 16]>, info: &PrintInfo) -> Result<(), Error> {
        // TODO: should we check info (and size) match status here?


        // Print sequence from raster guide Section 2.1
        // 1. Set to raster mode
        self.switch_mode(Mode::Raster)?;

        // 2. Enable status notification
        self.set_status_notify(true)?;

        // 3. Set print information (media type etc.)
        self.set_print_info(info)?;

        // 4. Set various mode settings
        self.set_various_mode(VariousMode::AUTO_CUT)?;

        // 5. Specify page number in "cut each * labels"
        // Note this is not supported on the PT-P710BT
        // TODO: add this for other printers

        // 6. Set advanced mode settings
        self.set_advanced_mode(AdvancedMode::NO_CHAIN)?;

        // 7. Specify margin amount
        // TODO: based on what?
        self.set_margin(0)?;

        // 8. Set compression mode
        // TODO: fix broken TIFF mode and add compression flag
        self.set_compression_mode(CompressionMode::None)?;

        // Send raster data
        for line in data {
            // TODO: re-add when TIFF mode issues resolved
            //let l = tiff::compress(&line);

            self.raster_transfer(&line)?;
        }

        // Execute print operation
        self.print_and_feed()?;


        // Poll on print completion
        let mut i = 0;
        loop {
            if let Ok(s) = self.read_status(self.timeout) {
                if !s.error1.is_empty() || !s.error2.is_empty() {
                    debug!("Print error: {:?} {:?}", s.error1, s.error2);
                    return Err(Error::PTouch(s.error1, s.error2));
                }
    
                if s.status_type == DeviceStatus::PhaseChange {
                    debug!("Started printing");
                }

                if s.status_type == DeviceStatus::Completed {
                    debug!("Print completed");
                    break;
                }
            }

            if i > 10 {
                debug!("Print timeout");
                return Err(Error::Timeout);
            }

            i += 1;

            std::thread::sleep(Duration::from_secs(1));
        }


        Ok(())
    }

    /// Read from status EP (with specified timeout)
    fn read(&mut self, timeout: Duration) -> Result<[u8; 32], Error> {
        let mut buff = [0u8; 32];

        // Execute read
        let n = self.handle.read_bulk(self.stat_ep, &mut buff, timeout)?;

        if n != 32 {
            return Err(Error::Timeout)
        }

        // TODO: parse out status?

        Ok(buff)
    }

    /// Write to command EP (with specified timeout)
    fn write(&mut self, data: &[u8], timeout: Duration) -> Result<(), Error> {
        debug!("WRITE: {:02x?}", data);

        // Execute write
        let n = self.handle.write_bulk(self.cmd_ep, &data, timeout)?;

        // Check write length for timeouts
        if n != data.len() {
            return Err(Error::Timeout)
        }

        Ok(())
    }
}
