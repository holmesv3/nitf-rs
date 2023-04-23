//! Header Definition
use std::io::Read;
use std::fmt::Display;
use std::string::FromUtf8Error;

use super::elements::header::*;

impl NitfHeader {
    pub fn from_reader(reader: &mut impl Read) -> Result<Self, FromUtf8Error> {
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
            let mut seg = ImageSegmentElem::default();
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
            let mut seg = GraphicsSegmentElem::default();
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
            let mut seg = TextSegmentElem::default();
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
            let mut seg = DataExtSegmentElem::default();
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
            let mut seg = ReservedSegmentElem::default();
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
    pub fhdr: Fhdr,
    pub fver: Fver,
    pub clevel: Clevel,
    pub stype: Stype,
    pub ostaid: Ostaid,
    pub fdt: Fdt,
    pub ftitle: Ftitle,
    pub fsclas: Fsclas,
    pub fsclsy: Fsclsy,
    pub fscode: Fscode,
    pub fsctlh: Fsctlh,
    pub fsrel: Fsrel,
    pub fsdctp: Fsdctp,
    pub fsdcdt: Fsdcdt,
    pub fsdcxm: Fsdcxm,
    pub fsdg: Fsdg,
    pub fsdgdt: Fsdgdt,
    pub fscltx: Fscltx,
    pub fscatp: Fscatp,
    pub fscaut: Fscaut,
    pub fscrsn: Fscrsn,
    pub fssrdt: Fssrdt,
    pub fsctln: Fsctln,
    pub fscop: Fscop,
    pub fscpys: Fscpys,
    pub encryp: Encryp,
    pub fbkgc: Fbkgc,
    pub oname: Oname,
    pub ophone: Ophone,
    pub fl: Fl,
    pub hl: Hl,
    pub numi: NumImageSegments,
    pub imheaders: ImageSegments,
    pub nums: NumGraphicSegment,
    pub graphheaders: GraphicsSegments,
    pub numx: Numx,
    pub numt: NumTextFiles,
    pub textfiles: TextSegments,
    pub numdes: Numdes,
    pub dextheaders: DataExtSegments,
    pub numres: Numres,
    pub resheaders: ReservedSegments,
    pub udhdl: Udhdl,
    pub xhdl: Xhdl,
}
impl Display for NitfHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.fhdr).unwrap();
        write!(f, "{}", self.fver).unwrap();
        write!(f, "{}", self.clevel).unwrap();
        write!(f, "{}", self.stype).unwrap();
        write!(f, "{}", self.ostaid).unwrap();
        write!(f, "{}", self.fdt).unwrap();
        write!(f, "{}", self.ftitle).unwrap();
        write!(f, "{}", self.fsclas).unwrap();
        write!(f, "{}", self.fsclsy).unwrap();
        write!(f, "{}", self.fscode).unwrap();
        write!(f, "{}", self.fsctlh).unwrap();
        write!(f, "{}", self.fsrel).unwrap();
        write!(f, "{}", self.fsdctp).unwrap();
        write!(f, "{}", self.fsdcdt).unwrap();
        write!(f, "{}", self.fsdcxm).unwrap();
        write!(f, "{}", self.fsdg).unwrap();
        write!(f, "{}", self.fsdgdt).unwrap();
        write!(f, "{}", self.fscltx).unwrap();
        write!(f, "{}", self.fscatp).unwrap();
        write!(f, "{}", self.fscaut).unwrap();
        write!(f, "{}", self.fscrsn).unwrap();
        write!(f, "{}", self.fssrdt).unwrap();
        write!(f, "{}", self.fsctln).unwrap();
        write!(f, "{}", self.fscop).unwrap();
        write!(f, "{}", self.fscpys).unwrap();
        write!(f, "{}", self.encryp).unwrap();
        write!(f, "{}", self.fbkgc).unwrap();
        write!(f, "{}", self.oname).unwrap();
        write!(f, "{}", self.ophone).unwrap();
        write!(f, "{}", self.fl).unwrap();
        write!(f, "{}", self.hl).unwrap();
        write!(f, "{}", self.numi).unwrap();
        write!(f, "{}", self.imheaders).unwrap();
        write!(f, "{}", self.nums).unwrap();
        write!(f, "{}", self.graphheaders).unwrap();
        write!(f, "{}", self.numx).unwrap();
        write!(f, "{}", self.numt).unwrap();
        write!(f, "{}", self.textfiles).unwrap();
        write!(f, "{}", self.numdes).unwrap();
        write!(f, "{}", self.dextheaders).unwrap();
        write!(f, "{}", self.numres).unwrap();
        write!(f, "{}", self.resheaders).unwrap();
        write!(f, "{}", self.udhdl).unwrap();
        write!(f, "{}", self.xhdl).unwrap();
        Ok(())
    }
}