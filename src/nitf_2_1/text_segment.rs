//! Text segment definition
use std::io::{Read, Seek};
use std::fmt::Display;
use std::string::FromUtf8Error;

use crate::types::NitfField;

#[allow(non_snake_case)]
#[derive(Default, Clone, Hash, Debug)]
pub struct TextSegment {
    /// File Part Type 
    pub TE: NitfField,
    /// Text Identifier 
    pub TEXTID: NitfField,
    /// Text Attachment Level 
    pub TXTALVL: NitfField,
    /// Text Date and Time 
    pub TXTDT: NitfField,
    /// Text Title 
    pub TXTTITL: NitfField,
    /// Text Security Classification 
    pub TSCLAS: NitfField,
    /// Text Classification Security System 
    pub TSCLSY: NitfField,
    /// Text Codewords 
    pub TSCODE: NitfField,
    /// Text Control and Handling 
    pub TSCTLH: NitfField,
    /// Text Releasing Instructions 
    pub TSREL: NitfField,
    /// Text Declassification Type 
    pub TSDCTP: NitfField,
    /// Text Declassification Date 
    pub TSDCDT: NitfField,
    /// Text Declassification Exemption 
    pub TSDCXM: NitfField,
    /// Text Downgrade 
    pub TSDG: NitfField,
    /// Text Downgrade Date 
    pub TSDGDT: NitfField,
    /// Text Classification Text 
    pub TSCLTTX: NitfField,
    /// Text Classification Authority Type 
    pub TSCATP: NitfField,
    /// Text Classification Authority 
    pub TSCAUT: NitfField,
    /// Text Classification Reason 
    pub TSCSN: NitfField,
    /// Text Security Source Date 
    pub TSSRDT: NitfField,
    /// Text Security Control Number 
    pub TSCTLN: NitfField,
    /// Encryption 
    pub ENCRYP: NitfField,
    /// Text Format 
    pub TXTFMT: NitfField,
    /// Text Extended Subheader Data Length 
    pub TXSHDL: NitfField,
}
impl Display for TextSegment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out_str = String::default();
        out_str += format!("\nTE: {}", self.TE).as_ref();
        out_str += format!("\nTEXTID: {}", self.TEXTID).as_ref();
        out_str += format!("\nTXTALVL: {}", self.TXTALVL).as_ref();
        out_str += format!("\nTXTDT: {}", self.TXTDT).as_ref();
        out_str += format!("\nTXTTITL: {}", self.TXTTITL).as_ref();
        out_str += format!("\nTSCLAS: {}", self.TSCLAS).as_ref();
        out_str += format!("\nTSCLSY: {}", self.TSCLSY).as_ref();
        out_str += format!("\nTSCODE: {}", self.TSCODE).as_ref();
        out_str += format!("\nTSCTLH: {}", self.TSCTLH).as_ref();
        out_str += format!("\nTSREL: {}", self.TSREL).as_ref();
        out_str += format!("\nTSDCTP: {}", self.TSDCTP).as_ref();
        out_str += format!("\nTSDCDT: {}", self.TSDCDT).as_ref();
        out_str += format!("\nTSDCXM: {}", self.TSDCXM).as_ref();
        out_str += format!("\nTSDG: {}", self.TSDG).as_ref();
        out_str += format!("\nTSDGDT: {}", self.TSDGDT).as_ref();
        out_str += format!("\nTSCLTTX: {}", self.TSCLTTX).as_ref();
        out_str += format!("\nTSCATP: {}", self.TSCATP).as_ref();
        out_str += format!("\nTSCAUT: {}", self.TSCAUT).as_ref();
        out_str += format!("\nTSCSN: {}", self.TSCSN).as_ref();
        out_str += format!("\nTSSRDT: {}", self.TSSRDT).as_ref();
        out_str += format!("\nTSCTLN: {}", self.TSCTLN).as_ref();
        out_str += format!("\nENCRYP: {}", self.ENCRYP).as_ref();
        out_str += format!("\nTXTFMT: {}", self.TXTFMT).as_ref();
        out_str += format!("\nTXSHDL: {}", self.TXSHDL).as_ref();
        write!(f, "TextSegment: [{}]", out_str)
    }
}
impl TextSegment {
    pub fn  from_reader(reader: &mut (impl Read + Seek)) -> Result<Self, FromUtf8Error> {
        let mut txt_seg = TextSegment::default(); 
        txt_seg.TE.read(reader, 2);
        txt_seg.TEXTID.read(reader, 7);
        txt_seg.TXTALVL.read(reader, 3);
        txt_seg.TXTDT.read(reader, 14);
        txt_seg.TXTTITL.read(reader, 80);
        txt_seg.TSCLAS.read(reader, 1);
        txt_seg.TSCLSY.read(reader, 2);
        txt_seg.TSCODE.read(reader, 11);
        txt_seg.TSCTLH.read(reader, 2);
        txt_seg.TSREL.read(reader, 20);
        txt_seg.TSDCTP.read(reader, 2);
        txt_seg.TSDCDT.read(reader, 8);
        txt_seg.TSDCXM.read(reader, 4);
        txt_seg.TSDG.read(reader, 1);
        txt_seg.TSDGDT.read(reader, 8);
        txt_seg.TSCLTTX.read(reader, 43);
        txt_seg.TSCATP.read(reader, 1);
        txt_seg.TSCAUT.read(reader, 40);
        txt_seg.TSCSN.read(reader, 1);
        txt_seg.TSSRDT.read(reader, 8);
        txt_seg.TSCTLN.read(reader, 15);
        txt_seg.ENCRYP.read(reader, 1);
        txt_seg.TXTFMT.read(reader, 3);
        txt_seg.TXSHDL.read(reader, 5);
        Ok(txt_seg)
    }
}