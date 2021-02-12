
use std::path::Path;
use std::str::FromStr;

use structopt::StructOpt;
use serde::{Serialize, Deserialize};

use embedded_graphics::{
    image::{Image, ImageRaw},
    fonts::{Font6x8, Font8x16, Text},
    style::{TextStyle, TextStyleBuilder},
    pixelcolor::BinaryColor,
    prelude::*,
};
use embedded_graphics_simulator::{BinaryColorTheme, SimulatorDisplay, Window, OutputSettingsBuilder};
use qrcode::QrCode;

use crate::Error;

pub mod display;
pub use display::*;
pub mod ops;
pub use ops::*;


#[derive(Clone, PartialEq, Debug, StructOpt)]
pub struct RenderConfig {
    /// Image minimum X size
    min_x: usize,
    /// Image maximum X size
    max_x: usize,
    /// Image Y size
    y: usize,
}

impl Default for RenderConfig {
    fn default() -> Self {
        Self {
            min_x: 32,
            max_x: 1024,
            y: 64,
        }
    }
}

pub struct Render {
    cfg: RenderConfig,
    display: Display,
}


impl Render {
    /// Create a new render instance
    pub fn new(cfg: RenderConfig) -> Self {
        // Setup virtual display for rendering
        let mut display = Display::new(cfg.y as usize, cfg.min_x as usize);

        Self{ cfg, display }
    }

    pub fn render(&mut self, ops: &[Op]) -> Result<&Self, Error> {
        let mut x = 0;
        for operation in ops {
            x += match operation {
                Op::Text{ value, opts} => self.render_text(x, value, opts)?,
                Op::Pad(c) => self.pad(x, *c)?,
                _ => unimplemented!(),
            }
        }

        // TODO: store data? idk

        Ok(self)
    }

    fn render_text(&mut self, x: usize, value: &str, opts: &TextOptions) -> Result<usize, Error> {
        // TODO: customise styles

        // TODO: compute width / height, centre and scale as appropriate
        let lines: Vec<_> = value.split("\n").collect();

        // Find max line width
        let line_width = lines.iter().map(|v| v.len() * opts.font.char_width() ).max().unwrap();
        
        // TODO: configurable vspace
        let height = lines.len() * opts.font.char_height() + 2 * (lines.len() - 1);

        // Compute vertical centering
        let y = match opts.vcentre {
            true => (self.cfg.y / 2) - (height / 2),
            false => 0,
        };

        // Render font
        for i in 0..lines.len() {
            // TODO: compute horizontal centring

            opts.font.render(&mut self.display, x, y + (opts.font.char_height() + 2) * i, lines[i])?;
        }
        

        Ok(line_width)
    }

    fn pad(&mut self, x: usize, columns: usize) -> Result<usize, Error> {
        self.display.draw_pixel(Pixel(Point::new((x + columns) as i32, 0), BinaryColor::Off))?;
        Ok(columns)
    }

    #[cfg(nope)]
    fn render_qrcode(&self, x: usize, value: &str, opts: &BarcodeOptions) -> Result<(), Error> {
        // Generate QR
        let qr = QrCode::new(value)?;
        let img = qr.render::<Luma<u8>>().build();

        // Rescale if possible
        while (img.height() < self.opts.max_y / 2) {

        }

        unimplemented!()
    }

    pub fn save<P: AsRef<Path>>(&self, _path: P) -> Result<(), anyhow::Error> {
        unimplemented!()
    }

    /// Show the rendered image (note that this blocks until the window is closed)
    pub fn show(&self) -> Result<(), anyhow::Error> {
        // Fetch rendered size
        let s = self.display.size();

        println!("Render display size: {:?}", s);

        // Create simulated display
        let mut sim_display: SimulatorDisplay<BinaryColor> = SimulatorDisplay::new(s);

        // Copy buffer into simulated display
        for y in 0..s.height as usize {
            for x in 0..s.width as usize {
                let p = self.display.get_pixel(x, y)?;
                sim_display.draw_pixel(p)?;
            }
        }

        let output_settings = OutputSettingsBuilder::new()
            // TODO: set theme based on tape?
            .theme(BinaryColorTheme::OledBlue)
            .build();
        
        Window::new("Label preview", &output_settings).show_static(&sim_display);

        Ok(())
    }

    pub fn bytes(&self) -> &[u8] {
        unimplemented!()
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_display() {

        let mut d = Display::new(8);
        d.set(0, 0, true).unwrap();
        assert_eq!(d.data, vec![vec![0b0000_0001]]);
        assert_eq!(d.image().unwrap(), vec![
            0b0000_0001, 0b0000_0000, 0b0000_0000, 0b0000_0000, 
            0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000, 
        ]);

        let mut d = Display::new(8);
        d.set(1, 0, true).unwrap();
        assert_eq!(d.data, vec![vec![0b0000_0000], vec![0b0000_0001]]);
        assert_eq!(d.image().unwrap(), vec![
            0b0000_0010, 0b0000_0000, 0b0000_0000, 0b0000_0000, 
            0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000, 
        ]);
        

        let mut d = Display::new(8);
        d.set(0, 1, true).unwrap();
        assert_eq!(d.data, vec![vec![0b0000_0010]]);
        assert_eq!(d.image().unwrap(), vec![
            0b0000_0000, 0b0000_0001, 0b0000_0000, 0b0000_0000, 
            0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000, 
        ]);
    }

}
