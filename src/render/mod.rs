use std::path::Path;

use barcoders::sym::code39::Code39;
use structopt::StructOpt;

use embedded_graphics::prelude::*;
use embedded_text::prelude::*;

use embedded_graphics::{
    pixelcolor::BinaryColor,
};
use embedded_graphics_simulator::{
    BinaryColorTheme, OutputSettingsBuilder, SimulatorDisplay, Window,
};
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
            max_x: 10 * 1024,
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
        let display = Display::new(cfg.y as usize, cfg.min_x as usize);

        Self { cfg, display }
    }

    pub fn render(&mut self, ops: &[Op]) -> Result<&Self, Error> {
        let mut x = 0;
        for operation in ops {
            x += match operation {
                Op::Text { value, opts } => self.render_text(x, value, opts)?,
                Op::Pad(c) => self.pad(x, *c)?,
                Op::Qr(v) => self.render_qrcode(x, v)?,
                Op::Barcode(v) => self.render_barcode(x, v)?,
                _ => unimplemented!(),
            }
        }

        // TODO: store data? idk

        Ok(self)
    }

    fn render_text(&mut self, start_x: usize, value: &str, opts: &TextOptions) -> Result<usize, Error> {
        use embedded_graphics::fonts::*;
        use embedded_text::style::vertical_overdraw::Hidden;


        // TODO: customise styles

        // TODO: custom alignment

        // Compute maximum line width
        let max_line_x = value
            .split("\n")
            .map(|line| opts.font.char_width() * line.len())
            .max()
            .unwrap();
        let max_x = self.cfg.max_x.min(start_x + max_line_x);

        // Create textbox instance
        let tb = TextBox::new(
            value,
            Rectangle::new(
                Point::new(start_x as i32, 0 as i32),
                Point::new(max_x as i32, self.cfg.y as i32),
            ),
        );

        println!("Textbox: {:?}", tb);

        #[cfg(nope)]
        let a = match opts.h_align {
            HAlign::Centre => CenterAligned,
            HAlign::Left => LeftAligned,
            HAlign::Right => RightAligned,
            HAlign::Justify => Justified,
        };
        #[cfg(nope)]
        let v = match opts.v_align {
            VAlign::Centre => CenterAligned,
            VAlign::Top => TopAligned,
            VAlign::Bottom => BottomAligned,
        };

        let a = CenterAligned;
        let v = CenterAligned;

        let h = Exact(Hidden);

        // Render with loaded style
        let res = match opts.font {
            FontKind::Font6x6 => {
                let ts = TextBoxStyleBuilder::new(Font6x6)
                    .text_color(BinaryColor::On)
                    .height_mode(h)
                    .alignment(a)
                    .vertical_alignment(v)
                    .build();

                let tb = tb.into_styled(ts);

                tb.draw(&mut self.display).unwrap();

                tb.size()
            }
            FontKind::Font6x8 => {
                let ts = TextBoxStyleBuilder::new(Font6x8)
                    .text_color(BinaryColor::On)
                    .height_mode(h)
                    .alignment(a)
                    .vertical_alignment(v)
                    .build();

                let tb = tb.into_styled(ts);

                tb.draw(&mut self.display).unwrap();

                tb.size()
            }
            FontKind::Font6x12 => {
                let ts = TextBoxStyleBuilder::new(Font6x12)
                    .text_color(BinaryColor::On)
                    .height_mode(h)
                    .alignment(a)
                    .vertical_alignment(v)
                    .build();

                let tb = tb.into_styled(ts);

                tb.draw(&mut self.display).unwrap();

                tb.size()
            }
            FontKind::Font8x16 => {
                let ts = TextBoxStyleBuilder::new(Font8x16)
                    .text_color(BinaryColor::On)
                    .height_mode(h)
                    .alignment(a)
                    .vertical_alignment(v)
                    .build();

                let tb = tb.into_styled(ts);

                tb.draw(&mut self.display).unwrap();

                tb.size()
            }
            FontKind::Font12x16 => {
                let ts = TextBoxStyleBuilder::new(Font12x16)
                    .text_color(BinaryColor::On)
                    .height_mode(h)
                    .alignment(a)
                    .vertical_alignment(v)
                    .build();

                let tb = tb.into_styled(ts);

                tb.draw(&mut self.display).unwrap();

                tb.size()
            }
            FontKind::Font24x32 => {
                let ts = TextBoxStyleBuilder::new(Font24x32)
                    .text_color(BinaryColor::On)
                    .height_mode(h)
                    .alignment(a)
                    .vertical_alignment(v)
                    .build();

                let tb = tb.into_styled(ts);

                tb.draw(&mut self.display).unwrap();

                tb.size()
            }
        };

        Ok(res.width as usize)
    }

    fn pad(&mut self, x: usize, columns: usize) -> Result<usize, Error> {
        self.display
            .draw_pixel(Pixel(Point::new((x + columns) as i32, 0), BinaryColor::Off))?;
        Ok(columns)
    }

    fn render_qrcode(&mut self, x_start: usize, value: &str) -> Result<usize, Error> {
        // Generate QR
        let qr = QrCode::new(value).unwrap();
        let img = qr.render()
            .dark_color(image::Rgb([0, 0, 0]))
            .light_color(image::Rgb([255, 255, 255]))
            .quiet_zone(false)
            .max_dimensions(self.cfg.y as u32, self.cfg.y as u32)
            .build();

        // Generate offsets
        let y_offset = (self.cfg.y as i32 - img.height() as i32) / 2;
        let x_offset = (x_start as i32 + y_offset);

        // Write to display
        for (x, y, v) in img.enumerate_pixels() {
            let c = match v {
                image::Rgb([0, 0, 0]) => BinaryColor::On,
                _ => BinaryColor::Off,
            };
            let p = Pixel(Point::new(x_offset + x as i32, y_offset + y as i32), c);
            self.display.draw_pixel(p)?
        }

        Ok(img.width() as usize + x_offset as usize)
    }

    fn render_barcode(&mut self, x_start: usize, value: &str) -> Result<usize, Error> {
        let barcode = Code39::new(value).unwrap();
        let encoded: Vec<u8> = barcode.encode();

        let x_offset = x_start as i32;

        // TODO: something is not quite right here...
        for i in 0..encoded.len() {
            //let v = (encoded[i / 8] & ( 1 << (i % 8) ) ) == 0;

            for y in 0..self.cfg.y {
                let c = match encoded[i] != 0 {
                    true => BinaryColor::On,
                    false => BinaryColor::Off,
                };

                let p = Pixel(Point::new(x_offset + i as i32, y as i32), c);
                self.display.draw_pixel(p)?
            }
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
            .theme(BinaryColorTheme::LcdWhite)
            .build();

        let name = format!("Label preview ({}, {})", s.width, s.height);
        Window::new(&name, &output_settings).show_static(&sim_display);

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

        let mut d = Display::new(8);
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

        let mut d = Display::new(8);
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
}
