//! Header definition
use std::io::{Read, Seek};
use std::fmt::Display;
use std::string::FromUtf8Error;

use super::elements::header::*;

impl NitfHeader {
    pub fn from_reader(reader: &mut (impl Read + Seek)) -> Result<Self, FromUtf8Error> {
        let mut hdr = Self::default();
        reader.read(&mut hdr.fhdr.val).unwrap();
        reader.read(&mut hdr.fver.val).unwrap();
        reader.read(&mut hdr.clevel.val).unwrap();
        reader.read(&mut hdr.stype.val).unwrap();
        reader.read(&mut hdr.ostaid.val).unwrap();
        reader.read(&mut hdr.fdt.val).unwrap();
        reader.read(&mut hdr.ftitle.val).unwrap();
        // Security fields
        reader.read(&mut hdr.fsclas.val).unwrap();
        reader.read(&mut hdr.fsclsy.val).unwrap();
        reader.read(&mut hdr.fscode.val).unwrap();
        reader.read(&mut hdr.fsctlh.val).unwrap();
        reader.read(&mut hdr.fsrel.val).unwrap();
        reader.read(&mut hdr.fsdctp.val).unwrap();
        reader.read(&mut hdr.fsdcdt.val).unwrap();
        reader.read(&mut hdr.fsdcxm.val).unwrap();
        reader.read(&mut hdr.fsdg.val).unwrap();
        reader.read(&mut hdr.fsdgdt.val).unwrap();
        reader.read(&mut hdr.fscltx.val).unwrap();
        reader.read(&mut hdr.fscatp.val).unwrap();
        reader.read(&mut hdr.fscaut.val).unwrap();
        reader.read(&mut hdr.fscrsn.val).unwrap();
        reader.read(&mut hdr.fssrdt.val).unwrap();
        reader.read(&mut hdr.fsctln.val).unwrap();
        // /Security
        reader.read(&mut hdr.fscop.val).unwrap();
        reader.read(&mut hdr.fscpys.val).unwrap();
        reader.read(&mut hdr.encryp.val).unwrap();
        reader.read(&mut hdr.fbkgc.val).unwrap();
        reader.read(&mut hdr.oname.val).unwrap();
        reader.read(&mut hdr.ophone.val).unwrap();
        reader.read(&mut hdr.fl.val).unwrap();
        reader.read(&mut hdr.hl.val).unwrap();
        // Image segments
        reader.read(&mut hdr.numi.val).unwrap();
        let n_image: usize = String::from_utf8(hdr.numi.val.to_vec())
            .unwrap()
            .parse()
            .unwrap();
        for _ in 0..n_image {
            let mut seg = IMAGESEGMENTELEM::default();
            reader.read(&mut seg.subheader_size).unwrap();
            reader.read(&mut seg.item_size).unwrap();        
            hdr.imheaders.val.push(seg);
        }
        // Graphics segments
        reader.read(&mut hdr.nums.val).unwrap();
        let n_gphx: usize = String::from_utf8(hdr.nums.val.to_vec())
            .unwrap()
            .parse()
            .unwrap();
        for _ in 0..n_gphx {
            let mut seg = GRAPHICSSEGMENTELEM::default();
            reader.read(&mut seg.subheader_size).unwrap();
            reader.read(&mut seg.item_size).unwrap();        
            hdr.graphheaders.val.push(seg);
        }
        reader.read(&mut hdr.numx.val).unwrap();
        // Text files
        reader.read(&mut hdr.numt.val).unwrap();
        let n_txt: usize = String::from_utf8(hdr.numt.val.to_vec())
            .unwrap()
            .parse()
            .unwrap();
        for _ in 0..n_txt {
            let mut seg = TEXTSEGMENTELEM::default();
            reader.read(&mut seg.subheader_size).unwrap();
            reader.read(&mut seg.item_size).unwrap();        
            hdr.textfiles.val.push(seg);
        }
        // Data Extensions
        reader.read(&mut hdr.numdes.val).unwrap();
        let n_dext: usize = String::from_utf8(hdr.numdes.val.to_vec())
            .unwrap()
            .parse()
            .unwrap();
        for _ in 0..n_dext {
            let mut seg = DATAEXTSEGMENTELEM::default();
            reader.read(&mut seg.subheader_size).unwrap();
            reader.read(&mut seg.item_size).unwrap();        
            hdr.dextheaders.val.push(seg);
        }
        // Reserved Segments
        reader.read(&mut hdr.numres.val).unwrap();
        
        let n_res: usize = String::from_utf8(hdr.numres.val.to_vec())
            .unwrap()
            .parse()
            .unwrap();
        for _ in 0..n_res {
            let mut seg = RESERVEDSEGMENTELEM::default();
            reader.read(&mut seg.subheader_size).unwrap();
            reader.read(&mut seg.item_size).unwrap();        
            hdr.resheaders.val.push(seg);
        }
        reader.read(&mut hdr.udhdl.val).unwrap();
        reader.read(&mut hdr.xhdl.val).unwrap();
        Ok(hdr)
    }
}

