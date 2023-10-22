//! Common types use throughout
use std::fmt::{Debug, Display};
use std::str::FromStr;
use std::io::{Read, Seek};
use std::fs::File;

use crate::error::NitfError;

/// Lowest level object for file parsing
#[derive(Default, Clone, Debug, Eq, PartialEq)]
pub struct NitfField<V: FromStr + Debug> {
    /// Byte representation
    pub bytes: Vec<u8>,
    /// String representation of field
    pub string: String,
    /// Parsed representation of value
    pub val: V,
    /// Number of bytes used to store value in file
    length: u64,
    /// Byte offset in file
    offset: u64,
}

/// Standard security metadata
#[derive(Default, Clone, Debug, Eq, PartialEq)]
pub struct Security {
    /// File Security Classification
    pub clas: NitfField<Classification>,
    /// File Classification Security System
    pub clsy: NitfField<String>, // TODO: Check value registry
    /// File Codewords
    pub code: NitfField<String>,
    /// File Control and Handling
    pub ctlh: NitfField<String>,
    /// File Releasing Instructions
    pub rel: NitfField<String>,
    /// File Declassification Type
    pub dctp: NitfField<DeclassificationType>,
    /// File Declassification Date
    pub dcdt: NitfField<String>,
    /// File Declassification Exemption
    pub dcxm: NitfField<DeclassificationExemption>,
    /// File Downgrade
    pub dg: NitfField<Downgrade>,
    /// File Downgrade Date
    pub dgdt: NitfField<String>,
    /// File Classification Text
    pub cltx: NitfField<String>,
    /// File Classification Authority Type
    pub catp: NitfField<ClassificationAuthorityType>,
    /// File Classification Authority
    pub caut: NitfField<String>,
    /// File Classification Reason
    pub crsn: NitfField<ClassificationReason>, // TODO: Check value registry
    /// File Security Source Date
    pub srdt: NitfField<String>,
    /// File Security Control Number
    pub ctln: NitfField<String>,
}

/// Classification codes
#[derive(Debug, Default, Clone, Eq, PartialEq)]
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
#[derive(Debug, Default, Clone, Eq, PartialEq)]
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
#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub enum DeclassificationExemption {
    #[default]
    /// Default value, four spaces
    DEFAULT,
    /// Valid value, see NitfField.string for value
    VALID,
}

/// Downgrade classification
#[derive(Debug, Default, Clone, Eq, PartialEq)]
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
#[derive(Debug, Default, Clone, Eq, PartialEq)]
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
#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub enum ClassificationReason {
    #[default]
    /// Default value, one space
    DEFAULT,
    /// Valid value, see NitfField.string for value
    VALID,
}

