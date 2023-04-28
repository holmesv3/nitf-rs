//! Graphic Segment Definition
use std::io::Read;
use std::fmt::Display;
use std::string::FromUtf8Error;

use super::elements::graphic::*;

impl GraphicSegment {
    pub fn from_reader(reader: &mut impl Read) -> Result<Self, FromUtf8Error> {
        let mut graphseg = GraphicSegment::default();
        reader.read(&mut graphseg.sy.val).unwrap();
        reader.read(&mut graphseg.sid.val).unwrap();
        reader.read(&mut graphseg.sname.val).unwrap();
        reader.read(&mut graphseg.ssclas.val).unwrap();
        reader.read(&mut graphseg.ssclsy.val).unwrap();
        reader.read(&mut graphseg.sscode.val).unwrap();
        reader.read(&mut graphseg.ssctlh.val).unwrap();
        reader.read(&mut graphseg.ssrel.val).unwrap();
        reader.read(&mut graphseg.ssdctp.val).unwrap();
        reader.read(&mut graphseg.ssdcdt.val).unwrap();
        reader.read(&mut graphseg.ssdcxm.val).unwrap();
        reader.read(&mut graphseg.ssdg.val).unwrap();
        reader.read(&mut graphseg.ssdgdt.val).unwrap();
        reader.read(&mut graphseg.sscltx.val).unwrap();
        reader.read(&mut graphseg.sscatp.val).unwrap();
        reader.read(&mut graphseg.sscaut.val).unwrap();
        reader.read(&mut graphseg.sscrsn.val).unwrap();
        reader.read(&mut graphseg.sssrdt.val).unwrap();
        reader.read(&mut graphseg.ssctln.val).unwrap();
        reader.read(&mut graphseg.encryp.val).unwrap();
        reader.read(&mut graphseg.sfmt.val).unwrap();
        reader.read(&mut graphseg.sstruct.val).unwrap();
        reader.read(&mut graphseg.sdlvl.val).unwrap();
        reader.read(&mut graphseg.salvl.val).unwrap();
        reader.read(&mut graphseg.sloc.val).unwrap();
        reader.read(&mut graphseg.sbnd1.val).unwrap();
        reader.read(&mut graphseg.scolor.val).unwrap();
        reader.read(&mut graphseg.sbnd2.val).unwrap();
        reader.read(&mut graphseg.sres2.val).unwrap();
        reader.read(&mut graphseg.sxshdl.val).unwrap();
        Ok(graphseg)
    }
}

#[derive(Default, Clone, Hash, Debug)]
pub struct GraphicSegment {
    pub sy: SY,
    pub sid: SID,
    pub sname: SNAME,
    pub ssclas: SSCLAS,
    pub ssclsy: SSCLSY,
    pub sscode: SSCODE,
    pub ssctlh: SSCTLH,
    pub ssrel: SSREL,
    pub ssdctp: SSDCTP,
    pub ssdcdt: SSDCDT,
    pub ssdcxm: SSDCXM,
    pub ssdg: SSDG,
    pub ssdgdt: SSDGDT,
    pub sscltx: SSCLTX,
    pub sscatp: SSCATP,
    pub sscaut: SSCAUT,
    pub sscrsn: SSCRSN,
    pub sssrdt: SSSRDT,
    pub ssctln: SSCTLN,
    pub encryp: ENCRYP,
    pub sfmt: SFMT,
    pub sstruct: SSTRUCT,
    pub sdlvl: SDLVL,
    pub salvl: SALVL,
    pub sloc: SLOC,
    pub sbnd1: SBND1,
    pub scolor: SCOLOR,
    pub sbnd2: SBND2,
    pub sres2: SRES2,
    pub sxshdl: SXSHDL,
}
impl Display for GraphicSegment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out_str = String::default();
        out_str += format!("{}", self.sy).as_ref();
        out_str += format!("{}", self.sid).as_ref();
        out_str += format!("{}", self.sname).as_ref();
        out_str += format!("{}", self.ssclas).as_ref();
        out_str += format!("{}", self.ssclsy).as_ref();
        out_str += format!("{}", self.sscode).as_ref();
        out_str += format!("{}", self.ssctlh).as_ref();
        out_str += format!("{}", self.ssrel).as_ref();
        out_str += format!("{}", self.ssdctp).as_ref();
        out_str += format!("{}", self.ssdcdt).as_ref();
        out_str += format!("{}", self.ssdcxm).as_ref();
        out_str += format!("{}", self.ssdg).as_ref();
        out_str += format!("{}", self.ssdgdt).as_ref();
        out_str += format!("{}", self.sscltx).as_ref();
        out_str += format!("{}", self.sscatp).as_ref();
        out_str += format!("{}", self.sscaut).as_ref();
        out_str += format!("{}", self.sscrsn).as_ref();
        out_str += format!("{}", self.sssrdt).as_ref();
        out_str += format!("{}", self.ssctln).as_ref();
        out_str += format!("{}", self.encryp).as_ref();
        out_str += format!("{}", self.sfmt).as_ref();
        out_str += format!("{}", self.sstruct).as_ref();
        out_str += format!("{}", self.sdlvl).as_ref();
        out_str += format!("{}", self.salvl).as_ref();
        out_str += format!("{}", self.sloc).as_ref();
        out_str += format!("{}", self.sbnd1).as_ref();
        out_str += format!("{}", self.scolor).as_ref();
        out_str += format!("{}", self.sbnd2).as_ref();
        out_str += format!("{}", self.sres2).as_ref();
        out_str += format!("{}", self.sxshdl).as_ref();
        write!(f, "GraphicSegment: [{}]", out_str)
    }
}