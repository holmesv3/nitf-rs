//! Image header definition
use std::io::{Read, Seek};
use std::fmt::Display;
use std::string::FromUtf8Error;

use crate::types::*;

#[allow(non_snake_case)]
#[derive(Default, Clone, Hash, Debug)]
pub struct ImageSegment {
    /// File Part Type
    pub FILEPARTTYPE: NitfField,
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
    /// Number of Image Comments
    pub NICOM: NitfField,
    /// Image Comments
    pub ICOMS: NitfFieldVec,
    /// Image Compression
    pub IC: NitfField,
    /// Number of Bands
    pub NBANDS: NitfField,
    /// 1st Band Representation
    pub IREPBAND1: NitfField,
    /// 1st Band Subcategory
    pub ISUBCAT1: NitfField,
    /// 1st Band Image Filter Condition
    pub IFC1: NitfField,
    /// 1st Band Standard Image Filter Code
    pub IMFLT1: NitfField,
    /// Number of LUTs for the 1st Image Band
    pub NLUTS1: NitfField,
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
    /// Image Extended Subheader Data Length
    pub IXSHDL: NitfField,
}
impl Display for ImageSegment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out_str = String::default();
        out_str += format!("\n{}", self.FILEPARTTYPE).as_ref();
        out_str += format!("\n{}", self.IID1).as_ref();
        out_str += format!("\n{}", self.IDATIM).as_ref();
        out_str += format!("\n{}", self.TGTID).as_ref();
        out_str += format!("\n{}", self.IID2).as_ref();
        out_str += format!("\n{}", self.ISCLAS).as_ref();
        out_str += format!("\n{}", self.ISCLSY).as_ref();
        out_str += format!("\n{}", self.ISCODE).as_ref();
        out_str += format!("\n{}", self.ISCTLH).as_ref();
        out_str += format!("\n{}", self.ISREL).as_ref();
        out_str += format!("\n{}", self.ISDCTP).as_ref();
        out_str += format!("\n{}", self.ISDCDT).as_ref();
        out_str += format!("\n{}", self.ISDCXM).as_ref();
        out_str += format!("\n{}", self.ISDG).as_ref();
        out_str += format!("\n{}", self.ISDGDT).as_ref();
        out_str += format!("\n{}", self.ISCLTX).as_ref();
        out_str += format!("\n{}", self.ISCATP).as_ref();
        out_str += format!("\n{}", self.ISCAUT).as_ref();
        out_str += format!("\n{}", self.ISCRSN).as_ref();
        out_str += format!("\n{}", self.ISSRDT).as_ref();
        out_str += format!("\n{}", self.ISCTLN).as_ref();
        out_str += format!("\n{}", self.ENCRYP).as_ref();
        out_str += format!("\n{}", self.ISORCE).as_ref();
        out_str += format!("\n{}", self.NROWS).as_ref();
        out_str += format!("\n{}", self.NCOLS).as_ref();
        out_str += format!("\n{}", self.PVTYPE).as_ref();
        out_str += format!("\n{}", self.IREP).as_ref();
        out_str += format!("\n{}", self.ICAT).as_ref();
        out_str += format!("\n{}", self.ABPP).as_ref();
        out_str += format!("\n{}", self.PJUST).as_ref();
        out_str += format!("\n{}", self.ICORDS).as_ref();
        out_str += format!("\n{}", self.NICOM).as_ref();
        out_str += format!("\n{}", self.ICOMS).as_ref();
        out_str += format!("\n{}", self.IC).as_ref();
        out_str += format!("\n{}", self.NBANDS).as_ref();
        out_str += format!("\n{}", self.IREPBAND1).as_ref();
        out_str += format!("\n{}", self.ISUBCAT1).as_ref();
        out_str += format!("\n{}", self.IFC1).as_ref();
        out_str += format!("\n{}", self.IMFLT1).as_ref();
        out_str += format!("\n{}", self.NLUTS1).as_ref();
        out_str += format!("\n{}", self.ISYNC).as_ref();
        out_str += format!("\n{}", self.IMODE).as_ref();
        out_str += format!("\n{}", self.NBPR).as_ref();
        out_str += format!("\n{}", self.NBPC).as_ref();
        out_str += format!("\n{}", self.NPPBH).as_ref();
        out_str += format!("\n{}", self.NPPBV).as_ref();
        out_str += format!("\n{}", self.NBPP).as_ref();
        out_str += format!("\n{}", self.IDLVL).as_ref();
        out_str += format!("\n{}", self.IALVL).as_ref();
        out_str += format!("\n{}", self.ILOC).as_ref();
        out_str += format!("\n{}", self.IMAG).as_ref();
        out_str += format!("\n{}", self.UDIDL).as_ref();
        out_str += format!("\n{}", self.IXSHDL).as_ref();
        write!(f, "ImageSegment: [{}]", out_str)
    }
}
impl Segment<ImageSegment> for ImageSegment {
    fn from_reader(reader: &mut (impl Read + Seek)) -> Result<Self, std::io::Error> {
        let mut imseg = ImageSegment::default();
        imseg.FILEPARTTYPE.read(reader, 2);
        imseg.IID1.read(reader, 10);
        imseg.IDATIM.read(reader, 14);
        imseg.TGTID.read(reader, 17);
        imseg.IID2.read(reader, 80);
        imseg.ISCLAS.read(reader, 1);
        imseg.ISCLSY.read(reader, 2);
        imseg.ISCODE.read(reader, 11);
        imseg.ISCTLH.read(reader, 2);
        imseg.ISREL.read(reader, 20);
        imseg.ISDCTP.read(reader, 2);
        imseg.ISDCDT.read(reader, 8);
        imseg.ISDCXM.read(reader, 4);
        imseg.ISDG.read(reader, 1);
        imseg.ISDGDT.read(reader, 8);
        imseg.ISCLTX.read(reader, 43);
        imseg.ISCATP.read(reader, 1);
        imseg.ISCAUT.read(reader, 40);
        imseg.ISCRSN.read(reader, 1);
        imseg.ISSRDT.read(reader, 8);
        imseg.ISCTLN.read(reader, 15);
        imseg.ENCRYP.read(reader, 1);
        imseg.ISORCE.read(reader, 42);
        imseg.NROWS.read(reader, 8);
        imseg.NCOLS.read(reader, 8);
        imseg.PVTYPE.read(reader, 3);
        imseg.IREP.read(reader, 8);
        imseg.ICAT.read(reader, 8);
        imseg.ABPP.read(reader, 2);
        imseg.PJUST.read(reader, 1);
        imseg.ICORDS.read(reader, 1);
        imseg.NICOM.read(reader, 1);
        imseg.ICOMS.read(reader,&imseg.NICOM, 80);
        imseg.IC.read(reader, 2);
        imseg.NBANDS.read(reader, 1);
        imseg.IREPBAND1.read(reader, 2);
        imseg.ISUBCAT1.read(reader, 6);
        imseg.IFC1.read(reader, 1);
        imseg.IMFLT1.read(reader, 3);
        imseg.NLUTS1.read(reader, 1);
        imseg.ISYNC.read(reader, 1);
        imseg.IMODE.read(reader, 1);
        imseg.NBPR.read(reader, 4);
        imseg.NBPC.read(reader, 4);
        imseg.NPPBH.read(reader, 4);
        imseg.NPPBV.read(reader, 4);
        imseg.NBPP.read(reader, 2);
        imseg.IDLVL.read(reader, 3);
        imseg.IALVL.read(reader, 3);
        imseg.ILOC.read(reader, 10);
        imseg.IMAG.read(reader, 4);
        imseg.UDIDL.read(reader, 5);
        imseg.IXSHDL.read(reader, 5);
        Ok(imseg)
    }
}