/// Use Default implementation
impl<V> NitfField<V>
where
    V: FromStr + Debug,
    <V as FromStr>::Err: Debug,
{
    /// Read the specified number of bytes and parse the value of a given field
    pub fn read<T: Sized + Into<u64>>(&mut self, reader: &mut File, n_bytes: T) {
        self.length = n_bytes.into();
        self.bytes = vec![0; self.length as usize];
        self.offset = reader.stream_position().unwrap();
        reader.read_exact(&mut self.bytes).unwrap();
        let result = String::from_utf8(self.bytes.to_vec());
        match result {
            Ok(str) => {
                self.string = str.trim().to_string();
                self.val = self.string.parse().unwrap();
            }
            Err(err) => {
                self.string = String::from("Error parsing string");
                println!("{}", err)
            }
        }
    }
}
impl<V: FromStr + Debug> Display for NitfField<V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.string)
    }
}
impl Security {
    pub fn read(&mut self, reader: &mut File) {
        self.clas.read(reader, 1u8);
        self.clsy.read(reader, 2u8);
        self.code.read(reader, 11u8);
        self.ctlh.read(reader, 2u8);
        self.rel.read(reader, 20u8);
        self.dctp.read(reader, 2u8);
        self.dcdt.read(reader, 8u8);
        self.dcxm.read(reader, 4u8);
        self.dg.read(reader, 1u8);
        self.dgdt.read(reader, 8u8);
        self.cltx.read(reader, 43u8);
        self.catp.read(reader, 1u8);
        self.caut.read(reader, 40u8);
        self.crsn.read(reader, 1u8);
        self.srdt.read(reader, 8u8);
        self.ctln.read(reader, 15u8);
    }
}
impl Display for Security {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out_str = String::default();
        out_str += format!("CLAS: {}, ", self.clas).as_ref();
        out_str += format!("CLSY: {}, ", self.clsy).as_ref();
        out_str += format!("CODE: {}, ", self.code).as_ref();
        out_str += format!("CTLH: {}, ", self.ctlh).as_ref();
        out_str += format!("REL: {}, ", self.rel).as_ref();
        out_str += format!("DCTP: {}, ", self.dctp).as_ref();
        out_str += format!("DCDT: {}, ", self.dcdt).as_ref();
        out_str += format!("DCXM: {}, ", self.dcxm).as_ref();
        out_str += format!("DG: {}, ", self.dg).as_ref();
        out_str += format!("DGDT: {}, ", self.dgdt).as_ref();
        out_str += format!("CLTX: {}, ", self.cltx).as_ref();
        out_str += format!("CATP: {}, ", self.catp).as_ref();
        out_str += format!("CAUT: {}, ", self.caut).as_ref();
        out_str += format!("CRSN: {}, ", self.crsn).as_ref();
        out_str += format!("SRDT: {}, ", self.srdt).as_ref();
        out_str += format!("CTLN: {}", self.ctln).as_ref();
        write!(f, "{}", out_str)
    }
}
impl FromStr for Classification {
    type Err = NitfError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "U" => Ok(Self::U),
            "T" => Ok(Self::T),
            "S" => Ok(Self::S),
            "C" => Ok(Self::C),
            "R" => Ok(Self::R),
            _ => Err(NitfError::FieldError),
        }
    }
}
impl FromStr for DeclassificationType {
    type Err = NitfError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "" => Ok(Self::DEFAULT),
            "DD" => Ok(Self::DD),
            "DE" => Ok(Self::DE),
            "GD" => Ok(Self::GD),
            "GE" => Ok(Self::GE),
            "O" => Ok(Self::O),
            "X" => Ok(Self::X),
            _ => Err(NitfError::FieldError),
        }
    }
}
impl FromStr for DeclassificationExemption {
    type Err = NitfError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "" => Ok(Self::DEFAULT),
            "X1" => Ok(Self::VALID),   // DOD 5200.01-V1, 4-201b(1)
            "X2" => Ok(Self::VALID),   // DOD 5200.01-V1, 4-201b(2)
            "X3" => Ok(Self::VALID),   // DOD 5200.01-V1, 4-201b(3)
            "X4" => Ok(Self::VALID),   // DOD 5200.01-V1, 4-201b(4)
            "X5" => Ok(Self::VALID),   // DOD 5200.01-V1, 4-201b(5)
            "X6" => Ok(Self::VALID),   // DOD 5200.01-V1, 4-201b(6)
            "X7" => Ok(Self::VALID),   // DOD 5200.01-V1, 4-201b(7)
            "X8" => Ok(Self::VALID),   // DOD 5200.01-V1, 4-201b(8)
            "25X1" => Ok(Self::VALID), // DOD 5200.01-V1, 4-301b(1)
            "25X2" => Ok(Self::VALID), // DOD 5200.01-V1, 4-301b(2)
            "25X3" => Ok(Self::VALID), // DOD 5200.01-V1, 4-301b(3)
            "25X4" => Ok(Self::VALID), // DOD 5200.01-V1, 4-301b(4)
            "25X5" => Ok(Self::VALID), // DOD 5200.01-V1, 4-301b(5)
            "25X6" => Ok(Self::VALID), // DOD 5200.01-V1, 4-301b(6)
            "25X7" => Ok(Self::VALID), // DOD 5200.01-V1, 4-301b(7)
            "25X8" => Ok(Self::VALID), // DOD 5200.01-V1, 4-301b(8)
            "25X9" => Ok(Self::VALID), // DOD 5200.01-V1, 4-301b(9)
            _ => Err(NitfError::FieldError),
        }
    }
}
impl FromStr for Downgrade {
    type Err = NitfError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "" => Ok(Self::DEFAULT),
            "S" => Ok(Self::S),
            "C" => Ok(Self::C),
            "R" => Ok(Self::R),
            _ => Err(NitfError::FieldError),
        }
    }
}
impl FromStr for ClassificationAuthorityType {
    type Err = NitfError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "" => Ok(Self::DEFAULT),
            "O" => Ok(Self::O),
            "D" => Ok(Self::D),
            "M" => Ok(Self::M),
            _ => Err(NitfError::FieldError),
        }
    }
}
impl FromStr for ClassificationReason {
    type Err = NitfError;
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
            _ => Err(NitfError::FieldError),
        }
    }
}

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct ExtendedSubheader {
    /// User defined tagged record entries (TREs)
    pub tre: Vec<u8>,
    /// Length of subheader
    pub size: usize,

}
impl ExtendedSubheader {
    pub fn read(&mut self, reader: &mut File, n_bytes: usize) {
        self.size = n_bytes;
        self.tre = vec![0; n_bytes];
        reader.read_exact(self.tre.as_mut_slice()).unwrap();

    }
}
impl Display for ExtendedSubheader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Can't copy vector directly, convert to slice, then clone into new vector
        let out_str = String::from_utf8(self.tre.to_vec()).unwrap();
        write!(f, "[{out_str}]")
    }
}
