//! Image segment definition
//! 
//! Need to implement data mask - which also means need to implement some kind of nicer parsing (enums, among other things)
use std::fmt::Display;
use std::io::{Read, Seek};
use std::str::FromStr;

use crate::nitf_2_1::types::*;

/// Metadata for Image Segment
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
    pub ICAT: NitfField<String>,  // TODO: Check value registry
    /// Actual Bits-Per-Pixel Per Band
    pub ABPP: NitfField<u8>,
    /// Pixel Justification
    pub PJUST: NitfField<PixelJustification>,
    /// Image Coordinate Representation
    pub ICORDS: NitfField<CoordinateRepresentation>,
    /// Image Geographic Location
    pub IGEOLO: Vec<NitfField<String>>,  // TODO: Check this out
    /// Number of Image Comments
    pub NICOM: NitfField<u8>,
    /// Image Comments
    pub ICOMS: Vec<NitfField<String>>,
    /// Image Compression
    pub IC: NitfField<Compression>,
    /// Compression Rate Code
    pub COMRAT: NitfField<String>,  // TODO: Figure out how to parse this
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
    pub ILOC: NitfField<String>,  // TODO fix this or something
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
    pub IXSHD: NitfField<String>,  // TODO
}

impl Display for ImageHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out_str = String::default();
        out_str += format!("IM: {},\n", self.IM).as_ref();
        out_str += format!("IID1: {},\n", self.IID1).as_ref();
        out_str += format!("IDATIM: {},\n", self.IDATIM).as_ref();
        out_str += format!("TGTID: {},\n", self.TGTID).as_ref();
        out_str += format!("IID2: {},\n", self.IID2).as_ref();
        out_str += format!("SECURITY: [\n{}],\n", self.SECURITY).as_ref();
        out_str += format!("ENCRYP: {},\n", self.ENCRYP).as_ref();
        out_str += format!("ISORCE: {},\n", self.ISORCE).as_ref();
        out_str += format!("NROWS: {},\n", self.NROWS).as_ref();
        out_str += format!("NCOLS: {},\n", self.NCOLS).as_ref();
        out_str += format!("PVTYPE: {},\n", self.PVTYPE).as_ref();
        out_str += format!("IREP: {},\n", self.IREP).as_ref();
        out_str += format!("ICAT: {},\n", self.ICAT).as_ref();
        out_str += format!("ABPP: {},\n", self.ABPP).as_ref();
        out_str += format!("PJUST: {},\n", self.PJUST).as_ref();
        out_str += format!("ICORDS: {},\n", self.ICORDS).as_ref();
        for geolocation in &self.IGEOLO {
            out_str += format!("\tGEOLO: [{}],\n", geolocation).as_ref();
        }
        out_str += format!("NICOM: {},\n", self.NICOM).as_ref();
        for comment in &self.ICOMS {
            out_str += format!("\tICOM: [{}], \n", comment).as_ref();
        }
        out_str += format!("IC: {},\n", self.IC).as_ref();
        out_str += format!("NBANDS: {},\n", self.NBANDS).as_ref();
        for band in &self.BANDS {
            out_str += format!("BAND: [\n{}],\n", band).as_ref();
        }
        out_str += format!("ISYNC: {},\n", self.ISYNC).as_ref();
        out_str += format!("IMODE: {},\n", self.IMODE).as_ref();
        out_str += format!("NBPR: {},\n", self.NBPR).as_ref();
        out_str += format!("NBPC: {},\n", self.NBPC).as_ref();
        out_str += format!("NPPBH: {},\n", self.NPPBH).as_ref();
        out_str += format!("NPPBV: {},\n", self.NPPBV).as_ref();
        out_str += format!("NBPP: {},\n", self.NBPP).as_ref();
        out_str += format!("IDLVL: {},\n", self.IDLVL).as_ref();
        out_str += format!("IALVL: {},\n", self.IALVL).as_ref();
        out_str += format!("ILOC: {},\n", self.ILOC).as_ref();
        out_str += format!("IMAG: {},\n", self.IMAG).as_ref();
        out_str += format!("UDIDL: {},\n", self.UDIDL).as_ref();
        out_str += format!("UDOFL: {},\n", self.UDOFL).as_ref();
        out_str += format!("UDID: [{}],\n", self.UDID).as_ref();
        out_str += format!("IXSHDL: {},\n", self.IXSHDL).as_ref();
        out_str += format!("IXSOFL: {},\n", self.IXSOFL).as_ref();
        out_str += format!("IXSHD: [{}],", self.IXSHD).as_ref();
        return write!(f, "Image Subheader: [{}]", out_str);
    }
}
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
            self.BANDS = bands_from_reader(reader, self.NBANDS.val as u32)
        } else {
            self.XBANDS.read(reader, 5u8);
            self.BANDS = bands_from_reader(reader, self.XBANDS.val)
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

/// Struct for Band metadata
#[allow(non_snake_case)]
#[derive(Default, Clone, Hash, Debug)]
pub struct Band {
    /// Band Representation
    pub IREPBAND: NitfField<String>,  // TODO: Check how to do this
    /// Band Subcategory
    pub ISUBCAT: NitfField<String>,  // User specified
    /// Band Image Filter Condition
    pub IFC: NitfField<String>,  // Reserved for future use 
    /// Band Standard Image Filter Code
    pub IMFLT: NitfField<String>,  // Reserved for future use
    /// Number of Look-Up-Tables for the Image Band
    pub NLUTS: NitfField<u8>,  // 
    /// Number of Look-Up-Table Entries for the Image Band
    pub NELUT: NitfField<u16>,
    /// Image Band Look-Up-Tables
    pub LUTD: Vec<NitfField<u8>>,
}

impl Display for Band {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out_str = String::default();
        out_str += format!("\tIREPBAND: {},\n", self.IREPBAND).as_ref();
        out_str += format!("\tISUBCAT: {},\n", self.ISUBCAT).as_ref();
        out_str += format!("\tIFC: {},\n", self.IFC).as_ref();
        out_str += format!("\tIMFLT: {},\n", self.IMFLT).as_ref();
        out_str += format!("\tNLUTS: {},\n", self.NLUTS).as_ref();
        out_str += format!("\tNELUT: {}, ", self.NELUT).as_ref();
        for look_up in &self.LUTD{
            out_str += format!("\n\t\tLUTD: [{}]", look_up).as_ref();
        }
        return write!(f, "{}", out_str);
    }
}

