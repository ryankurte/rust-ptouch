
use bitflags::bitflags;
use strum_macros::{EnumString, ToString};

#[derive(Copy, Clone, PartialEq, Debug, EnumString, ToString)]
pub enum PTouchDevice {
    #[strum(serialize = "pt-e550w")]
    PtE550W = 0x2060,
    #[strum(serialize = "pt-p750w")]
    PtP750W = 0x2062,
    #[strum(serialize = "pt-p710bt")]
    PtP710Bt = 0x20af
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum MediaWidth {
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
}

impl MediaWidth {
    pub fn width(&self) -> u8 {
        use MediaWidth::*;

        match self {
            Tze6mm | Hs6mm => 6,
            Tze9mm | Hs9mm => 9,
            Tze12mm | Hs12mm => 12,
            Tze18mm | Hs18mm => 18,
            Tze24mm | Hs24mm => 24,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum MediaKind {
    None = 0x00,
    LaminatedTape = 0x01,
    NonLaminatedTape = 0x03,
    HeatShrinkTube = 0x11,
    IncompatibleTape = 0xFF,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum DeviceStatus {
    Reply = 0x00,
    Completed = 0x01,
    Error = 0x02,
    ExitIF = 0x03,
    TurnedOff = 0x04,
    Notification = 0x05,
    PhaseChange = 0x06,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Mode {
    EscP = 0x00,
    Raster = 0x01,
    PTouchTemplate = 0x03,
}

bitflags! {
    pub struct VariousMode: u8 {
        const AUTO_CUT = (1 << 6);
        const MIRROR = (1 << 7);
    }
}

bitflags! {
    pub struct AdvancedMode: u8 {
        const HALF_CUT = (1 << 2);
        const NO_CHAIN = (1 << 3);
        const SPECIAL_TAPE = (1 << 4);
        const HIGH_RES = (1 << 6);
        const NO_BUFF_CLEAR = (1 << 7);
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Notification {
    NotAvailable = 0x00,
    CoverOpen = 0x01,
    CoverClosed = 0x02,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum TapeColour {

}

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

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum CompressionMode {
    None = 0x00,
    Tiff = 0x02,
}
