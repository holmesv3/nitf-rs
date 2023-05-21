use std::fmt::Display;
use std::io::{Read, Seek};

use crate::nitf_2_1::types::{NitfField, NitfSegmentHeader, Security};

/// Metadata for Reserved Extension Segment
///
/// RESDATA is accessed through [Segment] `read_data()` function
#[allow(non_snake_case)]
#[derive(Default, Clone, Hash, Debug)]
pub struct ReservedExtensionHeader {
    /// File Part Type
    pub RE: NitfField,
    /// Unique RES Type Identifier
    pub RESID: NitfField,
    /// Version of the Data Definition
    pub RESVER: NitfField,
    /// Security information
    pub SECURITY: Security,
    /// User-defined Subheader Length
    pub RESSHL: NitfField,
    /// User-Defined Subheader Fields
    pub RESSHF: NitfField,
    /// User-Defined Data
    pub RESDATA: NitfField,
}
impl Display for ReservedExtensionHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out_str = String::default();
        out_str += format!("RE: {}, ", self.RE).as_ref();
        out_str += format!("RESID: {}, ", self.RESID).as_ref();
        out_str += format!("RESVER: {}, ", self.RESVER).as_ref();
        out_str += format!("SECURITY: [{}], ", self.SECURITY).as_ref();
        out_str += format!("RESSHL: {}, ", self.RESSHL).as_ref();
        out_str += format!("RESSHF: {}, ", self.RESSHF).as_ref();
        out_str += format!("RESDATA: {}, ", self.RESDATA).as_ref();
        write!(f, "ReservedExtensionSegment: [{}]", out_str)
    }
}
impl NitfSegmentHeader for ReservedExtensionHeader {
    fn read(&mut self, reader: &mut (impl Read + Seek)) {
        self.RE.read(reader, 2u8);
        self.RESID.read(reader, 25u8);
        self.RESVER.read(reader, 2u8);
        self.SECURITY.read(reader);
        self.RESSHL.read(reader, 4u8);
        let user_data_length: u32 = self.RESSHL.string.parse().unwrap();
        if user_data_length != 0 {
            self.RESSHF.read(reader, user_data_length);
        }
    }
}
