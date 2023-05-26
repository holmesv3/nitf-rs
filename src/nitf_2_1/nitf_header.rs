//! File header definition
use std::fmt::Display;
use std::io::{Read, Seek};

use crate::nitf_2_1::types::*;

/// Metadata for Nitf File Header
#[allow(non_snake_case)]
#[derive(Default, Clone, Hash, Debug)]
pub struct NitfHeader {
    /// File Profile Name
    pub FHDR: NitfField<String>,
    /// File Version
    pub FVER: NitfField<String>,
    /// Complexity Level
    pub CLEVEL: NitfField<u8>,
    /// Standard Type
    pub STYPE: NitfField<String>,
    /// Originating Station ID
    pub OSTAID: NitfField<String>,
    /// File Date and Time
    pub FDT: NitfField<String>,
    /// File Title
    pub FTITLE: NitfField<String>,
    /// Security information
    pub SECURITY: Security,
    /// File Copy Number
    pub FSCOP: NitfField<u32>,
    /// File Number of Copies
    pub FSCPYS: NitfField<u32>,
    /// Encryption
    pub ENCRYP: NitfField<String>,
    /// File Background Color
    pub FBKGC: Vec<NitfField<String>>,  // TODO: Fix the parsing of this
    /// Originator's Name
    pub ONAME: NitfField<String>,
    /// Originator's Phone Number
    pub OPHONE: NitfField<String>,
    /// File Length
    pub FL: NitfField<u64>,
    /// NITF File Header Length
    pub HL: NitfField<u32>,
    /// Number of Image Segments
    pub NUMI: NitfField<u16>,
    /// Image Segments
    pub IMHEADERS: Vec<NitfSubHeader>,
    /// Number of Graphics Segments
    pub NUMS: NitfField<u16>,
    /// Graphic Segments
    pub GRAPHHEADERS: Vec<NitfSubHeader>,
    /// Reserved for future use
    pub NUMX: NitfField<u16>,
    /// Number of Text Segments
    pub NUMT: NitfField<u16>,
    /// Text Segments
    pub TEXTHEADERS: Vec<NitfSubHeader>,
    /// Number of Data Extension Segments
    pub NUMDES: NitfField<u16>,
    /// Data Extenstion Segments
    pub DEXTHEADERS: Vec<NitfSubHeader>,
    /// Number of Reserved Extension Segments
    pub NUMRES: NitfField<u16>,
    /// Reserved Extension Segments
    pub RESHEADERS: Vec<NitfSubHeader>,
    /// User Defined Header Data Length
    pub UDHDL: NitfField<u32>,
    /// User Defined Header Overflow
    pub UDHOFL: NitfField<u16>,
    /// User Defined Header Data
    pub UDHD: NitfField<String>,  // TODO: Figure out what to do with this
    /// Extended Header Data Length
    pub XHDL: NitfField<u32>,
    /// Extended Header Data Overflow
    pub XHDLOFL: NitfField<u16>,
    /// Extended Header Data
    pub XHD: NitfField<String>,  // TODO: Figure out what to do with this
}
impl Display for NitfHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out_str = String::default();
        out_str += format!("FHDR: {}, \n", self.FHDR).as_ref();
        out_str += format!("FVER: {}, \n", self.FVER).as_ref();
        out_str += format!("CLEVEL: {}, \n", self.CLEVEL).as_ref();
        out_str += format!("STYPE: {}, \n", self.STYPE).as_ref();
        out_str += format!("OSTAID: {}, \n", self.OSTAID).as_ref();
        out_str += format!("FDT: {}, \n", self.FDT).as_ref();
        out_str += format!("FTITLE: {}, \n", self.FTITLE).as_ref();
        out_str += format!("SECURITY: [\n{}], \n", self.SECURITY).as_ref();
        out_str += format!("FSCOP: {}, \n", self.FSCOP).as_ref();
        out_str += format!("FSCPYS: {}, \n", self.FSCPYS).as_ref();
        out_str += format!("ENCRYP: {}, \n", self.ENCRYP).as_ref();
        out_str += format!(
            "FBKGC: [R: {}, G: {}, B: {}], \n",
             self.FBKGC[0],
             self.FBKGC[1],
             self.FBKGC[2],
        ).as_ref();
        out_str += format!("ONAME: {}, \n", self.ONAME).as_ref();
        out_str += format!("OPHONE: {}, \n", self.OPHONE).as_ref();
        out_str += format!("FL: {}, \n", self.FL).as_ref();
        out_str += format!("HL: {}, \n", self.HL).as_ref();
        out_str += format!("NUMI: {}, \n", self.NUMI).as_ref();
        for seg in &self.IMHEADERS {
            out_str += format!("\tIMHEADER: {}, \n", seg).as_ref()
        }
        out_str += format!("NUMS: {}, \n", self.NUMS).as_ref();
        for seg in &self.GRAPHHEADERS {
            out_str += format!("\tGRAPHHEADERS: {}, \n", seg).as_ref()
        }
        out_str += format!("NUMX: {}, \n", self.NUMX).as_ref();
        out_str += format!("NUMT: {}, \n", self.NUMT).as_ref();
        for seg in &self.TEXTHEADERS {
            out_str += format!("\tTEXTHEADER: {}, \n", seg).as_ref()
        }
        out_str += format!("NUMDES: {}, \n", self.NUMDES).as_ref();
        for seg in &self.DEXTHEADERS {
            out_str += format!("\tDEXTHEADER: {}, \n", seg).as_ref()
        }
        out_str += format!("NUMRES: {}, \n", self.NUMRES).as_ref();
        for seg in &self.RESHEADERS {
            out_str += format!("\tRESHEADER: {}, \n", seg).as_ref()
        }
        out_str += format!("UDHDL: {}, \n", self.UDHDL).as_ref();
        out_str += format!("UDHOFL: {}, \n", self.UDHOFL).as_ref();
        out_str += format!("UDHD: {}, \n", self.UDHD).as_ref();
        out_str += format!("XHDL: {}, \n", self.XHDL).as_ref();
        out_str += format!("XHDLOFL: {}, \n", self.XHDLOFL).as_ref();
        out_str += format!("XHD: {}", self.XHD).as_ref();
        write!(f, "NitfHeader: [{}]", out_str)
    }
}
impl NitfSegmentHeader for NitfHeader {
    fn read(&mut self, reader: &mut (impl Read + Seek)) {
        self.FHDR.read(reader, 4u8);
        self.FVER.read(reader, 5u8);
        self.CLEVEL.read(reader, 2u8);
        self.STYPE.read(reader, 4u8);
        self.OSTAID.read(reader, 10u8);
        self.FDT.read(reader, 14u8);
        self.FTITLE.read(reader, 80u8);
        self.SECURITY.read(reader);
        self.FSCOP.read(reader, 5u8);
        self.FSCPYS.read(reader, 5u8);
        self.ENCRYP.read(reader, 1u8);
        for _ in 0..3 {
            let mut color: NitfField<String> = NitfField::default();
            color.read(reader, 1u8);
            self.FBKGC.push(color);
        }

        self.ONAME.read(reader, 24u8);
        self.OPHONE.read(reader, 18u8);
        self.FL.read(reader, 12u8);
        self.HL.read(reader, 6u8);
        self.NUMI.read(reader, 3u8);
        for _ in 0..self.NUMI.val {
            let mut subheader = NitfSubHeader::default();
            subheader.read(reader, 6, 10);
            self.IMHEADERS.push(subheader);
        }

        self.NUMS.read(reader, 3u8);
        for _ in 0..self.NUMS.val {
            let mut subheader = NitfSubHeader::default();
            subheader.read(reader, 4, 6);
            self.GRAPHHEADERS.push(subheader);
        }

        self.NUMX.read(reader, 3u8);
        self.NUMT.read(reader, 3u8);
        for _ in 0..self.NUMT.val {
            let mut subheader = NitfSubHeader::default();
            subheader.read(reader, 4, 5);
            self.TEXTHEADERS.push(subheader);
        }

        self.NUMDES.read(reader, 3u8);
        for _ in 0..self.NUMDES.val {
            let mut subheader = NitfSubHeader::default();
            subheader.read(reader, 4, 9);
            self.DEXTHEADERS.push(subheader);
        }

        self.NUMRES.read(reader, 3u8);
        for _ in 0..self.NUMRES.val {
            let mut subheader = NitfSubHeader::default();
            subheader.read(reader, 4, 7);
            self.RESHEADERS.push(subheader);
        }

        self.UDHDL.read(reader, 5u8);
        if self.UDHDL.val != 0 {
            self.UDHOFL.read(reader, 3u8);
            self.UDHD.read(reader, self.UDHDL.val - 3);
        }

        self.XHDL.read(reader, 5u8);
        if self.XHDL.val != 0 {
            self.XHDLOFL.read(reader, 3u8);
            self.XHD.read(reader, &self.XHDL.val - 3);
        }
    }
}
