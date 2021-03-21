//! PTouch printer device definitions
// Rust PTouch Driver / Utility
//
// https://github.com/ryankurte/rust-ptouch
// Copyright 2021 Ryan Kurte

use bitflags::bitflags;

#[cfg(feature = "strum")]
use strum_macros::{Display, EnumString, EnumVariantNames};

bitflags::bitflags! {
    /// First error byte
    pub struct Error1: u8 {
        const NO_MEDIA = 0x01;
        const CUTTER_JAM = 0x04;
        const WEAK_BATT = 0x08;
        const HIGH_VOLT = 0x40;
    }
}

bitflags::bitflags! {
    /// Second device error type
    pub struct Error2: u8 {
        const WRONG_MEDIA = 0x01;
        const COVER_OPEN = 0x10;
        const OVERHEAT = 0x20;
    }
}

/// PTouch device type.
/// Note that only the p710bt has been tested
#[derive(Copy, Clone, PartialEq, Debug)]
#[cfg_attr(feature = "strum", derive(Display, EnumString, EnumVariantNames))]
#[cfg_attr(feature = "strum", strum(serialize_all = "snake_case"))]
pub enum PTouchDevice {
    #[cfg_attr(feature = "strum", strum(serialize = "pt-e550w"))]
    PtE550W = 0x2060,
    #[cfg_attr(feature = "strum", strum(serialize = "pt-p750w"))]
    PtP750W = 0x2062,
    #[cfg_attr(feature = "strum", strum(serialize = "pt-p710bt"))]
    PtP710Bt = 0x20af,
}


/// Media width encoding for Status message
#[derive(Copy, Clone, PartialEq, Debug)]
#[cfg_attr(feature = "strum", derive(Display, EnumString, EnumVariantNames))]
#[cfg_attr(feature = "strum", strum(serialize_all = "snake_case"))]
pub enum Media {
    /// 6mm TZe Tape
    Tze6mm = 257,
    /// 9mm TZe Tape
    Tze9mm = 258,
    /// 12mm TZe Tape
    Tze12mm = 259,
    /// 18mm TZe Tape
    Tze18mm = 260,
    /// 24mm TZe Tape
    Tze24mm = 261,

    /// 6mm HeatShrink Tube
    Hs6mm = 415,
    /// 9mm HeatShrink Tube
    Hs9mm = 416,
    /// 12mm HeatShrink Tube
    Hs12mm = 417,
    /// 18mm HeatShrink Tube
    Hs18mm = 418,
    /// 24mm HeatShrink Tube
    Hs24mm = 419,

    /// Unknown media width
    Unknown = 0xFFFF,
}

/// Generate a MediaWidth from provided MediaKind and u8 width
impl From<(MediaKind, u8)> for Media {
    fn from(v: (MediaKind, u8)) -> Self {
        use MediaKind::*;
        use Media::*;

        match v {
            (LaminatedTape, 6) | (NonLaminatedTape, 6) => Tze6mm,
            (LaminatedTape, 9) | (NonLaminatedTape, 9) => Tze9mm,
            (LaminatedTape, 12) | (NonLaminatedTape, 12) => Tze12mm,
            (LaminatedTape, 18) | (NonLaminatedTape, 18) => Tze18mm,
            (LaminatedTape, 24) | (NonLaminatedTape, 24) => Tze24mm,
            (HeatShrinkTube, 6) => Hs6mm,
            (HeatShrinkTube, 9) => Hs9mm,
            (HeatShrinkTube, 12)  => Hs12mm,
            (HeatShrinkTube, 18)  => Hs18mm,
            (HeatShrinkTube, 24)  => Hs24mm,
            _ => Unknown,
        }
    }
}

impl Media {
    /// Fetch media print area (left margin, print area, right margin)
    pub fn area(&self) -> (usize, usize, usize) {
        use Media::*;

        match self {
            Tze6mm => (52, 32, 52),
            Tze9mm => (39, 50, 39),
            Tze12mm => (29, 70, 29),
            Tze18mm => (8, 112, 8),
            Tze24mm => (0, 128, 0),

            Hs6mm => (50, 28, 50),
            Hs9mm => (40, 48, 40),
            Hs12mm => (31, 66, 31),
            Hs18mm => (11, 106, 11),
            Hs24mm => (0, 128, 0),

            Unknown => (0, 0, 0)
        }
    }

