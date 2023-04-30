//! Graphic segment definition
use std::io::Read;
use std::fmt::Display;
use std::string::FromUtf8Error;

use super::elements::graphic::*;

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
        out_str += format!("SY: {}", self.sy).as_ref();
        out_str += format!("SID: {}", self.sid).as_ref();
        out_str += format!("SNAME: {}", self.sname).as_ref();
        out_str += format!("SSCLAS: {}", self.ssclas).as_ref();
        out_str += format!("SSCLSY: {}", self.ssclsy).as_ref();
        out_str += format!("SSCODE: {}", self.sscode).as_ref();
        out_str += format!("SSCTLH: {}", self.ssctlh).as_ref();
        out_str += format!("SSREL: {}", self.ssrel).as_ref();
        out_str += format!("SSDCTP: {}", self.ssdctp).as_ref();
        out_str += format!("SSDCDT: {}", self.ssdcdt).as_ref();
        out_str += format!("SSDCXM: {}", self.ssdcxm).as_ref();
        out_str += format!("SSDG: {}", self.ssdg).as_ref();
        out_str += format!("SSDGDT: {}", self.ssdgdt).as_ref();
        out_str += format!("SSCLTX: {}", self.sscltx).as_ref();
        out_str += format!("SSCATP: {}", self.sscatp).as_ref();
        out_str += format!("SSCAUT: {}", self.sscaut).as_ref();
        out_str += format!("SSCRSN: {}", self.sscrsn).as_ref();
        out_str += format!("SSSRDT: {}", self.sssrdt).as_ref();
        out_str += format!("SSCTLN: {}", self.ssctln).as_ref();
        out_str += format!("ENCRYP: {}", self.encryp).as_ref();
        out_str += format!("SFMT: {}", self.sfmt).as_ref();
        out_str += format!("SSTRUCT: {}", self.sstruct).as_ref();
        out_str += format!("SDLVL: {}", self.sdlvl).as_ref();
        out_str += format!("SALVL: {}", self.salvl).as_ref();
        out_str += format!("SLOC: {}", self.sloc).as_ref();
        out_str += format!("SBND1: {}", self.sbnd1).as_ref();
        out_str += format!("SCOLOR: {}", self.scolor).as_ref();
        out_str += format!("SBND2: {}", self.sbnd2).as_ref();
        out_str += format!("SRES2: {}", self.sres2).as_ref();
        out_str += format!("SXSHDL: {}", self.sxshdl).as_ref();
        write!(f, "Graphic Segment: [{}]", out_str)
    }
}
impl GraphicSegment {
    pub fn from_reader(reader: &mut impl Read) -> Result<Self, FromUtf8Error> {
        let mut gphx = GraphicSegment::default(); 
        reader.read(&mut gphx.sy.val).unwrap();
        reader.read(&mut gphx.sid.val).unwrap();
        reader.read(&mut gphx.sname.val).unwrap();
        reader.read(&mut gphx.ssclas.val).unwrap();
        reader.read(&mut gphx.ssclsy.val).unwrap();
        reader.read(&mut gphx.sscode.val).unwrap();
        reader.read(&mut gphx.ssctlh.val).unwrap();
        reader.read(&mut gphx.ssrel.val).unwrap();
        reader.read(&mut gphx.ssdctp.val).unwrap();
        reader.read(&mut gphx.ssdcdt.val).unwrap();
        reader.read(&mut gphx.ssdcxm.val).unwrap();
        reader.read(&mut gphx.ssdg.val).unwrap();
        reader.read(&mut gphx.ssdgdt.val).unwrap();
        reader.read(&mut gphx.sscltx.val).unwrap();
        reader.read(&mut gphx.sscatp.val).unwrap();
        reader.read(&mut gphx.sscaut.val).unwrap();
        reader.read(&mut gphx.sscrsn.val).unwrap();
        reader.read(&mut gphx.sssrdt.val).unwrap();
        reader.read(&mut gphx.ssctln.val).unwrap();
        reader.read(&mut gphx.encryp.val).unwrap();
        reader.read(&mut gphx.sfmt.val).unwrap();
        reader.read(&mut gphx.sstruct.val).unwrap();
        reader.read(&mut gphx.sdlvl.val).unwrap();
        reader.read(&mut gphx.salvl.val).unwrap();
        reader.read(&mut gphx.sloc.val).unwrap();
        reader.read(&mut gphx.sbnd1.val).unwrap();
        reader.read(&mut gphx.scolor.val).unwrap();
        reader.read(&mut gphx.sbnd2.val).unwrap();
        reader.read(&mut gphx.sres2.val).unwrap();
        reader.read(&mut gphx.sxshdl.val).unwrap();
        Ok(gphx)
    }
}