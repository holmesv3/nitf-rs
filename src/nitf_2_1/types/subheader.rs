use std::fmt::Display;
use std::io::{Read, Seek};

use super::*;

/// Subheader element type
///
/// Used within the NITF header to denote the subheader segments contained in the file
///
///     /// Bytes of header description
///     pub subheader_size: Vec<u8>,
///     /// Bytes of the data
///     pub item_size: Vec<u8>,
#[derive(Default, Clone, Hash, Debug)]
pub struct NitfSubHeader {
    /// Bytes of header description
    pub subheader_size: NitfField,
    /// Bytes of the data
    pub item_size: NitfField,
}
impl NitfSubHeader {
    pub fn read(&mut self, reader: &mut (impl Read + Seek), sh_size: u64, item_size: u64) {
        self.subheader_size.read(reader, sh_size);
        self.item_size.read(reader, item_size);
    }
}
impl Display for NitfSubHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(
            f,
            "[{}, {}]",
            &self.subheader_size.string, &self.item_size.string
        );
    }
}

/// Subheader vector type
///
///     /// Vector of subheader info
///     pub val: Vec<NitfSubHeader>,
#[derive(Default, Clone, Hash, Debug)]
pub struct NitfSubHeaderVec {
    /// Vector of subheader info
    pub val: Vec<NitfSubHeader>,
}
impl NitfSubHeaderVec {
    /// Read subheaders into a vector
    /// 
    /// - `n_subheader` defines how many to read
    /// - `sh_size` defines how many bytes each subheader is
    /// - `item_size` defines the data length in bytes
    pub fn read(
        &mut self,
        reader: &mut (impl Read + Seek),
        n_subheader: &NitfField,
        sh_size: u64,
        item_size: u64,
    ) {
        let n_seg: usize = n_subheader.string.parse().unwrap();
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
