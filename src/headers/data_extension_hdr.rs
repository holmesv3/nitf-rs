//! Data Extension segment subheader definition
use std::fmt::Display;
use std::fs::File;
use std::str::FromStr;

use crate::headers::NitfSegmentHeader;
use crate::types::{ExtendedSubheader, NitfField, Security};
use crate::{NitfError, NitfResult};


/// Metadata for Data Extension Segment
#[derive(Default, Clone, Debug, Eq, PartialEq)]
pub struct DataExtensionHeader {
    /// File Part Type
    pub de: NitfField<String>,
    /// Unique DES Type Identifier
    pub desid: NitfField<String>,
    /// Check on this registration
    /// Version of the Data Definition
    pub desver: NitfField<u8>,
    //// Security information
    pub security: Security,
    /// Overflowed Header Type
    pub desoflw: NitfField<OverflowedHeaderType>,
    /// Data Item Overflowed
    pub desitem: NitfField<u16>,
    /// DES User-defined Subheader Length
    pub desshl: NitfField<u16>,
    /// User-defined Subheader Fields
    pub desshf: ExtendedSubheader,
}

/// Selection of which header/subheader this extension corresponds to
#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub enum OverflowedHeaderType {
    #[default]
    /// Image subheader extended subheader data overflow
    IXSHD,
    /// Graphic subheader extended subheader data overflow
    SXSHD,
    /// Text subheader extended subheader data overflow
    TXSHD,
    /// Header user defined header overflow
    UDHD,
    /// Image subheader user defined image data overflow
    UDID,
}

impl NitfSegmentHeader for DataExtensionHeader {
    fn read(&mut self, reader: &mut File) -> NitfResult<()> {
        self.de.read(reader, 2u8, "DE")?;
        self.desid.read(reader, 25u8, "DESID")?;
        self.desver.read(reader, 2u8, "DESVER")?;
        self.security.read(reader)?;
        if self.desid.string().trim() == "TRE_OVERFLOW" {
            self.desoflw.read(reader, 6u8, "DESOFLW")?;
            self.desitem.read(reader, 3u8, "DESITEM")?;
        }
        self.desshl.read(reader, 4u8, "DESSHL")?;
        if self.desshl.val().clone() != 0 {
            self.desshf
                .read(reader, self.desshl.val().clone() as usize, "DESSHF")?;
        }
        Ok(())
    }
    fn write(&self, writer: &mut File) -> NitfResult<usize> {
        let mut bytes_written = 0;
        bytes_written += self.de.write(writer, "DE")?;
        bytes_written += self.desid.write(writer, "DESID")?;
        bytes_written += self.desver.write(writer, "DESVER")?;
        bytes_written += self.security.write(writer)?;
        if self.desid.string().trim() == "TRE_OVERFLOW" {
            bytes_written += self.desoflw.write(writer, "DESOFLW")?;
            bytes_written += self.desitem.write(writer, "DESITEM")?;
        }
        bytes_written += self.desshl.write(writer, "DESSHL")?;
        if self.desshl.val().clone() != 0 {
            bytes_written += self.desshf.write(writer, "DESSHF")?;
        }
        Ok(bytes_written)
    }
    fn length(&self) -> usize {
        let mut length: usize = 0;
        length += self.de.length();
        length += self.desid.length();
        length += self.desver.length();
        length += self.security.length();
        length += self.desoflw.length();
        length += self.desitem.length();
        length += self.desshl.length();
        length += self.desshf.length() as usize;
        length
    }
}
impl Display for DataExtensionHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out_str = String::default();
        out_str += format!("DE: {}, ", self.de).as_ref();
        out_str += format!("DESID: {}, ", self.desid).as_ref();
        out_str += format!("DESVER: {}, ", self.desver).as_ref();
        out_str += format!("SECURITY: [{}], ", self.security).as_ref();
        out_str += format!("DESOFLW: {}, ", self.desoflw).as_ref();
        out_str += format!("DESITEM: {}, ", self.desitem).as_ref();
        out_str += format!("DESSHL: {}, ", self.desshl).as_ref();
        out_str += format!("DESSHF: {}", self.desshf).as_ref();
        write!(f, "[Data Extension Subheader: {out_str}]")
    }
}
impl FromStr for OverflowedHeaderType {
    type Err = NitfError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "IXSHD" => Ok(Self::IXSHD),
            "SXSHD" => Ok(Self::SXSHD),
            "TXSHD" => Ok(Self::TXSHD),
            "UDHD" => Ok(Self::UDHD),
            "UDID" => Ok(Self::UDID),
            _ => Err(NitfError::EnumError("OverflowedHeaderType")),
        }
    }
}
impl Display for OverflowedHeaderType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IXSHD => write!(f, "IXSHD"),
            Self::SXSHD => write!(f, "SXSHD"),
            Self::TXSHD => write!(f, "TXSHD"),
            Self::UDHD => write!(f, "UDHD"),
            Self::UDID => write!(f, "UDID"),
        }
    }
}