// Struct definition
#[derive(Default, Clone, Hash, Debug)]
pub struct NitfHeader {
    pub fhdr: FHDR,
    pub fver: FVER,
    pub clevel: CLEVEL,
    pub stype: STYPE,
    pub ostaid: OSTAID,
    pub fdt: FDT,
    pub ftitle: FTITLE,
    pub fsclas: FSCLAS,
    pub fsclsy: FSCLSY,
    pub fscode: FSCODE,
    pub fsctlh: FSCTLH,
    pub fsrel: FSREL,
    pub fsdctp: FSDCTP,
    pub fsdcdt: FSDCDT,
    pub fsdcxm: FSDCXM,
    pub fsdg: FSDG,
    pub fsdgdt: FSDGDT,
    pub fscltx: FSCLTX,
    pub fscatp: FSCATP,
    pub fscaut: FSCAUT,
    pub fscrsn: FSCRSN,
    pub fssrdt: FSSRDT,
    pub fsctln: FSCTLN,
    pub fscop: FSCOP,
    pub fscpys: FSCPYS,
    pub encryp: ENCRYP,
    pub fbkgc: FBKGC,
    pub oname: ONAME,
    pub ophone: OPHONE,
    pub fl: FL,
    pub hl: HL,
    pub numi: NUMIMAGESEGMENTS,
    pub imheaders: IMAGESEGMENTS,
    pub nums: NUMGRAPHICSEGMENT,
    pub graphheaders: GRAPHICSSEGMENTS,
    pub numx: NUMX,
    pub numt: NUMTEXTFILES,
    pub textfiles: TEXTSEGMENTS,
    pub numdes: NUMDES,
    pub dextheaders: DATAEXTSEGMENTS,
    pub numres: NUMRES,
    pub resheaders: RESERVEDSEGMENTS,
    pub udhdl: UDHDL,
    pub xhdl: XHDL,
}
impl Display for NitfHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out_str = String::default();
        out_str += format!("{}", self.fhdr).as_ref();
        out_str += format!("{}", self.fver).as_ref();
        out_str += format!("{}", self.clevel).as_ref();
        out_str += format!("{}", self.stype).as_ref();
        out_str += format!("{}", self.ostaid).as_ref();
        out_str += format!("{}", self.fdt).as_ref();
        out_str += format!("{}", self.ftitle).as_ref();
        out_str += format!("{}", self.fsclas).as_ref();
        out_str += format!("{}", self.fsclsy).as_ref();
        out_str += format!("{}", self.fscode).as_ref();
        out_str += format!("{}", self.fsctlh).as_ref();
        out_str += format!("{}", self.fsrel).as_ref();
        out_str += format!("{}", self.fsdctp).as_ref();
        out_str += format!("{}", self.fsdcdt).as_ref();
        out_str += format!("{}", self.fsdcxm).as_ref();
        out_str += format!("{}", self.fsdg).as_ref();
        out_str += format!("{}", self.fsdgdt).as_ref();
        out_str += format!("{}", self.fscltx).as_ref();
        out_str += format!("{}", self.fscatp).as_ref();
        out_str += format!("{}", self.fscaut).as_ref();
        out_str += format!("{}", self.fscrsn).as_ref();
        out_str += format!("{}", self.fssrdt).as_ref();
        out_str += format!("{}", self.fsctln).as_ref();
        out_str += format!("{}", self.fscop).as_ref();
        out_str += format!("{}", self.fscpys).as_ref();
        out_str += format!("{}", self.encryp).as_ref();
        out_str += format!("{}", self.fbkgc).as_ref();
        out_str += format!("{}", self.oname).as_ref();
        out_str += format!("{}", self.ophone).as_ref();
        out_str += format!("{}", self.fl).as_ref();
        out_str += format!("{}", self.hl).as_ref();
        out_str += format!("{}", self.numi).as_ref();
        out_str += format!("{}", self.imheaders).as_ref();
        out_str += format!("{}", self.nums).as_ref();
        out_str += format!("{}", self.graphheaders).as_ref();
        out_str += format!("{}", self.numx).as_ref();
        out_str += format!("{}", self.numt).as_ref();
        out_str += format!("{}", self.textfiles).as_ref();
        out_str += format!("{}", self.numdes).as_ref();
        out_str += format!("{}", self.dextheaders).as_ref();
        out_str += format!("{}", self.numres).as_ref();
        out_str += format!("{}", self.resheaders).as_ref();
        out_str += format!("{}", self.udhdl).as_ref();
        out_str += format!("{}", self.xhdl).as_ref();
        write!(f, "NitfHeader: [{}]", out_str)
    }
}