//! Subheader element type
use std::fmt::Display;
use std::io::{Read, Seek};
use super::field::*;

/// Subheader element type
///
/// Used within the NITF header to denote the subheader segments contained in the file
#[derive(Default, Clone, Hash, Debug)]
pub struct NitfSubHeader {
    /// Bytes of header description
    pub subheader_size: NitfField<u32>,
    /// Bytes of the data
    pub item_size: NitfField<u64>,
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
            "[subheader_size: {}, item_size: {}]",
            &self.subheader_size.string, &self.item_size.string
        );
    }
}
