//! Render operations
// Rust PTouch Driver / Utility
//
// https://github.com/ryankurte/rust-ptouch
// Copyright 2021 Ryan Kurte

use embedded_graphics::mono_font::MonoFont;
#[cfg(feature = "strum")]
use strum_macros::{Display, EnumString, VariantNames};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "clap")]
use clap::Args;

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RenderTemplate {
    pub ops: Vec<Op>,
}

/// Render operations, used to construct an image
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "kind", rename_all = "snake_case"))]
pub enum Op {
    Text {
        text: String,
        #[cfg_attr(feature = "serde", serde(flatten))]
        opts: TextOptions,
    },
    Pad {
        count: usize,
    },
    Qr {
        code: String,
    },
    DataMatrix {
        code: String,
    },
    Barcode {
        code: String,
        #[cfg_attr(feature = "serde", serde(flatten, default))]
        opts: BarcodeOptions,
    },
    Image {
        file: String,
        #[cfg_attr(feature = "serde", serde(flatten, default))]
        opts: ImageOptions,
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
            opts: TextOptions {
                font: f,
                ..Default::default()
            },
        }
    }

    pub fn pad(columns: usize) -> Self {
        Self::Pad { count: columns }
    }

    pub fn qr(code: &str) -> Self {
        Self::Qr {
            code: code.to_string(),
        }
    }

    pub fn datamatrix(code: &str) -> Self {
        Self::DataMatrix {
            code: code.to_string(),
        }
    }

    pub fn barcode(code: &str) -> Self {
        Self::Barcode {
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
#[cfg_attr(feature = "strum", derive(Display, EnumString, VariantNames))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum FontKind {
    #[cfg_attr(feature = "strum", strum(serialize = "4x6"))]
    Font4x6,
    #[cfg_attr(feature = "strum", strum(serialize = "6x9"))]
    Font6x9,
    #[cfg_attr(feature = "strum", strum(serialize = "6x10"))]
    Font6x10,
    #[cfg_attr(feature = "strum", strum(serialize = "6x12"))]
    Font6x12,
    #[cfg_attr(feature = "strum", strum(serialize = "8x13"))]
    Font8x13,
    #[cfg_attr(feature = "strum", strum(serialize = "9x15"))]
    Font9x15,
    #[cfg_attr(feature = "strum", strum(serialize = "10x20"))]
    Font10x20,
}

impl AsRef<MonoFont<'static>> for FontKind {
    fn as_ref(&self) -> &MonoFont<'static> {
        use embedded_graphics_unicodefonts::*;

        match self {
            FontKind::Font4x6 => &MONO_4X6,
            FontKind::Font6x9 => &MONO_6X9,
            FontKind::Font6x10 => &MONO_6X10,
            FontKind::Font6x12 => &MONO_6X12,
            FontKind::Font8x13 => &MONO_8X13,
            FontKind::Font9x15 => &MONO_9X15,
            FontKind::Font10x20 => &MONO_10X20,
        }
    }
}

impl FontKind {
    pub fn char_width(&self) -> usize {
        self.as_ref().character_size.width as usize
    }

    pub fn char_height(&self) -> usize {
        self.as_ref().character_size.height as usize
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
#[cfg_attr(feature = "strum", derive(EnumString, VariantNames))]
pub enum HAlign {
    Left,
    Centre,
    Right,
}

#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "strum", derive(EnumString, VariantNames))]
pub enum VAlign {
    Top,
    Centre,
    Bottom,
}

impl Default for TextOptions {
    fn default() -> Self {
        Self {
            font: FontKind::Font10x20,
            h_align: HAlign::Centre,
            v_align: VAlign::Centre,
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "clap", derive(Args))]
pub struct BarcodeOptions {
    #[cfg_attr(feature = "clap", arg(default_value = "4"))]
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
