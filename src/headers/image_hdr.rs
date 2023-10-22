//! Image segment definition
//!
//! Need to implement data mask - which also means need to implement some kind of nicer parsing (enums, among other things)
use std::fmt::Display;
use std::fs::File;
use std::str::FromStr;

use crate::error::NitfError;
use crate::headers::NitfSegmentHeader;
use crate::types::{NitfField, Security, ExtendedSubheader};

/// Metadata for Image Segment subheader
#[derive(Default, Clone, Debug, Eq, PartialEq)]
pub struct ImageHeader {
    /// File Part Type
    pub im: NitfField<String>,
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
    pub icat: NitfField<String>, // TODO: Check value registry
    /// Actual Bits-Per-Pixel Per Band
    pub abpp: NitfField<u8>,
    /// Pixel Justification
    pub pjust: NitfField<PixelJustification>,
    /// Image Coordinate Representation
    pub icords: NitfField<CoordinateRepresentation>,
    /// Image Geographic Location
    pub igeolo: Vec<NitfField<String>>, // TODO: Check this out
    /// Number of Image Comments
    pub nicom: NitfField<u8>,
    /// Image Comments
    pub icoms: Vec<NitfField<String>>,
    /// Image Compression
    pub ic: NitfField<Compression>,
    /// Compression Rate Code
    pub comrat: NitfField<String>, // TODO: Figure out how to parse this
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
    pub iloc: NitfField<String>, // TODO fix this or something
    /// Image Magnification
    pub imag: NitfField<String>,
    /// User Defined Image Data Length
    pub udidl: NitfField<u32>,
    /// User Defined Overflow
    pub udofl: NitfField<u16>,
    /// User Defined Image Data
    pub udid: NitfField<String>, // TODO
    /// Image Extended Subheader Data Length
    pub ixshdl: NitfField<u32>,
    /// Image Extended Subheader Overflow
    pub ixsofl: NitfField<u16>,
    /// Image Extended Subheader Data
    pub ixshd: ExtendedSubheader
}

