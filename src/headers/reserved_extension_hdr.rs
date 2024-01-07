//! Reserved Extension segment subheader definition
use std::fmt::Display;
use std::fs::File;

use crate::headers::NitfSegmentHeader;
use crate::types::{ExtendedSubheader, NitfField, Security};
use crate::NitfResult;
/// Metadata for Reserved Extension Segment
#[derive(Default, Clone, Debug, Eq, PartialEq)]
pub struct ReservedExtensionHeader {
    /// File Part Type
    pub re: NitfField<String>,
    /// Unique RES Type Identifier
    pub resid: NitfField<String>,
    /// Version of the Data Definition
    pub resver: NitfField<u8>,
    /// Security information
    pub security: Security,
    /// User-defined Subheader Length
    pub resshl: NitfField<u16>,
    /// User-Defined Subheader Fields
    pub resshf: ExtendedSubheader,
}
impl Display for ReservedExtensionHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out_str = String::default();
        out_str += format!("RE: {}, ", self.re).as_ref();
        out_str += format!("RESID: {}, ", self.resid).as_ref();
        out_str += format!("RESVER: {}, ", self.resver).as_ref();
        out_str += format!("SECURITY: [{}], ", self.security).as_ref();
        out_str += format!("RESSHL: {}, ", self.resshl).as_ref();
        out_str += format!("RESSHF: {}, ", self.resshf).as_ref();
        write!(f, "[Reserved Extension Subheader: {out_str}]")
    }
}
impl NitfSegmentHeader for ReservedExtensionHeader {
    fn read(&mut self, reader: &mut File) -> NitfResult<()> {
        self.re.read(reader, 2u8, "RE")?;
        self.resid.read(reader, 25u8, "RESID")?;
        self.resver.read(reader, 2u8, "RESVER")?;
        self.security.read(reader)?;
        self.resshl.read(reader, 4u8, "RESSHL")?;
        if self.resshl.val().clone() != 0 {
            self.resshf
                .read(reader, self.resshl.val().clone() as usize, "RESSHF")?;
        }
        Ok(())
    }
    fn write(&self, writer: &mut File) -> NitfResult<usize> {
        let mut bytes_written = 0;
        bytes_written += self.re.write(writer, "RE")?;
        bytes_written += self.resid.write(writer, "RESID")?;
        bytes_written += self.resver.write(writer, "RESVER")?;
        bytes_written += self.security.write(writer)?;
        bytes_written += self.resshl.write(writer, "RESSHL")?;
        if self.resshl.val().clone() != 0 {
            bytes_written += self.resshf.write(writer, "RESSHF")?;
        }
        Ok(bytes_written)
    }
    fn length(&self) -> usize {
        let mut length = 0;
        length += self.re.length();
        length += self.resid.length();
        length += self.resver.length();
        length += self.security.length();
        length += self.resshl.length();
        if self.resshl.val().clone() != 0 {
            length += self.resshf.length();
        }
        length
        
    }
}
