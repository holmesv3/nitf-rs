//! Image header definition
use std::fmt::Display;
use std::io::{Read, Seek};

use crate::nitf_2_1::types::*;

#[allow(non_snake_case)]
#[derive(Default, Clone, Hash, Debug)]
pub struct ImageSegmentHeader {
    /// File Part Type
    pub IM: NitfField,
    /// Image Identifier 1
    pub IID1: NitfField,
    /// Image Date and Time
    pub IDATIM: NitfField,
    /// Target Identifier
    pub TGTID: NitfField,
    /// Image Identifier 2
    pub IID2: NitfField,
    /// Security information
    pub SECURITY: Security,
    /// Encryption
    pub ENCRYP: NitfField,
    /// Image Source
    pub ISORCE: NitfField,
    /// Number of Significant Rows in image
    pub NROWS: NitfField,
    /// Number of Significant Columns in image
    pub NCOLS: NitfField,
    /// Pixel Value Type
    pub PVTYPE: NitfField,
    /// Image Representation
    pub IREP: NitfField,
    /// Image Category
    pub ICAT: NitfField,
    /// Actual Bits-Per-Pixel Per Band
    pub ABPP: NitfField,
    /// Pixel Justification
    pub PJUST: NitfField,
    /// Image Coordinate Representation
    pub ICORDS: NitfField,
    /// Image Geographic Location
    pub IGEOLO: NitfField,
    /// Number of Image Comments
    pub NICOM: NitfField,
    /// Image Comments
    pub ICOMS: NitfFieldVec,
    /// Image Compression
    pub IC: NitfField,
    /// Compression Rate Code
    pub COMRAT: NitfField,
    /// Number of Bands
    pub NBANDS: NitfField,
    /// Number of Multispectral Bands
    pub XBANDS: NitfField,
    /// Data bands
    pub BANDS: Vec<Band>,
    /// Image Sync Code
    pub ISYNC: NitfField,
    /// Image Mode
    pub IMODE: NitfField,
    /// Number of Blocks per Row
    pub NBPR: NitfField,
    /// Number of Blocks per Column
    pub NBPC: NitfField,
    /// Number of Pixels Per Block Horizontal
    pub NPPBH: NitfField,
    /// Number of Pixels Per Block Vertical
    pub NPPBV: NitfField,
    /// Number of Bits Per Pixel
    pub NBPP: NitfField,
    /// Image Display Level
    pub IDLVL: NitfField,
    /// Image Attachment Level
    pub IALVL: NitfField,
    /// Image Location
    pub ILOC: NitfField,
    /// Image Magnification
    pub IMAG: NitfField,
    /// User Defined Image Data Length
    pub UDIDL: NitfField,
    /// User Defined Overflow
    pub UDOFL: NitfField,
    /// User Defined Image Data
    pub UDID: NitfField,
    /// Image Extended Subheader Data Length
    pub IXSHDL: NitfField,
    /// Image Extended Subheader Overflow
    pub IXSOFL: NitfField,
    /// Image Extended Subheader Data
    pub IXSHD: NitfField,
}

impl Display for ImageSegmentHeader {
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
        out_str += format!("IGEOLO: {}, ", self.IGEOLO).as_ref();
        out_str += format!("NICOM: {}, ", self.NICOM).as_ref();
        out_str += format!("ICOMS: {}, ", self.ICOMS).as_ref();
        out_str += format!("IC: {}, ", self.IC).as_ref();
        out_str += format!("NBANDS: {}, ", self.NBANDS).as_ref();
        for band in &self.BANDS {
            out_str += format!("BAND: [{}], ", band).as_ref();
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
        out_str += format!("IXSHD: {}", self.IXSHD).as_ref();
        return write!(f, "ImageSegment: [{}]", out_str);
    }
}
impl NitfSegmentHeader for ImageSegmentHeader {
    fn read(&mut self, reader: &mut (impl Read + Seek)) {
        self.IM.read(reader, 2);
        self.IID1.read(reader, 10);
        self.IDATIM.read(reader, 14);
        self.TGTID.read(reader, 17);
        self.IID2.read(reader, 80);
        self.SECURITY.read(reader);
        self.ENCRYP.read(reader, 1);
        self.ISORCE.read(reader, 42);
        self.NROWS.read(reader, 8);
        self.NCOLS.read(reader, 8);
        self.PVTYPE.read(reader, 3);
        self.IREP.read(reader, 8);
        self.ICAT.read(reader, 8);
        self.ABPP.read(reader, 2);
        self.PJUST.read(reader, 1);
        self.ICORDS.read(reader, 1);
        self.IGEOLO.read(reader, 60);
        self.NICOM.read(reader, 1);
        self.ICOMS.read_vec(reader, &self.NICOM, 80);
        self.IC.read(reader, 2);
        self.NBANDS.read(reader, 1);
        // If NBANDS = 0, use XBANDS
        if self.NBANDS.string != "0" {
            self.BANDS = bands_from_reader(&self.NBANDS, reader)
        } else {
            self.XBANDS.read(reader, 5);
            self.BANDS = bands_from_reader(&self.XBANDS, reader)
        }
        self.ISYNC.read(reader, 1);
        self.IMODE.read(reader, 1);
        self.NBPR.read(reader, 4);
        self.NBPC.read(reader, 4);
        self.NPPBH.read(reader, 4);
        self.NPPBV.read(reader, 4);
        self.NBPP.read(reader, 2);
        self.IDLVL.read(reader, 3);
        self.IALVL.read(reader, 3);
        self.ILOC.read(reader, 10);
        self.IMAG.read(reader, 4);
        self.UDIDL.read(reader, 5);
        let udi_data_length: usize = self.UDIDL.string.parse().unwrap();
        if udi_data_length != 0 {
            self.UDOFL.read(reader, 3);
            self.UDID.read(reader, (udi_data_length - 3) as usize);
        }
        self.IXSHDL.read(reader, 5);
        let ixsh_data_length: usize = self.IXSHDL.string.parse().unwrap();
        if ixsh_data_length != 0 {
            self.IXSOFL.read(reader, 3);
            self.IXSHD.read(reader, (ixsh_data_length - 3) as usize);
        }
    }
}
#[allow(non_snake_case)]
#[derive(Default, Clone, Hash, Debug)]
pub struct Band {
    pub IREPBAND: NitfField,
    pub ISUBCAT: NitfField,
    pub IFC: NitfField,
    pub IMFLT: NitfField,
    pub NLUTS: NitfField,
    pub NELUT: NitfField,
    pub LUTD: NitfFieldVec,
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
        out_str += format!("LUTD: [{}]", self.LUTD).as_ref();
        return write!(f, "{}", out_str);
    }
}
impl Band {
    pub fn read(&mut self, reader: &mut (impl Read + Seek)) {
        self.IREPBAND.read(reader, 2);
        self.ISUBCAT.read(reader, 6);
        self.IFC.read(reader, 1);
        self.IMFLT.read(reader, 3);
        self.NLUTS.read(reader, 1);
        if self.NLUTS.string != "0" {
            self.NELUT.read(reader, 5);
            self.LUTD.read_vec(reader, &self.NLUTS, 1);
        }
    }
}

fn bands_from_reader(elem: &NitfField, reader: &mut (impl Read + Seek)) -> Vec<Band> {
    let n_band: usize = String::from_utf8(elem.bytes.to_vec())
        .unwrap()
        .parse()
        .unwrap();
    let mut bands: Vec<Band> = vec![Band::default(); n_band];
    for band in &mut bands {
        band.read(reader)
    }
    return bands;
}