    /// Check if a media type is _tape_
    pub fn is_tape(&self) -> bool {
        use Media::*;

        match self {
            Tze6mm | Tze9mm | Tze12mm | Tze18mm | Tze24mm => true,
            _ => false,
        }
    }

    /// Fetch the (approximate) media width in mm
    pub fn width(&self) -> usize {
        use Media::*;

        match self {
            Tze6mm => 6,
            Tze9mm => 9,
            Tze12mm => 2,
            Tze18mm => 8,
            Tze24mm => 4,
            Hs6mm => 6,
            Hs9mm => 9,
            Hs12mm => 12,
            Hs18mm => 18,
            Hs24mm => 24,
            _ => panic!("Unknown media width"),
        }
    }
}

/// Kind of media loaded in printer
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum MediaKind {
    None = 0x00,
    LaminatedTape = 0x01,
    NonLaminatedTape = 0x03,
    HeatShrinkTube = 0x11,
    IncompatibleTape = 0xFF,
}

/// Device operating phase
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Phase {
    Editing,
    Printing,
    Unknown,
}

impl From<u8> for Phase {
    fn from(v: u8) -> Self {
        use Phase::*;

        match v {
            0 => Editing,
            1 => Printing,
            _ => Unknown
        }
    }
}

/// Create media kind from status values
impl From<u8> for MediaKind {
    fn from(v: u8) -> Self {
        match v {
           0x00 => MediaKind::None,
           0x01 => MediaKind::LaminatedTape,
           0x03 => MediaKind::NonLaminatedTape,
           0x11 => MediaKind::HeatShrinkTube,
           0xFF => MediaKind::IncompatibleTape,
           _ => MediaKind::IncompatibleTape,
       }
    }
}

/// Device state enumeration
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum DeviceStatus {
    Reply = 0x00,
    Completed = 0x01,
    Error = 0x02,
    ExitIF = 0x03,
    TurnedOff = 0x04,
    Notification = 0x05,
    PhaseChange = 0x06,

    Unknown = 0xFF,
}

impl From<u8> for DeviceStatus {
    fn from(v: u8) -> Self {
        use DeviceStatus::*;

        match v {
            0x00 => Reply,
            0x01 => Completed,
            0x02 => Error,
            0x03 => ExitIF,
            0x04 => TurnedOff,
            0x05 => Notification,
            0x06 => PhaseChange,
            _ => Unknown,
       }
    }
}

/// Device mode for set_mode command
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Mode {
    /// Not sure tbqh?
    EscP = 0x00,
    /// Raster mode, what this driver uses
    Raster = 0x01,
    /// Note PTouchTemplate is not available on most devices
    PTouchTemplate = 0x03,
}

bitflags! {
    /// Various mode flags
    pub struct VariousMode: u8 {
        const AUTO_CUT = (1 << 6);
        const MIRROR = (1 << 7);
    }
}

bitflags! {
    /// Advanced mode flags
    pub struct AdvancedMode: u8 {
        const HALF_CUT = (1 << 2);
        const NO_CHAIN = (1 << 3);
        const SPECIAL_TAPE = (1 << 4);
        const HIGH_RES = (1 << 6);
        const NO_BUFF_CLEAR = (1 << 7);
    }
}

/// Notification enumerations
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Notification {
    NotAvailable = 0x00,
    CoverOpen = 0x01,
    CoverClosed = 0x02,
}

