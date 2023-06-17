//! Image segment definition
//!
//! Need to implement data mask - which also means need to implement some kind of nicer parsing (enums, among other things)
use std::fmt::Display;
use std::io::{Read, Seek};
use std::str::FromStr;

use crate::nitf::segments::headers::NitfSegmentHeader;
use crate::nitf::types::field::{InvalidNitfValue, NitfField};
use crate::nitf::types::security::Security;

/// Metadata for Image Segment subheader
#[allow(non_snake_case)]
#[derive(Default, Clone, Hash, Debug)]
pub struct ImageHeader {
    /// File Part Type
    pub IM: NitfField<String>,
    /// Image Identifier 1
    pub IID1: NitfField<String>,
    /// Image Date and Time
    pub IDATIM: NitfField<String>,
    /// Target Identifier
    pub TGTID: NitfField<String>,
    /// Image Identifier 2
    pub IID2: NitfField<String>,
    /// Security information
    pub SECURITY: Security,
    /// Encryption
    pub ENCRYP: NitfField<String>,
    /// Image Source
    pub ISORCE: NitfField<String>,
    /// Number of Significant Rows in image
    pub NROWS: NitfField<u32>,
    /// Number of Significant Columns in image
    pub NCOLS: NitfField<u32>,
    /// Pixel Value Type
    pub PVTYPE: NitfField<PixelValueType>,
    /// Image Representation
    pub IREP: NitfField<ImageRepresentation>,
    /// Image Category
    pub ICAT: NitfField<String>, // TODO: Check value registry
    /// Actual Bits-Per-Pixel Per Band
    pub ABPP: NitfField<u8>,
    /// Pixel Justification
    pub PJUST: NitfField<PixelJustification>,
    /// Image Coordinate Representation
    pub ICORDS: NitfField<CoordinateRepresentation>,
    /// Image Geographic Location
    pub IGEOLO: Vec<NitfField<String>>, // TODO: Check this out
    /// Number of Image Comments
    pub NICOM: NitfField<u8>,
    /// Image Comments
    pub ICOMS: Vec<NitfField<String>>,
    /// Image Compression
    pub IC: NitfField<Compression>,
    /// Compression Rate Code
    pub COMRAT: NitfField<String>, // TODO: Figure out how to parse this
    /// Number of Bands
    pub NBANDS: NitfField<u8>,
    /// Number of Multispectral Bands
    pub XBANDS: NitfField<u32>,
    /// Data bands
    pub BANDS: Vec<Band>,
    /// Image Sync Code
    pub ISYNC: NitfField<u8>,
    /// Image Mode
    pub IMODE: NitfField<Mode>,
    /// Number of Blocks per Row
    pub NBPR: NitfField<u16>,
    /// Number of Blocks per Column
    pub NBPC: NitfField<u16>,
    /// Number of Pixels Per Block Horizontal
    pub NPPBH: NitfField<u16>,
    /// Number of Pixels Per Block Vertical
    pub NPPBV: NitfField<u16>,
    /// Number of Bits Per Pixel
    pub NBPP: NitfField<u8>,
    /// Image Display Level
    pub IDLVL: NitfField<u16>,
    /// Image Attachment Level
    pub IALVL: NitfField<u16>,
    /// Image Location
    pub ILOC: NitfField<String>, // TODO fix this or something
    /// Image Magnification
    pub IMAG: NitfField<String>,
    /// User Defined Image Data Length
    pub UDIDL: NitfField<u32>,
    /// User Defined Overflow
    pub UDOFL: NitfField<u16>,
    /// User Defined Image Data
    pub UDID: NitfField<String>, // TODO
    /// Image Extended Subheader Data Length
    pub IXSHDL: NitfField<u32>,
    /// Image Extended Subheader Overflow
    pub IXSOFL: NitfField<u16>,
    /// Image Extended Subheader Data
    pub IXSHD: NitfField<String>, // TODO
}

/// Band metadata
#[allow(non_snake_case)]
#[derive(Default, Clone, Hash, Debug)]
pub struct Band {
    /// Band Representation
    pub IREPBAND: NitfField<String>, // TODO: Check how to do this
    /// Band Subcategory
    pub ISUBCAT: NitfField<String>, // User specified
    /// Band Image Filter Condition
    pub IFC: NitfField<String>, // Reserved for future use
    /// Band Standard Image Filter Code
    pub IMFLT: NitfField<String>, // Reserved for future use
    /// Number of Look-Up-Tables for the Image Band
    pub NLUTS: NitfField<u8>, //
    /// Number of Look-Up-Table Entries for the Image Band
    pub NELUT: NitfField<u16>,
    /// Image Band Look-Up-Tables
    pub LUTD: Vec<NitfField<u8>>,
}

