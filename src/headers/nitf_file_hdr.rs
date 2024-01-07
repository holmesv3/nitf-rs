//! File header definition
use std::fmt::Display;
use std::fs::File;

use crate::headers::NitfSegmentHeader;
use crate::types::{ExtendedSubheader, NitfField, Security};
use crate::{NitfError, NitfResult};
/// Metadata for Nitf File Header
#[derive(Default, Clone, Debug, Eq, PartialEq)]
pub struct NitfHeader {
    /// File Profile Name
    pub fhdr: NitfField<String>,
    /// File Version
    pub fver: NitfField<String>,
    /// Complexity Level
    pub clevel: NitfField<u8>,
    /// Standard Type
    pub stype: NitfField<String>,
    /// Originating Station ID
    pub ostaid: NitfField<String>,
    /// File Date and Time
    pub fdt: NitfField<String>,
    /// File Title
    pub ftitle: NitfField<String>,
    /// Security information
    pub security: Security,
    /// File Copy Number
    pub fscop: NitfField<u32>,
    /// File Number of Copies
    pub fscpys: NitfField<u32>,
    /// Encryption
    pub encryp: NitfField<String>,
    /// File Background Color
    pub fbkgc: Vec<NitfField<String>>, // TODO: Fix the parsing of this
    /// Originator's Name
    pub oname: NitfField<String>,
    /// Originator's Phone Number
    pub ophone: NitfField<String>,
    /// File Length
    pub fl: NitfField<u64>,
    /// NITF File Header Length
    pub hl: NitfField<u32>,
    /// Number of Image Segments
    pub numi: NitfField<u16>,
    /// Image Segments
    pub imheaders: Vec<SubHeader>,
    /// Number of Graphics Segments
    pub nums: NitfField<u16>,
    /// Graphic Segments
    pub graphheaders: Vec<SubHeader>,
    /// Reserved for future use
    pub numx: NitfField<u16>,
    /// Number of Text Segments
    pub numt: NitfField<u16>,
    /// Text Segments
    pub textheaders: Vec<SubHeader>,
    /// Number of Data Extension Segments
    pub numdes: NitfField<u16>,
    /// Data Extenstion Segments
    pub dextheaders: Vec<SubHeader>,
    /// Number of Reserved Extension Segments
    pub numres: NitfField<u16>,
    /// Reserved Extension Segments
    pub resheaders: Vec<SubHeader>,
    /// User Defined Header Data Length
    pub udhdl: NitfField<u32>,
    /// User Defined Header Overflow
    pub udhofl: NitfField<u16>,
    /// User Defined Header Data
    pub udhd: ExtendedSubheader, // TODO: Figure out what to do with this
    /// Extended Header Data Length
    pub xhdl: NitfField<u32>,
    /// Extended Header Data Overflow
    pub xhdlofl: NitfField<u16>,
    /// Extended Header Data
    pub xhd: ExtendedSubheader,
}
impl Display for NitfHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out_str = String::default();
        out_str += format!("FHDR: {}, ", self.fhdr).as_ref();
        out_str += format!("FVER: {}, ", self.fver).as_ref();
        out_str += format!("CLEVEL: {}, ", self.clevel).as_ref();
        out_str += format!("STYPE: {}, ", self.stype).as_ref();
        out_str += format!("OSTAID: {}, ", self.ostaid).as_ref();
        out_str += format!("FDT: {}, ", self.fdt).as_ref();
        out_str += format!("FTITLE: {}, ", self.ftitle).as_ref();
        out_str += format!("SECURITY: [{}], ", self.security).as_ref();
        out_str += format!("FSCOP: {}, ", self.fscop).as_ref();
        out_str += format!("FSCPYS: {}, ", self.fscpys).as_ref();
        out_str += format!("ENCRYP: {}, ", self.encryp).as_ref();
        out_str += format!(
            "FBKGC: [R: {}, G: {}, B: {}], ",
            self.fbkgc[0], self.fbkgc[1], self.fbkgc[2],
        )
        .as_ref();
        out_str += format!("ONAME: {}, ", self.oname).as_ref();
        out_str += format!("OPHONE: {}, ", self.ophone).as_ref();
        out_str += format!("FL: {}, ", self.fl).as_ref();
        out_str += format!("HL: {}, ", self.hl).as_ref();
        out_str += format!("NUMI: {}, ", self.numi).as_ref();
        for seg in &self.imheaders {
            out_str += format!("[IMHEADER: {}], ", seg).as_ref()
        }
        out_str += format!("NUMS: {}, ", self.nums).as_ref();
        for seg in &self.graphheaders {
            out_str += format!("[GRAPHHEADERS: {}], ", seg).as_ref()
        }
        out_str += format!("NUMX: {}, ", self.numx).as_ref();
        out_str += format!("NUMT: {}, ", self.numt).as_ref();
        for seg in &self.textheaders {
            out_str += format!("[TEXTHEADER: {}], ", seg).as_ref()
        }
        out_str += format!("NUMDES: {}, ", self.numdes).as_ref();
        for seg in &self.dextheaders {
            out_str += format!("[DEXTHEADER: {}], ", seg).as_ref()
        }
        out_str += format!("NUMRES: {}, ", self.numres).as_ref();
        for seg in &self.resheaders {
            out_str += format!("[RESHEADER: {}], ", seg).as_ref()
        }
        out_str += format!("UDHDL: {}, ", self.udhdl).as_ref();
        out_str += format!("UDHOFL: {}, ", self.udhofl).as_ref();
        out_str += format!("UDHD: {}, ", self.udhd).as_ref();
        out_str += format!("XHDL: {}, ", self.xhdl).as_ref();
        out_str += format!("XHDLOFL: {}, ", self.xhdlofl).as_ref();
        out_str += format!("XHD: {}", self.xhd).as_ref();
        write!(f, "[NitfHeader: {out_str}]")
    }
}

