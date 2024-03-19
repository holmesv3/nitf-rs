//! Graphic segment subheader definition
use std::fmt::Display;
use std::io::{Read, Seek, Write};
use std::str::FromStr;

use crate::headers::NitfSegmentHeader;
use crate::types::{ExtendedSubheader, NitfField, Security};
use crate::{NitfError, NitfResult};
/// Header fields for Graphic Segment
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct GraphicHeader {
    /// File Part Type
    pub sy: NitfField<SY>,
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
    pub sloc: NitfField<BoundLocation>, // TODO: Same image image ILOC type thing
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
    pub sxshd: ExtendedSubheader,
}
impl Default for GraphicHeader {
    fn default() -> Self {
        Self {
            sy: NitfField::init(2u8, "SY"),
            sid: NitfField::init(10u8, "SID"),
            sname: NitfField::init(20u8, "SNAME"),
            security: Security::default(),
            encryp: NitfField::init(1u8, "ENCRYP"),
            sfmt: NitfField::init(1u8, "SFMT"),
            sstruct: NitfField::init(13u8, "SSTRUCT"),
            sdlvl: NitfField::init(3u8, "SDLVL"),
            salvl: NitfField::init(3u8, "SALVL"),
            sloc: NitfField::init(10u8, "SLOC"),
            sbnd1: NitfField::init(10u8, "SBND1"),
            scolor: NitfField::init(1u8, "SCOLOR"),
            sbnd2: NitfField::init(10u8, "SBND2"),
            sres2: NitfField::init(2u8, "SRES2"),
            sxshdl: NitfField::init(5u8, "SXSHDL"),
            sxsofl: NitfField::init(3u8, "SXSOFL"),
            sxshd: ExtendedSubheader::init("SXSHD"),
        }
    }
}
#[derive(Default, Clone, Debug, Eq, PartialEq, Copy, Ord, PartialOrd)]
pub enum SY {
    #[default]
    SY,
}
impl FromStr for SY {
    type Err = NitfError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "SY" => Ok(Self::default()),
            _ => Err(NitfError::ParseError("SY".to_string())),
        }
    }
}
impl Display for SY {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SY")
    }
}

impl Display for GraphicHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out_str = String::default();
        out_str += format!("{}, ", self.sy).as_ref();
        out_str += format!("{}, ", self.sid).as_ref();
        out_str += format!("{}, ", self.sname).as_ref();
        out_str += format!("SECURITY: [{}], ", self.security).as_ref();
        out_str += format!("{}, ", self.encryp).as_ref();
        out_str += format!("{}, ", self.sfmt).as_ref();
        out_str += format!("{}, ", self.sstruct).as_ref();
        out_str += format!("{}, ", self.sdlvl).as_ref();
        out_str += format!("{}, ", self.salvl).as_ref();
        out_str += format!("{}, ", self.sloc).as_ref();
        out_str += format!("{}, ", self.sbnd1).as_ref();
        out_str += format!("{}, ", self.scolor).as_ref();
        out_str += format!("{}, ", self.sbnd2).as_ref();
        out_str += format!("{}, ", self.sres2).as_ref();
        out_str += format!("{}, ", self.sxshdl).as_ref();
        out_str += format!("{}, ", self.sxsofl).as_ref();
        out_str += format!("{}", self.sxshd).as_ref();
        write!(f, "Graphic Header: [{out_str}]")
    }
}
impl NitfSegmentHeader for GraphicHeader {
    fn read(&mut self, reader: &mut (impl Read + Seek)) -> NitfResult<()> {
        self.sy.read(reader)?;
        self.sid.read(reader)?;
        self.sname.read(reader)?;
        self.security.read(reader)?;
        self.encryp.read(reader)?;
        self.sfmt.read(reader)?;
        self.sstruct.read(reader)?;
        self.sdlvl.read(reader)?;
        self.salvl.read(reader)?;
        self.sloc.read(reader)?;
        self.sbnd1.read(reader)?;
        self.scolor.read(reader)?;
        self.sbnd2.read(reader)?;
        self.sres2.read(reader)?;
        self.sxshdl.read(reader)?;
        if self.sxshdl.val != 0 {
            self.sxsofl.read(reader)?;
            self.sxshd.read(reader, (self.sxshdl.val - 3) as usize)?;
        }
        Ok(())
    }
    fn write(&self, writer: &mut (impl Write + Seek)) -> NitfResult<usize> {
        let mut bytes_written = 0;
        bytes_written += self.sy.write(writer)?;
        bytes_written += self.sid.write(writer)?;
        bytes_written += self.sname.write(writer)?;
        bytes_written += self.security.write(writer)?;
        bytes_written += self.encryp.write(writer)?;
        bytes_written += self.sfmt.write(writer)?;
        bytes_written += self.sstruct.write(writer)?;
        bytes_written += self.sdlvl.write(writer)?;
        bytes_written += self.salvl.write(writer)?;
        bytes_written += self.sloc.write(writer)?;
        bytes_written += self.sbnd1.write(writer)?;
        bytes_written += self.scolor.write(writer)?;
        bytes_written += self.sbnd2.write(writer)?;
        bytes_written += self.sres2.write(writer)?;
        bytes_written += self.sxshdl.write(writer)?;
        if self.sxshdl.val != 0 {
            bytes_written += self.sxsofl.write(writer)?;
            bytes_written += self.sxshd.write(writer)?;
        }
        Ok(bytes_written)
    }
    fn length(&self) -> usize {
        let mut length = 0;
        length += self.sy.length;
        length += self.sid.length;
        length += self.sname.length;
        length += self.security.length();
        length += self.encryp.length;
        length += self.sfmt.length;
        length += self.sstruct.length;
        length += self.sdlvl.length;
        length += self.salvl.length;
        length += self.sloc.length;
        length += self.sbnd1.length;
        length += self.scolor.length;
        length += self.sbnd2.length;
        length += self.sres2.length;
        length += self.sxshdl.length;
        if self.sxshdl.val != 0 {
            length += self.sxsofl.length;
            length += self.sxshd.size();
        }
        length
    }
}

/// Graphic type. Right now standard only supports C
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
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
            _ => Err(NitfError::ParseError("Format".to_string())),
        }
    }
}
impl Display for Format {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::C => write!(f, "C"),
        }
    }
}

/// Graphic bound position relative to origin of coordinate system
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct BoundLocation {
    pub row: i32,
    pub col: i32,
}

impl FromStr for BoundLocation {
    type Err = NitfError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() == 10 {
            let bounds = Self {
                row: s[..5]
                    .parse()
                    .or(Err(NitfError::ParseError("BoundLocation.row".to_string())))?,
                col: s[5..]
                    .parse()
                    .or(Err(NitfError::ParseError("BoundLocation.col".to_string())))?,
            };
            Ok(bounds)
        } else {
            Err(NitfError::ParseError("BoundLocation".to_string()))
        }
    }
}
impl Display for BoundLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:0<5}{:0<5}", self.row, self.col)
    }
}
/// Color type of graphics
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
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
            _ => Err(NitfError::ParseError("Color".to_string())),
        }
    }
}
impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::C => write!(f, "C"),
            Self::M => write!(f, "M"),
        }
    }
}
