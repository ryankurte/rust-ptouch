//! Render operations
// Rust PTouch Driver / Utility
//
// https://github.com/ryankurte/rust-ptouch
// Copyright 2021 Ryan Kurte

#[cfg(feature = "strum")]
use strum_macros::{Display, EnumString, EnumVariantNames};

#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};

#[cfg(feature = "clap")]
use clap::Args;

use embedded_graphics::mono_font::{ascii::FONT_6X9, MonoFont};
use embedded_vintage_fonts::{FONT_6X8, FONT_6X12, FONT_8X16, FONT_12X16, FONT_24X32};

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RenderTemplate {
    pub ops: Vec<Op>,
}


/// Render operations, used to construct an image
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag="kind", rename_all="snake_case"))]
pub enum Op {
    Text {
        text: String,
        #[cfg_attr(feature = "serde", serde(flatten))]
        opts: TextOptions
    },
    Pad{
        count: usize
    },
    Qr{
        code: String
    },
    DataMatrix{
        code: String
    },
    Barcode{
        code: String,
        #[cfg_attr(feature = "serde", serde(flatten, default))]
        opts: BarcodeOptions
    },
    Image{
        file: String,
        #[cfg_attr(feature = "serde", serde(flatten, default))]
        opts: ImageOptions
    },
}

impl Op {
    pub fn text(s: &str) -> Self {
        Self::Text {
            text: s.to_string(),
            opts: TextOptions::default(),
        }
    }

    pub fn text_with_font(s: &str, f: FontKind) -> Self {
        Self::Text {
            text: s.to_string(),
            opts: TextOptions{
                font: f,
                ..Default::default()
            },
        }
    }

    pub fn pad(columns: usize) -> Self {
        Self::Pad{ count: columns }
    }

    pub fn qr(code: &str) -> Self {
        Self::Qr{ code: code.to_string() }
    }

    pub fn datamatrix(code: &str) -> Self {
        Self::DataMatrix{ code: code.to_string() }
    }

    pub fn barcode(code: &str) -> Self {
        Self::Barcode{
            code: code.to_string(), 
            opts: BarcodeOptions::default(),
        }
    }

    pub fn image(file: &str) -> Self {
        Self::Image {
            file: file.to_string(),
            opts: ImageOptions::default(),
        }
    }
}


#[derive(Copy, Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "strum", derive(Display, EnumString, EnumVariantNames))]
#[cfg_attr(feature = "serde", serde(rename_all="snake_case"))]
pub enum FontKind {
    #[cfg_attr(feature = "strum", strum(serialize = "6x6"))]
    Font6x6,
    #[cfg_attr(feature = "strum", strum(serialize = "6x8"))]
    Font6x8,
    #[cfg_attr(feature = "strum", strum(serialize = "6x12"))]
    Font6x12,
    #[cfg_attr(feature = "strum", strum(serialize = "8x16"))]
    Font8x16,
    #[cfg_attr(feature = "strum", strum(serialize = "12x16"))]
    Font12x16,
    #[cfg_attr(feature = "strum", strum(serialize = "24x32"))]
    Font24x32,
}

impl FontKind {
    /// Fetch the embedded-graphics MonoFont matching this font kind.
    ///
    /// `Font6x8`/`Font6x12`/`Font8x16`/`Font12x16`/`Font24x32` use embedded-vintage-fonts,
    /// which republishes the exact bitmaps embedded-graphics 0.6 used to ship. There is no
    /// 6x6 equivalent anywhere, so `Font6x6` falls back to the closest built-in font.
    pub fn font(&self) -> &'static MonoFont<'static> {
        match self {
            FontKind::Font6x6 => &FONT_6X9,
            FontKind::Font6x8 => &FONT_6X8,
            FontKind::Font6x12 => &FONT_6X12,
            FontKind::Font8x16 => &FONT_8X16,
            FontKind::Font12x16 => &FONT_12X16,
            FontKind::Font24x32 => &FONT_24X32,
        }
    }

    pub fn char_width(&self) -> usize {
        self.font().character_size.width as usize
    }

    pub fn char_height(&self) -> usize {
        self.font().character_size.height as usize
    }
}

#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
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
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "clap", derive(Args))]
pub struct BarcodeOptions {
    #[cfg_attr(feature = "clap", arg(default_value="4"))]
    /// Y offset from top and bottom of label
    pub y_offset: usize,

    #[cfg_attr(feature = "clap", arg(long))]
    /// Double barcode width
    pub double: bool,
}

impl Default for BarcodeOptions {
    fn default() -> Self {
        Self {
            y_offset: 4,
            double: false,
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "clap", derive(Args))]
pub struct ImageOptions {
    // TODO: scaling, invert, etc...
}

impl Default for ImageOptions {
    fn default() -> Self {
        Self {}
    }
}
