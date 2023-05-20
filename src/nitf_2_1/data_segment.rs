use std::fmt::Display;
use std::io::{Read, Seek};

use crate::nitf_2_1::types::{NitfField, NitfSegmentHeader, Security};

#[allow(non_snake_case)]
#[derive(Default, Clone, Hash, Debug)]
pub struct DataExtensionSegmentHeader {
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
    // User-defined Subheader Fields
    pub DESSHF: NitfField,
    // DES User-Defined Data
    pub DESDATA: NitfField,
}
impl Display for DataExtensionSegmentHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out_str = String::default();
        out_str += format!("DE: {}, ", self.DE).as_ref();
        out_str += format!("DESID: {}, ", self.DESID).as_ref();
        out_str += format!("DESVER: {}, ", self.DESVER).as_ref();
        out_str += format!("SECURITY: [{}], ", self.SECURITY).as_ref();
        out_str += format!("DESOFLW: {}, ", self.DESOFLW).as_ref();
        out_str += format!("DESITEM: {}, ", self.DESITEM).as_ref();
        out_str += format!("DESSHL: {}, ", self.DESSHL).as_ref();
        out_str += format!("DESSHF: {}, ", self.DESSHF).as_ref();
        out_str += format!("DESDATA: {}", self.DESDATA).as_ref();
        write!(f, "DataExtension: [{}]", out_str)
    }
}
impl NitfSegmentHeader for DataExtensionSegmentHeader {
    fn read(&mut self, reader: &mut (impl Read + Seek)) {
        self.DE.read(reader, 2);
        self.DESID.read(reader, 25);
        self.DESVER.read(reader, 2);
        self.SECURITY.read(reader);
        if self.DESID.string == "             TRE_OVERFLOW" { // 25 characters
            self.DESOFLW.read(reader, 6);
            self.DESITEM.read(reader, 3);
        }
        self.DESSHL.read(reader, 4);
        let header_length: usize = self.DESSHL.string.parse().unwrap();
        if header_length != 0 {
            self.DESSHF.read(reader, header_length);
        }
        // self.DESDATA.read(reader, data_length);
    }
}
