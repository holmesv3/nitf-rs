//! Image segment definition
//!
//! This is by far the most complicated part of the interface, and requires
//! a lot of manual action to setup properly. Future work will hopefully be done
//! to smooth out the process
use std::fmt::Display;
use std::io::{Read, Write, Seek};
use std::str::FromStr;

use crate::headers::NitfSegmentHeader;
use crate::types::{ExtendedSubheader, NitfField, Security};
use crate::{NitfError, NitfResult};

/// Metadata for Image Segment subheader
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ImageHeader {
    /// File Part Type
    pub im: NitfField<IM>,
    /// Image Identifier 1
    pub iid1: NitfField<String>,
    /// Image Date and Time
    pub idatim: NitfField<String>,
    /// Target Identifier
    pub tgtid: NitfField<String>,
    /// Image Identifier 2
    pub iid2: NitfField<String>,
    /// Security information
    pub security: Security,
    /// Encryption
    pub encryp: NitfField<String>,
    /// Image Source
    pub isorce: NitfField<String>,
    /// Number of Significant Rows in image
    pub nrows: NitfField<u32>,
    /// Number of Significant Columns in image
    pub ncols: NitfField<u32>,
    /// Pixel Value Type
    pub pvtype: NitfField<PixelValueType>,
    /// Image Representation
    pub irep: NitfField<ImageRepresentation>,
    /// Image Category
    pub icat: NitfField<String>,
    /// Actual Bits-Per-Pixel Per Band
    pub abpp: NitfField<u8>,
    /// Pixel Justification
    pub pjust: NitfField<PixelJustification>,
    /// Image Coordinate Representation
    pub icords: NitfField<CoordinateRepresentation>,
    /// Image Geographic Location
    pub igeolo: NitfField<String>,
    /// Number of Image Comments
    pub nicom: NitfField<u8>,
    /// Image Comments
    pub icoms: Vec<NitfField<String>>,
    /// Image Compression
    pub ic: NitfField<Compression>,
    /// Compression Rate Code
    pub comrat: NitfField<String>,
    /// Number of Bands
    pub nbands: NitfField<u8>,
    /// Number of Multispectral Bands
    pub xbands: NitfField<u32>,
    /// Data bands
    pub bands: Vec<Band>,
    /// Image Sync Code
    pub isync: NitfField<u8>,
    /// Image Mode
    pub imode: NitfField<Mode>,
    /// Number of Blocks per Row
    pub nbpr: NitfField<u16>,
    /// Number of Blocks per Column
    pub nbpc: NitfField<u16>,
    /// Number of Pixels Per Block Horizontal
    pub nppbh: NitfField<u16>,
    /// Number of Pixels Per Block Vertical
    pub nppbv: NitfField<u16>,
    /// Number of Bits Per Pixel
    pub nbpp: NitfField<u8>,
    /// Image Display Level
    pub idlvl: NitfField<u16>,
    /// Image Attachment Level
    pub ialvl: NitfField<u16>,
    /// Image Location
    pub iloc: NitfField<String>,
    /// Image Magnification
    pub imag: NitfField<String>,
    /// User Defined Image Data Length
    pub udidl: NitfField<u32>,
    /// User Defined Overflow
    pub udofl: NitfField<u16>,
    /// User Defined Image Data
    pub udid: ExtendedSubheader,
    /// Image Extended Subheader Data Length
    pub ixshdl: NitfField<u32>,
    /// Image Extended Subheader Overflow
    pub ixsofl: NitfField<u16>,
    /// Image Extended Subheader Data
    pub ixshd: ExtendedSubheader,
}
impl Default for ImageHeader {
    fn default() -> Self {
        Self {
            im: NitfField::init(2u8, "IM"),
            iid1: NitfField::init(10u8, "IID1"),
            idatim: NitfField::init(14u8, "IDATIM"),
            tgtid: NitfField::init(17u8, "TGTID"),
            iid2: NitfField::init(80u8, "IID2"),
            security: Security::default(),
            encryp: NitfField::init(1u8, "ENCRYP"),
            isorce: NitfField::init(42u8, "ISORCE"),
            nrows: NitfField::init(8u8, "NROWS"),
            ncols: NitfField::init(8u8, "NCOLS"),
            pvtype: NitfField::init(3u8, "PVTYPE"),
            irep: NitfField::init(8u8, "IREP"),
            icat: NitfField::init(8u8, "ICAT"),
            abpp: NitfField::init(2u8, "ABPP"),
            pjust: NitfField::init(1u8, "PJUST"),
            icords: NitfField::init(1u8, "ICORDS"),
            igeolo: NitfField::init(60u8, "IGEOLO"),
            nicom: NitfField::init(1u8, "NICOM"),
            icoms: vec![],
            ic: NitfField::init(2u8, "IC"),
            comrat: NitfField::init(4u8, "COMRAT"),
            nbands: NitfField::init(1u8, "NBANDS"),
            xbands: NitfField::init(5u8, "XBANDS"),
            bands: vec![],
            isync: NitfField::init(1u8, "ISYNC"),
            imode: NitfField::init(1u8, "IMODE"),
            nbpr: NitfField::init(4u8, "NBPR"),
            nbpc: NitfField::init(4u8, "NBPC"),
            nppbh: NitfField::init(4u8, "NPPBH"),
            nppbv: NitfField::init(4u8, "NPPBV"),
            nbpp: NitfField::init(2u8, "NBPP"),
            idlvl: NitfField::init(3u8, "IDLVL"),
            ialvl: NitfField::init(3u8, "IALVL"),
            iloc: NitfField::init(10u8, "ILOC"),
            imag: NitfField::init(4u8, "IMAG"),
            udidl: NitfField::init(5u8, "UDIDL"),
            udofl: NitfField::init(3u8, "UDOFL"),
            udid: ExtendedSubheader::init("UDID"),
            ixshdl: NitfField::init(5u8, "IXSHDL"),
            ixsofl: NitfField::init(3u8, "IXSOFL"),
            ixshd: ExtendedSubheader::init("IXSHD"),
        }
    }
}

