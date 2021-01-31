
use std::path::Path;

use structopt::StructOpt;

use embedded_graphics::{
    image::{Image, ImageRaw},
    fonts::{Font6x8, Text},
    style::{TextStyle, TextStyleBuilder},
    pixelcolor::BinaryColor,
    prelude::*,
};
use embedded_graphics_simulator::{BinaryColorTheme, SimulatorDisplay, Window, OutputSettingsBuilder};
use qrcode::QrCode;

use crate::Error;

#[derive(Clone, PartialEq, Debug)]
pub enum Op {
    Text(TextOptions),
    //Barcode(BarcodeOptions),
}

#[derive(Clone, PartialEq, Debug, StructOpt)]
pub struct RenderConfig {
    /// Image maximum X size
    max_x: u32,
    /// Image maximum Y size
    max_y: u32,
}

impl Default for RenderConfig {
    fn default() -> Self {
        Self {
            max_x: 1024,
            max_y: 64,
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct TextOptions {
    value: String,
}

impl From<&str> for TextOptions {
    fn from(v: &str) -> Self {
        Self{ value: v.to_string() }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct BarcodeOptions {
    
}

impl Default for BarcodeOptions {
    fn default() -> Self {
        Self{}
    }
}

pub struct Render {
    cfg: RenderConfig,
    ops: Vec<Op>,
}

pub struct Display {
    y: usize,
    data: Vec<Vec<u8>>,
}

impl Display {
    pub fn new(y: usize) -> Self {
        Self { y, data: vec![] }
    }

    pub fn image(&self) -> Result<Vec<u8>, Error> {
        // Generate new buffer
        let mut x_len = self.data.len();
        while x_len % 8 != 0 {
            x_len += 1;
        }

        println!("Using {} rows {}({}) columns", self.y, x_len, self.data.len());

        let mut buff = vec![0u8; x_len * self.y / 8];

        // Reshape from row first to column first
        for x in 0..self.data.len() {
            for y in 0..self.y {

                let i = x / 8 + y * x_len / 8;
                let m = 1 << (x % 8) as u8;
                let v = self.get(x, y)?;
                let p = &mut buff[i];

                println!("x: {} y: {} p: {:5?} i: {} m: 0b{:08b}", x, y, v, i, m);
                

                match v {
                    true => *p |= m,
                    false => *p &= !m,
                }
            }
        }

        Ok(buff)
    }

    fn set(&mut self, x: usize, y: usize, v: bool) -> Result<(), Error> {
        // Check Y bounds
        if y > self.y {
            return Err(Error::Render);
        }

        // Extend buffer in X direction
        while x >= self.data.len() {
            self.data.push(vec![0u8; self.y / 8])
        }

        // Fetch pixel storage
        let c = &mut self.data[x][y / 8];

        // Update pixel
        match v {
            true => *c |= 1 << ((y % 8) as u8),
            false => *c &= !(1 << ((y % 8) as u8)),
        }

        Ok(())
    }

    fn get(&self, x: usize, y: usize) -> Result<bool, Error> {
        // Check Y bounds
        if y > self.y {
            return Err(Error::Render);
        }

        // Fetch pixel storage
        let c = self.data[x][y / 8];

        // Check bits
        Ok(c & (1 << (y % 8) as u8) != 0)
    }
}

impl DrawTarget<BinaryColor> for Display {
    type Error = Error;

    fn draw_pixel(&mut self, pixel: Pixel<BinaryColor>) -> Result<(), Self::Error> {
        let Pixel(coord, color) = pixel;
        self.set(coord.x as usize, coord.y as usize, color.is_on())
    }
    
    fn size(&self) -> Size {
        Size::new((self.data.len() / self.y / 8) as u32, self.y as u32)
    }
}

impl Render {
    pub fn new(cfg: RenderConfig, ops: Vec<Op>) -> Self {

        //       let display: SimulatorDisplay<BinaryColor> = SimulatorDisplay::new((opts.max_x, opts.max_y).into());

        Self{ cfg, ops }
    }

    pub fn render<D: DrawTarget<BinaryColor>>(mut display: D, ops: &[Op]) -> Result<(), Error> {
  
        let style = TextStyleBuilder::new(Font6x8).build();

        let mut x = 0;
        for operation in ops {
            match operation {
                Op::Text(o) => {
                    Text::new(&o.value, Point::new(x, 0))
                        .into_styled(style).draw(&mut display)
                        .map_err(|_| Error::Render)?;
                }
            }
        }

        // TODO: store data? idk

        Ok(())
    }

    fn render_text(&mut self, x: u32, opts: &TextOptions) -> anyhow::Result<u32> {
        unimplemented!()

    }

    #[cfg(nope)]
    fn render_qrcode(&self, x: u32, value: &str, opts: &BarcodeOptions) -> anyhow::Result<()> {
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

    pub fn show(&self) -> Result<(), anyhow::Error> {
        let mut display: SimulatorDisplay<BinaryColor> = SimulatorDisplay::new(Size::new(128, 64));

        let output_settings = OutputSettingsBuilder::new()
            .theme(BinaryColorTheme::OledBlue)
            .build();
        
        Window::new("Hello World", &output_settings).show_static(&display);

        unimplemented!()
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
