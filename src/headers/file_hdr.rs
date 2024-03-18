//! File header definition
use std::fmt::Display;
use std::io::{Read, Seek, Write};
use std::str::FromStr;

use crate::headers::NitfSegmentHeader;
use crate::types::{ExtendedSubheader, NitfField, Security};
use crate::{NitfError, NitfResult};
/// Metadata for Nitf File Header
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NitfHeader {
    /// File Profile Name
    pub fhdr: NitfField<FHDR>,
    /// File Version
    pub fver: NitfField<FVER>,
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

#[derive(Default, Clone, Debug, Eq, PartialEq)]
pub enum FHDR {
    #[default]
    NITF,
}
impl FromStr for FHDR {
    type Err = NitfError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "NITF" => Ok(Self::default()),
            _ => Err(NitfError::ParseError("FHDR".to_string())),
        }
    }
}
impl Display for FHDR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NITF")
    }
}

#[derive(Default, Clone, Debug, Eq, PartialEq)]
pub enum FVER {
    #[default]
    V02_10,
}
impl FromStr for FVER {
    type Err = NitfError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "02.10" => Ok(Self::default()),
            _ => Err(NitfError::ParseError("FVER".to_string())),
        }
    }
}
impl Display for FVER {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "02.10")
    }
}
pub(crate) enum Segment {
    Image,
    Graphic,
    Text,
    DataExtension,
    ReservedExtension,
}
impl Segment {
    pub fn size(&self) -> (u8, u8) {
        match self {
            Self::Image => (6u8, 10u8),
            Self::Graphic => (4u8, 6u8),
            Self::Text => (4u8, 5u8),
            Self::DataExtension => (4u8, 9u8),
            Self::ReservedExtension => (4u8, 7u8),
        }
    }
}

impl NitfHeader {
    pub(crate) fn write_header(
        &mut self,
        writer: &mut (impl Write + Seek),
        file_length: u64,
    ) -> NitfResult<usize> {
        self.hl.val = self.length() as u32;
        self.fl.val = file_length;
        self.write(writer)
    }
    pub(crate) fn add_subheader(
        &mut self,
        segment_type: Segment,
        subheader_size: u32,
        item_size: u64,
    ) {
        use Segment::*;
        let mut subheader = SubHeader::new(&segment_type);
        subheader.subheader_size.val = subheader_size;
        subheader.item_size.val = item_size;

        match segment_type {
            Image => {
                self.numi.val += 1;
                self.imheaders.push(subheader);
            }
            Graphic => {
                self.nums.val += 1;
                self.graphheaders.push(subheader);
            }
            Text => {
                self.numt.val += 1;
                self.textheaders.push(subheader);
            }
            DataExtension => {
                self.numdes.val += 1;
                self.dextheaders.push(subheader);
            }
            ReservedExtension => {
                self.numres.val += 1;
                self.resheaders.push(subheader);
            }
        }
    }
}
impl Default for NitfHeader {
    fn default() -> Self {
        Self {
            fhdr: NitfField::init(4u8, "FHDR"),
            fver: NitfField::init(5u8, "FVER"),
            clevel: NitfField::init(2u8, "CLEVEL"),
            stype: NitfField::init(4u8, "STYPE"),
            ostaid: NitfField::init(10u8, "OSTAID"),
            fdt: NitfField::init(14u8, "FDT"),
            ftitle: NitfField::init(80u8, "FTITLE"),
            security: Security::default(),
            fscop: NitfField::init(5u8, "FSCOP"),
            fscpys: NitfField::init(5u8, "FSCPYS"),
            encryp: NitfField::init(1u8, "ENCRYP"),
            fbkgc: vec![NitfField::init(1u8, "FBKGC"); 3],
            oname: NitfField::init(24u8, "ONAME"),
            ophone: NitfField::init(18u8, "OPHONE"),
            fl: NitfField::init(12u8, "FL"),
            hl: NitfField::init(6u8, "HL"),
            numi: NitfField::init(3u8, "NUMI"),
            imheaders: vec![],
            nums: NitfField::init(3u8, "NUMS"),
            graphheaders: vec![],
            numx: NitfField::init(3u8, "NUMX"),
            numt: NitfField::init(3u8, "NUMT"),
            textheaders: vec![],
            numdes: NitfField::init(3u8, "NUMDES"),
            dextheaders: vec![],
            numres: NitfField::init(3u8, "NUMRES"),
            resheaders: vec![],
            udhdl: NitfField::init(5u8, "UDHDL"),
            udhofl: NitfField::init(3u8, "UDHOFL"),
            udhd: ExtendedSubheader::init("UDHD"),
            xhdl: NitfField::init(5u8, "XHDL"),
            xhdlofl: NitfField::init(3u8, "XHDLOFL"),
            xhd: ExtendedSubheader::init("XHD"),
        }
    }
}
impl Display for NitfHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out_str = String::default();
        out_str += format!("{}, ", self.fhdr).as_ref();
        out_str += format!("{}, ", self.clevel).as_ref();
        out_str += format!("{}, ", self.stype).as_ref();
        out_str += format!("{}, ", self.ostaid).as_ref();
        out_str += format!("{}, ", self.fdt).as_ref();
        out_str += format!("{}, ", self.ftitle).as_ref();
        out_str += format!("SECURITY: [{}], ", self.security).as_ref();
        out_str += format!("{}, ", self.fscop).as_ref();
        out_str += format!("{}, ", self.fscpys).as_ref();
        out_str += format!("{}, ", self.encryp).as_ref();
        out_str += format!(
            "FBKGC: [R: {}, G: {}, B: {}], ",
            self.fbkgc[0], self.fbkgc[1], self.fbkgc[2],
        )
        .as_ref();
        out_str += format!("{}, ", self.oname).as_ref();
        out_str += format!("{}, ", self.ophone).as_ref();
        out_str += format!("{}, ", self.fl).as_ref();
        out_str += format!("{}, ", self.hl).as_ref();
        out_str += format!("{}, ", self.numi).as_ref();
        for (i_seg, seg) in self.imheaders.iter().enumerate() {
            out_str += format!("Image Segment {i_seg}: [{seg}], ").as_ref();
        }
        out_str += format!("{}, ", self.nums).as_ref();
        for (i_seg, seg) in self.graphheaders.iter().enumerate() {
            out_str += format!("Graphic Segment {i_seg}: [{seg}], ").as_ref();
        }
        out_str += format!("{}, ", self.numx).as_ref();
        out_str += format!("{}, ", self.numt).as_ref();
        for (i_seg, seg) in self.textheaders.iter().enumerate() {
            out_str += format!("Text Segment {i_seg}: [{seg}], ").as_ref();
        }
        out_str += format!("{}, ", self.numdes).as_ref();
        for (i_seg, seg) in self.dextheaders.iter().enumerate() {
            out_str += format!("Data Extension Segment {i_seg}: [{seg}], ").as_ref();
        }
        out_str += format!("{}, ", self.numres).as_ref();
        for (i_seg, seg) in self.resheaders.iter().enumerate() {
            out_str += format!("Reserved Extension Segment {i_seg}: [{seg}], ").as_ref();
        }
        out_str += format!("{}, ", self.udhdl).as_ref();
        out_str += format!("{}, ", self.udhofl).as_ref();
        out_str += format!("{}, ", self.udhd).as_ref();
        out_str += format!("{}, ", self.xhdl).as_ref();
        out_str += format!("{}, ", self.xhdlofl).as_ref();
        out_str += format!("{}", self.xhd).as_ref();
        write!(f, "Nitf Header: [{out_str}]")
    }
}

