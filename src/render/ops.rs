use std::str::FromStr;

use strum_macros::{Display, EnumString, EnumVariantNames};
use serde::{Serialize, Deserialize};

/// Render operations, used to construct an image
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Op {
    Text { value: String, opts: TextOptions },
    Pad(usize),
    Qr(String),
    Barcode(String),
}

impl FromStr for Op {
    type Err = ();

    fn from_str(_s: &str) -> Result<Op, Self::Err> {
        unimplemented!()
    }
}

impl Op {
    pub fn text(s: &str) -> Self {
        Self::Text {
            value: s.to_string(),
            opts: TextOptions::default(),
        }
    }

    pub fn pad(columns: usize) -> Self {
        Self::Pad(columns)
    }

    pub fn qr(value: &str) -> Self {
        Self::Qr(value.to_string())
    }

    pub fn barcode(value: &str) -> Self {
        Self::Barcode(value.to_string())
    }
}


#[derive(Clone, PartialEq, Debug, Display)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "strum", derive(EnumString, EnumVariantNames))]
pub enum FontKind {
    Font6x6,
    Font6x8,
    Font6x12,
    Font8x16,
    Font12x16,
    Font24x32,
}

impl FontKind {
    pub fn char_width(&self) -> usize {
        use embedded_graphics::fonts::*;

        match self {
            FontKind::Font6x6 => Font6x6::CHARACTER_SIZE.width as usize,
            FontKind::Font6x8 => Font6x8::CHARACTER_SIZE.width as usize,
            FontKind::Font6x12 => Font6x12::CHARACTER_SIZE.width as usize,
            FontKind::Font8x16 => Font8x16::CHARACTER_SIZE.width as usize,
            FontKind::Font12x16 => Font12x16::CHARACTER_SIZE.width as usize,
            FontKind::Font24x32 => Font24x32::CHARACTER_SIZE.width as usize,
        }
    }

    pub fn char_height(&self) -> usize {
        use embedded_graphics::fonts::*;

        match self {
            FontKind::Font6x6 => Font6x6::CHARACTER_SIZE.height as usize,
            FontKind::Font6x8 => Font6x8::CHARACTER_SIZE.height as usize,
            FontKind::Font6x12 => Font6x12::CHARACTER_SIZE.height as usize,
            FontKind::Font8x16 => Font8x16::CHARACTER_SIZE.height as usize,
            FontKind::Font12x16 => Font12x16::CHARACTER_SIZE.height as usize,
            FontKind::Font24x32 => Font24x32::CHARACTER_SIZE.height as usize,
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TextOptions {
    pub font: FontKind,
    pub v_align: VAlign,
    pub h_align: HAlign,
}

#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "strum", derive(EnumString, EnumVariantNames))]
pub enum HAlign {
    Left,
    Centre,
    Right,
}

#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "strum", derive(EnumString, EnumVariantNames))]
pub enum VAlign {
    Top,
    Centre,
    Bottom,
}

impl Default for TextOptions {
    fn default() -> Self {
        Self {
            font: FontKind::Font12x16,
            h_align: HAlign::Centre,
            v_align: VAlign::Centre,
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct BarcodeOptions {}

impl Default for BarcodeOptions {
    fn default() -> Self {
        Self {}
    }
}
