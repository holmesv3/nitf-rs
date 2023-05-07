//! Header definition
use std::io::{Read, Seek};
use std::fmt::Display;

use crate::nitf_2_1::types::*;

// Struct definition
#[allow(non_snake_case)]
#[derive(Default, Clone, Hash, Debug)]
pub struct NitfHeader {
    /// File Profile Name
    pub FHDR: NitfField,
    /// File Version
    pub FVER: NitfField,
    /// Complexity Level
    pub CLEVEL: NitfField,
    /// Standard Type
    pub STYPE: NitfField,
    /// Originating Station ID
    pub OSTAID: NitfField,
    /// File Date and Time
    pub FDT: NitfField,
    /// File Title
    pub FTITLE: NitfField,
    /// Security information
    pub SECURITY: Security,
    /// File Copy Number
    pub FSCOP: NitfField,
    /// File Number of Copies
    pub FSCPYS: NitfField,
    /// Encryption
    pub ENCRYP: NitfField,
    /// File Background Color
    pub FBKGC: NitfField,
    /// Originator's Name
    pub ONAME: NitfField,
    /// Originator's Phone Number
    pub OPHONE: NitfField,
    /// File Length
    pub FL: NitfField,
    /// NITF File Header Length
    pub HL: NitfField,
    /// Number of Image Segments
    pub NUMI: NitfField,
    /// Image Segments
    pub IMHEADERS: NitfSubHeaderVec,
    /// Number of Graphics Segments
    pub NUMS: NitfField,
    /// Graphic Segments
    pub GRAPHHEADERS: NitfSubHeaderVec,
    /// Reserved for future use
    pub NUMX: NitfField,
    /// Number of Text Files
    pub NUMT: NitfField,
    /// Text Segments
    pub TEXTFILES: NitfSubHeaderVec,
    /// Number of Data Extension Segments
    pub NUMDES: NitfField,
    /// Data Extenstion Segments
    pub DEXTHEADERS: NitfSubHeaderVec,
    /// Number of Reserved Extension Segments
    pub NUMRES: NitfField,
    /// Reserved Extension Segments
    pub RESHEADERS: NitfSubHeaderVec,
    /// User Defined Header Data Length
    pub UDHDL: NitfField,
    /// Extended Header Data Length
    pub XHDL: NitfField,
}
impl Display for NitfHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out_str = String::default();
        out_str += format!("FHDR: {}, ", self.FHDR).as_ref();
        out_str += format!("FVER: {}, ", self.FVER).as_ref();
        out_str += format!("CLEVEL: {}, ", self.CLEVEL).as_ref();
        out_str += format!("STYPE: {}, ", self.STYPE).as_ref();
        out_str += format!("OSTAID: {}, ", self.OSTAID).as_ref();
        out_str += format!("FDT: {}, ", self.FDT).as_ref();
        out_str += format!("FTITLE: {}, ", self.FTITLE).as_ref();
        out_str += format!("SECURITY: [{}], ", self.SECURITY).as_ref();
        out_str += format!("FSCOP: {}, ", self.FSCOP).as_ref();
        out_str += format!("FSCPYS: {}, ", self.FSCPYS).as_ref();
        out_str += format!("ENCRYP: {}, ", self.ENCRYP).as_ref();
        out_str += format!("FBKGC: {}, ", self.FBKGC).as_ref();
        out_str += format!("ONAME: {}, ", self.ONAME).as_ref();
        out_str += format!("OPHONE: {}, ", self.OPHONE).as_ref();
        out_str += format!("FL: {}, ", self.FL).as_ref();
        out_str += format!("HL: {}, ", self.HL).as_ref();
        out_str += format!("NUMI: {}, ", self.NUMI).as_ref();
        out_str += format!("IMHEADERS: {}, ", self.IMHEADERS).as_ref();
        out_str += format!("NUMS: {}, ", self.NUMS).as_ref();
        out_str += format!("GRAPHHEADERS: {}, ", self.GRAPHHEADERS).as_ref();
        out_str += format!("NUMX: {}, ", self.NUMX).as_ref();
        out_str += format!("NUMT: {}, ", self.NUMT).as_ref();
        out_str += format!("TEXTFILES: {}, ", self.TEXTFILES).as_ref();
        out_str += format!("NUMDES: {}, ", self.NUMDES).as_ref();
        out_str += format!("DEXTHEADERS: {}, ", self.DEXTHEADERS).as_ref();
        out_str += format!("NUMRES: {}, ", self.NUMRES).as_ref();
        out_str += format!("RESHEADERS: {}, ", self.RESHEADERS).as_ref();
        out_str += format!("UDHDL: {}, ", self.UDHDL).as_ref();
        out_str += format!("XHDL: {}", self.XHDL).as_ref();
        write!(f, "NitfHeader: [{}]", out_str)
    }
}
impl NitfSegmentHeader for NitfHeader {
    fn read(&mut self, reader: &mut (impl Read + Seek)) {
        self.FHDR.read(reader, 4);
        self.FVER.read(reader, 5);
        self.CLEVEL.read(reader, 2);
        self.STYPE.read(reader, 4);
        self.OSTAID.read(reader, 10);
        self.FDT.read(reader, 14);
        self.FTITLE.read(reader, 80);
        self.SECURITY.read(reader);
        self.FSCOP.read(reader, 5);
        self.FSCPYS.read(reader, 5);
        self.ENCRYP.read(reader, 1);
        self.FBKGC.read(reader, 3);
        self.ONAME.read(reader, 24);
        self.OPHONE.read(reader, 18);
        self.FL.read(reader, 12);
        self.HL.read(reader, 6);
        self.NUMI.read(reader, 3);
        self.IMHEADERS.read(
            reader, 
            &self.NUMI, 
            6, 
            10);
        self.NUMS.read(reader, 3);
        self.GRAPHHEADERS.read(
            reader, 
            &self.NUMS, 
            4, 
            6);
        self.NUMX.read(reader, 3);
        self.NUMT.read(reader, 3);
        self.TEXTFILES.read(
            reader, 
            &self.NUMT, 
            4, 
            5);
        self.NUMDES.read(reader, 3);
        self.DEXTHEADERS.read(
            reader, 
            &self.NUMDES, 
            4, 
            9);
        self.NUMRES.read(reader, 3);
        self.RESHEADERS.read(
            reader, 
            &self.NUMRES, 
            4, 
            7);
        self.UDHDL.read(reader, 5);
        self.XHDL.read(reader, 5);
    }
}