/// Tape colour enumerations
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum TapeColour {
    White = 0x01,
    Other = 0x02,
    ClearBlack = 0x03,
    Red = 0x04,
    Blue = 0x05,
    Black = 0x08,
    ClearWhite = 0x09,
    MatteWhite = 0x20,
    MatteClear = 0x21,
    MatteSilver = 0x22,
    SatinGold = 0x23,
    SatinSilver = 0x24,
    BlueD = 0x30,
    RedD = 0x31,
    FluroOrange=0x40,
    FluroYellow=0x41,
    BerryPinkS = 0x50,
    LightGrayS = 0x51,
    LimeGreenS = 0x52,
    YellowF = 0x60,
    PinkF = 0x61,
    BlueF = 0x62,
    WhiteHst = 0x70,
    WhiteFlexId = 0x90,
    YellowFlexId = 0x91,
    Cleaning = 0xF0,
    Stencil = 0xF1,
    Incompatible = 0xFF,
}

impl From<u8> for TapeColour {
    fn from(v: u8) -> TapeColour {
        use TapeColour::*;

        match v {
            0x01 => White,
            0x02 => Other,
            0x03 => ClearBlack,
            0x04 => Red,
            0x05 => Blue,
            0x08 => Black,
            0x09 => ClearWhite,
            0x20 => MatteWhite,
            0x21 => MatteClear,
            0x22 => MatteSilver,
            0x23 => SatinGold,
            0x24 => SatinSilver,
            0x30 => BlueD,
            0x31 => RedD,
            0x40 => FluroOrange,
            0x41 => FluroYellow,
            0x50 => BerryPinkS,
            0x51 => LightGrayS,
            0x52 => LimeGreenS,
            0x60 => YellowF,
            0x61 => PinkF,
            0x62 => BlueF,
            0x70 => WhiteHst,
            0x90 => WhiteFlexId,
            0x91 => YellowFlexId,
            0xF0 => Cleaning,
            0xF1 => Stencil,
            0xFF | _ => Incompatible,
        }
    }
}

/// Text colour enumerations
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum TextColour {
    White = 0x01,
    Red = 0x04,
    Blue = 0x05,
    Black = 0x08,
    Gold = 0x0A,
    BlueF = 0x62,
    Cleaning = 0xf0,
    Stencil = 0xF1,
    Other = 0x02,
    Incompatible = 0xFF,
}

impl From<u8> for TextColour {
    fn from(v: u8) -> TextColour {
        use TextColour::*;
        
        match v {
            0x01 => White,
            0x04 => Red,
            0x05 => Blue,
            0x08 => Black,
            0x0A => Gold,
            0x62 => BlueF,
            0xf0 => Cleaning,
            0xF1 => Stencil,
            0x02 => Other,
            0xFF | _=> Incompatible,
        }
    }
}

/// Device status message
#[derive(Clone, PartialEq, Debug)]
pub struct Status {
    pub model: u8,

    pub error1: Error1,
    pub error2: Error2,

    pub media_width: u8,
    pub media_kind: MediaKind,

    pub status_type: DeviceStatus,
    pub phase: Phase,

    pub tape_colour: TapeColour,
    pub text_colour: TextColour,
}

impl From<[u8; 32]> for Status {

    fn from(r: [u8; 32]) -> Self {
        Self {
            model: r[0],
            error1: Error1::from_bits_truncate(r[8]),
            error2: Error2::from_bits_truncate(r[9]),
            media_width: r[10],
            media_kind: MediaKind::from(r[11]),

            status_type: DeviceStatus::from(r[18]),
            phase: Phase::from(r[20]),
            tape_colour: TapeColour::from(r[24]),
            text_colour: TextColour::from(r[25]),
        }
    }
}

/// Print information command
#[derive(Clone, PartialEq, Debug)]
pub struct PrintInfo {
    /// Media kind
    pub kind: Option<MediaKind>,
    /// Tape width in mm
    pub width: Option<u8>,
    /// Tape length, always set to 0
    pub length: Option<u8>,
    /// Raster number (??)
    pub raster_no: u32,
    /// Enable print recovery
    pub recover: bool,
}

impl Default for PrintInfo {
    fn default() -> Self {
        Self {
            kind: None,
            width: None,
            length: Some(0),
            raster_no: 0,
            recover: true,
        }
    }
}

/// Compression mode enumeration
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum CompressionMode {
    None = 0x00,
    Tiff = 0x02,
}
