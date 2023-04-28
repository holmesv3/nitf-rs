//! Image Segment Definition
use std::io::Read;
use std::fmt::Display;
use std::string::FromUtf8Error;

use super::elements::image::*;

impl ImageSegment {
    pub fn from_reader(reader: &mut impl Read) -> Result<Self, FromUtf8Error> {
        let mut imseg = ImageSegment::default();
        reader.read(&mut imseg.fileparttype.val).unwrap();
        reader.read(&mut imseg.iid1.val).unwrap();
        reader.read(&mut imseg.idatim.val).unwrap();
        reader.read(&mut imseg.tgtid.val).unwrap();
        reader.read(&mut imseg.iid2.val).unwrap();
        reader.read(&mut imseg.isclas.val).unwrap();
        reader.read(&mut imseg.isclsy.val).unwrap();
        reader.read(&mut imseg.iscode.val).unwrap();
        reader.read(&mut imseg.isctlh.val).unwrap();
        reader.read(&mut imseg.isrel.val).unwrap();
        reader.read(&mut imseg.isdctp.val).unwrap();
        reader.read(&mut imseg.isdcdt.val).unwrap();
        reader.read(&mut imseg.isdcxm.val).unwrap();
        reader.read(&mut imseg.isdg.val).unwrap();
        reader.read(&mut imseg.isdgdt.val).unwrap();
        reader.read(&mut imseg.iscltx.val).unwrap();
        reader.read(&mut imseg.iscatp.val).unwrap();
        reader.read(&mut imseg.iscaut.val).unwrap();
        reader.read(&mut imseg.iscrsn.val).unwrap();
        reader.read(&mut imseg.issrdt.val).unwrap();
        reader.read(&mut imseg.isctln.val).unwrap();
        reader.read(&mut imseg.encryp.val).unwrap();
        reader.read(&mut imseg.isorce.val).unwrap();
        reader.read(&mut imseg.nrows.val).unwrap();
        reader.read(&mut imseg.ncols.val).unwrap();
        reader.read(&mut imseg.pvtype.val).unwrap();
        reader.read(&mut imseg.irep.val).unwrap();
        reader.read(&mut imseg.icat.val).unwrap();
        reader.read(&mut imseg.abpp.val).unwrap();
        reader.read(&mut imseg.pjust.val).unwrap();
        reader.read(&mut imseg.icords.val).unwrap();
        // Image comments
        reader.read(&mut imseg.nicom.val).unwrap();
        let n_com : usize=  String::from_utf8(imseg.nicom.val.to_vec())
            .unwrap()
            .parse()
            .unwrap();
        for _ in 0..n_com {
            let mut comment = ICOM::default();
            reader.read(&mut comment.val).unwrap();
            imseg.icoms.val.push(comment);
        }
        reader.read(&mut imseg.ic.val).unwrap();
        reader.read(&mut imseg.nbands.val).unwrap();
        reader.read(&mut imseg.irepband1.val).unwrap();
        reader.read(&mut imseg.isubcat1.val).unwrap();
        reader.read(&mut imseg.ifc1.val).unwrap();
        reader.read(&mut imseg.imflt1.val).unwrap();
        reader.read(&mut imseg.nluts1.val).unwrap();
        reader.read(&mut imseg.isync.val).unwrap();
        reader.read(&mut imseg.imode.val).unwrap();
        reader.read(&mut imseg.nbpr.val).unwrap();
        reader.read(&mut imseg.nbpc.val).unwrap();
        reader.read(&mut imseg.nppbh.val).unwrap();
        reader.read(&mut imseg.nppbv.val).unwrap();
        reader.read(&mut imseg.nbpp.val).unwrap();
        reader.read(&mut imseg.idlvl.val).unwrap();
        reader.read(&mut imseg.ialvl.val).unwrap();
        reader.read(&mut imseg.iloc.val).unwrap();
        reader.read(&mut imseg.imag.val).unwrap();
        reader.read(&mut imseg.udidl.val).unwrap();
        reader.read(&mut imseg.ixshdl.val).unwrap();
        Ok(imseg)
    }
}

