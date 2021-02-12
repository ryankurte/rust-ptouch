use std::str::FromStr;

use log::error;
use strum_macros::{EnumString, Display, EnumVariantNames};


use embedded_graphics::prelude::*;
use embedded_graphics::{pixelcolor::BinaryColor, prelude::Font};
use embedded_graphics::style::TextStyle;
use embedded_graphics::fonts::*;
use embedded_graphics::DrawTarget;


use crate::Error;


/// Render operations, used to construct an image
#[derive(Clone, PartialEq, Debug)]
pub enum Op {
    Text{
        value: String,
        opts: TextOptions,
    },
    Pad(usize),
    //Barcode(BarcodeOptions),
}

impl FromStr for Op {
    type Err = ();

    fn from_str(s: &str) -> Result<Op, Self::Err> {
        unimplemented!()
    }
}

impl Op {
    pub fn text(s: &str) -> Self {
        Self::Text{ value: s.to_string(), opts: TextOptions::default() }
    }

    pub fn pad(columns: usize) -> Self {
        Self::Pad(columns)
    }
}

#[derive(Clone, PartialEq, Debug, Display, EnumString, EnumVariantNames)]
pub enum FontKind {
    Font6x6,
    Font6x8,
    Font6x12,
    Font8x16,
    Font12x16,
    Font24x32
}

impl FontKind {
    pub fn char_width(&self) -> usize {
        match self {
            FontKind::Font6x6 => Font6x6::CHARACTER_SIZE.width as usize,
            FontKind::Font6x8 => Font6x8::CHARACTER_SIZE.width as usize,
            FontKind::Font6x12 => Font6x12::CHARACTER_SIZE.width as usize,
            FontKind::Font8x16 => Font8x16::CHARACTER_SIZE.width as usize,
            FontKind::Font12x16 => Font12x16::CHARACTER_SIZE.width as usize,
            FontKind::Font24x32 => Font24x32::CHARACTER_SIZE.width as usize
        }
    }

    pub fn char_height(&self) -> usize {
        match self {
            FontKind::Font6x6 => Font6x6::CHARACTER_SIZE.height as usize,
            FontKind::Font6x8 => Font6x8::CHARACTER_SIZE.height as usize,
            FontKind::Font6x12 => Font6x12::CHARACTER_SIZE.height as usize,
            FontKind::Font8x16 => Font8x16::CHARACTER_SIZE.height as usize,
            FontKind::Font12x16 => Font12x16::CHARACTER_SIZE.height as usize,
            FontKind::Font24x32 => Font24x32::CHARACTER_SIZE.height as usize
        }
    }

    pub fn render<D>(&self, display: &mut D, x: usize, y: usize, value: &str) -> Result<usize, Error> 
    where
        D: DrawTarget<BinaryColor>,
    {
        // Create text instance
        let t = Text::new(&value, Point::new(x as i32, y as i32));

        // Render with loaded style
        let res = match self {
            FontKind::Font6x6 => t.into_styled(TextStyle::new(Font6x6, BinaryColor::On)).draw(display),
            FontKind::Font6x8 => t.into_styled(TextStyle::new(Font6x8, BinaryColor::On)).draw(display),
            FontKind::Font6x12 => t.into_styled(TextStyle::new(Font6x12, BinaryColor::On)).draw(display),
            FontKind::Font8x16 => t.into_styled(TextStyle::new(Font8x16, BinaryColor::On)).draw(display),
            FontKind::Font12x16 => t.into_styled(TextStyle::new(Font12x16, BinaryColor::On)).draw(display),
            FontKind::Font24x32 => t.into_styled(TextStyle::new(Font24x32, BinaryColor::On)).draw(display),
        };

        match res {
            Ok(_) => Ok(self.char_width() * value.len()),
            Err(e) => {
                error!("Error rendering text at x: {} y: {} with value: '{}'", x, y, value);
                Err(Error::Render)
            }
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct TextOptions {
    pub font: FontKind,
    pub vcentre: bool,
    pub hcentre: bool,
}

impl Default for TextOptions {
    fn default() -> Self {
        Self {
            font: FontKind::Font12x16,
            vcentre: true,
            hcentre: false,
        }
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