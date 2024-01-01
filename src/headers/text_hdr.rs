//! Text segment definition
use std::fmt::Display;
use std::fs::File;

use std::str::FromStr;

use crate::headers::NitfSegmentHeader;
use crate::types::{ExtendedSubheader, NitfField, Security};
use crate::{NitfError, NitfResult};
/// Text Segment Metadata
#[derive(Default, Clone, Debug, Eq, PartialEq)]
pub struct TextHeader {
    /// File Part Type
    pub te: NitfField<String>,
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

/// Formatting specification
#[derive(Debug, Default, Clone, Eq, PartialEq)]
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
    fn read(&mut self, reader: &mut File) -> NitfResult<()> {
        self.te.read(reader, 2u8, "TE")?;
        self.textid.read(reader, 7u8, "TEXTID")?;
        self.txtalvl.read(reader, 3u8, "TXTALVL")?;
        self.txtdt.read(reader, 14u8, "TXTDT")?;
        self.txttitl.read(reader, 80u8, "TXTTITL")?;
        self.security.read(reader)?;
        self.encryp.read(reader, 1u8, "ENCRYP")?;
        self.txtfmt.read(reader, 3u8, "TXTFMT")?;
        self.txshdl.read(reader, 5u8, "TXSHDL")?;
        let extended_length = self.txshdl.val().clone();
        if extended_length != 0 {
            self.txsofl.read(reader, 3u8, "TXSOFL")?;
            self.txshd
                .read(reader, (extended_length - 3) as usize, "TXSHD")?;
        }
        Ok(())
    }
}
impl Display for TextHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out_str = String::default();
        out_str += format!("TE: {}, ", self.te).as_ref();
        out_str += format!("TEXTID: {}, ", self.textid).as_ref();
        out_str += format!("TXTALVL: {}, ", self.txtalvl).as_ref();
        out_str += format!("TXTDT: {}, ", self.txtdt).as_ref();
        out_str += format!("TXTTITL: {}, ", self.txttitl).as_ref();
        out_str += format!("SECURITY: [{}], ", self.security).as_ref();
        out_str += format!("ENCRYP: {}, ", self.encryp).as_ref();
        out_str += format!("TXTFMT: {}, ", self.txtfmt).as_ref();
        out_str += format!("TXSHDL: {}", self.txshdl).as_ref();
        out_str += format!("TXSOFL: {}", self.txsofl).as_ref();
        out_str += format!("TXSHD: {}", self.txshd).as_ref();
        write!(f, "[Text Subheader: {out_str}]")
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
            _ => Err(NitfError::EnumError("TextFormat")),
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