#[derive(Default, Clone, Hash, Debug)]
pub struct ImageSegment {
    fileparttype: FILEPARTTYPE,
    iid1: IID1,
    idatim: IDATIM,
    tgtid: TGTID,
    iid2: IID2,
    isclas: ISCLAS,
    isclsy: ISCLSY,
    iscode: ISCODE,
    isctlh: ISCTLH,
    isrel: ISREL,
    isdctp: ISDCTP,
    isdcdt: ISDCDT,
    isdcxm: ISDCXM,
    isdg: ISDG,
    isdgdt: ISDGDT,
    iscltx: ISCLTX,
    iscatp: ISCATP,
    iscaut: ISCAUT,
    iscrsn: ISCRSN,
    issrdt: ISSRDT,
    isctln: ISCTLN,
    encryp: ENCRYP,
    isorce: ISORCE,
    nrows: NROWS,
    ncols: NCOLS,
    pvtype: PVTYPE,
    irep: IREP,
    icat: ICAT,
    abpp: ABPP,
    pjust: PJUST,
    icords: ICORDS,
    nicom: NICOM,
    icoms: ICOMS,
    icom: ICOM,
    ic: IC,
    nbands: NBANDS,
    irepband1: IREPBAND1,
    isubcat1: ISUBCAT1,
    ifc1: IFC1,
    imflt1: IMFLT1,
    nluts1: NLUTS1,
    isync: ISYNC,
    imode: IMODE,
    nbpr: NBPR,
    nbpc: NBPC,
    nppbh: NPPBH,
    nppbv: NPPBV,
    nbpp: NBPP,
    idlvl: IDLVL,
    ialvl: IALVL,
    iloc: ILOC,
    imag: IMAG,
    udidl: UDIDL,
    ixshdl: IXSHDL,
}
impl Display for ImageSegment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out_str = String::default();
        out_str += format!("{}", self.fileparttype).as_ref();
        out_str += format!("{}", self.iid1).as_ref();
        out_str += format!("{}", self.idatim).as_ref();
        out_str += format!("{}", self.tgtid).as_ref();
        out_str += format!("{}", self.iid2).as_ref();
        out_str += format!("{}", self.isclas).as_ref();
        out_str += format!("{}", self.isclsy).as_ref();
        out_str += format!("{}", self.iscode).as_ref();
        out_str += format!("{}", self.isctlh).as_ref();
        out_str += format!("{}", self.isrel).as_ref();
        out_str += format!("{}", self.isdctp).as_ref();
        out_str += format!("{}", self.isdcdt).as_ref();
        out_str += format!("{}", self.isdcxm).as_ref();
        out_str += format!("{}", self.isdg).as_ref();
        out_str += format!("{}", self.isdgdt).as_ref();
        out_str += format!("{}", self.iscltx).as_ref();
        out_str += format!("{}", self.iscatp).as_ref();
        out_str += format!("{}", self.iscaut).as_ref();
        out_str += format!("{}", self.iscrsn).as_ref();
        out_str += format!("{}", self.issrdt).as_ref();
        out_str += format!("{}", self.isctln).as_ref();
        out_str += format!("{}", self.encryp).as_ref();
        out_str += format!("{}", self.isorce).as_ref();
        out_str += format!("{}", self.nrows).as_ref();
        out_str += format!("{}", self.ncols).as_ref();
        out_str += format!("{}", self.pvtype).as_ref();
        out_str += format!("{}", self.irep).as_ref();
        out_str += format!("{}", self.icat).as_ref();
        out_str += format!("{}", self.abpp).as_ref();
        out_str += format!("{}", self.pjust).as_ref();
        out_str += format!("{}", self.icords).as_ref();
        out_str += format!("{}", self.nicom).as_ref();
        out_str += format!("{}", self.icoms).as_ref();
        out_str += format!("{}", self.icom).as_ref();
        out_str += format!("{}", self.ic).as_ref();
        out_str += format!("{}", self.nbands).as_ref();
        out_str += format!("{}", self.irepband1).as_ref();
        out_str += format!("{}", self.isubcat1).as_ref();
        out_str += format!("{}", self.ifc1).as_ref();
        out_str += format!("{}", self.imflt1).as_ref();
        out_str += format!("{}", self.nluts1).as_ref();
        out_str += format!("{}", self.isync).as_ref();
        out_str += format!("{}", self.imode).as_ref();
        out_str += format!("{}", self.nbpr).as_ref();
        out_str += format!("{}", self.nbpc).as_ref();
        out_str += format!("{}", self.nppbh).as_ref();
        out_str += format!("{}", self.nppbv).as_ref();
        out_str += format!("{}", self.nbpp).as_ref();
        out_str += format!("{}", self.idlvl).as_ref();
        out_str += format!("{}", self.ialvl).as_ref();
        out_str += format!("{}", self.iloc).as_ref();
        out_str += format!("{}", self.imag).as_ref();
        out_str += format!("{}", self.udidl).as_ref();
        out_str += format!("{}", self.ixshdl).as_ref();
        write!(f, "ImageSegment: [{}]", out_str)
    }
}