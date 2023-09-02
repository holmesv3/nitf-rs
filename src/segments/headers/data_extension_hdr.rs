//! Data Extension segment subheader definition
use std::fmt::Display;
use std::io::{Read, Seek};
use std::str::FromStr;

use crate::error::NitfError;
use crate::segments::headers::NitfSegmentHeader;
use crate::types::{NitfField, Security};

/// Metadata for Data Extension Segment
#[derive(Default, Clone, Hash, Debug)]
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
    pub desshf: NitfField<String>, // TODO: Figure out what to do here
}

/// Selection of which header/subheader this extension corresponds to
#[derive(Debug, Default, Clone, Hash)]
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
    fn read(&mut self, reader: &mut (impl Read + Seek)) {
        self.de.read(reader, 2u8);
        self.desid.read(reader, 25u8);
        self.desver.read(reader, 2u8);
        self.security.read(reader);
        if self.desid.string.trim() == "TRE_OVERFLOW" {
            self.desoflw.read(reader, 6u8);
            self.desitem.read(reader, 3u8);
        }
        self.desshl.read(reader, 4u8);
        self.desshf.read(reader, self.desshl.val);
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
        out_str += format!("DESSHF: {}", self.desshl).as_ref();
        write!(f, "[Data Extension Subheader: {}]", out_str)
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
            _ => Err(NitfError::FieldError),
        }
    }
}
