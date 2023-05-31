//! Reserved Extension segment subheader definition
use std::fmt::Display;
use std::io::{Read, Seek};

use crate::nitf_2_1::segments::headers::NitfSegmentHeader;
use crate::nitf_2_1::types::field::NitfField;
use crate::nitf_2_1::types::security::Security;

/// Metadata for Reserved Extension Segment
///
/// RESDATA is accessed through [Segment] `read_data()` function
#[allow(non_snake_case)]
#[derive(Default, Clone, Hash, Debug)]
pub struct ReservedExtensionHeader {
    /// File Part Type
    pub RE: NitfField<String>,
    /// Unique RES Type Identifier
    pub RESID: NitfField<String>,
    /// Version of the Data Definition
    pub RESVER: NitfField<u8>,
    /// Security information
    pub SECURITY: Security,
    /// User-defined Subheader Length
    pub RESSHL: NitfField<u16>,
    /// User-Defined Subheader Fields
    pub RESSHF: NitfField<String>,
    // User-Defined Data
    // pub RESDATA: NitfField<String>,  // Determined by NTB?
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
        // out_str += format!("RESDATA: {}, ", self.RESDATA).as_ref();
        write!(f, "Reserved Extension Subheader: [{}]", out_str)
    }
}
impl NitfSegmentHeader for ReservedExtensionHeader {
    fn read(&mut self, reader: &mut (impl Read + Seek)) {
        self.RE.read(reader, 2u8);
        self.RESID.read(reader, 25u8);
        self.RESVER.read(reader, 2u8);
        self.SECURITY.read(reader);
        self.RESSHL.read(reader, 4u8);
        if self.RESSHL.val != 0 {
            self.RESSHF.read(reader, self.RESSHL.val);
        }
    }
}
