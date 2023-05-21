//! Data Extension segment subheader definition
use std::fmt::Display;
use std::io::{Read, Seek};

use crate::nitf_2_1::types::{NitfField, NitfSegmentHeader, Security};

/// Metadata for Data Extension Segment
#[allow(non_snake_case)]
#[derive(Default, Clone, Hash, Debug)]
pub struct DataExtensionHeader {
    // File Part Type
    pub DE: NitfField,
    // Unique DES Type Identifier
    pub DESID: NitfField,
    // Version of the Data Definition
    pub DESVER: NitfField,
    /// Security information
    pub SECURITY: Security,
    // Overflowed Header Type
    pub DESOFLW: NitfField,
    // Data Item Overflowed
    pub DESITEM: NitfField,
    // DES User-defined Subheader Length
    pub DESSHL: NitfField,
    // DES User-Defined Data
    pub DESDATA: NitfField,
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
        out_str += format!("DESDATA: {}", self.DESDATA).as_ref();
        write!(f, "DataExtension: [{}]", out_str)
    }
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
        let header_length: u16 = self.DESSHL.string.parse().unwrap();
        self.DESDATA.read(reader, header_length - 3);
    }
}
