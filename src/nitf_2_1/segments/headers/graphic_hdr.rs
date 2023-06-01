//! Graphic segment subheader definition
use std::fmt::Display;
use std::io::{Read, Seek};
use std::str::FromStr;

use crate::nitf_2_1::segments::headers::NitfSegmentHeader;
use crate::nitf_2_1::types::field::{InvalidNitfValue, NitfField};
use crate::nitf_2_1::types::security::Security;

/// Header fields for Graphic Segment
#[allow(non_snake_case)]
#[derive(Default, Clone, Hash, Debug)]
pub struct GraphicHeader {
    /// File Part Type
    pub SY: NitfField<String>,
    /// Graphic Identifier
    pub SID: NitfField<String>,
    /// Graphic Name
    pub SNAME: NitfField<String>,
    /// Security information
    pub SECURITY: Security,
    /// Encryption
    pub ENCRYP: NitfField<String>,
    /// Graphic Type
    pub SFMT: NitfField<Format>,
    /// Reserved for Future Use
    pub SSTRUCT: NitfField<u64>,
    /// Graphic Display Level
    pub SDLVL: NitfField<u16>,
    /// Graphic Attachment Level
    pub SALVL: NitfField<u16>,
    /// Graphic Location
    pub SLOC: NitfField<String>, // TODO: Same image image ILOC type thing
    /// First Graphic Bound Location
    pub SBND1: NitfField<BoundLocation>,
    /// Graphic Color
    pub SCOLOR: NitfField<Color>,
    /// Second Graphic Bound Location
    pub SBND2: NitfField<BoundLocation>,
    /// Reserved for Future Use
    pub SRES2: NitfField<u8>,
    /// Graphic Extended Subheader Data Length
    pub SXSHDL: NitfField<u16>,
    /// Graphic Extended Subheader Overflow
    pub SXSOFL: NitfField<u16>,
    /// Graphic Extended Subheader Data
    pub SXSHD: NitfField<String>,
}

impl Display for GraphicHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out_str = String::default();
        out_str += format!("SY: {}, ", self.SY).as_ref();
        out_str += format!("SID: {}, ", self.SID).as_ref();
        out_str += format!("SNAME: {}, ", self.SNAME).as_ref();
        out_str += format!("SECURITY: [{}], ", self.SECURITY).as_ref();
        out_str += format!("ENCRYP: {}, ", self.ENCRYP).as_ref();
        out_str += format!("SFMT: {}, ", self.SFMT).as_ref();
        out_str += format!("SSTRUCT: {}, ", self.SSTRUCT).as_ref();
        out_str += format!("SDLVL: {}, ", self.SDLVL).as_ref();
        out_str += format!("SALVL: {}, ", self.SALVL).as_ref();
        out_str += format!("SLOC: {}, ", self.SLOC).as_ref();
        out_str += format!("SBND1: {}, ", self.SBND1).as_ref();
        out_str += format!("SCOLOR: {}, ", self.SCOLOR).as_ref();
        out_str += format!("SBND2: {}, ", self.SBND2).as_ref();
        out_str += format!("SRES2: {}, ", self.SRES2).as_ref();
        out_str += format!("SXSHDL: {}, ", self.SXSHDL).as_ref();
        out_str += format!("SXSOFL: {}, ", self.SXSOFL).as_ref();
        out_str += format!("[SXSHD: {}]", self.SXSHD).as_ref();
        write!(f, "[Graphic Subheader: {}]", out_str)
    }
}
impl NitfSegmentHeader for GraphicHeader {
    fn read(&mut self, reader: &mut (impl Read + Seek)) {
        self.SY.read(reader, 2u8);
        self.SID.read(reader, 10u8);
        self.SNAME.read(reader, 20u8);
        self.SECURITY.read(reader);
        self.ENCRYP.read(reader, 1u8);
        self.SFMT.read(reader, 1u8);
        self.SSTRUCT.read(reader, 13u8);
        self.SDLVL.read(reader, 3u8);
        self.SALVL.read(reader, 3u8);
        self.SLOC.read(reader, 10u8);
        self.SBND1.read(reader, 10u8);
        self.SCOLOR.read(reader, 1u8);
        self.SBND2.read(reader, 10u8);
        self.SRES2.read(reader, 2u8);
        self.SXSHDL.read(reader, 5u8);
        let gphx_data_length: u32 = self.SXSHDL.string.parse().unwrap();
        if gphx_data_length != 0 {
            self.SXSOFL.read(reader, 3u8);
            self.SXSHD.read(reader, gphx_data_length - 3);
        }
    }
}

/// Graphic type. Right now standard only supports C
#[derive(Debug, Default, Hash, Clone)]
pub enum Format {
    #[default]
    /// Computer graphics metafile
    C,
}

impl FromStr for Format {
    type Err = InvalidNitfValue;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "C" => Ok(Self::C),
            _ => Err(InvalidNitfValue),
        }
    }
}

/// Graphic bound position relative to origin of coordinate system
#[derive(Debug, Default, Clone, Hash)]
pub struct BoundLocation {
    pub row: i32,
    pub col: i32,
}

impl FromStr for BoundLocation {
    type Err = InvalidNitfValue;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let n_char_tot = s.len();
        if n_char_tot % 2 == 0 {
            let mut bounds = Self::default();
            let n_char = n_char_tot / 2;
            bounds.row = s[..n_char].parse().unwrap();
            bounds.col = s[n_char..].parse().unwrap();
            return Ok(bounds);
        } else {
            return Err(InvalidNitfValue);
        }
    }
}

/// Color type of graphics
#[derive(Debug, Default, Hash, Clone)]
pub enum Color {
    #[default]
    /// Color pieces
    C,
    /// Monochrome
    M,
}

impl FromStr for Color {
    type Err = InvalidNitfValue;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "C" => Ok(Self::C),
            "M" => Ok(Self::M),
            _ => Err(InvalidNitfValue),
        }
    }
}
