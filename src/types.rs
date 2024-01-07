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
}

/// Provide default implementation of reading a field.
impl<V> NitfField<V>
where
V: FromStr + Debug + Default + Display,
<V as FromStr>::Err: Debug,
{
    // Access functios
    pub fn bytes(&self) -> &Vec<u8> {
        &self.bytes
    }
    pub fn string(&self) -> &String {
        &self.string
    }
    pub fn val(&self) -> &V {
        &self.val
    }
    pub fn length(&self) -> usize {
        self.length as usize
    }
    // Setters need to updated other fields upon change
    pub fn set_val(&mut self, new_val: V) -> NitfResult<()> { 
        self.string = new_val.to_string();
        self.bytes = self.string.clone().into_bytes();
        self.val = new_val; 
        Ok(()) 
    }
    
    pub fn set_string(&mut self, new_string: String) -> NitfResult<()> { 
        self.val = new_string.parse().unwrap();
        self.bytes = new_string.clone().into_bytes();
        self.string = new_string; 
        Ok(()) 
    }
    
    pub fn set_bytes(&mut self, new_bytes: Vec<u8>) -> NitfResult<()> { 
        self.string = String::from_utf8(new_bytes.clone()).unwrap();
        self.val = self.string.parse().unwrap();
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
        reader
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
        trace!("Read {field_name}: {:?}", self.val);
        Ok(())
    }
    
    pub fn write(
        &self,
        writer: &mut File,
        field_name: &str
    ) -> NitfResult<usize> {
        trace!("Writing {field_name}: {:?}", self.val);
        writer.write(&self.bytes).map_err(|e| NitfError::IOError(e))
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
    pub fn write(&self, writer: &mut File) -> NitfResult<usize> {
        let mut bytes_written = 0;
        bytes_written += self.clas.write(writer, "CLAS")?;
        bytes_written += self.clsy.write(writer, "CLSY")?;
        bytes_written += self.code.write(writer, "CODE")?;
        bytes_written += self.ctlh.write(writer, "CTLH")?;
        bytes_written += self.rel.write(writer, "REL")?;
        bytes_written += self.dctp.write(writer, "DCTP")?;
        bytes_written += self.dcdt.write(writer, "DCDT")?;
        bytes_written += self.dcxm.write(writer, "DCXM")?;
        bytes_written += self.dg.write(writer, "DG")?;
        bytes_written += self.dgdt.write(writer, "DGDT")?;
        bytes_written += self.cltx.write(writer, "CLTX")?;
        bytes_written += self.catp.write(writer, "CATP")?;
        bytes_written += self.caut.write(writer, "CAUT")?;
        bytes_written += self.crsn.write(writer, "CRSN")?;
        bytes_written += self.srdt.write(writer, "SRDT")?;
        bytes_written += self.ctln.write(writer, "CTLN")?;
        Ok(bytes_written)
    }
    /// Sum of a all security fields
    pub fn length(&self) -> usize {
        167
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
impl Display for Classification {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::U => write!(f, "U"),
            Self::T => write!(f, "T"),
            Self::S => write!(f, "S"),
            Self::C => write!(f, "C"),
            Self::R => write!(f, "R"),
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
impl Display for DeclassificationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DEFAULT => write!(f, "  "),
            Self::DD => write!(f, "DD"),
            Self::DE => write!(f, "DE"),
            Self::GD => write!(f, "GD"),
            Self::GE => write!(f, "GE"),
            Self::O => write!(f, " O"),
            Self::X => write!(f, " X"),
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
impl Display for DeclassificationExemption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DEFAULT => write!(f, "    "),
            Self::DExX1 => write!(f, "  X1"),   // DOD 5200.01-V1, 4-201b(1)
            Self::DExX2 => write!(f, "  X2"),   // DOD 5200.01-V1, 4-201b(2)
            Self::DExX3 => write!(f, "  X3"),   // DOD 5200.01-V1, 4-201b(3)
            Self::DExX4 => write!(f, "  X4"),   // DOD 5200.01-V1, 4-201b(4)
            Self::DExX5 => write!(f, "  X5"),   // DOD 5200.01-V1, 4-201b(5)
            Self::DExX6 => write!(f, "  X6"),   // DOD 5200.01-V1, 4-201b(6)
            Self::DExX7 => write!(f, "  X7"),   // DOD 5200.01-V1, 4-201b(7)
            Self::DExX8 => write!(f, "  X8"),   // DOD 5200.01-V1, 4-201b(8)
            Self::DEx25X1 => write!(f, "25X1"), // DOD 5200.01-V1, 4-301b(1)
            Self::DEx25X2 => write!(f, "25X2"), // DOD 5200.01-V1, 4-301b(2)
            Self::DEx25X3 => write!(f, "25X3"), // DOD 5200.01-V1, 4-301b(3)
            Self::DEx25X4 => write!(f, "25X4"), // DOD 5200.01-V1, 4-301b(4)
            Self::DEx25X5 => write!(f, "25X5"), // DOD 5200.01-V1, 4-301b(5)
            Self::DEx25X6 => write!(f, "25X6"), // DOD 5200.01-V1, 4-301b(6)
            Self::DEx25X7 => write!(f, "25X7"), // DOD 5200.01-V1, 4-301b(7)
            Self::DEx25X8 => write!(f, "25X8"), // DOD 5200.01-V1, 4-301b(8)
            Self::DEx25X9 => write!(f, "25X9"), // DOD 5200.01-V1, 4-301b(9)
            Self::DExDN10 => write!(f, "DN10"),
            Self::DExDNI => write!(f, " DNI"),
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
impl Display for Downgrade {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DEFAULT => write!(f, " "),
            Self::S => write!(f, "S"),
            Self::C => write!(f, "C"),
            Self::R => write!(f, "R"),
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
impl Display for ClassificationAuthorityType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DEFAULT => write!(f, " "),
            Self::O => write!(f, "O"),
            Self::D => write!(f, "D"),
            Self::M => write!(f, "M"),
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
impl Display for ClassificationReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DEFAULT => write!(f, " "),
            Self::A => write!(f, "A"),
            Self::B => write!(f, "B"),
            Self::C => write!(f, "C"),
            Self::D => write!(f, "D"),
            Self::E => write!(f, "E"),
            Self::F => write!(f, "F"),
            Self::G => write!(f, "G"),
            Self::H => write!(f, "H"),
        }
    }
}

/// Extended sub-header TRE data type
#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct ExtendedSubheader {
    /// User defined tagged record entries (TREs)
    tre: Vec<u8>,
    /// Length of subheader
    size: usize,
}
impl ExtendedSubheader {    
    /// Get `tre` 
    pub fn tre(&self) -> &Vec<u8> {
        &self.tre
    }
    /// Get `size` 
    pub fn size(&self) -> &usize {
        &self.size
    } 
    /// Updates the TRE byte vector and size field. 
    pub fn set_tre(&mut self, new_tre: Vec<u8>) {
        self.size = new_tre.len();
        self.tre = new_tre;
    }
    pub fn read(&mut self, reader: &mut File, n_bytes: usize, name: &str) -> NitfResult<()> {
        self.size = n_bytes;
        self.tre = vec![0; n_bytes];
        trace!("Reading: {name}");
        reader
            .read_exact(self.tre.as_mut_slice())
            .map_err(|e| NitfError::IOError(e))
    }
    pub fn write(&self, writer: &mut File, name: &str) -> NitfResult<usize> {
        trace!("Writing: {name}");
        writer.write(self.tre.as_slice()).map_err(|e| NitfError::IOError(e))
    }
    pub fn length(&self) -> usize {
        self.size
    }
}
impl Display for ExtendedSubheader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Can't copy vector directly, convert to slice, then clone into new vector
        let out_str = String::from_utf8(self.tre.to_vec()).or(Err(std::fmt::Error))?;
        write!(f, "[{out_str}]")
    }
}
