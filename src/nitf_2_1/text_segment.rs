//! Text Segment Definition
use std::io::Read;
use std::fmt::Display;
use std::string::FromUtf8Error;

use super::elements::text::*;


#[derive(Default, Clone, Hash, Debug)]
pub struct TextSegment {
    pub te: TE,
    pub textid: TEXTID,
    pub txtalvl: TXTALVL,
    pub txtdt: TXTDT,
    pub txttitl: TXTTITL,
    pub tsclas: TSCLAS,
    pub tsclsy: TSCLSY,
    pub tscode: TSCODE,
    pub tsctlh: TSCTLH,
    pub tsrel: TSREL,
    pub tsdctp: TSDCTP,
    pub tsdcdt: TSDCDT,
    pub tsdcxm: TSDCXM,
    pub tsdg: TSDG,
    pub tsdgdt: TSDGDT,
    pub tsclttx: TSCLTTX,
    pub tscatp: TSCATP,
    pub tscaut: TSCAUT,
    pub tscsn: TSCSN,
    pub tssrdt: TSSRDT,
    pub tsctln: TSCTLN,
    pub encryp: ENCRYP,
    pub txtfmt: TXTFMT,
    pub txshdl: TXSHDL,
}
impl Display for TextSegment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out_str = String::default();
        out_str += format!("{}", self.te).as_ref();
        out_str += format!("{}", self.textid).as_ref();
        out_str += format!("{}", self.txtalvl).as_ref();
        out_str += format!("{}", self.txtdt).as_ref();
        out_str += format!("{}", self.txttitl).as_ref();
        out_str += format!("{}", self.tsclas).as_ref();
        out_str += format!("{}", self.tsclsy).as_ref();
        out_str += format!("{}", self.tscode).as_ref();
        out_str += format!("{}", self.tsctlh).as_ref();
        out_str += format!("{}", self.tsrel).as_ref();
        out_str += format!("{}", self.tsdctp).as_ref();
        out_str += format!("{}", self.tsdcdt).as_ref();
        out_str += format!("{}", self.tsdcxm).as_ref();
        out_str += format!("{}", self.tsdg).as_ref();
        out_str += format!("{}", self.tsdgdt).as_ref();
        out_str += format!("{}", self.tsclttx).as_ref();
        out_str += format!("{}", self.tscatp).as_ref();
        out_str += format!("{}", self.tscaut).as_ref();
        out_str += format!("{}", self.tscsn).as_ref();
        out_str += format!("{}", self.tssrdt).as_ref();
        out_str += format!("{}", self.tsctln).as_ref();
        out_str += format!("{}", self.encryp).as_ref();
        out_str += format!("{}", self.txtfmt).as_ref();
        out_str += format!("{}", self.txshdl).as_ref();
        write!(f, "{}", out_str)
    }
}