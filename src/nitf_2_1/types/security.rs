use std::io::{Read, Seek};
use std::fmt::Display;

use super::*;

#[allow(non_snake_case)]
#[derive(Default, Clone, Hash, Debug)]
pub struct Security {
    /// File Security Classification
    pub CLAS: NitfField,
    /// File Classification Security System
    pub CLSY: NitfField,
    /// File Codewords
    pub CODE: NitfField,
    /// File Control and Handling
    pub CTLH: NitfField,
    /// File Releasing Instructions
    pub REL: NitfField,
    /// File Declassification Type
    pub DCTP: NitfField,
    /// File Declassification Date
    pub DCDT: NitfField,
    /// File Declassification Exemption
    pub DCXM: NitfField,
    /// File Downgrade
    pub DG: NitfField,
    /// File Downgrade Date
    pub DGDT: NitfField,
    /// File Classification Text
    pub CLTX: NitfField,
    /// File Classification Authority Type
    pub CATP: NitfField,
    /// File Classification Authority
    pub CAUT: NitfField,
    /// File Classification Reason
    pub CRSN: NitfField,
    /// File Security Source Date
    pub SRDT: NitfField,
    /// File Security Control Number
    pub CTLN: NitfField,
}
impl Display for Security {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out_str = String::default();
        out_str += format!("CLAS: {}, ", self.CLAS).as_ref();
        out_str += format!("CLSY: {}, ", self.CLSY).as_ref();
        out_str += format!("CODE: {}, ", self.CODE).as_ref();
        out_str += format!("CTLH: {}, ", self.CTLH).as_ref();
        out_str += format!("REL: {}, ", self.REL).as_ref();
        out_str += format!("DCTP: {}, ", self.DCTP).as_ref();
        out_str += format!("DCDT: {}, ", self.DCDT).as_ref();
        out_str += format!("DCXM: {}, ", self.DCXM).as_ref();
        out_str += format!("DG: {}, ", self.DG).as_ref();
        out_str += format!("DGDT: {}, ", self.DGDT).as_ref();
        out_str += format!("CLTX: {}, ", self.CLTX).as_ref();
        out_str += format!("CATP: {}, ", self.CATP).as_ref();
        out_str += format!("CAUT: {}, ", self.CAUT).as_ref();
        out_str += format!("CRSN: {}, ", self.CRSN).as_ref();
        out_str += format!("SRDT: {}, ", self.SRDT).as_ref();
        out_str += format!("CTLN: {}", self.CTLN).as_ref();
        return write!(f, "{}", out_str)
    }
}
impl Security {
    pub fn read(&mut self, reader: &mut (impl Read + Seek)) {
        self.CLAS.read(reader, 1);
        self.CLSY.read(reader, 2);
        self.CODE.read(reader, 11);
        self.CTLH.read(reader, 2);
        self.REL.read(reader, 20);
        self.DCTP.read(reader, 2);
        self.DCDT.read(reader, 8);
        self.DCXM.read(reader, 4);
        self.DG.read(reader, 1);
        self.DGDT.read(reader, 8);
        self.CLTX.read(reader, 43);
        self.CATP.read(reader, 1);
        self.CAUT.read(reader, 40);
        self.CRSN.read(reader, 1);
        self.SRDT.read(reader, 8);
        self.CTLN.read(reader, 15);
    }
}