impl NitfSegmentHeader for NitfHeader {
    fn read(&mut self, reader: &mut (impl Read + Seek)) -> NitfResult<()> {
        self.fhdr.read(reader)?;
        self.fver.read(reader)?;
        self.clevel.read(reader)?;
        self.stype.read(reader)?;
        self.ostaid.read(reader)?;
        self.fdt.read(reader)?;
        self.ftitle.read(reader)?;
        self.security.read(reader)?;
        self.fscop.read(reader)?;
        self.fscpys.read(reader)?;
        self.encryp.read(reader)?;
        self.fbkgc = vec![NitfField::init(1u8, "FBKGC"); 3];
        self.fbkgc
            .iter_mut()
            .try_for_each(|color| color.read(reader))?;

        self.oname.read(reader)?;
        self.ophone.read(reader)?;
        self.fl.read(reader)?;
        self.hl.read(reader)?;
        self.numi.read(reader)?;
        self.imheaders = vec![SubHeader::new(&Segment::Image); self.numi.val.into()];
        self.imheaders
            .iter_mut()
            .try_for_each(|hdr| hdr.read(reader))?;

        self.nums.read(reader)?;
        self.graphheaders = vec![SubHeader::new(&Segment::Graphic); self.nums.val.into()];
        self.graphheaders
            .iter_mut()
            .try_for_each(|hdr| hdr.read(reader))?;

        self.numx.read(reader)?;
        self.numt.read(reader)?;
        self.textheaders = vec![SubHeader::new(&Segment::Text); self.numt.val.into()];
        self.textheaders
            .iter_mut()
            .try_for_each(|hdr| hdr.read(reader))?;

        self.numdes.read(reader)?;
        self.dextheaders = vec![SubHeader::new(&Segment::DataExtension); self.numdes.val.into()];
        self.dextheaders
            .iter_mut()
            .try_for_each(|hdr| hdr.read(reader))?;

        self.numres.read(reader)?;
        self.resheaders = vec![SubHeader::new(&Segment::ReservedExtension); self.numres.val.into()];
        self.resheaders
            .iter_mut()
            .try_for_each(|hdr| hdr.read(reader))?;

        self.udhdl.read(reader)?;
        if self.udhdl.val != 0 {
            self.udhofl.read(reader)?;
            self.udhd.read(reader, (self.udhdl.val - 3) as usize)?;
        }

        self.xhdl.read(reader)?;
        if self.xhdl.val != 0 {
            self.xhdlofl.read(reader)?;
            self.xhd.read(reader, (self.xhdl.val - 3) as usize)?;
        }
        Ok(())
    }
    fn write(&self, writer: &mut (impl Write + Seek)) -> NitfResult<usize> {
        let mut bytes_written = self.fhdr.write(writer)?;
        bytes_written += self.fver.write(writer)?;
        bytes_written += self.clevel.write(writer)?;
        bytes_written += self.stype.write(writer)?;
        bytes_written += self.ostaid.write(writer)?;
        bytes_written += self.fdt.write(writer)?;
        bytes_written += self.ftitle.write(writer)?;
        bytes_written += self.security.write(writer)?;
        bytes_written += self.fscop.write(writer)?;
        bytes_written += self.fscpys.write(writer)?;
        bytes_written += self.encryp.write(writer)?;
        for color in &self.fbkgc {
            bytes_written += color.write(writer)?;
        }
        bytes_written += self.oname.write(writer)?;
        bytes_written += self.ophone.write(writer)?;
        bytes_written += self.fl.write(writer)?;
        bytes_written += self.hl.write(writer)?;
        bytes_written += self.numi.write(writer)?;
        for subheader in &self.imheaders {
            bytes_written += subheader.write(writer)?;
        }

        bytes_written += self.nums.write(writer)?;
        for subheader in &self.graphheaders {
            bytes_written += subheader.write(writer)?;
        }

        bytes_written += self.numx.write(writer)?;
        bytes_written += self.numt.write(writer)?;
        for subheader in &self.textheaders {
            bytes_written += subheader.write(writer)?;
        }

        bytes_written += self.numdes.write(writer)?;
        for subheader in &self.dextheaders {
            bytes_written += subheader.write(writer)?;
        }

        bytes_written += self.numres.write(writer)?;
        for subheader in &self.resheaders {
            bytes_written += subheader.write(writer)?;
        }

        bytes_written += self.udhdl.write(writer)?;
        if self.udhdl.val != 0 {
            bytes_written += self.udhofl.write(writer)?;
            bytes_written += self.udhd.write(writer)?;
        }

        bytes_written += self.xhdl.write(writer)?;
        if self.xhdl.val != 0 {
            bytes_written += self.xhdlofl.write(writer)?;
            bytes_written += self.xhd.write(writer)?;
        }
        Ok(bytes_written)
    }
    fn length(&self) -> usize {
        let mut length = self.fhdr.length;
        length += self.fver.length;
        length += self.clevel.length;
        length += self.stype.length;
        length += self.ostaid.length;
        length += self.fdt.length;
        length += self.ftitle.length;
        length += self.security.length();
        length += self.fscop.length;
        length += self.fscpys.length;
        length += self.encryp.length;
        length += self.fbkgc.iter().map(|c| c.length).sum::<usize>();
        length += self.oname.length;
        length += self.ophone.length;
        length += self.fl.length;
        length += self.hl.length;
        length += self.numi.length;
        length += self.imheaders.iter().map(|s| s.length()).sum::<usize>();
        length += self.nums.length;
        length += self.graphheaders.iter().map(|s| s.length()).sum::<usize>();
        length += self.numx.length;
        length += self.numt.length;
        length += self.textheaders.iter().map(|s| s.length()).sum::<usize>();
        length += self.numdes.length;
        length += self.dextheaders.iter().map(|s| s.length()).sum::<usize>();
        length += self.numres.length;
        length += self.resheaders.iter().map(|s| s.length()).sum::<usize>();
        length += self.udhdl.length;
        if self.udhdl.val != 0 {
            length += self.udhofl.length;
            length += self.udhd.size();
        }
        length += self.xhdl.length;
        if self.xhdl.val != 0 {
            length += self.xhdlofl.length;
            length += self.xhd.size();
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
    pub fn init(subheader_len: u8, item_len: u8) -> Self {
        Self {
            subheader_size: NitfField::init(subheader_len, "SUBHEADER_SIZE"),
            item_size: NitfField::init(item_len, "ITEM_SIZE"),
        }
    }

    pub fn read(&mut self, reader: &mut (impl Read + Seek)) -> NitfResult<()> {
        self.subheader_size.read(reader)?;
        self.item_size.read(reader)?;
        Ok(())
    }

    pub fn write(&self, writer: &mut (impl Write + Seek)) -> NitfResult<usize> {
        let mut bytes_written = self.subheader_size.write(writer)?;
        bytes_written += self.item_size.write(writer)?;
        Ok(bytes_written)
    }

    pub fn length(&self) -> usize {
        self.subheader_size.length + self.item_size.length
    }

    pub(crate) fn new(segment_type: &Segment) -> Self {
        let (sh_size, item_size) = Segment::size(segment_type);
        SubHeader::init(sh_size, item_size)
    }
}
impl Display for SubHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}", self.subheader_size, self.item_size)
    }
}