/// Pixel Value type options
#[derive(Debug, Default, Hash, Clone)]
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
#[derive(Debug, Default, Hash, Clone)]
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
#[derive(Debug, Default, Hash, Clone)]
pub enum PixelJustification {
    #[default]
    /// Right justified
    R,
    /// Left justified
    L,
}

/// Coordinate representation
#[derive(Debug, Default, Hash, Clone)]
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
#[derive(Debug, Default, Hash, Clone)]
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
#[derive(Debug, Default, Hash, Clone)]
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
fn read_bands(reader: &mut (impl Read + Seek), n_band: u32) -> Vec<Band> {
    let mut bands: Vec<Band> = vec![Band::default(); n_band as usize];
    for band in &mut bands {
        band.IREPBAND.read(reader, 2u8);
        band.ISUBCAT.read(reader, 6u8);
        band.IFC.read(reader, 1u8);
        band.IMFLT.read(reader, 3u8);
        band.NLUTS.read(reader, 1u8);
        if band.NLUTS.val != 0 {
            band.NELUT.read(reader, 5u8);
            for _ in 0..band.NELUT.val {
                let mut lut: NitfField<u8> = NitfField::default();
                lut.read(reader, 1u8);
                band.LUTD.push(lut);
            }
        }
    }
    return bands;
}

