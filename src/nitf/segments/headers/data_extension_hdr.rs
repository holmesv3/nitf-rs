//! Data Extension segment subheader definition
use std::fmt::Display;
use std::io::{Read, Seek};
use std::str::FromStr;

use crate::nitf::segments::headers::NitfSegmentHeader;
use crate::nitf::types::field::{InvalidNitfValue, NitfField};
use crate::nitf::types::security::Security;

/// Metadata for Data Extension Segment
#[allow(non_snake_case)]
#[derive(Default, Clone, Hash, Debug)]
pub struct DataExtensionHeader {
    /// File Part Type
    pub DE: NitfField<String>,
    /// Unique DES Type Identifier
    pub DESID: NitfField<String>,
    /// Check on this registration
    /// Version of the Data Definition
    pub DESVER: NitfField<u8>,
    //// Security information
    pub SECURITY: Security,
    /// Overflowed Header Type
    pub DESOFLW: NitfField<OverflowedHeaderType>,
    /// Data Item Overflowed
    pub DESITEM: NitfField<u16>,
    /// DES User-defined Subheader Length
    pub DESSHL: NitfField<u16>,
    /// User-defined Subheader Fields
    pub DESSHF: NitfField<String>, // TODO: Figure out what to do here
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
        self.DE.read(reader, 2u8);
        self.DESID.read(reader, 25u8);
        self.DESVER.read(reader, 2u8);
        self.SECURITY.read(reader);
        if self.DESID.string.trim() == "TRE_OVERFLOW" {
            self.DESOFLW.read(reader, 6u8);
            self.DESITEM.read(reader, 3u8);
        }
        self.DESSHL.read(reader, 4u8);
        self.DESSHF.read(reader, self.DESSHL.val);
    }
}
impl Display for DataExtensionHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out_str = String::default();
        out_str += format!("DE: {}, ", self.DE).as_ref();
        out_str += format!("DESID: {}, ", self.DESID).as_ref();
        out_str += format!("DESVER: {}, ", self.DESVER).as_ref();
        out_str += format!("SECURITY: [{}], ", self.SECURITY).as_ref();
        out_str += format!("DESOFLW: {}, ", self.DESOFLW).as_ref();
        out_str += format!("DESITEM: {}, ", self.DESITEM).as_ref();
        out_str += format!("DESSHL: {}, ", self.DESSHL).as_ref();
        out_str += format!("DESSHF: {}", self.DESSHL).as_ref();
        write!(f, "[Data Extension Subheader: {}]", out_str)
    }
}
impl FromStr for OverflowedHeaderType {
    type Err = InvalidNitfValue;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "IXSHD" => Ok(Self::IXSHD),
            "SXSHD" => Ok(Self::SXSHD),
            "TXSHD" => Ok(Self::TXSHD),
            "UDHD" => Ok(Self::UDHD),
            "UDID" => Ok(Self::UDID),
            _ => Err(InvalidNitfValue),
        }
    }
}