impl Band {
    pub fn read(&mut self, reader: &mut (impl Read + Seek)) {
        self.IREPBAND.read(reader, 2u8);
        self.ISUBCAT.read(reader, 6u8);
        self.IFC.read(reader, 1u8);
        self.IMFLT.read(reader, 3u8);
        self.NLUTS.read(reader, 1u8);
        if self.NLUTS.val != 0 {
            self.NELUT.read(reader, 5u8);
            for _ in 0..self.NELUT.val {
                let mut lut: NitfField<u8> = NitfField::default();
                lut.read(reader, 1u8);
                self.LUTD.push(lut);
            }
        }
    }
}

/// Helper function for parsing bands
fn bands_from_reader(reader: &mut (impl Read + Seek), n_band: u32) -> Vec<Band> {
    let mut bands: Vec<Band> = vec![Band::default(); n_band as usize];
    for band in &mut bands {
        band.read(reader)
    }
    return bands;
}

/// PVTYPE definition from standard
#[derive(Debug, Default, Hash, Clone)]
pub enum PixelValueType{
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

/// Enumeration for image representation values
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

/// Enumeration for pixel justification
#[derive(Debug, Default, Hash, Clone)]
pub enum PixelJustification {
    #[default]
    /// Right justified
    R,
    /// Left justified
    L,
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

/// Enumeration for coordinate representation
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

/// Enumeration for image compression values
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


/// Enumeration of image data storage mode
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

