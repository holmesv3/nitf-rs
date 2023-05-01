use std::io::{Read, Seek};
use std::fmt::Display;

use crate::types::NitfField;

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
impl DataExtensionSegment {
    pub fn from_reader(reader: &mut (impl Read + Seek)) -> Result<Self, std::io::Error> {
        let mut data_ext = DataExtensionSegment::default();
        data_ext.DE.read(reader, 2);
        data_ext.DESID.read(reader, 25);
        data_ext.DESVER.read(reader, 2);
        data_ext.DECLAS.read(reader, 1);
        data_ext.DESCLSY.read(reader, 2);
        data_ext.DESCODE.read(reader, 11);
        data_ext.DESCTLH.read(reader, 2);
        data_ext.DESREL.read(reader, 20);
        data_ext.DESDCTP.read(reader, 2);
        data_ext.DESDCDT.read(reader, 8);
        data_ext.DESDCXM.read(reader, 4);
        data_ext.DESDG.read(reader, 1);
        data_ext.DESDGDT.read(reader, 8);
        data_ext.DESCLTX.read(reader, 43);
        data_ext.DESCATP.read(reader, 1);
        data_ext.DESCAUT.read(reader, 40);
        data_ext.DESCRSN.read(reader, 1);
        data_ext.DESSRDT.read(reader, 8);
        data_ext.DESCTLN.read(reader, 15);
        data_ext.DESOFLW.read(reader, 6);
        data_ext.DESITEM.read(reader, 3);
        data_ext.DESSHL.read(reader, 4);
        // Todo, first is from above, second is from input (from nitf header value I think)
        // data_ext.DESSHF.read(reader, );
        // data_ext.DESDATA.read(reader, );
        Ok(data_ext)
    }
}