//! Common types use throughout
use log::{trace, warn};
use std::fmt::{Debug, Display};
use std::fs::File;
use std::io::{Read, Seek, Write};
use std::str::FromStr;

use crate::{NitfError, NitfResult};

/// Lowest level object for file parsing
#[derive(Default, Clone, Debug, Eq, PartialEq)]
pub struct NitfField<V: FromStr + Debug> {
    /// Byte representation
    bytes: Vec<u8>,
    /// String representation of field
    string: String,
    /// Parsed representation of value
    val: V,
    /// Number of bytes used to store value in file
    length: u64,
    /// Byte offset in file
    offset: u64,
}

/// Provide default implementation of reading a field.
impl<V> NitfField<V>
where
V: FromStr + Debug + Default + Into<String>,
<V as FromStr>::Err: Debug,
{
    // Getters
    pub fn get_val(&self) -> V { self.val }
    pub fn get_string(&self) -> String { self.string }
    pub fn get_bytes(&self) -> Vec<u8> { self.bytes }
    
    // Setters need to updated other fields upon change
    pub fn set_val(&mut self, new_val: V) -> NitfResult<()> { 
        self.val = new_val; 
        self.string = new_val.into();
        Ok(()) 
    }
    
    pub fn set_string(&mut self, new_string: String) -> NitfResult<()> { 
        self.string = new_string; 
        Ok(()) 
    }
    
    pub fn set_bytes(&mut self, new_bytes: Vec<u8>) -> NitfResult<()> { 
        self.bytes = new_bytes; 
        Ok(()) 
    }
    
    // Reading/Writing
    
    /// Read the specified number of bytes and parse the value of a given field
    pub fn read<T: Sized + Into<u64>>(
        &mut self,
        reader: &mut File,
        n_bytes: T,
        field_name: &str,
    ) -> NitfResult<()> {
        self.length = n_bytes.into();
        self.bytes = vec![0; self.length as usize];

        // Crash if something goes wrong with the cursor
        self.offset = reader
            .stream_position()
            .or(Err(NitfError::Fatal(field_name.to_string())))?;

        // Crash if there is an error reading the bytes
        reader
            .read_exact(&mut self.bytes)
            .or(Err(NitfError::Fatal(field_name.to_string())))?;

        // Try to read the bytes to a string
        match String::from_utf8(self.bytes.to_vec()) {
            // If it's ok, trim and try to parse to enum/native representation
            Ok(str) => {
                self.string = str.to_string();

                // Warn and assign a default value if error parsing
                self.val = self.string.parse().unwrap_or_else(|_| {
                    warn!("Non-fatal error parsing {}", field_name);
                    V::default()
                });
            }

            Err(_) => {
                self.string = String::from("Error parsing string");
                warn!("Failed to parse {field_name} from bytes: {:?}", self.bytes);
            }
        }
        trace!("{:?}", self.val);
        Ok(())
    }
    
    pub fn write(
        &mut self,
        writer: &mut File,
        field_name: &str
    ) -> NitfResult<()> {
        Ok(())
    }
}

impl<V: FromStr + Debug> Display for NitfField<V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.string)
    }
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
impl Security {
    pub fn read(&mut self, reader: &mut File) -> NitfResult<()> {
        self.clas.read(reader, 1u8, "CLAS")?;
        self.clsy.read(reader, 2u8, "CLSY")?;
        self.code.read(reader, 11u8, "CODE")?;
        self.ctlh.read(reader, 2u8, "CTLH")?;
        self.rel.read(reader, 20u8, "REL")?;
        self.dctp.read(reader, 2u8, "DCTP")?;
        self.dcdt.read(reader, 8u8, "DCDT")?;
        self.dcxm.read(reader, 4u8, "DCXM")?;
        self.dg.read(reader, 1u8, "DG")?;
        self.dgdt.read(reader, 8u8, "DGDT")?;
        self.cltx.read(reader, 43u8, "CLTX")?;
        self.catp.read(reader, 1u8, "CATP")?;
        self.caut.read(reader, 40u8, "CAUT")?;
        self.crsn.read(reader, 1u8, "CRSN")?;
        self.srdt.read(reader, 8u8, "SRDT")?;
        self.ctln.read(reader, 15u8, "CTLN")?;
        Ok(())
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
impl FromStr for Classification {
    type Err = NitfError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "U" => Ok(Self::U),
            "T" => Ok(Self::T),
            "S" => Ok(Self::S),
            "C" => Ok(Self::C),
            "R" => Ok(Self::R),
            _ => Err(NitfError::EnumError("Classification")),
        }
    }
}
impl Into<String> for Classification {
    fn into(self) -> String {
        match self {
            Self::U => "U".to_string(),
            Self::T => "T".to_string(),
            Self::S => "S".to_string(),
            Self::C => "C".to_string(),
            Self::R => "R".to_string(),
        }
    }
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
impl FromStr for DeclassificationType {
    type Err = NitfError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "  " => Ok(Self::DEFAULT),
            "DD" => Ok(Self::DD),
            "DE" => Ok(Self::DE),
            "GD" => Ok(Self::GD),
            "GE" => Ok(Self::GE),
            " O" => Ok(Self::O),
            " X" => Ok(Self::X),
            _ => Err(NitfError::EnumError("DeclassificationType")),
        }
    }
}
impl Into<String> for DeclassificationType {
    fn into(self) -> String {
        match self {
            Self::DEFAULT => "  ".to_string(),
            Self::DD => "DD".to_string(),
            Self::DE => "DE".to_string(),
            Self::GD => "GD".to_string(),
            Self::GE => "GE".to_string(),
            Self::O => " O".to_string(),
            Self::X => " X".to_string(),
        }
    }
}

