
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

#[derive(Clone, PartialEq, Debug)]
pub enum Op {
    Text(TextOptions),
    //Barcode(BarcodeOptions),
}

#[derive(Clone, PartialEq, Debug, StructOpt)]
pub struct RenderOptions {
    /// Image maximum X size
    max_x: u32,
    /// Image maximum Y size
    max_y: u32,
}

impl Default for RenderOptions {
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
    display: SimulatorDisplay<BinaryColor>,
    opts: RenderOptions,
}

impl Render {
    pub fn new(opts: RenderOptions) -> Self {
        let display: SimulatorDisplay<BinaryColor> = SimulatorDisplay::new((opts.max_x, opts.max_y).into());
        Self{ display, opts }
    }

    pub fn render<'a>(&'a mut self, ops: &[Op]) {
        let style = TextStyleBuilder::new(Font6x8).build();

        let mut x = 0;
        for operation in ops {
            match operation {
                Op::Text(o) => {
                    Text::new(&o.value, Point::new(x, 0)).into_styled(style).draw(&mut self.display).unwrap();
                }
            }
        }
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
    use tempdir::TempDir;
    use std::env::var;

    use super::{Render, Op, RenderOptions, TextOptions};

    #[test]
    fn test_render_text() {
        let dir = TempDir::new("ptouch").unwrap();
        println!("Using tempdir: {:?}", dir);

        let tests = &[
            ("hello-world.png", [Op::Text("Hello World".into())])
        ];

        
        for (f, o) in tests {
            // Render options
            let r = Render::new(RenderOptions::default(), o).unwrap();

            // Save to temporary file
            let generated_file = dir.path().join(f);
            r.save(format!("{}/{}", "target", f)).unwrap();

            // Load and compare files
            
        }
        

        

        assert!(false);
    }

}