#[derive(Default, Clone, Debug, Eq, PartialEq)]
pub enum IM {
    #[default]
    IM,
}
impl FromStr for IM {
    type Err = NitfError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "IM" => Ok(Self::default()),
            _ => Err(NitfError::ParseError("IM".to_string())),
        }
    }
}
impl Display for IM {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "IM")
    }
}

/// Band metadata
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Band {
    /// Band Representation
    pub irepband: NitfField<ImageRepresentationBand>,
    /// Band Subcategory
    pub isubcat: NitfField<String>, // User specified
    /// Band Image Filter Condition
    pub ifc: NitfField<ImageFilterCondition>, // Reserved for future use
    /// Band Standard Image Filter Code
    pub imflt: NitfField<String>, // Reserved for future use
    /// Number of Look-Up-Tables for the Image Band
    pub nluts: NitfField<u8>, //
    /// Number of Look-Up-Table Entries for the Image Band
    pub nelut: NitfField<u16>,
    /// Image Band Look-Up-Tables
    pub lutd: Vec<NitfField<u8>>,
}
impl Default for Band {
    fn default() -> Self {
        Self {
            irepband: NitfField::init(2u8, "IREPBAND"),
            isubcat: NitfField::init(6u8, "ISUBCAT"),
            ifc: NitfField::init(1u8, "IFC"),
            imflt: NitfField::init(3u8, "IMFLT"),
            nluts: NitfField::init(1u8, "NLUTS"),
            nelut: NitfField::init(5u8, "NELUT"),
            lutd: vec![],
        }
    }
}
impl Band {
    pub fn length(&self) -> usize {
        let mut length = 0;
        length += self.irepband.length;
        length += self.isubcat.length;
        length += self.ifc.length;
        length += self.imflt.length;
        length += self.nluts.length;
        if self.nluts.val != 0 {
            length += self.nelut.length;
            length += self.lutd.len(); // each element is 1 byte,
        }
        length
    }
}

/// Pixel Value type options
#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub enum PixelValueType {
    #[default]
    /// ComplexFloat, 32 or 64 bits, real then imaginary
    C,
    /// Float, 32 or 64 bits
    R,
    /// Bi-level, single bit
    B,
    /// 2's complement signed integer, 8, 12, 16, 32, or 64 bits
    SI,
    /// Integer, 8, 12, 16, 32, or 64 bits
    INT,
}

