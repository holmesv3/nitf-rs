//! Reserved Extension segment subheader definition
use std::fmt::Display;
use std::io::{Read, Seek};

use crate::segments::headers::NitfSegmentHeader;
use crate::types::{NitfField, Security};

/// Metadata for Reserved Extension Segment
#[derive(Default, Clone, Hash, Debug)]
pub struct ReservedExtensionHeader {
    /// File Part Type
    pub re: NitfField<String>,
    /// Unique RES Type Identifier
    pub resid: NitfField<String>,
    /// Version of the Data Definition
    pub resver: NitfField<u8>,
    /// Security information
    pub security: Security,
    /// User-defined Subheader Length
    pub resshl: NitfField<u16>,
    /// User-Defined Subheader Fields
    pub resshf: NitfField<String>,
}
impl Display for ReservedExtensionHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out_str = String::default();
        out_str += format!("RE: {}, ", self.re).as_ref();
        out_str += format!("RESID: {}, ", self.resid).as_ref();
        out_str += format!("RESVER: {}, ", self.resver).as_ref();
        out_str += format!("SECURITY: [{}], ", self.security).as_ref();
        out_str += format!("RESSHL: {}, ", self.resshl).as_ref();
        out_str += format!("RESSHF: {}, ", self.resshf).as_ref();
        write!(f, "[Reserved Extension Subheader: {}]", out_str)
    }
}
impl NitfSegmentHeader for ReservedExtensionHeader {
    fn read(&mut self, reader: &mut (impl Read + Seek)) {
        self.re.read(reader, 2u8);
        self.resid.read(reader, 25u8);
        self.resver.read(reader, 2u8);
        self.security.read(reader);
        self.resshl.read(reader, 4u8);
        if self.resshl.val != 0 {
            self.resshf.read(reader, self.resshl.val);
        }
    }
}
