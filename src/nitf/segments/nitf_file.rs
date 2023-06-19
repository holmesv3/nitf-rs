//! Nitf file header structure

use std::fmt::Display;
use std::io::{Read, Seek};

use super::headers::{nitf_file_hdr::NitfHeader, NitfSegmentHeader};

#[derive(Default, Debug, Clone, Hash)]
pub struct FileHeader {
    /// Header fields defined in module
    pub meta: NitfHeader,
    /// Byte offset of header start
    pub header_offset: u64,
    /// Byte size of header
    pub header_size: u64,
}

impl FileHeader {
    pub fn read(&mut self, reader: &mut (impl Read + Seek)) {
        self.meta.read(reader);
        self.header_offset = 0;
        self.header_size = reader.stream_position().unwrap();
    }
}
impl Display for FileHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.meta)
    }
}
