use std::time::Duration;

use log::{trace, debug};

use crate::{Error, PTouch, device::Status};
use crate::device::{AdvancedMode, Mode, PrintInfo, VariousMode, CompressionMode};

/// Raw command API for the PTouch device.
/// This provides low-level access to the device (if desired)
pub trait Commands {
    /// Null command
    fn null(&mut self) -> Result<(), Error>;
    
    /// Init command, sets up the device for printing
    fn init(&mut self) -> Result<(), Error>;
    
    /// Invalidate command, resets the device
    fn invalidate(&mut self) -> Result<(), Error>;
    
    /// Issue a status request
    fn status_req(&mut self) -> Result<(), Error>;

    /// Read a status response with the provided timeout
    fn read_status(&mut self, timeout: Duration) -> Result<Status, Error>;

    /// Switch mode, required for raster printing
    fn switch_mode(&mut self, mode: Mode) -> Result<(), Error>;

    /// Set status notify (printer automatically sends status on change)
    fn set_status_notify(&mut self, enabled: bool) -> Result<(), Error>;

    /// Set print information
    fn set_print_info(&mut self, info: &PrintInfo) -> Result<(), Error>;

    /// Set various mode flags
    fn set_various_mode(&mut self, mode: VariousMode) -> Result<(), Error>;

    /// Set advanced mode flags
    fn set_advanced_mode(&mut self, mode: AdvancedMode) -> Result<(), Error>;

    /// Set pre/post print margin
    fn set_margin(&mut self, dots: u16) -> Result<(), Error>;

    /// Set print page number
    fn set_page_no(&mut self, no: u8) -> Result<(), Error>;

    /// Set compression mode (None or Tiff).
    /// Note TIFF mode is currently... broken
    fn set_compression_mode(&mut self, mode: CompressionMode) -> Result<(), Error>;

    /// Transfer raster data
    fn raster_transfer(&mut self, data: &[u8]) -> Result<(), Error>;

    /// Send a zero raster line
    fn raster_zero(&mut self) -> Result<(), Error>;

    /// Start a print
    fn print(&mut self) -> Result<(), Error>;

    /// Start a print and feed
    fn print_and_feed(&mut self) -> Result<(), Error>;
}

impl Commands for PTouch {
    fn null(&mut self) -> Result<(), Error> {
        self.write(&[0x00], self.timeout)
    }

    fn init(&mut self) -> Result<(), Error> {
        self.write(&[0x1b, 0x40], self.timeout)
    }

    fn invalidate(&mut self) -> Result<(), Error> {
        self.write(&[0u8; 100], self.timeout)
    }

    fn status_req(&mut self) -> Result<(), Error> {
        self.write(&[0x1b, 0x69, 0x53], self.timeout)
    }

    fn read_status(&mut self, timeout: Duration) -> Result<Status, Error> {
        let status_raw = self.read(timeout)?;

        let status = Status::from(status_raw);

        debug!("Status: {:?}", status);
        trace!("Raw status: {:?}", &status_raw);

        Ok(status)
    }

    fn switch_mode(&mut self, mode: Mode) -> Result<(), Error> {
        self.write(&[0x1b, 0x69, 0x61, mode as u8], self.timeout)
    }

    fn set_status_notify(&mut self, enabled: bool) -> Result<(), Error> {
        let en = match enabled {
            true => 0,
            false => 1,
        };

        self.write(&[0x1b, 0x69, 0x21, en], self.timeout)
    }

    fn set_print_info(&mut self, info: &PrintInfo) -> Result<(), Error> {
        let mut buff = [0u8; 13];

        debug!("Set print info: {:?}", info);

        // Command header
        buff[0] = 0x1b;
        buff[1] = 0x69;
        buff[2] = 0x7a;

        if let Some(i) = &info.kind {
            buff[3] |= 0x02;
            buff[4] = *i as u8;
        }

        if let Some(w) = &info.width {
            buff[3] |= 0x04;
            buff[5] = *w as u8;
        }

        if let Some(l) = &info.length {
            buff[3] |= 0x08;
            buff[6] = *l as u8;
        }

        let raster_bytes = info.raster_no.to_le_bytes();
        &buff[7..11].copy_from_slice(&raster_bytes);

        if info.recover {
            buff[3] |= 0x80;
        }

        self.write(&buff, self.timeout)
    }

    fn set_various_mode(&mut self, mode: VariousMode) -> Result<(), Error> {
        debug!("Set various mode: {:?}", mode);

        self.write(&[0x1b, 0x69, 0x4d, mode.bits()], self.timeout)
    }

    fn set_advanced_mode(&mut self, mode: AdvancedMode) -> Result<(), Error> {
        debug!("Set advanced mode: {:?}", mode);

        self.write(&[0x1b, 0x69, 0x4b, mode.bits()], self.timeout)
    }

    fn set_margin(&mut self, dots: u16) -> Result<(), Error> {
        debug!("Set margin: {:?}", dots);

        self.write(
            &[0x1b, 0x69, 0x64, dots as u8, (dots >> 8) as u8],
            self.timeout,
        )
    }

    fn set_page_no(&mut self, no: u8) -> Result<(), Error> {
        debug!("Set page no: {:?}", no);

        self.write(&[0x1b, 0x69, 0x41, no], self.timeout)
    }

    fn set_compression_mode(&mut self, mode: CompressionMode) -> Result<(), Error> {
        debug!("Set compression mode: {:?}", mode);

        self.write(&[0x4D, mode as u8], self.timeout)
    }

    fn raster_transfer(&mut self, data: &[u8]) -> Result<(), Error> {
        let mut buff = vec![0u8; data.len() + 3];

        buff[0] = 0x47;
        buff[1] = (data.len() & 0xFF) as u8;
        buff[2] = (data.len() >> 8) as u8;

        (&mut buff[3..3+data.len()]).copy_from_slice(data);

        trace!("Raster transfer: {:02x?}", &buff[..3+data.len()]);

        self.write(&buff[..3+data.len()], self.timeout)
    }

    fn raster_zero(&mut self) -> Result<(), Error> {
        debug!("Raster zero line");
        
        self.write(&[0x5a], self.timeout)
    }

    fn print(&mut self) -> Result<(), Error> {
        debug!("Print command");
        self.write(&[0x0c], self.timeout)
    }

    fn print_and_feed(&mut self) -> Result<(), Error> {
        debug!("Print feed command");
        self.write(&[0x1a], self.timeout)
    }
}
