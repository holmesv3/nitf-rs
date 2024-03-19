//! Text segment definition
use std::fmt::Display;
use std::io::{Read, Seek, Write};

use std::str::FromStr;

use crate::headers::NitfSegmentHeader;
use crate::types::{ExtendedSubheader, NitfField, Security};
use crate::{NitfError, NitfResult};
/// Text Segment Metadata
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct TextHeader {
    /// File Part Type
    pub te: NitfField<TE>,
    /// Text Identifier
    pub textid: NitfField<String>,
    /// Text Attachment Level
    pub txtalvl: NitfField<u16>,
    /// Text Date and Time
    pub txtdt: NitfField<String>,
    /// Text Title
    pub txttitl: NitfField<String>,
    /// Security information
    pub security: Security,
    /// Encryption
    pub encryp: NitfField<String>,
    /// Text Format
    pub txtfmt: NitfField<TextFormat>,
    /// Text Extended Subheader Data Length
    pub txshdl: NitfField<u16>,
    /// Text Extended Subheader Overflow
    pub txsofl: NitfField<u16>,
    /// Text Extended Subheader Data
    pub txshd: ExtendedSubheader,
}
impl Default for TextHeader {
    fn default() -> Self {
        Self {
            te: NitfField::init(2u8, "TE"),
            textid: NitfField::init(7u8, "TEXTID"),
            txtalvl: NitfField::init(3u8, "TXTALVL"),
            txtdt: NitfField::init(14u8, "TXTDT"),
            txttitl: NitfField::init(80u8, "TXTTITL"),
            security: Security::default(),
            encryp: NitfField::init(1u8, "ENCRYP"),
            txtfmt: NitfField::init(3u8, "TXTFMT"),
            txshdl: NitfField::init(5u8, "TXSHDL"),
            txsofl: NitfField::init(3u8, "TXSOFL"),
            txshd: ExtendedSubheader::init("TXSHD"),
        }
    }
}

#[derive(Default, Clone, Debug, Eq, PartialEq, Copy, Ord, PartialOrd)]
pub enum TE {
    #[default]
    TE,
}
impl FromStr for TE {
    type Err = NitfError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "TE" => Ok(Self::default()),
            _ => Err(NitfError::ParseError("TE".to_string())),
        }
    }
}
impl Display for TE {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TE")
    }
}

/// Formatting specification
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum TextFormat {
    #[default]
    /// USMTF formatting
    MTF,
    /// BCS formatting
    STA,
    /// ECS formatting
    UT1,
    /// U8S formatting
    U8S,
}

impl NitfSegmentHeader for TextHeader {
    fn read(&mut self, reader: &mut (impl Read + Seek)) -> NitfResult<()> {
        self.te.read(reader)?;
        self.textid.read(reader)?;
        self.txtalvl.read(reader)?;
        self.txtdt.read(reader)?;
        self.txttitl.read(reader)?;
        self.security.read(reader)?;
        self.encryp.read(reader)?;
        self.txtfmt.read(reader)?;
        self.txshdl.read(reader)?;
        if self.txshdl.val != 0 {
            self.txsofl.read(reader)?;
            self.txshd.read(reader, (self.txshdl.val - 3) as usize)?;
        }
        Ok(())
    }
    fn write(&self, writer: &mut (impl Write + Seek)) -> NitfResult<usize> {
        let mut bytes_written = 0;
        bytes_written += self.te.write(writer)?;
        bytes_written += self.textid.write(writer)?;
        bytes_written += self.txtalvl.write(writer)?;
        bytes_written += self.txtdt.write(writer)?;
        bytes_written += self.txttitl.write(writer)?;
        bytes_written += self.security.write(writer)?;
        bytes_written += self.encryp.write(writer)?;
        bytes_written += self.txtfmt.write(writer)?;
        bytes_written += self.txshdl.write(writer)?;
        if self.txshdl.val != 0 {
            bytes_written += self.txsofl.write(writer)?;
            bytes_written += self.txshd.write(writer)?;
        }
        Ok(bytes_written)
    }
    fn length(&self) -> usize {
        let mut length = 0;
        length += self.te.length;
        length += self.textid.length;
        length += self.txtalvl.length;
        length += self.txtdt.length;
        length += self.txttitl.length;
        length += self.security.length();
        length += self.encryp.length;
        length += self.txtfmt.length;
        length += self.txshdl.length;
        if self.txshdl.val != 0 {
            length += self.txsofl.length;
            length += self.txshd.size();
        }
        length
    }
}
impl Display for TextHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out_str = String::default();
        out_str += format!("{}, ", self.te).as_ref();
        out_str += format!("{}, ", self.textid).as_ref();
        out_str += format!("{}, ", self.txtalvl).as_ref();
        out_str += format!("{}, ", self.txtdt).as_ref();
        out_str += format!("{}, ", self.txttitl).as_ref();
        out_str += format!("SECURITY: [{}], ", self.security).as_ref();
        out_str += format!("{}, ", self.encryp).as_ref();
        out_str += format!("{}, ", self.txtfmt).as_ref();
        out_str += format!("{}", self.txshdl).as_ref();
        out_str += format!("{}", self.txsofl).as_ref();
        out_str += format!("{}", self.txshd).as_ref();
        write!(f, "Text Segment: [{out_str}]")
    }
}
impl FromStr for TextFormat {
    type Err = NitfError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "MTF" => Ok(Self::MTF),
            "STA" => Ok(Self::STA),
            "UT1" => Ok(Self::UT1),
            "U8S" => Ok(Self::U8S),
            _ => Err(NitfError::ParseError("TextFormat".to_string())),
        }
    }
}
impl Display for TextFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MTF => write!(f, "MTF"),
            Self::STA => write!(f, "STA"),
            Self::UT1 => write!(f, "UT1"),
            Self::U8S => write!(f, "U8S"),
        }
    }
}
