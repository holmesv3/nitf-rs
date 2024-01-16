//! Reserved Extension segment subheader definition
use std::fmt::Display;
use std::fs::File;
use std::str::FromStr;

use crate::headers::NitfSegmentHeader;
use crate::types::{ExtendedSubheader, NitfField, Security};
use crate::{NitfResult, NitfError};
/// Metadata for Reserved Extension Segment
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReservedExtensionHeader {
    /// File Part Type
    pub re: NitfField<RE>,
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
impl Default for ReservedExtensionHeader {
    fn default() -> Self {
        Self {
            re: NitfField::init(2u8, "RE"),
            resid: NitfField::init(25u8, "RESID"),
            resver: NitfField::init(2u8, "RESVER"),
            security: Security::default(),
            resshl: NitfField::init(4u8, "RESSHL"),
            resshf: ExtendedSubheader::init("RESSHF"), 
        }
    }
}
#[derive(Default, Clone, Debug, Eq, PartialEq)]
pub enum RE {
    #[default]
    RE    
}
impl FromStr for RE {
    type Err = NitfError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "RE" => Ok(Self::default()),
            _ => Err(NitfError::ParseError("RE".to_string()))
        }
    }
}
impl Display for RE {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RE")
    }
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
        self.re.read(reader)?;
        self.resid.read(reader)?;
        self.resver.read(reader)?;
        self.security.read(reader)?;
        self.resshl.read(reader)?;
        if self.resshl.val != 0 {
            self.resshf
                .read(reader, self.resshl.val as usize)?;
        }
        Ok(())
    }
    fn write(&self, writer: &mut File) -> NitfResult<usize> {
        let mut bytes_written = 0;
        bytes_written += self.re.write(writer)?;
        bytes_written += self.resid.write(writer)?;
        bytes_written += self.resver.write(writer)?;
        bytes_written += self.security.write(writer)?;
        bytes_written += self.resshl.write(writer)?;
        if self.resshl.val != 0 {
            bytes_written += self.resshf.write(writer)?;
        }
        Ok(bytes_written)
    }
    fn length(&self) -> usize {
        let mut length = 0;
        length += self.re.length;
        length += self.resid.length;
        length += self.resver.length;
        length += self.security.length();
        length += self.resshl.length;
        if self.resshl.val != 0 {
            length += self.resshf.size();
        }
        length
        
    }
}
