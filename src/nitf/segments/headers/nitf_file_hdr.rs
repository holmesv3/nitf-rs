//! File header definition
use std::fmt::Display;
use std::io::{Read, Seek};

use crate::nitf::segments::headers::NitfSegmentHeader;
use crate::nitf::types::{field::*, security::*};

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
    pub FBKGC: Vec<NitfField<String>>, // TODO: Fix the parsing of this
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
    pub IMHEADERS: Vec<SubHeader>,
    /// Number of Graphics Segments
    pub NUMS: NitfField<u16>,
    /// Graphic Segments
    pub GRAPHHEADERS: Vec<SubHeader>,
    /// Reserved for future use
    pub NUMX: NitfField<u16>,
    /// Number of Text Segments
    pub NUMT: NitfField<u16>,
    /// Text Segments
    pub TEXTHEADERS: Vec<SubHeader>,
    /// Number of Data Extension Segments
    pub NUMDES: NitfField<u16>,
    /// Data Extenstion Segments
    pub DEXTHEADERS: Vec<SubHeader>,
    /// Number of Reserved Extension Segments
    pub NUMRES: NitfField<u16>,
    /// Reserved Extension Segments
    pub RESHEADERS: Vec<SubHeader>,
    /// User Defined Header Data Length
    pub UDHDL: NitfField<u32>,
    /// User Defined Header Overflow
    pub UDHOFL: NitfField<u16>,
    /// User Defined Header Data
    pub UDHD: NitfField<String>, // TODO: Figure out what to do with this
    /// Extended Header Data Length
    pub XHDL: NitfField<u32>,
    /// Extended Header Data Overflow
    pub XHDLOFL: NitfField<u16>,
    /// Extended Header Data
    pub XHD: NitfField<String>, // TODO: Figure out what to do with this
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
        out_str += format!(
            "FBKGC: [R: {}, G: {}, B: {}], ",
            self.FBKGC[0], self.FBKGC[1], self.FBKGC[2],
        )
        .as_ref();
        out_str += format!("ONAME: {}, ", self.ONAME).as_ref();
        out_str += format!("OPHONE: {}, ", self.OPHONE).as_ref();
        out_str += format!("FL: {}, ", self.FL).as_ref();
        out_str += format!("HL: {}, ", self.HL).as_ref();
        out_str += format!("NUMI: {}, ", self.NUMI).as_ref();
        for seg in &self.IMHEADERS {
            out_str += format!("[IMHEADER: {}], ", seg).as_ref()
        }
        out_str += format!("NUMS: {}, ", self.NUMS).as_ref();
        for seg in &self.GRAPHHEADERS {
            out_str += format!("[GRAPHHEADERS: {}], ", seg).as_ref()
        }
        out_str += format!("NUMX: {}, ", self.NUMX).as_ref();
        out_str += format!("NUMT: {}, ", self.NUMT).as_ref();
        for seg in &self.TEXTHEADERS {
            out_str += format!("[TEXTHEADER: {}], ", seg).as_ref()
        }
        out_str += format!("NUMDES: {}, ", self.NUMDES).as_ref();
        for seg in &self.DEXTHEADERS {
            out_str += format!("[DEXTHEADER: {}], ", seg).as_ref()
        }
        out_str += format!("NUMRES: {}, ", self.NUMRES).as_ref();
        for seg in &self.RESHEADERS {
            out_str += format!("[RESHEADER: {}], ", seg).as_ref()
        }
        out_str += format!("UDHDL: {}, ", self.UDHDL).as_ref();
        out_str += format!("UDHOFL: {}, ", self.UDHOFL).as_ref();
        out_str += format!("UDHD: {}, ", self.UDHD).as_ref();
        out_str += format!("XHDL: {}, ", self.XHDL).as_ref();
        out_str += format!("XHDLOFL: {}, ", self.XHDLOFL).as_ref();
        out_str += format!("XHD: {}", self.XHD).as_ref();
        write!(f, "[NitfHeader: {}]", out_str)
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
            let mut subheader = SubHeader::default();
            subheader.read(reader, 6, 10);
            self.IMHEADERS.push(subheader);
        }

        self.NUMS.read(reader, 3u8);
        for _ in 0..self.NUMS.val {
            let mut subheader = SubHeader::default();
            subheader.read(reader, 4, 6);
            self.GRAPHHEADERS.push(subheader);
        }

        self.NUMX.read(reader, 3u8);
        self.NUMT.read(reader, 3u8);
        for _ in 0..self.NUMT.val {
            let mut subheader = SubHeader::default();
            subheader.read(reader, 4, 5);
            self.TEXTHEADERS.push(subheader);
        }

        self.NUMDES.read(reader, 3u8);
        for _ in 0..self.NUMDES.val {
            let mut subheader = SubHeader::default();
            subheader.read(reader, 4, 9);
            self.DEXTHEADERS.push(subheader);
        }

        self.NUMRES.read(reader, 3u8);
        for _ in 0..self.NUMRES.val {
            let mut subheader = SubHeader::default();
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

/// Subheader element type
///
/// Used within the NITF header to denote the subheader segments contained in the file
#[derive(Default, Clone, Hash, Debug)]
pub struct SubHeader {
    /// Bytes of header description
    pub subheader_size: NitfField<u32>,
    /// Bytes of the data
    pub item_size: NitfField<u64>,
}
impl SubHeader {
    pub fn read(&mut self, reader: &mut (impl Read + Seek), sh_size: u64, item_size: u64) {
        self.subheader_size.read(reader, sh_size);
        self.item_size.read(reader, item_size);
    }
}
impl Display for SubHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(
            f,
            "[subheader_size: {}, item_size: {}]",
            &self.subheader_size.string, &self.item_size.string
        );
    }
}