///  Declassification exemption
#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub enum DeclassificationExemption {
    #[default]
    /// Default value, four spaces
    DEFAULT,
    /// Valid values
    DExX1,
    DExX2,
    DExX3,
    DExX4,
    DExX5,
    DExX6,
    DExX7,
    DExX8,
    DEx25X1,
    DEx25X2,
    DEx25X3,
    DEx25X4,
    DEx25X5,
    DEx25X6,
    DEx25X7,
    DEx25X8,
    DEx25X9,
    DExDN10,
    DExDNI,
}
impl FromStr for DeclassificationExemption {
    type Err = NitfError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "    " => Ok(Self::DEFAULT),
            "  X1" => Ok(Self::DExX1),   // DOD 5200.01-V1, 4-201b(1)
            "  X2" => Ok(Self::DExX2),   // DOD 5200.01-V1, 4-201b(2)
            "  X3" => Ok(Self::DExX3),   // DOD 5200.01-V1, 4-201b(3)
            "  X4" => Ok(Self::DExX4),   // DOD 5200.01-V1, 4-201b(4)
            "  X5" => Ok(Self::DExX5),   // DOD 5200.01-V1, 4-201b(5)
            "  X6" => Ok(Self::DExX6),   // DOD 5200.01-V1, 4-201b(6)
            "  X7" => Ok(Self::DExX7),   // DOD 5200.01-V1, 4-201b(7)
            "  X8" => Ok(Self::DExX8),   // DOD 5200.01-V1, 4-201b(8)
            "25X1" => Ok(Self::DEx25X1),   // DOD 5200.01-V1, 4-301b(1)
            "25X2" => Ok(Self::DEx25X2),   // DOD 5200.01-V1, 4-301b(2)
            "25X3" => Ok(Self::DEx25X3),   // DOD 5200.01-V1, 4-301b(3)
            "25X4" => Ok(Self::DEx25X4),   // DOD 5200.01-V1, 4-301b(4)
            "25X5" => Ok(Self::DEx25X5),   // DOD 5200.01-V1, 4-301b(5)
            "25X6" => Ok(Self::DEx25X6),   // DOD 5200.01-V1, 4-301b(6)
            "25X7" => Ok(Self::DEx25X7),   // DOD 5200.01-V1, 4-301b(7)
            "25X8" => Ok(Self::DEx25X8),   // DOD 5200.01-V1, 4-301b(8)
            "25X9" => Ok(Self::DEx25X9),   // DOD 5200.01-V1, 4-301b(9)
            "DN10" => Ok(Self::DExDN10),
            " DNI" => Ok(Self::DExDNI),
            _ => {Err(NitfError::EnumError("DeclassificationExemption"))}
        }
    }
}
impl Into<String> for DeclassificationExemption {
    fn into(self) -> String {
        match self {
            Self::DEFAULT => "    ".to_string(),
            Self::DExX1 => "  X1".to_string(),   // DOD 5200.01-V1, 4-201b(1)
            Self::DExX2 => "  X2".to_string(),   // DOD 5200.01-V1, 4-201b(2)
            Self::DExX3 => "  X3".to_string(),   // DOD 5200.01-V1, 4-201b(3)
            Self::DExX4 => "  X4".to_string(),   // DOD 5200.01-V1, 4-201b(4)
            Self::DExX5 => "  X5".to_string(),   // DOD 5200.01-V1, 4-201b(5)
            Self::DExX6 => "  X6".to_string(),   // DOD 5200.01-V1, 4-201b(6)
            Self::DExX7 => "  X7".to_string(),   // DOD 5200.01-V1, 4-201b(7)
            Self::DExX8 => "  X8".to_string(),   // DOD 5200.01-V1, 4-201b(8)
            Self::DEx25X1 => "25X1".to_string(), // DOD 5200.01-V1, 4-301b(1)
            Self::DEx25X2 => "25X2".to_string(), // DOD 5200.01-V1, 4-301b(2)
            Self::DEx25X3 => "25X3".to_string(), // DOD 5200.01-V1, 4-301b(3)
            Self::DEx25X4 => "25X4".to_string(), // DOD 5200.01-V1, 4-301b(4)
            Self::DEx25X5 => "25X5".to_string(), // DOD 5200.01-V1, 4-301b(5)
            Self::DEx25X6 => "25X6".to_string(), // DOD 5200.01-V1, 4-301b(6)
            Self::DEx25X7 => "25X7".to_string(), // DOD 5200.01-V1, 4-301b(7)
            Self::DEx25X8 => "25X8".to_string(), // DOD 5200.01-V1, 4-301b(8)
            Self::DEx25X9 => "25X9".to_string(), // DOD 5200.01-V1, 4-301b(9)
            Self::DExDN10 => "DN10".to_string(),
            Self::DExDNI => " DNI".to_string(),
        }
    }
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
impl FromStr for Downgrade {
    type Err = NitfError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            " " => Ok(Self::DEFAULT),
            "S" => Ok(Self::S),
            "C" => Ok(Self::C),
            "R" => Ok(Self::R),
            _ => Err(NitfError::EnumError("Downgrade")),
        }
    }
}
impl Into<String> for Downgrade {
    fn into(self) -> String {
        match self {
            Self::DEFAULT => " ".to_string(),
            Self::S => "S".to_string(),
            Self::C => "C".to_string(),
            Self::R => "R".to_string(),
        }
    }
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
impl FromStr for ClassificationAuthorityType {
    type Err = NitfError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "" => Ok(Self::DEFAULT),
            "O" => Ok(Self::O),
            "D" => Ok(Self::D),
            "M" => Ok(Self::M),
            _ => Err(NitfError::EnumError("ClassificationAuthorityType")),
        }
    }
}
impl Into<String> for ClassificationAuthorityType {
    fn into(self) -> String {
        match self {
            Self::DEFAULT => " ".to_string(),
            Self::O => "O".to_string(),
            Self::D => "D".to_string(),
            Self::M => "M".to_string(),
        }
    }
}

