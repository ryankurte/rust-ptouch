use bitflags::bitflags;
use strum_macros::{EnumString, ToString};

bitflags::bitflags! {
    pub struct Error1: u8 {
        const NO_MEDIA = 0x01;
        const CUTTER_JAM = 0x04;
        const WEAK_BATT = 0x08;
        const HIGH_VOLT = 0x40;
    }
}

bitflags::bitflags! {
    pub struct Error2: u8 {
        const WRONG_MEDIA = 0x01;
        const COVER_OPEN = 0x10;
        const OVERHEAT = 0x20;
    }
}
#[derive(Copy, Clone, PartialEq, Debug, EnumString, ToString)]
pub enum PTouchDevice {
    #[strum(serialize = "pt-e550w")]
    PtE550W = 0x2060,
    #[strum(serialize = "pt-p750w")]
    PtP750W = 0x2062,
    #[strum(serialize = "pt-p710bt")]
    PtP710Bt = 0x20af,
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

    /// Unknown media width
    Unknown = 0xFFFF,
}

impl From<(MediaKind, u8)> for MediaWidth {
    fn from(v: (MediaKind, u8)) -> Self {
        use MediaKind::*;
        use MediaWidth::*;

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

impl MediaWidth {
    /// Fetch media area (left margin, print area, right margin)
    pub fn area(&self) -> (u8, u8, u8) {
        use MediaWidth::*;

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

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum CompressionMode {
    None = 0x00,
    Tiff = 0x02,
}
