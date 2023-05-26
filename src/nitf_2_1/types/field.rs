//! Inidividual header/subheader element type
use std::fmt::{Display, Debug};
use std::io::{Read, Seek};
use std::str::FromStr;

/// Lowest level object for file parsing
#[derive(Default, Clone, Hash, Debug)]
pub struct NitfField<V: FromStr + Debug> {
    /// Byte representation
    pub bytes: Vec<u8>,
    /// Byte offset in file
    pub offset: u64,
    /// String representation of field
    pub string: String,
    /// Parsed representation of value
    pub val: V,
    /// Number of bytes used to store value in file
    pub length: u64,
}

/// Use Default implementation
impl<V> NitfField<V> 
where V: FromStr + Debug, 
     <V as FromStr>::Err : Debug {
    /// Read the specified number of bytes and parse the value of a given field
    pub fn read<T: Sized + Into<u64>>(&mut self, reader: &mut (impl Read + Seek), n_bytes: T) {
        self.length = n_bytes.into();
        for _ in 0..self.length {
            self.bytes.push(0u8)
        }
        self.offset = reader.stream_position().unwrap();
        reader.read(&mut self.bytes).unwrap();
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
        return write!(f, "{}", &self.string);
    }
}


/// General Error type for parsed value
#[derive(Debug, Clone)]
pub struct InvalidNitfValue;

impl Display for InvalidNitfValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid Value")
    }
}