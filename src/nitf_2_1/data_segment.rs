use std::io::{Read, Seek};
use std::fmt::Display;

use crate::nitf_2_1::types::NitfField;

use super::types::NitfSegmentHeader;

#[allow(non_snake_case)]
#[derive(Default, Clone, Hash, Debug)]
pub struct DataExtensionSegment {
    // File Part Type
    pub DE: NitfField,
    // Unique DES Type Identifier
    pub DESID: NitfField,
    // Version of the Data Definition
    pub DESVER: NitfField,
    // Extension File Security Classification
    pub DECLAS: NitfField,
    // Security Classification System
    pub DESCLSY: NitfField,
    // Codewords
    pub DESCODE: NitfField,
    // Control and Handling
    pub DESCTLH: NitfField,
    // Releasing Instructions
    pub DESREL: NitfField,
    // Declassification Type
    pub DESDCTP: NitfField,
    // Declassification Date
    pub DESDCDT: NitfField,
    // Declassification Exemption
    pub DESDCXM: NitfField,
    // Downgrade
    pub DESDG: NitfField,
    // Downgrade Date
    pub DESDGDT: NitfField,
    // Classification Text
    pub DESCLTX: NitfField,
    // Classification Authority Type
    pub DESCATP: NitfField,
    // Classification Authority
    pub DESCAUT: NitfField,
    // Classification Reason
    pub DESCRSN: NitfField,
    // Security Source Date
    pub DESSRDT: NitfField,
    // Security Control Number
    pub DESCTLN: NitfField,
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
impl Display for DataExtensionSegment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out_str = String::default();
        out_str += format!("\nDE: {}", self.DE).as_ref();
        out_str += format!("\nDESID: {}", self.DESID).as_ref();
        out_str += format!("\nDESVER: {}", self.DESVER).as_ref();
        out_str += format!("\nDECLAS: {}", self.DECLAS).as_ref();
        out_str += format!("\nDESCLSY: {}", self.DESCLSY).as_ref();
        out_str += format!("\nDESCODE: {}", self.DESCODE).as_ref();
        out_str += format!("\nDESCTLH: {}", self.DESCTLH).as_ref();
        out_str += format!("\nDESREL: {}", self.DESREL).as_ref();
        out_str += format!("\nDESDCTP: {}", self.DESDCTP).as_ref();
        out_str += format!("\nDESDCDT: {}", self.DESDCDT).as_ref();
        out_str += format!("\nDESDCXM: {}", self.DESDCXM).as_ref();
        out_str += format!("\nDESDG: {}", self.DESDG).as_ref();
        out_str += format!("\nDESDGDT: {}", self.DESDGDT).as_ref();
        out_str += format!("\nDESCLTX: {}", self.DESCLTX).as_ref();
        out_str += format!("\nDESCATP: {}", self.DESCATP).as_ref();
        out_str += format!("\nDESCAUT: {}", self.DESCAUT).as_ref();
        out_str += format!("\nDESCRSN: {}", self.DESCRSN).as_ref();
        out_str += format!("\nDESSRDT: {}", self.DESSRDT).as_ref();
        out_str += format!("\nDESCTLN: {}", self.DESCTLN).as_ref();
        out_str += format!("\nDESOFLW: {}", self.DESOFLW).as_ref();
        out_str += format!("\nDESITEM: {}", self.DESITEM).as_ref();
        out_str += format!("\nDESSHL: {}", self.DESSHL).as_ref();
        out_str += format!("\nDESSHF: {}", self.DESSHF).as_ref();
        out_str += format!("\nDESDATA: {}", self.DESDATA).as_ref();
        write!(f, "DataExtension: [{}]", out_str)
    }
}
impl NitfSegmentHeader for DataExtensionSegment {
    fn read(&mut self, reader: &mut (impl Read + Seek)) {
        self.DE.read(reader, 2);
        self.DESID.read(reader, 25);
        self.DESVER.read(reader, 2);
        self.DECLAS.read(reader, 1);
        self.DESCLSY.read(reader, 2);
        self.DESCODE.read(reader, 11);
        self.DESCTLH.read(reader, 2);
        self.DESREL.read(reader, 20);
        self.DESDCTP.read(reader, 2);
        self.DESDCDT.read(reader, 8);
        self.DESDCXM.read(reader, 4);
        self.DESDG.read(reader, 1);
        self.DESDGDT.read(reader, 8);
        self.DESCLTX.read(reader, 43);
        self.DESCATP.read(reader, 1);
        self.DESCAUT.read(reader, 40);
        self.DESCRSN.read(reader, 1);
        self.DESSRDT.read(reader, 8);
        self.DESCTLN.read(reader, 15);
        self.DESOFLW.read(reader, 6);
        self.DESITEM.read(reader, 3);
        self.DESSHL.read(reader, 4);
        let header_length: usize = String::from_utf8(self.DESSHL.bytes.to_vec()).unwrap().parse().unwrap();
        self.DESSHF.read(reader, header_length);
        // self.DESDATA.read(reader, data_length);
    }
}