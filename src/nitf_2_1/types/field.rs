use std::io::{Read, Seek};
use std::fmt::Display;

/// Inidividual element type
/// 
///     // Vector of bytes
///     pub val: Vec<u8>, 
///     // Byte offset in file
///     pub offset: u64,
///     // String representation of field
///     pub string: String,
///     // Length of byte vector
///     length: usize,
#[derive(Default, Clone, Hash, Debug)]
pub struct NitfField{
    /// Vector of bytes
    pub bytes: Vec<u8>, 
    /// Byte offset in file
    pub offset: u64,
    /// String representation of field
    pub string: String,
    /// Length of byte vector
    length: usize,
}
impl NitfField{
    pub fn read(&mut self, reader: &mut (impl Read + Seek), n_bytes: usize) {
        self.length = n_bytes;
        for _ in 0..n_bytes {
            self.bytes.push(0u8)
        }
        self.offset = reader.stream_position().unwrap();    
        reader.read(&mut self.bytes).unwrap();
        let result = String::from_utf8(self.bytes.to_vec());
        match result {
            Ok(str) => self.string = str,
            Err(err) => {
                self.string = String::from("Error parsing string");
                println!("{}", err)
            }
        }
    }
}
impl Display for NitfField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{}", &self.string)
    }
}


/// Element vector type
/// 
///     // Vector of fields
///     pub val: Vec<NitfField>,
#[derive(Default, Clone, Hash, Debug)]
pub struct NitfFieldVec {
    /// Vector of fields
    pub val: Vec<NitfField>,
}
impl NitfFieldVec{
    pub fn read_vec(
        &mut self, 
        reader: &mut (impl Read + Seek), 
        n_field: &NitfField,
        n_bytes: usize) {
        let n_elem_str = String::from_utf8(n_field.bytes.to_vec()).unwrap();
        let n_elem: usize = match n_elem_str.parse() {
            Ok(uint) => uint,
            Err(e) => panic!("{}: {}", e, n_field),
        };  
        for _ in 0..n_elem {
            let mut elem = NitfField::default();
            elem.read(reader, n_bytes);
            self.val.push(elem);
        }
    }
}
impl Display for NitfFieldVec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out_str = String::default();
        for seg in self.val.iter() {
            out_str += format!("{}, ", seg).as_ref()
        }
        write!(f, "{}", out_str)
    }
}