impl NitfSegmentHeader for NitfHeader {
    fn read(&mut self, reader: &mut File) -> NitfResult<()> {
        self.fhdr.read(reader, 4u8, "FHDR")?;
        // Crash if file header is not NITF
        if self.fhdr.string() != "NITF" {
            return Err(NitfError::FileType(self.fhdr.string().clone()));
        }
        self.fver.read(reader, 5u8, "FVER")?;
        self.clevel.read(reader, 2u8, "CLEVEL")?;
        self.stype.read(reader, 4u8, "STYPE")?;
        self.ostaid.read(reader, 10u8, "OSTAID")?;
        self.fdt.read(reader, 14u8, "FDT")?;
        self.ftitle.read(reader, 80u8, "FTITLE")?;
        self.security.read(reader)?;
        self.fscop.read(reader, 5u8, "FSCOP")?;
        self.fscpys.read(reader, 5u8, "FSCPYS")?;
        self.encryp.read(reader, 1u8, "ENCRYP")?;
        for _ in 0..3 {
            let mut color: NitfField<String> = NitfField::default();
            color.read(reader, 1u8, "FBKGC")?;
            self.fbkgc.push(color);
        }

        self.oname.read(reader, 24u8, "ONAME")?;
        self.ophone.read(reader, 18u8, "OPHONE")?;
        self.fl.read(reader, 12u8, "FL")?;
        self.hl.read(reader, 6u8, "HL")?;
        self.numi.read(reader, 3u8, "NUMI")?;
        for _ in 0..self.numi.val().clone() {
            let mut subheader = SubHeader::default();
            subheader.read(reader, 6, 10)?;
            self.imheaders.push(subheader);
        }

        self.nums.read(reader, 3u8, "NUMS")?;
        for _ in 0..self.nums.val().clone() {
            let mut subheader = SubHeader::default();
            subheader.read(reader, 4, 6)?;
            self.graphheaders.push(subheader);
        }

        self.numx.read(reader, 3u8, "NUMX")?;
        self.numt.read(reader, 3u8, "NUMT")?;
        for _ in 0..self.numt.val().clone() {
            let mut subheader = SubHeader::default();
            subheader.read(reader, 4, 5)?;
            self.textheaders.push(subheader);
        }

        self.numdes.read(reader, 3u8, "NUMDES")?;
        for _ in 0..self.numdes.val().clone() {
            let mut subheader = SubHeader::default();
            subheader.read(reader, 4, 9)?;
            self.dextheaders.push(subheader);
        }

        self.numres.read(reader, 3u8, "NUMRES")?;
        for _ in 0..self.numres.val().clone() {
            let mut subheader = SubHeader::default();
            subheader.read(reader, 4, 7)?;
            self.resheaders.push(subheader);
        }

        self.udhdl.read(reader, 5u8, "UDHDL")?;
        if self.udhdl.val().clone() != 0 {
            self.udhofl.read(reader, 3u8, "UDHOFL")?;
            self.udhd
                .read(reader, (self.udhdl.val() - 3) as usize, "UDHD")?;
        }

        self.xhdl.read(reader, 5u8, "XHDL")?;
        if self.xhdl.val().clone() != 0 {
            self.xhdlofl.read(reader, 3u8, "XHDLOFL")?;
            self.xhd.read(reader, (self.xhdl.val() - 3) as usize, "XHD")?;
        }
        Ok(())
    }
    fn write(&self, writer: &mut File) -> NitfResult<usize> {
        let mut bytes_written = 0;
        // Crash if file header is not NITF
        if self.fhdr.string() != "NITF" {
            return Err(NitfError::FileType(self.fhdr.string().clone()));
        }
        bytes_written += self.fhdr.write(writer, "FHDR")?;
        bytes_written += self.fver.write(writer, "FVER")?;
        bytes_written += self.clevel.write(writer, "CLEVEL")?;
        bytes_written += self.stype.write(writer, "STYPE")?;
        bytes_written += self.ostaid.write(writer, "OSTAID")?;
        bytes_written += self.fdt.write(writer, "FDT")?;
        bytes_written += self.ftitle.write(writer, "FTITLE")?;
        bytes_written += self.security.write(writer)?;
        bytes_written += self.fscop.write(writer, "FSCOP")?;
        bytes_written += self.fscpys.write(writer, "FSCPYS")?;
        bytes_written += self.encryp.write(writer, "ENCRYP")?;
        for color in &self.fbkgc {
            bytes_written += color.write(writer, "FBKGC")?;
        }
        bytes_written += self.oname.write(writer, "ONAME")?;
        bytes_written += self.ophone.write(writer, "OPHONE")?;
        bytes_written += self.fl.write(writer, "FL")?;
        bytes_written += self.hl.write(writer, "HL")?;
        bytes_written += self.numi.write(writer, "NUMI")?;
        for subheader in &self.imheaders {
            bytes_written += subheader.write(writer)?;
        }

        bytes_written += self.nums.write(writer, "NUMS")?;
        for subheader in &self.graphheaders {
            bytes_written += subheader.write(writer)?;
        }

        bytes_written += self.numx.write(writer, "NUMX")?;
        bytes_written += self.numt.write(writer, "NUMT")?;
        for subheader in &self.textheaders {
            bytes_written += subheader.write(writer)?;
        }

        bytes_written += self.numdes.write(writer, "NUMDES")?;
        for subheader in &self.dextheaders {
            bytes_written += subheader.write(writer)?;
        }

        bytes_written += self.numres.write(writer, "NUMRES")?;
        for subheader in &self.resheaders {
            bytes_written += subheader.write(writer)?;
        }

        bytes_written += self.udhdl.write(writer, "UDHDL")?;
        if self.udhdl.val().clone() != 0 {
            bytes_written += self.udhofl.write(writer, "UDHOFL")?;
            bytes_written += self.udhd.write(writer, "UDHD")?;
        }

        bytes_written += self.xhdl.write(writer, "XHDL")?;
        if self.xhdl.val().clone() != 0 {
            bytes_written += self.xhdlofl.write(writer, "XHDLOFL")?;
            bytes_written += self.xhd.write(writer, "XHD")?;
        }
        Ok(bytes_written)
    }
    fn length(&self) -> usize {
        let mut length = 0;
        length += self.fhdr.length();
        length += self.fver.length();
        length += self.clevel.length();
        length += self.stype.length();
        length += self.ostaid.length();
        length += self.fdt.length();
        length += self.ftitle.length();
        length += self.security.length();
        length += self.fscop.length();
        length += self.fscpys.length();
        length += self.encryp.length();
        length += self.fbkgc.iter().map(|c| c.length()).sum::<usize>();
        length += self.oname.length();
        length += self.ophone.length();
        length += self.fl.length();
        length += self.hl.length();
        length += self.numi.length();
        length += self.imheaders.iter().map(|s| s.length()).sum::<usize>();
        length += self.nums.length();
        length += self.graphheaders.iter().map(|s| s.length()).sum::<usize>();
        length += self.numx.length();
        length += self.numt.length();
        length += self.textheaders.iter().map(|s| s.length()).sum::<usize>();
        length += self.numdes.length();
        length += self.dextheaders.iter().map(|s| s.length()).sum::<usize>();
        length += self.numres.length();
        length += self.resheaders.iter().map(|s| s.length()).sum::<usize>();
        length += self.udhdl.length();
        if self.udhdl.val().clone() != 0 {
            length += self.udhofl.length();
            length += self.udhd.length();
        }
        length += self.xhdl.length();
        if self.xhdl.val().clone() != 0 {
            length += self.xhdlofl.length();
            length += self.xhd.length();
        }
        length
    }
}

/// Subheader element type
///
/// Used within the NITF header to denote the subheader segments contained in the file
#[derive(Default, Clone, Debug, Eq, PartialEq)]
pub struct SubHeader {
    /// Bytes of header description
    pub subheader_size: NitfField<u32>,
    /// Bytes of the data
    pub item_size: NitfField<u64>,
}
impl SubHeader {
    pub fn read(&mut self, reader: &mut File, sh_size: u64, item_size: u64) -> NitfResult<()> {
        self.subheader_size
            .read(reader, sh_size, "SUBHEADER_SIZE")?;
        self.item_size.read(reader, item_size, "ITEM_SIZE")?;
        Ok(())
    }
    pub fn write(&self, writer: &mut File) -> NitfResult<usize> {
        let mut bytes_written = 0;
        bytes_written += self.subheader_size.write(writer, "SUBHEADER_SIZE")?;
        bytes_written += self.item_size.write(writer, "ITEM_SIZE")?;
        Ok(bytes_written)
    }
    pub fn length(&self) -> usize {
        self.subheader_size.length() + self.item_size.length()
    }
}
impl Display for SubHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[subheader_size: {}, item_size: {}]",
            self.subheader_size.string(), self.item_size.string()
        )
    }
}
