use std::io::Read;
use std::fmt::Display;

pub mod header;
pub mod image;
pub mod graphic;
pub mod text;

/// Inidividual element type
#[derive(Default, Clone, Hash, Debug)]
pub struct  NitfElement(pub Vec<u8>, pub usize);
impl NitfElement{
    pub fn read(&mut self, reader: &mut impl Read, n_bytes: usize) -> usize {
        for _ in 0..n_bytes {
            self.0.push(0u8)
        }
        reader.read(&mut self.0).unwrap()
    }
}
impl Display for NitfElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from_utf8(self.0.to_vec()).unwrap())
    }
}

/// Subheader type
#[derive(Default, Clone, Hash, Debug)]
pub struct  NitfSubHeader ( pub Vec<NitfSubHeaderElem> , usize);
impl NitfSubHeader{
    pub fn read(
        &mut self, 
        reader: &mut impl Read, 
        n_subheader: &NitfElement,
        sh_size: usize,
        item_size: usize
    ) -> usize {
        let n_seg: usize = String::from_utf8(n_subheader.0.to_vec())
            .unwrap()
            .parse()
            .unwrap();
        let mut offset: usize = 0;
        for _ in 0..n_seg {
            let mut seg = NitfSubHeaderElem::default();
            offset = seg.read(reader, sh_size, item_size);
            self.0.push(seg);
        }
        offset
   
    }
}
impl Display for NitfSubHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out_str = String::default();
        for seg in self.0.iter() {
            out_str += format!("\t{}", seg).as_ref()
        }
        write!(f, "{}", out_str)
    }
}
/// Subheader element type
#[derive(Default, Clone, Hash, Debug)]
pub struct NitfSubHeaderElem {
    pub subheader_size: Vec<u8>,
    pub item_size: Vec<u8>,
}
impl NitfSubHeaderElem {
    pub fn read(
        &mut self, 
        reader: &mut impl Read, 
        sh_size: usize,
        item_size: usize
    ) -> usize {
        for _ in 0..sh_size {
            self.subheader_size.push(0u8)
        }
        for _ in 0..item_size {
            self.item_size.push(0u8)
        }

        reader.read(&mut self.subheader_size).unwrap();
        reader.read(&mut self.item_size).unwrap()
    }
}
impl Display for NitfSubHeaderElem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str1 = String::from_utf8(self.subheader_size.to_vec()).unwrap();
        let str2 = String::from_utf8(self.item_size.to_vec()).unwrap();
        write!(f, "[{}, {}]", str1, str2)
    }
}
