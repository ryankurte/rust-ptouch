
use std::time::Duration;

use log::{trace, debug, error};
use structopt::StructOpt;

use rusb::{Context, Device, DeviceDescriptor, DeviceHandle, Direction, TransferType, UsbContext};

pub mod device;

pub mod commands;
pub use commands::Commands;

pub mod tiff;

pub mod render;

pub struct PTouch {
    _device: Device<Context>,
    handle: DeviceHandle<Context>,
    descriptor: DeviceDescriptor,
    //endpoints: Endpoints,
    timeout: Duration,

    cmd_ep: u8,
    stat_ep: u8,
}


pub const BROTHER_VID: u16 = 0x04F9;
pub const DEFAULT_TIMEOUT: Duration = Duration::from_millis(500);


/// Filter for selecting a specific PTouch device
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "structopt", derive(StructOpt))]
pub struct Filter {
    #[cfg_attr(feature = "structopt", structopt(long, default_value="pt-p710bt"))]
    /// Label maker device kind
    pub device: device::PTouchDevice,

    #[cfg_attr(feature = "structopt", structopt(long, default_value="0"))]
    /// Index (if multiple devices are connected)
    pub index: usize,
}


// Lazy initialised libusb context
lazy_static::lazy_static!{
    static ref CONTEXT: Context = {
        Context::new().unwrap()
    };
}


#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("USB error: {:?}", 0)]
    Usb(rusb::Error),

    #[error("Invalid device index")]
    InvalidIndex,

    #[error("No supported languages")]
    NoLanguages,

    #[error("Unable to locate expected endpoints")]
    InvalidEndpoints,

    #[error("Renderer error")]
    Render,
}

impl From<rusb::Error> for Error {
    fn from(e: rusb::Error) -> Self {
        Error::Usb(e)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Info {
    pub manufacturer: String,
    pub product: String,
    pub serial: String,
}

impl PTouch {
    /// Create a new PTouch driver with the provided USB options
    pub fn new(o: &Filter) -> Result<Self, Error> {
        Self::new_with_context(o, &CONTEXT)
    }

    /// Create a new PTouch driver with the provided USB options and rusb::Context
    pub fn new_with_context(o: &Filter, context: &Context) -> Result<Self, Error> {
        
        // List available devices
        let devices = context.devices()?;

        // Find matching VID/PIDs
        let mut matches: Vec<_> = devices.iter().filter_map(|d| {
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
        }).collect();

        // Check index is valid
        if matches.len() < o.index || matches.len() == 0 {
            error!("Device index ({}) exceeds number of discovered devices ({})",
                o.index, matches.len());
            return Err(Error::InvalidIndex)
        }

        debug!("Found matching devices: {:?}", matches);

        // Fetch matching device
        let (device, descriptor) = matches.remove(o.index);

        // Open device handle
        let mut handle = device.open()?;

        // Reset device
        handle.reset()?;

        // Locate endpoints
        let config_desc = device.config_descriptor(0)?;
        let interface = match config_desc.interfaces().next() {
            Some(i) => i,
            None => {
                error!("No interfaces found");
                return Err(Error::InvalidEndpoints)
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
                error!("Failed to locate command and status endpoints");
                return Err(Error::InvalidEndpoints)
            }
        };

        // Set endpoint configuration
        handle.set_active_configuration(config_desc.number())?;


        Ok(Self{_device: device, handle, descriptor, cmd_ep, stat_ep, timeout: DEFAULT_TIMEOUT})
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
            return Err(Error::NoLanguages)
        }

        // Fetch information
        let language = languages[0];
        let manufacturer = self.handle.read_manufacturer_string(language, &self.descriptor, timeout)?;
        let product = self.handle.read_product_string(language, &self.descriptor, timeout)?;
        let serial = self.handle.read_serial_number_string(language, &self.descriptor, timeout)?;
        

        Ok(Info{manufacturer, product, serial})
    }

    pub fn print(&mut self, img: ()) -> Result<(), Error> {

        // Check image size is viable

        // Send setup print commands

        // Write out print data

        unimplemented!()
    }

    /// Read from status EP (with specified timeout)
    fn read(&mut self, timeout: Duration) -> Result<[u8; 32], Error> {
        let mut buff = [0u8; 32];

        // Execute read
        let n = self.handle.read_bulk(
            self.stat_ep,
            &mut buff,
            timeout,
        )?;

        if n == 32 {
            debug!("Received raw status: {:?}", buff);
        }

        // TODO: parse out status?

        Ok(buff)
    }

    /// Write to command EP (with specified timeout)
    fn write(&mut self, data: &[u8], timeout: Duration) -> Result<(), Error> {
        
        // Execute write
        let _n = self.handle.write_bulk(
            self.cmd_ep,
            &data,
            timeout,
        )?;

        // TODO: check write length?

        Ok(())
    }
}
