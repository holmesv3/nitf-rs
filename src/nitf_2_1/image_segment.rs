//! Image header definition
use std::io::{Read, Seek};
use std::fmt::Display;

use crate::nitf_2_1::types::*;

#[allow(non_snake_case)]
#[derive(Default, Clone, Hash, Debug)]
pub struct ImageSegment {
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
    /// Image Security Classification
    pub ISCLAS: NitfField,
    /// Image Classification Security System
    pub ISCLSY: NitfField,
    /// Image Codewords
    pub ISCODE: NitfField,
    /// Image Control and Handling
    pub ISCTLH: NitfField,
    /// Image Releasing Instructions
    pub ISREL: NitfField,
    /// Image Declassification Type
    pub ISDCTP: NitfField,
    /// Image Declassification Date
    pub ISDCDT: NitfField,
    /// Image Declassification Exemption
    pub ISDCXM: NitfField,
    /// Image Downgrade
    pub ISDG: NitfField,
    /// Image Downgrade Date
    pub ISDGDT: NitfField,
    /// Image Classification Text
    pub ISCLTX: NitfField,
    /// Image Classification Authority Type
    pub ISCATP: NitfField,
    /// Image Classification Authority
    pub ISCAUT: NitfField,
    /// Image Classification Reason
    pub ISCRSN: NitfField,
    /// Image Security Source Date
    pub ISSRDT: NitfField,
    /// Image Security Control Number
    pub ISCTLN: NitfField,
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

impl Display for ImageSegment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out_str = String::default();
        out_str += format!("\nFILEPARTTYPE: {}", self.IM).as_ref();
        out_str += format!("\nIID1: {}", self.IID1).as_ref();
        out_str += format!("\nIDATIM: {}", self.IDATIM).as_ref();
        out_str += format!("\nTGTID: {}", self.TGTID).as_ref();
        out_str += format!("\nIID2: {}", self.IID2).as_ref();
        out_str += format!("\nISCLAS: {}", self.ISCLAS).as_ref();
        out_str += format!("\nISCLSY: {}", self.ISCLSY).as_ref();
        out_str += format!("\nISCODE: {}", self.ISCODE).as_ref();
        out_str += format!("\nISCTLH: {}", self.ISCTLH).as_ref();
        out_str += format!("\nISREL: {}", self.ISREL).as_ref();
        out_str += format!("\nISDCTP: {}", self.ISDCTP).as_ref();
        out_str += format!("\nISDCDT: {}", self.ISDCDT).as_ref();
        out_str += format!("\nISDCXM: {}", self.ISDCXM).as_ref();
        out_str += format!("\nISDG: {}", self.ISDG).as_ref();
        out_str += format!("\nISDGDT: {}", self.ISDGDT).as_ref();
        out_str += format!("\nISCLTX: {}", self.ISCLTX).as_ref();
        out_str += format!("\nISCATP: {}", self.ISCATP).as_ref();
        out_str += format!("\nISCAUT: {}", self.ISCAUT).as_ref();
        out_str += format!("\nISCRSN: {}", self.ISCRSN).as_ref();
        out_str += format!("\nISSRDT: {}", self.ISSRDT).as_ref();
        out_str += format!("\nISCTLN: {}", self.ISCTLN).as_ref();
        out_str += format!("\nENCRYP: {}", self.ENCRYP).as_ref();
        out_str += format!("\nISORCE: {}", self.ISORCE).as_ref();
        out_str += format!("\nNROWS: {}", self.NROWS).as_ref();
        out_str += format!("\nNCOLS: {}", self.NCOLS).as_ref();
        out_str += format!("\nPVTYPE: {}", self.PVTYPE).as_ref();
        out_str += format!("\nIREP: {}", self.IREP).as_ref();
        out_str += format!("\nICAT: {}", self.ICAT).as_ref();
        out_str += format!("\nABPP: {}", self.ABPP).as_ref();
        out_str += format!("\nPJUST: {}", self.PJUST).as_ref();
        out_str += format!("\nICORDS: {}", self.ICORDS).as_ref();
        out_str += format!("\nIGEOLO: {}", self.IGEOLO).as_ref();
        out_str += format!("\nNICOM: {}", self.NICOM).as_ref();
        out_str += format!("\nICOMS: {}", self.ICOMS).as_ref();
        out_str += format!("\nIC: {}", self.IC).as_ref();
        out_str += format!("\nNBANDS: {}", self.NBANDS).as_ref();
        for band in &self.BANDS {
            out_str += format!("\nBAND: [{}]", band).as_ref();
        }
        out_str += format!("\nISYNC: {}", self.ISYNC).as_ref();
        out_str += format!("\nIMODE: {}", self.IMODE).as_ref();
        out_str += format!("\nNBPR: {}", self.NBPR).as_ref();
        out_str += format!("\nNBPC: {}", self.NBPC).as_ref();
        out_str += format!("\nNPPBH: {}", self.NPPBH).as_ref();
        out_str += format!("\nNPPBV: {}", self.NPPBV).as_ref();
        out_str += format!("\nNBPP: {}", self.NBPP).as_ref();
        out_str += format!("\nIDLVL: {}", self.IDLVL).as_ref();
        out_str += format!("\nIALVL: {}", self.IALVL).as_ref();
        out_str += format!("\nILOC: {}", self.ILOC).as_ref();
        out_str += format!("\nIMAG: {}", self.IMAG).as_ref();
        out_str += format!("\nUDIDL: {}", self.UDIDL).as_ref();
        out_str += format!("\nIXSHDL: {}", self.IXSHDL).as_ref();
        return write!(f, "ImageSegment: [{}]", out_str)
    }
}
impl NitfSegmentHeader for ImageSegment {
    fn read(&mut self, reader: &mut (impl Read + Seek)) {
        self.IM.read(reader, 2);
        self.IID1.read(reader, 10);
        self.IDATIM.read(reader, 14);
        self.TGTID.read(reader, 17);
        self.IID2.read(reader, 80);
        self.ISCLAS.read(reader, 1);
        self.ISCLSY.read(reader, 2);
        self.ISCODE.read(reader, 11);
        self.ISCTLH.read(reader, 2);
        self.ISREL.read(reader, 20);
        self.ISDCTP.read(reader, 2);
        self.ISDCDT.read(reader, 8);
        self.ISDCXM.read(reader, 4);
        self.ISDG.read(reader, 1);
        self.ISDGDT.read(reader, 8);
        self.ISCLTX.read(reader, 43);
        self.ISCATP.read(reader, 1);
        self.ISCAUT.read(reader, 40);
        self.ISCRSN.read(reader, 1);
        self.ISSRDT.read(reader, 8);
        self.ISCTLN.read(reader, 15);
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
        self.ICOMS.read_vec(reader,&self.NICOM, 80);
        self.IC.read(reader, 2);
        self.NBANDS.read(reader, 1);
        if self.NBANDS.bytes == vec![0u8] {
            self.XBANDS.read(reader, 5);
            self.BANDS = bands_from_reader(&self.XBANDS, reader)
        } else {
            self.BANDS = bands_from_reader(&self.NBANDS, reader)
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
        let udi_data_length: u32 = String::from_utf8(self.UDIDL.bytes.to_vec()).unwrap().parse().unwrap();
        if udi_data_length != 0 {
            self.UDOFL.read(reader, 3);
            self.UDID.read(reader, (udi_data_length - 3) as usize);
        }
        self.IXSHDL.read(reader, 5);
        let ixsh_data_length: u32 = String::from_utf8(self.IXSHDL.bytes.to_vec()).unwrap().parse().unwrap();
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
        out_str += format!("\nIREPBAND: {}", self.IREPBAND).as_ref();
        out_str += format!("\nISUBCAT: {}", self.ISUBCAT).as_ref();
        out_str += format!("\nIFC: {}", self.IFC).as_ref();
        out_str += format!("\nIMFLT: {}", self.IMFLT).as_ref();
        out_str += format!("\nNLUTS: {}", self.NLUTS).as_ref();
        out_str += format!("\nNELUT: {}", self.NELUT).as_ref();
        out_str += format!("\nLUTD: [{}]", self.LUTD).as_ref();
        return write!(f, "{}", out_str)
    }
}
impl Band {
    pub fn read(&mut self, reader: &mut (impl Read + Seek)) {
        self.IREPBAND.read(reader, 2);
        self.ISUBCAT.read(reader, 6);
        self.IFC.read(reader, 1);
        self.IMFLT.read(reader, 3);
        self.NLUTS.read(reader, 1);
        self.NELUT.read(reader, 5);
        self.LUTD.read_vec(reader, &self.NELUT, 1);
    }
}

fn bands_from_reader(elem: &NitfField, reader: &mut (impl Read + Seek)) -> Vec<Band> {
    let n_band: usize = String::from_utf8(elem.bytes.to_vec()).unwrap().parse().unwrap();
    let mut bands: Vec<Band> = vec![Band::default(); n_band];
    for band in &mut bands {
        band.read(reader)
    }
    return bands
}