/// Reason for classification
#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub enum ClassificationReason {
    #[default]
    /// Default value, one space
    DEFAULT,
    /// Valid values
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}
impl FromStr for ClassificationReason {
    type Err = NitfError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            " " => Ok(Self::DEFAULT),
            "A" => Ok(Self::A),
            "B" => Ok(Self::B),
            "C" => Ok(Self::C),
            "D" => Ok(Self::D),
            "E" => Ok(Self::E),
            "F" => Ok(Self::F),
            "G" => Ok(Self::G),
            "H" => Ok(Self::H),
            _ => Err(NitfError::EnumError("ClassificationReason")),
        }
    }
}
impl Into<String> for ClassificationReason {
    fn into(self) -> String {
        match self {
            Self::DEFAULT => " ".to_string(),
            Self::A => "A".to_string(),
            Self::B => "B".to_string(),
            Self::C => "C".to_string(),
            Self::D => "D".to_string(),
            Self::E => "E".to_string(),
            Self::F => "F".to_string(),
            Self::G => "G".to_string(),
            Self::H => "H".to_string(),
        }
    }
}

/// Extended sub-header TRE data type
#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct ExtendedSubheader {
    /// User defined tagged record entries (TREs)
    pub tre: Vec<u8>,
    /// Length of subheader
    pub size: usize,
}
impl ExtendedSubheader {
    pub fn read(&mut self, reader: &mut File, n_bytes: usize, name: &str) -> NitfResult<()> {
        self.size = n_bytes;
        self.tre = vec![0; n_bytes];
        reader
            .read_exact(self.tre.as_mut_slice())
            .or(Err(NitfError::Fatal(name.to_string())))?;
        Ok(())
    }
}
impl Display for ExtendedSubheader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Can't copy vector directly, convert to slice, then clone into new vector
        let out_str = String::from_utf8(self.tre.to_vec()).or(Err(std::fmt::Error))?;
        write!(f, "[{out_str}]")
    }
}
