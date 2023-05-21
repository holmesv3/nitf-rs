//! Text segment definition
use std::fmt::Display;
use std::io::{Read, Seek};

use crate::nitf_2_1::types::{NitfField, NitfSegmentHeader, Security};

#[allow(non_snake_case)]
#[derive(Default, Clone, Hash, Debug)]
pub struct TextHeader {
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
    /// Security information
    pub SECURITY: Security,
    /// Encryption
    pub ENCRYP: NitfField,
    /// Text Format
    pub TXTFMT: NitfField,
    /// Text Extended Subheader Data Length
    pub TXSHDL: NitfField,
    /// Text Extended Subheader Overflow
    pub TXSOFL: NitfField,
    /// Text Extended Subheader Data
    pub TXSHD: NitfField,
}
impl Display for TextHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out_str = String::default();
        out_str += format!("TE: {}, ", self.TE).as_ref();
        out_str += format!("TEXTID: {}, ", self.TEXTID).as_ref();
        out_str += format!("TXTALVL: {}, ", self.TXTALVL).as_ref();
        out_str += format!("TXTDT: {}, ", self.TXTDT).as_ref();
        out_str += format!("TXTTITL: {}, ", self.TXTTITL).as_ref();
        out_str += format!("SECURITY: [{}], ", self.SECURITY).as_ref();
        out_str += format!("ENCRYP: {}, ", self.ENCRYP).as_ref();
        out_str += format!("TXTFMT: {}, ", self.TXTFMT).as_ref();
        out_str += format!("TXSHDL: {}", self.TXSHDL).as_ref();
        out_str += format!("TXSOFL: {}", self.TXSOFL).as_ref();
        out_str += format!("TXSHD: {}", self.TXSHD).as_ref();
        write!(f, "TextSegment: [{}]", out_str)
    }
}
impl NitfSegmentHeader for TextHeader {
    fn read(&mut self, reader: &mut (impl Read + Seek)) {
        self.TE.read(reader, 2u8);
        self.TEXTID.read(reader, 7u8);
        self.TXTALVL.read(reader, 3u8);
        self.TXTDT.read(reader, 14u8);
        self.TXTTITL.read(reader, 80u8);
        self.SECURITY.read(reader);
        self.ENCRYP.read(reader, 1u8);
        self.TXTFMT.read(reader, 3u8);
        self.TXSHDL.read(reader, 5u8);
        let extended_length: u32 = self.TXSHDL.string.parse().unwrap();
        if extended_length != 0 {
            self.TXSOFL.read(reader, 3u8);
            self.TXSHD.read(reader, extended_length - 3)
        }
    }
}
