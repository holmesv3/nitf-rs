//! Graphic segment definition
use std::io::{Read, Seek};
use std::fmt::Display;
use std::string::FromUtf8Error;

use crate::nitf_2_1::types::{NitfField, NitfSegmentHeader};

#[allow(non_snake_case)]
#[derive(Default, Clone, Hash, Debug)]
pub struct GraphicSegment {
    /// File Part Type
    pub SY: NitfField,
    /// Graphic Identifier
    pub SID: NitfField,
    /// Graphic Name
    pub SNAME: NitfField,
    /// Graphic Security Classification
    pub SSCLAS: NitfField,
    /// Graphic Classification Security System
    pub SSCLSY: NitfField,
    /// Graphic Codewords
    pub SSCODE: NitfField,
    /// Graphic Control and Handling
    pub SSCTLH: NitfField,
    /// Graphic Releasing Instructions
    pub SSREL: NitfField,
    /// Graphic Declassification Type
    pub SSDCTP: NitfField,
    /// Graphic Declassification Date
    pub SSDCDT: NitfField,
    /// Graphic Declassification Exemption
    pub SSDCXM: NitfField,
    /// Graphic Downgrade
    pub SSDG: NitfField,
    /// Graphic Downgrade Date
    pub SSDGDT: NitfField,
    /// Graphic Classification Text
    pub SSCLTX: NitfField,
    /// Graphic Classification Authority Type
    pub SSCATP: NitfField,
    /// Graphic Classification Authority
    pub SSCAUT: NitfField,
    /// Graphic Classification Reason
    pub SSCRSN: NitfField,
    /// Graphic Security Source Date
    pub SSSRDT: NitfField,
    /// Graphic Security Control Number
    pub SSCTLN: NitfField,
    /// Encryption
    pub ENCRYP: NitfField,
    /// Graphic Type
    pub SFMT: NitfField,
    /// Reserved for Future Use
    pub SSTRUCT: NitfField,
    /// Graphic Display Level
    pub SDLVL: NitfField,
    /// Graphic Attachment Level
    pub SALVL: NitfField,
    /// Graphic Location
    pub SLOC: NitfField,
    /// First Graphic Bound Location
    pub SBND1: NitfField,
    /// Graphic Color
    pub SCOLOR: NitfField,
    /// Second Graphic Bound Location
    pub SBND2: NitfField,
    /// Reserved for Future Use
    pub SRES2: NitfField,
    /// Graphic Extended Subheader Data Length
    pub SXSHDL: NitfField,
}
impl Display for GraphicSegment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out_str = String::default(); 
        out_str += format!("SY: {}", self.SY).as_ref();
        out_str += format!("SID: {}", self.SID).as_ref();
        out_str += format!("SNAME: {}", self.SNAME).as_ref();
        out_str += format!("SSCLAS: {}", self.SSCLAS).as_ref();
        out_str += format!("SSCLSY: {}", self.SSCLSY).as_ref();
        out_str += format!("SSCODE: {}", self.SSCODE).as_ref();
        out_str += format!("SSCTLH: {}", self.SSCTLH).as_ref();
        out_str += format!("SSREL: {}", self.SSREL).as_ref();
        out_str += format!("SSDCTP: {}", self.SSDCTP).as_ref();
        out_str += format!("SSDCDT: {}", self.SSDCDT).as_ref();
        out_str += format!("SSDCXM: {}", self.SSDCXM).as_ref();
        out_str += format!("SSDG: {}", self.SSDG).as_ref();
        out_str += format!("SSDGDT: {}", self.SSDGDT).as_ref();
        out_str += format!("SSCLTX: {}", self.SSCLTX).as_ref();
        out_str += format!("SSCATP: {}", self.SSCATP).as_ref();
        out_str += format!("SSCAUT: {}", self.SSCAUT).as_ref();
        out_str += format!("SSCRSN: {}", self.SSCRSN).as_ref();
        out_str += format!("SSSRDT: {}", self.SSSRDT).as_ref();
        out_str += format!("SSCTLN: {}", self.SSCTLN).as_ref();
        out_str += format!("ENCRYP: {}", self.ENCRYP).as_ref();
        out_str += format!("SFMT: {}", self.SFMT).as_ref();
        out_str += format!("SSTRUCT: {}", self.SSTRUCT).as_ref();
        out_str += format!("SDLVL: {}", self.SDLVL).as_ref();
        out_str += format!("SALVL: {}", self.SALVL).as_ref();
        out_str += format!("SLOC: {}", self.SLOC).as_ref();
        out_str += format!("SBND1: {}", self.SBND1).as_ref();
        out_str += format!("SCOLOR: {}", self.SCOLOR).as_ref();
        out_str += format!("SBND2: {}", self.SBND2).as_ref();
        out_str += format!("SRES2: {}", self.SRES2).as_ref();
        out_str += format!("SXSHDL: {}", self.SXSHDL).as_ref();
        write!(f, "Graphic Segment: [{}]", out_str)
    }
}
impl NitfSegmentHeader for GraphicSegment {
    fn read(&mut self, reader: &mut (impl Read + Seek)) {
        self.SY.read(reader, 2);
        self.SID.read(reader, 10);
        self.SNAME.read(reader, 20);
        self.SSCLAS.read(reader, 1);
        self.SSCLSY.read(reader, 2);
        self.SSCODE.read(reader, 11);
        self.SSCTLH.read(reader, 2);
        self.SSREL.read(reader, 20);
        self.SSDCTP.read(reader, 2);
        self.SSDCDT.read(reader, 8);
        self.SSDCXM.read(reader, 4);
        self.SSDG.read(reader, 1);
        self.SSDGDT.read(reader, 8);
        self.SSCLTX.read(reader, 43);
        self.SSCATP.read(reader, 1);
        self.SSCAUT.read(reader, 40);
        self.SSCRSN.read(reader, 1);
        self.SSSRDT.read(reader, 8);
        self.SSCTLN.read(reader, 15);
        self.ENCRYP.read(reader, 1);
        self.SFMT.read(reader, 1);
        self.SSTRUCT.read(reader, 13);
        self.SDLVL.read(reader, 3);
        self.SALVL.read(reader, 3);
        self.SLOC.read(reader, 10);
        self.SBND1.read(reader, 10);
        self.SCOLOR.read(reader, 1);
        self.SBND2.read(reader, 10);
        self.SRES2.read(reader, 2);
        self.SXSHDL.read(reader, 5);
    }
}