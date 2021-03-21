//! Basic label rendering engine
// Rust PTouch Driver / Utility
//
// https://github.com/ryankurte/rust-ptouch
// Copyright 2021 Ryan Kurte

use std::path::Path;
use log::debug;

use structopt::StructOpt;
use image::{Luma};
use barcoders::sym::code39::Code39;
use qrcode::QrCode;

use embedded_graphics::prelude::*;
use embedded_text::prelude::*;

use embedded_graphics::{
    pixelcolor::BinaryColor,
};
use embedded_graphics_simulator::{
    BinaryColorTheme, OutputSettingsBuilder, SimulatorDisplay, Window,
};

use crate::Error;

pub mod display;
pub use display::*;
pub mod ops;
pub use ops::*;

#[derive(Clone, PartialEq, Debug, StructOpt)]
pub struct RenderConfig {
    /// Image minimum X size
    pub min_x: usize,
    /// Image maximum X size
    pub max_x: usize,
    /// Image Y size
    pub y: usize,
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
        // Setup virtual display for render data
        let display = Display::new(cfg.y as usize, cfg.min_x as usize);

        // Return new renderer
        Self { cfg, display }
    }

    /// Save the render buffer as an image
    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<(), anyhow::Error> {
        // Fetch current display size
        let size = self.display.size();

        // Create image
        let i = image::DynamicImage::new_luma8(size.width, size.height);
        let mut i = i.into_luma8();

        // Copy data into image
        for x in 0..size.width {
            for y in 0..size.height {
                let p = self.display.get(x as usize, y as usize)?;
                if !p {
                    i.put_pixel(x, y, Luma([0xff]));
                }
            }
        }

        // Save image to file
        i.save(path)?;

        Ok(())
    }
    

    /// Execute render operations
    pub fn render(&mut self, ops: &[Op]) -> Result<&Self, Error> {
        let mut x = 0;
        for operation in ops {
            x += match operation {
                Op::Text { text, opts } => self.render_text(x, text, opts)?,
                Op::Pad{ count } => self.pad(x, *count)?,
                Op::Qr{ code } => self.render_qrcode(x, code)?,
                Op::Barcode{ code, opts } => self.render_barcode(x, code, opts)?,
                Op::Image{ file, opts } => self.render_image(x, file, opts)?,
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

        // TODO: clean this up when updated embedded-graphics font API lands 
        // https://github.com/embedded-graphics/embedded-graphics/issues/511

        // Fix for escaped newlines from shell
        // Otherwise "\n" becomes "\\n" and nothing works quite right
        let value = value.replace("\\n", "\n");

        // Compute maximum line width
        let max_line_x = value
            .split("\n")
            .map(|line| opts.font.char_width() * line.len() + 1)
            .max()
            .unwrap();
        let max_x = self.cfg.max_x.min(start_x + max_line_x);

        // Create textbox instance
        let tb = TextBox::new(
            &value,
            Rectangle::new(
                Point::new(start_x as i32, 0 as i32),
                Point::new(max_x as i32, self.cfg.y as i32),
            ),
        );

        debug!("Textbox: {:?}", tb);

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
        let l = 4;

        // Render with loaded style
        let res = match opts.font {
            FontKind::Font6x6 => {
                let ts = TextBoxStyleBuilder::new(Font6x6)
                    .text_color(BinaryColor::On)
                    .height_mode(h)
                    .alignment(a)
                    .line_spacing(l)
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
                    .line_spacing(l)
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
                    .line_spacing(l)
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
                    .line_spacing(l)
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
                    .line_spacing(l)
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
                    .line_spacing(l)
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
        let x_offset = x_start as i32 + y_offset;

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

    fn render_barcode(&mut self, x_start: usize, value: &str, opts: &BarcodeOptions) -> Result<usize, Error> {
        let barcode = Code39::new(value).unwrap();
        let encoded: Vec<u8> = barcode.encode();

        let x_offset = x_start as i32;

        // TODO: something is not quite right here...
        for i in 0..encoded.len() {
            //let v = (encoded[i / 8] & ( 1 << (i % 8) ) ) == 0;

            for y in opts.y_offset..self.cfg.y-opts.y_offset {
                let c = match encoded[i] != 0 {
                    true => BinaryColor::On,
                    false => BinaryColor::Off,
                };

                let p = Pixel(Point::new(x_offset + i as i32, y as i32), c);
                self.display.draw_pixel(p)?
            }
        }

        Ok(encoded.len() + x_offset as usize)
    }

    fn render_image(&mut self, x_start: usize, file: &str, _opts: &ImageOptions) -> Result<usize, Error> {
        // Load image and convert to greyscale
        let img = image::io::Reader::open(file)?.decode()?;
        let i = img.clone().into_luma8();
        let d = i.dimensions();

        // TODO: Rescale based on image options

        let x_offset = x_start as i32;
        let y_offset = (self.cfg.y / 2) as i32 - (d.1 as usize / 2) as i32;

        // Copy image data into display
        for x in 0..d.0 as i32 {
            for y in 0..d.1 as i32 {
                let p = i.get_pixel(x as u32, y as u32);

                let c = match p.0[0] == 0 {
                    true => BinaryColor::On,
                    false => BinaryColor::Off,
                };

                let p = Pixel(Point::new(x_offset + x as i32, y_offset + y as i32), c);
                self.display.draw_pixel(p)?
            }
        }

        Ok(d.0 as usize + x_offset as usize)
    }

    /// Raster data to a ptouch compatible buffer for printing
    pub fn raster(&self, margins: (usize, usize, usize)) -> Result<Vec<[u8; 16]>, anyhow::Error> {
        self.display.raster(margins)
    }

    /// Show the rendered image (note that this blocks until the window is closed)
    pub fn show(&self) -> Result<(), anyhow::Error> {
        // Fetch rendered size
        let s = self.display.size();

        debug!("Render display size: {:?}", s);

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
}