// TRAIT IMPLEMENTATIONS
impl NitfSegmentHeader for ImageHeader {
    fn read(&mut self, reader: &mut (impl Read + Seek)) {
        self.IM.read(reader, 2u8);
        self.IID1.read(reader, 10u8);
        self.IDATIM.read(reader, 14u8);
        self.TGTID.read(reader, 17u8);
        self.IID2.read(reader, 80u8);
        self.SECURITY.read(reader);
        self.ENCRYP.read(reader, 1u8);
        self.ISORCE.read(reader, 42u8);
        self.NROWS.read(reader, 8u8);
        self.NCOLS.read(reader, 8u8);
        self.PVTYPE.read(reader, 3u8);
        self.IREP.read(reader, 8u8);
        self.ICAT.read(reader, 8u8);
        self.ABPP.read(reader, 2u8);
        self.PJUST.read(reader, 1u8);
        self.ICORDS.read(reader, 1u8);
        for _ in 0..4 {
            let mut geoloc: NitfField<String> = NitfField::default();
            geoloc.read(reader, 15u8);
            self.IGEOLO.push(geoloc);
        }
        self.NICOM.read(reader, 1u8);
        for _ in 0..self.NICOM.val {
            let mut comment: NitfField<String> = NitfField::default();
            comment.read(reader, 80u8);
            self.ICOMS.push(comment);
        }

        self.IC.read(reader, 2u8);
        self.NBANDS.read(reader, 1u8);
        // If NBANDS = 0, use XBANDS
        if self.NBANDS.val != 0 {
            self.BANDS = read_bands(reader, self.NBANDS.val as u32)
        } else {
            self.XBANDS.read(reader, 5u8);
            self.BANDS = read_bands(reader, self.XBANDS.val)
        }
        self.ISYNC.read(reader, 1u8);
        self.IMODE.read(reader, 1u8);
        self.NBPR.read(reader, 4u8);
        self.NBPC.read(reader, 4u8);
        self.NPPBH.read(reader, 4u8);
        self.NPPBV.read(reader, 4u8);
        self.NBPP.read(reader, 2u8);
        self.IDLVL.read(reader, 3u8);
        self.IALVL.read(reader, 3u8);
        self.ILOC.read(reader, 10u8);
        self.IMAG.read(reader, 4u8);
        self.UDIDL.read(reader, 5u8);
        let udi_data_length: u32 = self.UDIDL.string.parse().unwrap();
        if udi_data_length != 0 {
            self.UDOFL.read(reader, 3u8);
            self.UDID.read(reader, udi_data_length - 3);
        }
        self.IXSHDL.read(reader, 5u8);
        let ixsh_data_length: u32 = self.IXSHDL.string.parse().unwrap();
        if ixsh_data_length != 0 {
            self.IXSOFL.read(reader, 3u8);
            self.IXSHD.read(reader, ixsh_data_length - 3);
        }
    }
}
impl Display for ImageHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out_str = String::default();
        out_str += format!("IM: {}, ", self.IM).as_ref();
        out_str += format!("IID1: {}, ", self.IID1).as_ref();
        out_str += format!("IDATIM: {}, ", self.IDATIM).as_ref();
        out_str += format!("TGTID: {}, ", self.TGTID).as_ref();
        out_str += format!("IID2: {}, ", self.IID2).as_ref();
        out_str += format!("SECURITY: [{}], ", self.SECURITY).as_ref();
        out_str += format!("ENCRYP: {}, ", self.ENCRYP).as_ref();
        out_str += format!("ISORCE: {}, ", self.ISORCE).as_ref();
        out_str += format!("NROWS: {}, ", self.NROWS).as_ref();
        out_str += format!("NCOLS: {}, ", self.NCOLS).as_ref();
        out_str += format!("PVTYPE: {}, ", self.PVTYPE).as_ref();
        out_str += format!("IREP: {}, ", self.IREP).as_ref();
        out_str += format!("ICAT: {}, ", self.ICAT).as_ref();
        out_str += format!("ABPP: {}, ", self.ABPP).as_ref();
        out_str += format!("PJUST: {}, ", self.PJUST).as_ref();
        out_str += format!("ICORDS: {}, ", self.ICORDS).as_ref();
        for geolocation in &self.IGEOLO {
            out_str += format!("[GEOLO: {}], ", geolocation).as_ref();
        }
        out_str += format!("NICOM: {}, ", self.NICOM).as_ref();
        for comment in &self.ICOMS {
            out_str += format!("[ICOM: {}], ", comment).as_ref();
        }
        out_str += format!("IC: {}, ", self.IC).as_ref();
        out_str += format!("NBANDS: {}, ", self.NBANDS).as_ref();
        for band in &self.BANDS {
            out_str += format!("[BAND: {}], ", band).as_ref();
        }
        out_str += format!("ISYNC: {}, ", self.ISYNC).as_ref();
        out_str += format!("IMODE: {}, ", self.IMODE).as_ref();
        out_str += format!("NBPR: {}, ", self.NBPR).as_ref();
        out_str += format!("NBPC: {}, ", self.NBPC).as_ref();
        out_str += format!("NPPBH: {}, ", self.NPPBH).as_ref();
        out_str += format!("NPPBV: {}, ", self.NPPBV).as_ref();
        out_str += format!("NBPP: {}, ", self.NBPP).as_ref();
        out_str += format!("IDLVL: {}, ", self.IDLVL).as_ref();
        out_str += format!("IALVL: {}, ", self.IALVL).as_ref();
        out_str += format!("ILOC: {}, ", self.ILOC).as_ref();
        out_str += format!("IMAG: {}, ", self.IMAG).as_ref();
        out_str += format!("UDIDL: {}, ", self.UDIDL).as_ref();
        out_str += format!("UDOFL: {}, ", self.UDOFL).as_ref();
        out_str += format!("UDID: {}, ", self.UDID).as_ref();
        out_str += format!("IXSHDL: {}, ", self.IXSHDL).as_ref();
        out_str += format!("IXSOFL: {}, ", self.IXSOFL).as_ref();
        out_str += format!("IXSHD: {},", self.IXSHD).as_ref();
        return write!(f, "[Image Subheader: {}]", out_str);
    }
}
impl Display for Band {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out_str = String::default();
        out_str += format!("IREPBAND: {}, ", self.IREPBAND).as_ref();
        out_str += format!("ISUBCAT: {}, ", self.ISUBCAT).as_ref();
        out_str += format!("IFC: {}, ", self.IFC).as_ref();
        out_str += format!("IMFLT: {}, ", self.IMFLT).as_ref();
        out_str += format!("NLUTS: {}, ", self.NLUTS).as_ref();
        out_str += format!("NELUT: {}, ", self.NELUT).as_ref();
        for look_up in &self.LUTD {
            out_str += format!("LUTD: {}, ", look_up).as_ref();
        }
        return write!(f, "{}", out_str);
    }
}
impl FromStr for PixelValueType {
    type Err = InvalidNitfValue;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "INT" => Ok(Self::INT),
            "B" => Ok(Self::B),
            "SI" => Ok(Self::SI),
            "R" => Ok(Self::R),
            "C" => Ok(Self::C),
            _ => Err(InvalidNitfValue),
        }
    }
}
impl FromStr for ImageRepresentation {
    type Err = InvalidNitfValue;
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
            _ => Err(InvalidNitfValue),
        }
    }
}
impl FromStr for PixelJustification {
    type Err = InvalidNitfValue;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "R" => Ok(Self::R),
            "L" => Ok(Self::L),
            _ => Err(InvalidNitfValue),
        }
    }
}
impl FromStr for CoordinateRepresentation {
    type Err = InvalidNitfValue;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "" => Ok(Self::DEFAULT),
            "U" => Ok(Self::U),
            "N" => Ok(Self::N),
            "S" => Ok(Self::S),
            "P" => Ok(Self::P),
            "G" => Ok(Self::G),
            "D" => Ok(Self::D),
            _ => Err(InvalidNitfValue),
        }
    }
}
impl FromStr for Compression {
    type Err = InvalidNitfValue;
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
            _ => Err(InvalidNitfValue),
        }
    }
}
impl FromStr for Mode {
    type Err = InvalidNitfValue;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "B" => Ok(Self::B),
            "P" => Ok(Self::P),
            "R" => Ok(Self::R),
            "S" => Ok(Self::S),
            _ => Err(InvalidNitfValue),
        }
    }
}