/// Image representation values
#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub enum ImageRepresentation {
    #[default]
    /// Monochrome
    MONO,
    /// RGB true color
    RGB,
    /// RGB/LUT for mapped color
    RGBLUT,
    /// Multiband imagery
    MULTI,
    /// Not intended for display
    NODISPLY,
    /// Vectors with cartesian coordinates
    NVECTOR,
    /// Vectors with polar coordinates
    POLAR,
    /// SAR video phase history
    VPH,
    /// Compressed in the ITU-R recommendation
    YCbCr601,
}
/// Image representation values
#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub enum ImageRepresentationBand {
    #[default]
    /// Default spaces
    DEFAULT,
    /// Monochrome
    M,
    /// RGB
    R,
    G,
    B,
    /// LUT Reperesentation
    LU,
    /// Luminance
    Y,
    /// Chrominance Blue
    Cb,
    /// Chrominance Red
    Cr,
}
#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub enum ImageFilterCondition {
    #[default]
    /// None
    N,
}

/// Pixel justification
#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub enum PixelJustification {
    #[default]
    /// Right justified
    R,
    /// Left justified
    L,
}

/// Coordinate representation
#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub enum CoordinateRepresentation {
    #[default]
    /// Default value, one space
    DEFAULT,
    /// UTM in Military Grid Reference System
    U,
    /// UTM/UPS Northern hemisphere
    N,
    /// UTM/UPS Southern hemisphere
    S,
    /// UPS
    P,
    /// Geographic
    G,
    /// Decimal degrees
    D,
}

/// Image compression values
#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub enum Compression {
    #[default]
    /// Not compressed
    NC,
    /// Uncompressed, contains mask
    NM,
    /// Bi-level
    C1,
    /// JPEG
    C3,
    /// Vector Quantization
    C4,
    /// Lossless JPEG
    C5,
    /// Reserved for future compression algorithm
    C6,
    /// Resrved for future complex SAR compression
    C7,
    /// ISO JPEG 2000
    C8,
    /// Downsampled JPEG
    I1,
    /// Compressed, contains mask
    M1,
    /// Compressed, contains mask
    M3,
    /// Compressed, contains mask
    M4,
    /// Compressed, contains mask
    M5,
    /// Reserved for future compression algorithm
    M6,
    /// Resrved for future complex SAR compression
    M7,
    /// ISO JPEG 2000
    M8,
}

/// Image data storage mode
#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub enum Mode {
    #[default]
    /// Band interleaved by block
    B,
    /// Band interleaved by pixel
    P,
    /// Band interleaved by row
    R,
    /// Band sequential
    S,
}

// FUNCTIONS
/// Helper function for parsing bands
fn read_bands(reader: &mut (impl Read + Seek), n_band: u32) -> NitfResult<Vec<Band>> {
    let mut bands: Vec<Band> = vec![Band::default(); n_band as usize];
    for band in &mut bands {
        band.irepband.read(reader)?;
        band.isubcat.read(reader)?;
        band.ifc.read(reader)?;
        band.imflt.read(reader)?;
        band.nluts.read(reader)?;
        if band.nluts.val != 0 {
            band.nelut.read(reader)?;
        }
        let n_lutd = (band.nelut.val * band.nluts.val as u16) as usize;
        band.lutd = vec![NitfField::init(1u8, "LUDT"); n_lutd];
        for lut in band.lutd.iter_mut() {
            lut.read(reader)?;
        }
    }
    Ok(bands)
}
/// Helper function for writing bands
fn write_bands(writer: &mut (impl Write + Seek), bands: &Vec<Band>) -> NitfResult<usize> {
    let mut bytes_written = 0;
    for band in bands {
        bytes_written += band.irepband.write(writer)?;
        bytes_written += band.isubcat.write(writer)?;
        bytes_written += band.ifc.write(writer)?;
        bytes_written += band.imflt.write(writer)?;
        bytes_written += band.nluts.write(writer)?;
        if band.nluts.val != 0 {
            band.nelut.write(writer)?;
        }
        // Assumes luts are setup properly
        for lut in band.lutd.iter() {
            bytes_written += lut.write(writer)?;
        }
    }
    Ok(bytes_written)
}
fn is_comrat(compression: &Compression) -> bool {
    matches!(
        compression,
        Compression::C1
            | Compression::C3
            | Compression::C4
            | Compression::C5
            | Compression::C8
            | Compression::M1
            | Compression::M3
            | Compression::M4
            | Compression::M5
            | Compression::M8
            | Compression::I1
    )
}

