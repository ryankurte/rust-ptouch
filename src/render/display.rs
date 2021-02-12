
use embedded_graphics::{
    image::{Image, ImageRaw},
    fonts::{Font6x8, Text},
    style::{TextStyle, TextStyleBuilder},
    pixelcolor::BinaryColor,
    prelude::*,
};

use embedded_graphics_simulator::{BinaryColorTheme, SimulatorDisplay, Window, OutputSettingsBuilder};

use crate::Error;

/// Display provides an supports drawing / viewing data
pub struct Display {
    y: usize,
    data: Vec<Vec<u8>>,
}

impl Display {
    pub fn new(y: usize, min_x: usize) -> Self {
        let mut s = Self { y, data: vec![vec![0u8; y / 8]; min_x] };
        
        s
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

    /// Set a value by X/Y location
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

    /// Fetch a display value by X/Y location
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

    pub fn get_pixel(&self, x: usize, y: usize) -> Result<Pixel<BinaryColor>, Error> {
        let v = match self.get(x, y)? {
            true => BinaryColor::On,
            false => BinaryColor::Off,
        };

        Ok(Pixel(Point::new(x as i32, y as i32), v))
    }
}

impl DrawTarget<BinaryColor> for Display {
    type Error = Error;

    fn draw_pixel(&mut self, pixel: Pixel<BinaryColor>) -> Result<(), Self::Error> {
        let Pixel(coord, color) = pixel;
        self.set(coord.x as usize, coord.y as usize, color.is_on())
    }
    
    fn size(&self) -> Size {
        Size::new(self.data.len() as u32, self.y as u32)
    }
}

