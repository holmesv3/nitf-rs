//! Graphic segment definition
use std::io::{Read, Seek};
use std::fmt::Display;

use crate::nitf_2_1::types::{NitfField, NitfSegmentHeader, Security};

#[allow(non_snake_case)]
#[derive(Default, Clone, Hash, Debug)]
pub struct GraphicSegment {
    /// File Part Type
    pub SY: NitfField,
    /// Graphic Identifier
    pub SID: NitfField,
    /// Graphic Name
    pub SNAME: NitfField,
    /// Security information
    pub SECURITY: Security,
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
        out_str += format!("SY: {}, ", self.SY).as_ref();
        out_str += format!("SID: {}, ", self.SID).as_ref();
        out_str += format!("SNAME: {}, ", self.SNAME).as_ref();
        out_str += format!("SECURITY: [{}], ", self.SECURITY).as_ref();
        out_str += format!("ENCRYP: {}, ", self.ENCRYP).as_ref();
        out_str += format!("SFMT: {}, ", self.SFMT).as_ref();
        out_str += format!("SSTRUCT: {}, ", self.SSTRUCT).as_ref();
        out_str += format!("SDLVL: {}, ", self.SDLVL).as_ref();
        out_str += format!("SALVL: {}, ", self.SALVL).as_ref();
        out_str += format!("SLOC: {}, ", self.SLOC).as_ref();
        out_str += format!("SBND1: {}, ", self.SBND1).as_ref();
        out_str += format!("SCOLOR: {}, ", self.SCOLOR).as_ref();
        out_str += format!("SBND2: {}, ", self.SBND2).as_ref();
        out_str += format!("SRES2: {}, ", self.SRES2).as_ref();
        out_str += format!("SXSHDL: {}", self.SXSHDL).as_ref();
        write!(f, "Graphic Segment: [{}]", out_str)
    }
}
impl NitfSegmentHeader for GraphicSegment {
    fn read(&mut self, reader: &mut (impl Read + Seek)) {
        self.SY.read(reader, 2);
        self.SID.read(reader, 10);
        self.SNAME.read(reader, 20);
        self.SECURITY.read(reader);
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