// TRAIT IMPLEMENTATIONS
impl NitfSegmentHeader for ImageHeader {
    fn read(&mut self, reader: &mut (impl Read + Seek)) -> NitfResult<()> {
        self.im.read(reader)?;
        self.iid1.read(reader)?;
        self.idatim.read(reader)?;
        self.tgtid.read(reader)?;
        self.iid2.read(reader)?;
        self.security.read(reader)?;
        self.encryp.read(reader)?;
        self.isorce.read(reader)?;
        self.nrows.read(reader)?;
        self.ncols.read(reader)?;
        self.pvtype.read(reader)?;
        self.irep.read(reader)?;
        self.icat.read(reader)?;
        self.abpp.read(reader)?;
        self.pjust.read(reader)?;
        self.icords.read(reader)?;
        self.igeolo.read(reader)?;
        self.nicom.read(reader)?;
        self.icoms = vec![NitfField::init(80u8, "ICOM"); self.nicom.val.into()];
        self.icoms.iter_mut().try_for_each(|com| com.read(reader))?;

        self.ic.read(reader)?;
        if is_comrat(&self.ic.val) {
            self.comrat.read(reader)?;
        }
        self.nbands.read(reader)?;
        // If NBANDS = 0, use XBANDS
        if self.nbands.val != 0 {
            self.bands = read_bands(reader, self.nbands.val as u32)?;
        } else {
            self.xbands.read(reader)?;
            self.bands = read_bands(reader, self.xbands.val)?;
        }
        self.isync.read(reader)?;
        self.imode.read(reader)?;
        self.nbpr.read(reader)?;
        self.nbpc.read(reader)?;
        self.nppbh.read(reader)?;
        self.nppbv.read(reader)?;
        self.nbpp.read(reader)?;
        self.idlvl.read(reader)?;
        self.ialvl.read(reader)?;
        self.iloc.read(reader)?;
        self.imag.read(reader)?;
        self.udidl.read(reader)?;
        if self.udidl.val != 0 {
            self.udofl.read(reader)?;
            self.udid.read(reader, (self.udidl.val - 3) as usize)?;
        }
        self.ixshdl.read(reader)?;
        if self.ixshdl.val != 0 {
            self.ixsofl.read(reader)?;
            self.ixshd.read(reader, (self.ixshdl.val - 3) as usize)?;
        }
        Ok(())
    }
    fn write(&self, writer: &mut (impl Write + Seek)) -> NitfResult<usize> {
        let mut bytes_written = 0;
        bytes_written += self.im.write(writer)?;
        bytes_written += self.iid1.write(writer)?;
        bytes_written += self.idatim.write(writer)?;
        bytes_written += self.tgtid.write(writer)?;
        bytes_written += self.iid2.write(writer)?;
        bytes_written += self.security.write(writer)?;
        bytes_written += self.encryp.write(writer)?;
        bytes_written += self.isorce.write(writer)?;
        bytes_written += self.nrows.write(writer)?;
        bytes_written += self.ncols.write(writer)?;
        bytes_written += self.pvtype.write(writer)?;
        bytes_written += self.irep.write(writer)?;
        bytes_written += self.icat.write(writer)?;
        bytes_written += self.abpp.write(writer)?;
        bytes_written += self.pjust.write(writer)?;
        bytes_written += self.icords.write(writer)?;
        bytes_written += self.igeolo.write(writer)?;
        self.nicom.write(writer)?;
        for comment in &self.icoms {
            bytes_written += comment.write(writer)?;
        }

        bytes_written += self.ic.write(writer)?;
        if is_comrat(&self.ic.val) {
            bytes_written += self.comrat.write(writer)?;
        }
        bytes_written += self.nbands.write(writer)?;
        // If NBANDS = 0, use XBANDS
        if self.nbands.val == 0 {
            bytes_written += self.xbands.write(writer)?;
        }
        bytes_written += write_bands(writer, &self.bands)?;
        bytes_written += self.isync.write(writer)?;
        bytes_written += self.imode.write(writer)?;
        bytes_written += self.nbpr.write(writer)?;
        bytes_written += self.nbpc.write(writer)?;
        bytes_written += self.nppbh.write(writer)?;
        bytes_written += self.nppbv.write(writer)?;
        bytes_written += self.nbpp.write(writer)?;
        bytes_written += self.idlvl.write(writer)?;
        bytes_written += self.ialvl.write(writer)?;
        bytes_written += self.iloc.write(writer)?;
        bytes_written += self.imag.write(writer)?;
        bytes_written += self.udidl.write(writer)?;
        if self.udidl.val != 0 {
            bytes_written += self.udofl.write(writer)?;
            bytes_written += self.udid.write(writer)?;
        }
        bytes_written += self.ixshdl.write(writer)?;
        if self.ixshdl.val != 0 {
            bytes_written += self.ixsofl.write(writer)?;
            bytes_written += self.ixshd.write(writer)?;
        }
        Ok(bytes_written)
    }
    fn length(&self) -> usize {
        let mut length = 0;
        length += self.im.length;
        length += self.iid1.length;
        length += self.idatim.length;
        length += self.tgtid.length;
        length += self.iid2.length;
        length += self.security.length();
        length += self.encryp.length;
        length += self.isorce.length;
        length += self.nrows.length;
        length += self.ncols.length;
        length += self.pvtype.length;
        length += self.irep.length;
        length += self.icat.length;
        length += self.abpp.length;
        length += self.pjust.length;
        length += self.icords.length;
        length += self.igeolo.length;
        length += self.nicom.length;
        for comment in &self.icoms {
            length += comment.length;
        }

        length += self.ic.length;
        if is_comrat(&self.ic.val) {
            length += self.comrat.length;
        }
        length += self.nbands.length;
        // If NBANDS = 0, use XBANDS
        if self.nbands.val == 0 {
            length += self.xbands.length;
        }
        length += &self.bands.iter().map(|b| b.length()).sum();
        length += self.isync.length;
        length += self.imode.length;
        length += self.nbpr.length;
        length += self.nbpc.length;
        length += self.nppbh.length;
        length += self.nppbv.length;
        length += self.nbpp.length;
        length += self.idlvl.length;
        length += self.ialvl.length;
        length += self.iloc.length;
        length += self.imag.length;
        length += self.udidl.length;
        length += self.udidl.val as usize;
        length += self.ixshdl.length;
        length += self.ixshdl.val as usize;
        length
    }
}
impl Display for ImageHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out_str = String::default();
        out_str += format!("{}, ", self.im).as_ref();
        out_str += format!("{}, ", self.iid1).as_ref();
        out_str += format!("{}, ", self.idatim).as_ref();
        out_str += format!("{}, ", self.tgtid).as_ref();
        out_str += format!("{}, ", self.iid2).as_ref();
        out_str += format!("SECURITY: [{}], ", self.security).as_ref();
        out_str += format!("{}, ", self.encryp).as_ref();
        out_str += format!("{}, ", self.isorce).as_ref();
        out_str += format!("{}, ", self.nrows).as_ref();
        out_str += format!("{}, ", self.ncols).as_ref();
        out_str += format!("{}, ", self.pvtype).as_ref();
        out_str += format!("{}, ", self.irep).as_ref();
        out_str += format!("{}, ", self.icat).as_ref();
        out_str += format!("{}, ", self.abpp).as_ref();
        out_str += format!("{}, ", self.pjust).as_ref();
        out_str += format!("{}, ", self.icords).as_ref();
        out_str += format!("{}, ", self.igeolo).as_ref();
        out_str += format!("{}, ", self.nicom).as_ref();
        for (i_com, com) in self.icoms.iter().enumerate() {
            out_str += format!("ICOM {i_com}: {com},",).as_ref();
        }
        out_str += format!("{}, ", self.ic).as_ref();
        if is_comrat(&self.ic.val) {
            out_str += format!("{}, ", self.comrat).as_ref();
        }
        out_str += format!("{}, ", self.nbands).as_ref();
        for (i_band, band) in self.bands.iter().enumerate() {
            out_str += format!("BAND {i_band}: [{band}], ").as_ref();
        }
        out_str += format!("{}, ", self.isync).as_ref();
        out_str += format!("{}, ", self.imode).as_ref();
        out_str += format!("{}, ", self.nbpr).as_ref();
        out_str += format!("{}, ", self.nbpc).as_ref();
        out_str += format!("{}, ", self.nppbh).as_ref();
        out_str += format!("{}, ", self.nppbv).as_ref();
        out_str += format!("{}, ", self.nbpp).as_ref();
        out_str += format!("{}, ", self.idlvl).as_ref();
        out_str += format!("{}, ", self.ialvl).as_ref();
        out_str += format!("{}, ", self.iloc).as_ref();
        out_str += format!("{}, ", self.imag).as_ref();
        out_str += format!("{}, ", self.udidl).as_ref();
        if self.udidl.val != 0 {
            out_str += format!("{}, ", self.udofl).as_ref();
            out_str += format!("{}, ", self.udid).as_ref();
        }
        out_str += format!("{}, ", self.ixshdl).as_ref();
        if self.ixshdl.val != 0 {
            out_str += format!("{}, ", self.ixsofl).as_ref();
            out_str += format!("{}", self.ixshd).as_ref();
        }
        write!(f, "Image Header: {out_str}")
    }
}
impl Display for Band {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out_str = String::default();
        out_str += format!("{}, ", self.irepband).as_ref();
        out_str += format!("{}, ", self.isubcat).as_ref();
        out_str += format!("{}, ", self.ifc).as_ref();
        out_str += format!("{}, ", self.imflt).as_ref();
        out_str += format!("{}, ", self.nluts).as_ref();
        if self.nluts.val != 0 {
            for i_lut in 0..self.nluts.val {
                out_str += format!("{}, ", self.nelut).as_ref();
                for (i_elem, lut) in self.lutd.iter().enumerate() {
                    out_str += format!("LUT{i_lut}{i_elem}: {}", lut.val).as_ref();
                }
            }
        }
        write!(f, "{out_str}")
    }
}
impl FromStr for PixelValueType {
    type Err = NitfError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "INT" => Ok(Self::INT),
            "B" => Ok(Self::B),
            "SI" => Ok(Self::SI),
            "R" => Ok(Self::R),
            "C" => Ok(Self::C),
            _ => Err(NitfError::ParseError("PixelValueType".to_string())),
        }
    }
}
impl Display for PixelValueType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::INT => write!(f, "INT"),
            Self::B => write!(f, "B"),
            Self::SI => write!(f, "SI"),
            Self::R => write!(f, "R"),
            Self::C => write!(f, "C"),
        }
    }
}
impl FromStr for ImageRepresentation {
    type Err = NitfError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "MONO" => Ok(Self::MONO),
            "RGB" => Ok(Self::RGB),
            "RGBLUT" => Ok(Self::RGBLUT),
            "MULTI" => Ok(Self::MULTI),
            "NODISPLY" => Ok(Self::NODISPLY),
            "" => Ok(Self::NVECTOR),
            "POLAR" => Ok(Self::POLAR),
            "VPH" => Ok(Self::VPH),
            "YCbCr601" => Ok(Self::YCbCr601),
            _ => Err(NitfError::ParseError("ImageRepresentation".to_string())),
        }
    }
}
impl Display for ImageRepresentation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MONO => write!(f, "MONO"),
            Self::RGB => write!(f, "RGB"),
            Self::RGBLUT => write!(f, "RGBLUT"),
            Self::MULTI => write!(f, "MULTI"),
            Self::NODISPLY => write!(f, "NODISPLY"),
            Self::NVECTOR => write!(f, "NVECTOR"),
            Self::POLAR => write!(f, "POLAR"),
            Self::VPH => write!(f, "VPH"),
            Self::YCbCr601 => write!(f, "YCbCr601"),
        }
    }
}
impl FromStr for ImageRepresentationBand {
    type Err = NitfError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "" => Ok(Self::DEFAULT),
            "M" => Ok(Self::M),
            "R" => Ok(Self::R),
            "G" => Ok(Self::G),
            "B" => Ok(Self::B),
            "LU" => Ok(Self::LU),
            "Y" => Ok(Self::Y),
            "Cb" => Ok(Self::Cb),
            "Cr" => Ok(Self::Cr),
            _ => Err(NitfError::ParseError("ImageRepresentationBand".to_string())),
        }
    }
}
impl Display for ImageRepresentationBand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DEFAULT => write!(f, ""),
            Self::M => write!(f, "M"),
            Self::R => write!(f, "R"),
            Self::G => write!(f, "G"),
            Self::B => write!(f, "B"),
            Self::LU => write!(f, "LU"),
            Self::Y => write!(f, "Y"),
            Self::Cb => write!(f, "Cb"),
            Self::Cr => write!(f, "Cr"),
        }
    }
}
impl FromStr for ImageFilterCondition {
    type Err = NitfError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "N" => Ok(Self::N),
            _ => Err(NitfError::ParseError("PixelJustification".to_string())),
        }
    }
}
impl Display for ImageFilterCondition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::N => write!(f, "N"),
        }
    }
}
impl FromStr for PixelJustification {
    type Err = NitfError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "R" => Ok(Self::R),
            "L" => Ok(Self::L),
            _ => Err(NitfError::ParseError("PixelJustification".to_string())),
        }
    }
}
impl Display for PixelJustification {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::R => write!(f, "R"),
            Self::L => write!(f, "L"),
        }
    }
}
impl FromStr for CoordinateRepresentation {
    type Err = NitfError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "" => Ok(Self::DEFAULT),
            "U" => Ok(Self::U),
            "N" => Ok(Self::N),
            "S" => Ok(Self::S),
            "P" => Ok(Self::P),
            "G" => Ok(Self::G),
            "D" => Ok(Self::D),
            _ => Err(NitfError::ParseError(
                "CoordinateRepresentation".to_string(),
            )),
        }
    }
}
impl Display for CoordinateRepresentation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DEFAULT => write!(f, ""),
            Self::U => write!(f, "U"),
            Self::N => write!(f, "N"),
            Self::S => write!(f, "S"),
            Self::P => write!(f, "P"),
            Self::G => write!(f, "G"),
            Self::D => write!(f, "D"),
        }
    }
}
impl FromStr for Compression {
    type Err = NitfError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "NC" => Ok(Self::NC),
            "NM" => Ok(Self::NM),
            "C1" => Ok(Self::C1),
            "C3" => Ok(Self::C3),
            "C4" => Ok(Self::C4),
            "C5" => Ok(Self::C5),
            "C6" => Ok(Self::C6),
            "C7" => Ok(Self::C7),
            "C8" => Ok(Self::C8),
            "I1" => Ok(Self::I1),
            "M1" => Ok(Self::M1),
            "M3" => Ok(Self::M3),
            "M4" => Ok(Self::M4),
            "M5" => Ok(Self::M5),
            "M6" => Ok(Self::M6),
            "M7" => Ok(Self::M7),
            "M8" => Ok(Self::M8),
            _ => Err(NitfError::ParseError("Compression".to_string())),
        }
    }
}
impl Display for Compression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NC => write!(f, "NC"),
            Self::NM => write!(f, "NM"),
            Self::C1 => write!(f, "C1"),
            Self::C3 => write!(f, "C3"),
            Self::C4 => write!(f, "C4"),
            Self::C5 => write!(f, "C5"),
            Self::C6 => write!(f, "C6"),
            Self::C7 => write!(f, "C7"),
            Self::C8 => write!(f, "C8"),
            Self::I1 => write!(f, "I1"),
            Self::M1 => write!(f, "M1"),
            Self::M3 => write!(f, "M3"),
            Self::M4 => write!(f, "M4"),
            Self::M5 => write!(f, "M5"),
            Self::M6 => write!(f, "M6"),
            Self::M7 => write!(f, "M7"),
            Self::M8 => write!(f, "M8"),
        }
    }
}
impl FromStr for Mode {
    type Err = NitfError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "B" => Ok(Self::B),
            "P" => Ok(Self::P),
            "R" => Ok(Self::R),
            "S" => Ok(Self::S),
            _ => Err(NitfError::ParseError("Mode".to_string())),
        }
    }
}
impl Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::B => write!(f, "B"),
            Self::P => write!(f, "P"),
            Self::R => write!(f, "R"),
            Self::S => write!(f, "S"),
        }
    }
}
