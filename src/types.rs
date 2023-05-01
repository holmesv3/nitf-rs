use std::io::{Read, Seek};
use std::fmt::Display;


/// Inidividual element type
#[derive(Default, Clone, Hash, Debug)]
pub struct NitfField{
    pub val: Vec<u8>, 
    pub offset: u64
}
impl NitfField{
    pub fn read(&mut self, reader: &mut (impl Read + Seek), n_bytes: usize) {
        for _ in 0..n_bytes {
            self.val.push(0u8)
        }
        self.offset = reader.stream_position().unwrap();    
        reader.read(&mut self.val).unwrap();
    }
}
impl Display for NitfField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// Element vector type
#[derive(Default, Clone, Hash, Debug)]
pub struct NitfFieldVec {
    pub val: Vec<NitfField>,
    pub offset: u64
}
impl NitfFieldVec{
    pub fn read(
        &mut self, 
        reader: &mut (impl Read + Seek), 
        n_elem: &NitfField,
        n_bytes: usize) {
        let n_elem: usize = String::from_utf8(n_elem.val.to_vec())
            .unwrap()
            .parse()
            .unwrap();
        self.offset = reader.stream_position().unwrap();        
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
            out_str += format!("\t{}", seg).as_ref()
        }
        write!(f, "{}", out_str)
    }
}

/// Subheader type
#[derive(Default, Clone, Hash, Debug)]
pub struct NitfSubHeaderVec { 
    pub val: Vec<NitfSubHeader>, 
    pub offset: u64
}
impl NitfSubHeaderVec{
    pub fn read(
        &mut self, 
        reader: &mut (impl Read + Seek), 
        n_subheader: &NitfField,
        sh_size: usize,
        item_size: usize) {
        let n_seg: usize = String::from_utf8(n_subheader.val.to_vec())
            .unwrap()
            .parse()
            .unwrap();
        self.offset = reader.stream_position().unwrap();        
        for _ in 0..n_seg {
            let mut seg = NitfSubHeader::default();
            seg.read(reader, sh_size, item_size);
            self.val.push(seg);
        }
    }
}
impl Display for NitfSubHeaderVec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out_str = String::default();
        for seg in self.val.iter() {
            out_str += format!("\t{}", seg).as_ref()
        }
        write!(f, "{}", out_str)
    }
}

/// Subheader element type
#[derive(Default, Clone, Hash, Debug)]
pub struct NitfSubHeader {
    pub subheader_size: Vec<u8>,
    pub item_size: Vec<u8>,
}
impl NitfSubHeader {
    pub fn read(
        &mut self, 
        reader: &mut impl Read, 
        sh_size: usize,
        item_size: usize) {
        for _ in 0..sh_size {
            self.subheader_size.push(0u8)
        }
        for _ in 0..item_size {
            self.item_size.push(0u8)
        }

        reader.read(&mut self.subheader_size).unwrap();
        reader.read(&mut self.item_size).unwrap();
    }
}
impl Display for NitfSubHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str1 = String::from_utf8(self.subheader_size.to_vec()).unwrap();
        let str2 = String::from_utf8(self.item_size.to_vec()).unwrap();
        write!(f, "[{}, {}]", str1, str2)
    }
}

pub trait Segment<T> {
    fn from_reader(reader: &mut (impl Read + Seek)) -> Result<T, std::io::Error>;
}