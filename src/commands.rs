use std::time::Duration;

use crate::{device::CompressionMode, Error, PTouch};

use crate::device::{AdvancedMode, Mode, PrintInfo, VariousMode};

/// Raw command API for the PTouch device
pub trait Commands {
    fn null(&mut self) -> Result<(), Error>;

    fn init(&mut self) -> Result<(), Error>;

    fn status_req(&mut self) -> Result<(), Error>;

    fn read_status(&mut self, timeout: Duration) -> Result<(), Error>;

    fn switch_mode(&mut self, mode: Mode) -> Result<(), Error>;

    fn set_status_notify(&mut self, enabled: bool) -> Result<(), Error>;

    fn set_print_info(&mut self, info: &PrintInfo) -> Result<(), Error>;

    fn set_various_mode(&mut self, mode: VariousMode) -> Result<(), Error>;

    fn set_advanced_mode(&mut self, mode: AdvancedMode) -> Result<(), Error>;

    fn set_margin(&mut self, dots: u16) -> Result<(), Error>;

    fn set_page_no(&mut self, no: u8) -> Result<(), Error>;

    fn set_compression_mode(&mut self, mode: CompressionMode) -> Result<(), Error>;

    fn raster_transfer(&mut self, data: &[u8]) -> Result<(), Error>;

    fn raster_zero(&mut self) -> Result<(), Error>;

    fn print(&mut self) -> Result<(), Error>;

    fn print_and_feed(&mut self) -> Result<(), Error>;
}

impl Commands for PTouch {
    fn null(&mut self) -> Result<(), Error> {
        self.write(&[0x00], self.timeout)
    }

    fn init(&mut self) -> Result<(), Error> {
        self.write(&[0x1b, 0x40], self.timeout)
    }

    fn status_req(&mut self) -> Result<(), Error> {
        self.write(&[0x1b, 0x69, 0x53], self.timeout)
    }

    fn read_status(&mut self, timeout: Duration) -> Result<(), Error> {
        let _status_raw = self.read(timeout);

        // TODO: parse out status object

        unimplemented!()
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

        let raster_bytes = info.raster_no.to_be_bytes();
        &buff[7..10].copy_from_slice(&raster_bytes);

        if info.recover {
            buff[3] |= 0x80;
        }

        self.write(&buff, self.timeout)
    }

    fn set_various_mode(&mut self, mode: VariousMode) -> Result<(), Error> {
        self.write(&[0x1b, 0x69, 0x4d, mode.bits()], self.timeout)
    }

    fn set_advanced_mode(&mut self, mode: AdvancedMode) -> Result<(), Error> {
        self.write(&[0x1b, 0x69, 0x4b, mode.bits()], self.timeout)
    }

    fn set_margin(&mut self, dots: u16) -> Result<(), Error> {
        self.write(
            &[0x1b, 0x69, 0x64, dots as u8, (dots >> 8) as u8],
            self.timeout,
        )
    }

    fn set_page_no(&mut self, no: u8) -> Result<(), Error> {
        self.write(&[0x1b, 0x69, 0x41, no], self.timeout)
    }

    fn set_compression_mode(&mut self, mode: CompressionMode) -> Result<(), Error> {
        self.write(&[0x4D, mode as u8], self.timeout)
    }

    fn raster_transfer(&mut self, data: &[u8]) -> Result<(), Error> {
        let mut buff = vec![0u8; data.len() + 3];

        buff[0] = 0x46;
        buff[1] = (data.len() & 0xFF) as u8;
        buff[2] = (data.len() >> 8) as u8;

        (&mut buff[3..]).copy_from_slice(data);

        self.write(&buff, self.timeout)
    }

    fn raster_zero(&mut self) -> Result<(), Error> {
        self.write(&[0x5a], self.timeout)
    }

    fn print(&mut self) -> Result<(), Error> {
        self.write(&[0x0c], self.timeout)
    }

    fn print_and_feed(&mut self) -> Result<(), Error> {
        self.write(&[0x1a], self.timeout)
    }
}
