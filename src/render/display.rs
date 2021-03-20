use embedded_graphics::{
    prelude::*,
    pixelcolor::BinaryColor,
};


use crate::Error;

/// In memory display for drawing / rendering data
pub struct Display {
    y: usize,
    y_max: usize,
    data: Vec<Vec<u8>>,
}

impl Display {
    /// Create a new display with the provided height and minimum width
    pub fn new(y: usize, min_x: usize) -> Self {
        let mut y_max = y;
        while y_max % 8 != 0 {
            y_max += 1;
        }

        Self {
            y,
            y_max,
            data: vec![vec![0u8; y_max / 8]; min_x],
        }
    }

    /// Fetch a flipped + compressed vector image for output to printer
    pub fn image(&self) -> Result<Vec<u8>, Error> {
        // Generate new buffer
        let mut x_len = self.data.len();
        while x_len % 8 != 0 {
            x_len += 1;
        }

        println!(
            "Using {} rows {}({}) columns",
            self.y,
            x_len,
            self.data.len()
        );

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


    pub fn raster(&self, margins: (usize, usize, usize)) -> Result<Vec<[u8; 16]>, anyhow::Error> {
        let s = self.size();

        println!("Raster display size: {:?} output area: {:?}", s, margins);
        if s.height != margins.1 as u32 {
            return Err(anyhow::anyhow!("Raster display and output size differ ({:?}, {:?})", s, margins));
        }

        let mut buff = vec![[0u8; 16]; s.width as usize];

        for x in 0..(s.width as usize) {
            for y in 0..(s.height as usize) {
                let p = self.get(x, y)?;

                let y_offset = y + margins.0 as usize;

                if p {
                    buff[x][y_offset / 8] |= 1 << 7 - (y_offset % 8);
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
            self.data.push(vec![0u8; self.y_max / 8])
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
    pub fn get(&self, x: usize, y: usize) -> Result<bool, Error> {
        // Check Y bounds
        if y > self.y {
            return Err(Error::Render);
        }

        // Fetch pixel storage
        let c = self.data[x][y / 8];

        // Check bits
        Ok(c & (1 << (y % 8) as u8) != 0)
    }

    /// Fetch a pixel value by X/Y location
    pub fn get_pixel(&self, x: usize, y: usize) -> Result<Pixel<BinaryColor>, Error> {
        let v = match self.get(x, y)? {
            true => BinaryColor::On,
            false => BinaryColor::Off,
        };

        Ok(Pixel(Point::new(x as i32, y as i32), v))
    }
}

/// DrawTarget impl for in-memory Display type
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

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_display() {
        let mut d = Display::new(8, 1);
        d.set(0, 0, true).unwrap();
        assert_eq!(d.data, vec![vec![0b0000_0001]]);
        assert_eq!(
            d.image().unwrap(),
            vec![
                0b0000_0001,
                0b0000_0000,
                0b0000_0000,
                0b0000_0000,
                0b0000_0000,
                0b0000_0000,
                0b0000_0000,
                0b0000_0000,
            ]
        );

        let mut d = Display::new(8, 1);
        d.set(1, 0, true).unwrap();
        assert_eq!(d.data, vec![vec![0b0000_0000], vec![0b0000_0001]]);
        assert_eq!(
            d.image().unwrap(),
            vec![
                0b0000_0010,
                0b0000_0000,
                0b0000_0000,
                0b0000_0000,
                0b0000_0000,
                0b0000_0000,
                0b0000_0000,
                0b0000_0000,
            ]
        );

        let mut d = Display::new(8, 1);
        d.set(0, 1, true).unwrap();
        assert_eq!(d.data, vec![vec![0b0000_0010]]);
        assert_eq!(
            d.image().unwrap(),
            vec![
                0b0000_0000,
                0b0000_0001,
                0b0000_0000,
                0b0000_0000,
                0b0000_0000,
                0b0000_0000,
                0b0000_0000,
                0b0000_0000,
            ]
        );
    }

    #[cfg(disabled)]
    #[test]
    fn test_raster() {
        let mut d = Display::new(112, 1);
        d.set(0, 0, true).unwrap();
        d.set(1, 1, true).unwrap();
        d.set(2, 2, true).unwrap();


        assert_eq!(
            &d.raster((8, 112, 8)).unwrap(),
            &[
                [0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, ],
                [0x00, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, ],
                [0x00, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, ],
            ]
        );
    }
}