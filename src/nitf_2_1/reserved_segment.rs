use std::io::{Read, Seek};
use std::fmt::Display;

use crate::nitf_2_1::types::{NitfField, NitfSegmentHeader, Security};


#[allow(non_snake_case)]
#[derive(Default, Clone, Hash, Debug)]
pub struct ReservedExtensionSegment {
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
impl Display for ReservedExtensionSegment {
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
impl NitfSegmentHeader for ReservedExtensionSegment {
    fn read(&mut self, reader: &mut (impl Read + Seek)) {
        self.RE.read(reader, 2);
        self.RESID.read(reader, 25);
        self.RESVER.read(reader, 2);
        self.SECURITY.read(reader);
        self.RESSHL.read(reader, 4);
        let header_length: usize = self.RESSHL.string.parse().unwrap();
        self.RESSHF.read(reader, header_length);
        // self.RESDATA.read(reader, data_length);
    }
}




















