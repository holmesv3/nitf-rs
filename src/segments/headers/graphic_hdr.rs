//! Graphic segment subheader definition
use std::fmt::Display;
use std::io::{Read, Seek};
use std::str::FromStr;

use crate::error::NitfError;
use crate::segments::headers::NitfSegmentHeader;
use crate::types::{NitfField, Security};

/// Header fields for Graphic Segment
#[derive(Default, Clone, Hash, Debug)]
pub struct GraphicHeader {
    /// File Part Type
    pub sy: NitfField<String>,
    /// Graphic Identifier
    pub sid: NitfField<String>,
    /// Graphic Name
    pub sname: NitfField<String>,
    /// Security information
    pub security: Security,
    /// Encryption
    pub encryp: NitfField<String>,
    /// Graphic Type
    pub sfmt: NitfField<Format>,
    /// Reserved for Future Use
    pub sstruct: NitfField<u64>,
    /// Graphic Display Level
    pub sdlvl: NitfField<u16>,
    /// Graphic Attachment Level
    pub salvl: NitfField<u16>,
    /// Graphic Location
    pub sloc: NitfField<String>, // TODO: Same image image ILOC type thing
    /// First Graphic Bound Location
    pub sbnd1: NitfField<BoundLocation>,
    /// Graphic Color
    pub scolor: NitfField<Color>,
    /// Second Graphic Bound Location
    pub sbnd2: NitfField<BoundLocation>,
    /// Reserved for Future Use
    pub sres2: NitfField<u8>,
    /// Graphic Extended Subheader Data Length
    pub sxshdl: NitfField<u16>,
    /// Graphic Extended Subheader Overflow
    pub sxsofl: NitfField<u16>,
    /// Graphic Extended Subheader Data
    pub sxshd: NitfField<String>,
}

impl Display for GraphicHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out_str = String::default();
        out_str += format!("SY: {}, ", self.sy).as_ref();
        out_str += format!("SID: {}, ", self.sid).as_ref();
        out_str += format!("SNAME: {}, ", self.sname).as_ref();
        out_str += format!("SECURITY: [{}], ", self.security).as_ref();
        out_str += format!("ENCRYP: {}, ", self.encryp).as_ref();
        out_str += format!("SFMT: {}, ", self.sfmt).as_ref();
        out_str += format!("SSTRUCT: {}, ", self.sstruct).as_ref();
        out_str += format!("SDLVL: {}, ", self.sdlvl).as_ref();
        out_str += format!("SALVL: {}, ", self.salvl).as_ref();
        out_str += format!("SLOC: {}, ", self.sloc).as_ref();
        out_str += format!("SBND1: {}, ", self.sbnd1).as_ref();
        out_str += format!("SCOLOR: {}, ", self.scolor).as_ref();
        out_str += format!("SBND2: {}, ", self.sbnd2).as_ref();
        out_str += format!("SRES2: {}, ", self.sres2).as_ref();
        out_str += format!("SXSHDL: {}, ", self.sxshdl).as_ref();
        out_str += format!("SXSOFL: {}, ", self.sxsofl).as_ref();
        out_str += format!("[SXSHD: {}]", self.sxshd).as_ref();
        write!(f, "[Graphic Subheader: {}]", out_str)
    }
}
impl NitfSegmentHeader for GraphicHeader {
    fn read(&mut self, reader: &mut (impl Read + Seek)) {
        self.sy.read(reader, 2u8);
        self.sid.read(reader, 10u8);
        self.sname.read(reader, 20u8);
        self.security.read(reader);
        self.encryp.read(reader, 1u8);
        self.sfmt.read(reader, 1u8);
        self.sstruct.read(reader, 13u8);
        self.sdlvl.read(reader, 3u8);
        self.salvl.read(reader, 3u8);
        self.sloc.read(reader, 10u8);
        self.sbnd1.read(reader, 10u8);
        self.scolor.read(reader, 1u8);
        self.sbnd2.read(reader, 10u8);
        self.sres2.read(reader, 2u8);
        self.sxshdl.read(reader, 5u8);
        let gphx_data_length = self.sxshdl.val;
        if gphx_data_length != 0 {
            self.sxsofl.read(reader, 3u8);
            self.sxshd.read(reader, gphx_data_length - 3);
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
    type Err = NitfError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "C" => Ok(Self::C),
            _ => Err(NitfError::FieldError),
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
    type Err = NitfError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let n_char_tot = s.len();
        if n_char_tot % 2 == 0 {
            let mut bounds = Self::default();
            let n_char = n_char_tot / 2;
            bounds.row = s[..n_char].parse().unwrap();
            bounds.col = s[n_char..].parse().unwrap();
            Ok(bounds)
        } else {
            Err(NitfError::FieldError)
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
    type Err = NitfError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "C" => Ok(Self::C),
            "M" => Ok(Self::M),
            _ => Err(NitfError::FieldError),
        }
    }
}
