//! Security information
use std::fmt::Display;
use std::io::{Read, Seek};
use std::str::FromStr;

use super::*;

/// Standard security metadata
#[allow(non_snake_case)]
#[derive(Default, Clone, Hash, Debug)]
pub struct Security {
    /// File Security Classification
    pub CLAS: NitfField<Classification>,
    /// File Classification Security System
    pub CLSY: NitfField<String>,  // TODO: Check value registry
    /// File Codewords
    pub CODE: NitfField<String>,
    /// File Control and Handling
    pub CTLH: NitfField<String>,
    /// File Releasing Instructions
    pub REL: NitfField<String>,
    /// File Declassification Type
    pub DCTP: NitfField<DeclassificationType>,
    /// File Declassification Date
    pub DCDT: NitfField<String>,
    /// File Declassification Exemption
    pub DCXM: NitfField<DeclassificationExemption>,
    /// File Downgrade
    pub DG: NitfField<Downgrade>,
    /// File Downgrade Date
    pub DGDT: NitfField<String>,
    /// File Classification Text
    pub CLTX: NitfField<String>,
    /// File Classification Authority Type
    pub CATP: NitfField<ClassificationAuthorityType>,
    /// File Classification Authority
    pub CAUT: NitfField<String>,
    /// File Classification Reason
    pub CRSN: NitfField<ClassificationReason>,  // TODO: Check value registry
    /// File Security Source Date
    pub SRDT: NitfField<String>,
    /// File Security Control Number
    pub CTLN: NitfField<String>,
}

/// Classification codes
#[derive(Debug, Default, Hash, Clone)]
pub enum Classification {
    #[default]
    /// Unclassified
    U,  
    /// Top Secret
    T,  
    /// Secret
    S,  
    /// Confidential
    C,  
    /// Restricted
    R,  
}

/// Declassification codes
#[derive(Debug, Default, Hash, Clone)]
pub enum DeclassificationType {
    #[default]
    /// Default value, two spaces
    DEFAULT,  
    /// Declassify on specific date
    DD,     
    /// Declassify on occurrence of event
    DE,     
    /// Downgrade to specified level on specific date
    GD,     
    /// Downgrade to specified level on occurrence of event
    GE,     
    /// OADR
    O,      
    /// Exempt from automatic declassification
    X,      
}

///  Declassification exemption
#[derive(Debug, Default, Hash, Clone)]
pub enum DeclassificationExemption {
    #[default]
    /// Default value, four spaces
    DEFAULT,
    /// Valid value, see NitfField.string for value
    VALID,
}

/// Downgrade classification
#[derive(Debug, Default, Hash, Clone)]
pub enum Downgrade {
    #[default]
    /// Default value, two spaces
    DEFAULT,  
    /// Secret
    S,  
    /// Confidential
    C,  
    /// Restricted
    R,  
}

/// Classification authority
#[derive(Debug, Default, Hash, Clone)]
pub enum ClassificationAuthorityType {
    #[default]
    /// Default, one space
    DEFAULT,
    /// Original classification authority
    O,
    /// Derivative from a single source
    D,
    /// Derivative from multiple sources
    M,
}

/// Reason for classification
#[derive(Debug, Default, Hash, Clone)]
pub enum ClassificationReason {
    #[default]
    /// Default value, one space
    DEFAULT,  
    /// Valid value, see NitfField.string for value
    VALID,  
}