/// Band metadata
#[derive(Default, Clone, Debug, Eq, PartialEq)]
pub struct Band {
    /// Band Representation
    pub irepband: NitfField<String>, // TODO: Check how to do this
    /// Band Subcategory
    pub isubcat: NitfField<String>, // User specified
    /// Band Image Filter Condition
    pub ifc: NitfField<String>, // Reserved for future use
    /// Band Standard Image Filter Code
    pub imflt: NitfField<String>, // Reserved for future use
    /// Number of Look-Up-Tables for the Image Band
    pub nluts: NitfField<u8>, //
    /// Number of Look-Up-Table Entries for the Image Band
    pub nelut: NitfField<u16>,
    /// Image Band Look-Up-Tables
    pub lutd: Vec<NitfField<u8>>,
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
fn read_bands(reader: &mut File, n_band: u32) -> Vec<Band> {
    let mut bands: Vec<Band> = vec![Band::default(); n_band as usize];
    for band in &mut bands {
        band.irepband.read(reader, 2u8);
        band.isubcat.read(reader, 6u8);
        band.ifc.read(reader, 1u8);
        band.imflt.read(reader, 3u8);
        band.nluts.read(reader, 1u8);
        if band.nluts.val != 0 {
            band.nelut.read(reader, 5u8);
            for _ in 0..band.nelut.val {
                let mut lut: NitfField<u8> = NitfField::default();
                lut.read(reader, 1u8);
                band.lutd.push(lut);
            }
        }
    }
    bands
}

// TRAIT IMPLEMENTATIONS
impl NitfSegmentHeader for ImageHeader {
    fn read(&mut self, reader: &mut File) {
        self.im.read(reader, 2u8);
        self.iid1.read(reader, 10u8);
        self.idatim.read(reader, 14u8);
        self.tgtid.read(reader, 17u8);
        self.iid2.read(reader, 80u8);
        self.security.read(reader);
        self.encryp.read(reader, 1u8);
        self.isorce.read(reader, 42u8);
        self.nrows.read(reader, 8u8);
        self.ncols.read(reader, 8u8);
        self.pvtype.read(reader, 3u8);
        self.irep.read(reader, 8u8);
        self.icat.read(reader, 8u8);
        self.abpp.read(reader, 2u8);
        self.pjust.read(reader, 1u8);
        self.icords.read(reader, 1u8);
        for _ in 0..4 {
            let mut geoloc: NitfField<String> = NitfField::default();
            geoloc.read(reader, 15u8);
            self.igeolo.push(geoloc);
        }
        self.nicom.read(reader, 1u8);
        for _ in 0..self.nicom.val {
            let mut comment: NitfField<String> = NitfField::default();
            comment.read(reader, 80u8);
            self.icoms.push(comment);
        }

        self.ic.read(reader, 2u8);
        self.nbands.read(reader, 1u8);
        // If NBANDS = 0, use XBANDS
        if self.nbands.val != 0 {
            self.bands = read_bands(reader, self.nbands.val as u32)
        } else {
            self.xbands.read(reader, 5u8);
            self.bands = read_bands(reader, self.xbands.val)
        }
        self.isync.read(reader, 1u8);
        self.imode.read(reader, 1u8);
        self.nbpr.read(reader, 4u8);
        self.nbpc.read(reader, 4u8);
        self.nppbh.read(reader, 4u8);
        self.nppbv.read(reader, 4u8);
        self.nbpp.read(reader, 2u8);
        self.idlvl.read(reader, 3u8);
        self.ialvl.read(reader, 3u8);
        self.iloc.read(reader, 10u8);
        self.imag.read(reader, 4u8);
        self.udidl.read(reader, 5u8);
        let udi_data_length = self.udidl.val;
        if udi_data_length != 0 {
            self.udofl.read(reader, 3u8);
            self.udid.read(reader, udi_data_length - 3);
        }
        self.ixshdl.read(reader, 5u8);
        let ixsh_data_length = self.ixshdl.val;
        if ixsh_data_length != 0 {
            self.ixsofl.read(reader, 3u8);
            self.ixshd.read(reader, (ixsh_data_length - 3) as usize);
            
        }
    }
}
impl Display for ImageHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out_str = String::default();
        out_str += format!("IM: {}, ", self.im).as_ref();
        out_str += format!("IID1: {}, ", self.iid1).as_ref();
        out_str += format!("IDATIM: {}, ", self.idatim).as_ref();
        out_str += format!("TGTID: {}, ", self.tgtid).as_ref();
        out_str += format!("IID2: {}, ", self.iid2).as_ref();
        out_str += format!("SECURITY: [{}], ", self.security).as_ref();
        out_str += format!("ENCRYP: {}, ", self.encryp).as_ref();
        out_str += format!("ISORCE: {}, ", self.isorce).as_ref();
        out_str += format!("NROWS: {}, ", self.nrows).as_ref();
        out_str += format!("NCOLS: {}, ", self.ncols).as_ref();
        out_str += format!("PVTYPE: {}, ", self.pvtype).as_ref();
        out_str += format!("IREP: {}, ", self.irep).as_ref();
        out_str += format!("ICAT: {}, ", self.icat).as_ref();
        out_str += format!("ABPP: {}, ", self.abpp).as_ref();
        out_str += format!("PJUST: {}, ", self.pjust).as_ref();
        out_str += format!("ICORDS: {}, ", self.icords).as_ref();
        for geolocation in &self.igeolo {
            out_str += format!("[GEOLO: {}], ", geolocation).as_ref();
        }
        out_str += format!("NICOM: {}, ", self.nicom).as_ref();
        for comment in &self.icoms {
            out_str += format!("[ICOM: {}], ", comment).as_ref();
        }
        out_str += format!("IC: {}, ", self.ic).as_ref();
        out_str += format!("NBANDS: {}, ", self.nbands).as_ref();
        for band in &self.bands {
            out_str += format!("[BAND: {}], ", band).as_ref();
        }
        out_str += format!("ISYNC: {}, ", self.isync).as_ref();
        out_str += format!("IMODE: {}, ", self.imode).as_ref();
        out_str += format!("NBPR: {}, ", self.nbpr).as_ref();
        out_str += format!("NBPC: {}, ", self.nbpc).as_ref();
        out_str += format!("NPPBH: {}, ", self.nppbh).as_ref();
        out_str += format!("NPPBV: {}, ", self.nppbv).as_ref();
        out_str += format!("NBPP: {}, ", self.nbpp).as_ref();
        out_str += format!("IDLVL: {}, ", self.idlvl).as_ref();
        out_str += format!("IALVL: {}, ", self.ialvl).as_ref();
        out_str += format!("ILOC: {}, ", self.iloc).as_ref();
        out_str += format!("IMAG: {}, ", self.imag).as_ref();
        out_str += format!("UDIDL: {}, ", self.udidl).as_ref();
        out_str += format!("UDOFL: {}, ", self.udofl).as_ref();
        out_str += format!("UDID: {}, ", self.udid).as_ref();
        out_str += format!("IXSHDL: {}, ", self.ixshdl).as_ref();
        if self.ixshdl.val != 0
        {
            out_str += format!("IXSOFL: {}, ", self.ixsofl).as_ref();
            out_str += format!("IXSHD: {}", self.ixshd).as_ref();
        }
        write!(f, "[Image Subheader: {out_str}]")
    }
}
impl Display for Band {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out_str = String::default();
        out_str += format!("IREPBAND: {}, ", self.irepband).as_ref();
        out_str += format!("ISUBCAT: {}, ", self.isubcat).as_ref();
        out_str += format!("IFC: {}, ", self.ifc).as_ref();
        out_str += format!("IMFLT: {}, ", self.imflt).as_ref();
        out_str += format!("NLUTS: {}, ", self.nluts).as_ref();
        out_str += format!("NELUT: {}, ", self.nelut).as_ref();
        for look_up in &self.lutd {
            out_str += format!("LUTD: {look_up}, ").as_ref();
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
            _ => Err(NitfError::FieldError),
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
            "NVECTOR" => Ok(Self::NVECTOR),
            "POLAR" => Ok(Self::POLAR),
            "VPH" => Ok(Self::VPH),
            "YCbCr601" => Ok(Self::YCbCr601),
            _ => Err(NitfError::FieldError),
        }
    }
}
impl FromStr for PixelJustification {
    type Err = NitfError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "R" => Ok(Self::R),
            "L" => Ok(Self::L),
            _ => Err(NitfError::FieldError),
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
            _ => Err(NitfError::FieldError),
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
            _ => Err(NitfError::FieldError),
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
            _ => Err(NitfError::FieldError),
        }
    }
}