impl Security {
    pub fn read(&mut self, reader: &mut (impl Read + Seek)) {
        self.CLAS.read(reader, 1u8);
        self.CLSY.read(reader, 2u8);
        self.CODE.read(reader, 11u8);
        self.CTLH.read(reader, 2u8);
        self.REL.read(reader, 20u8);
        self.DCTP.read(reader, 2u8);
        self.DCDT.read(reader, 8u8);
        self.DCXM.read(reader, 4u8);
        self.DG.read(reader, 1u8);
        self.DGDT.read(reader, 8u8);
        self.CLTX.read(reader, 43u8);
        self.CATP.read(reader, 1u8);
        self.CAUT.read(reader, 40u8);
        self.CRSN.read(reader, 1u8);
        self.SRDT.read(reader, 8u8);
        self.CTLN.read(reader, 15u8);
    }
}
impl Display for Security {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out_str = String::default();
        out_str += format!("\tCLAS: {},\n", self.CLAS).as_ref();
        out_str += format!("\tCLSY: {},\n", self.CLSY).as_ref();
        out_str += format!("\tCODE: {},\n", self.CODE).as_ref();
        out_str += format!("\tCTLH: {},\n", self.CTLH).as_ref();
        out_str += format!("\tREL: {},\n", self.REL).as_ref();
        out_str += format!("\tDCTP: {},\n", self.DCTP).as_ref();
        out_str += format!("\tDCDT: {},\n", self.DCDT).as_ref();
        out_str += format!("\tDCXM: {},\n", self.DCXM).as_ref();
        out_str += format!("\tDG: {},\n", self.DG).as_ref();
        out_str += format!("\tDGDT: {},\n", self.DGDT).as_ref();
        out_str += format!("\tCLTX: {},\n", self.CLTX).as_ref();
        out_str += format!("\tCATP: {},\n", self.CATP).as_ref();
        out_str += format!("\tCAUT: {},\n", self.CAUT).as_ref();
        out_str += format!("\tCRSN: {},\n", self.CRSN).as_ref();
        out_str += format!("\tSRDT: {},\n", self.SRDT).as_ref();
        out_str += format!("\tCTLN: {}", self.CTLN).as_ref();
        return write!(f, "{}", out_str);
    }
}
impl FromStr for Classification {
    type Err = InvalidNitfValue;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "U" => Ok(Self::U),
            "T" => Ok(Self::T),
            "S" => Ok(Self::S),
            "C" => Ok(Self::C),
            "R" => Ok(Self::R),
            _ => Err(InvalidNitfValue)
        }
    }
}
impl FromStr for DeclassificationType {
    type Err = InvalidNitfValue;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "" => Ok(Self::DEFAULT),
            "DD" => Ok(Self::DD),
            "DE" => Ok(Self::DE),
            "GD" => Ok(Self::GD),
            "GE" => Ok(Self::GE),
            "O" => Ok(Self::O),
            "X" => Ok(Self::X),
            _ => Err(InvalidNitfValue)
        }
    }
}
impl FromStr for DeclassificationExemption {
    type Err = InvalidNitfValue;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "" => Ok(Self::DEFAULT),
            "X1" => Ok(Self::VALID),    // DOD 5200.01-V1, 4-201b(1)
            "X2" => Ok(Self::VALID),    // DOD 5200.01-V1, 4-201b(2)
            "X3" => Ok(Self::VALID),    // DOD 5200.01-V1, 4-201b(3)
            "X4" => Ok(Self::VALID),    // DOD 5200.01-V1, 4-201b(4)
            "X5" => Ok(Self::VALID),    // DOD 5200.01-V1, 4-201b(5)
            "X6" => Ok(Self::VALID),    // DOD 5200.01-V1, 4-201b(6)
            "X7" => Ok(Self::VALID),    // DOD 5200.01-V1, 4-201b(7)
            "X8" => Ok(Self::VALID),    // DOD 5200.01-V1, 4-201b(8)
            "25X1" => Ok(Self::VALID),  // DOD 5200.01-V1, 4-301b(1)
            "25X2" => Ok(Self::VALID),  // DOD 5200.01-V1, 4-301b(2)
            "25X3" => Ok(Self::VALID),  // DOD 5200.01-V1, 4-301b(3)
            "25X4" => Ok(Self::VALID),  // DOD 5200.01-V1, 4-301b(4)
            "25X5" => Ok(Self::VALID),  // DOD 5200.01-V1, 4-301b(5)
            "25X6" => Ok(Self::VALID),  // DOD 5200.01-V1, 4-301b(6)
            "25X7" => Ok(Self::VALID),  // DOD 5200.01-V1, 4-301b(7)
            "25X8" => Ok(Self::VALID),  // DOD 5200.01-V1, 4-301b(8)
            "25X9" => Ok(Self::VALID),  // DOD 5200.01-V1, 4-301b(9)
            _ => Err(InvalidNitfValue),
        }
    }
}
impl FromStr for Downgrade {
    type Err = InvalidNitfValue;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "" => Ok(Self::DEFAULT),
            "S" => Ok(Self::S),
            "C" => Ok(Self::C),
            "R" => Ok(Self::R),
            _ => Err(InvalidNitfValue)
        }
    }
}
impl FromStr for ClassificationAuthorityType {
    type Err = InvalidNitfValue;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "" => Ok(Self::DEFAULT),
            "O" => Ok(Self::O),
            "D" => Ok(Self::D),
            "M" => Ok(Self::M),
            _ => Err(InvalidNitfValue) 
        }
    }
}
impl FromStr for ClassificationReason {
    type Err = InvalidNitfValue;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "" => Ok(Self::DEFAULT),
            "A" => Ok(Self::VALID),
            "B" => Ok(Self::VALID),
            "C" => Ok(Self::VALID),
            "D" => Ok(Self::VALID),
            "E" => Ok(Self::VALID),
            "F" => Ok(Self::VALID),
            "G" => Ok(Self::VALID),
            "H" => Ok(Self::VALID),
            _ => Err(InvalidNitfValue)
        }
    }
}