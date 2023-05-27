//! Graphic segment definition and associated functions
use std::fmt::Display;
use std::{io::Seek, fs::File, ops::Deref};
use std::io::SeekFrom::Start;
use memmap2::{Mmap, MmapOptions};
use super::headers::{graphic_hdr::GraphicHeader, NitfSegmentHeader};

#[derive(Debug)]
pub struct Graphic {
        /// Header fields defined in module
        pub meta: GraphicHeader,
        /// Segment data, must define function interface for access
        pub data: Mmap,
        /// Byte offset of header start
        pub header_offset: u64,
        /// Byte size of header
        pub header_size: u32,
        /// Data byte offset
        pub data_offset: u64,
        /// Data size in bytes
        pub data_size: u64,
}

impl Graphic {
    pub fn initialize(reader: &mut File, header_size: u32, data_size: u64) -> Self {
        let header_offset = reader.stream_position().unwrap();
        let header_size = header_size;
        let data_size = data_size;
        let data_offset = header_offset + header_size as u64;
        let meta = GraphicHeader::from_reader(reader);
        let mut memmap_opts = MmapOptions::new();
        let data = unsafe {
            memmap_opts.offset(data_offset)
            .len(data_size as usize)
            .map(reader.deref())
            .unwrap()};
        // Seek to end of data for next segment to be read
        reader.seek(Start(data_offset + data_size)).unwrap();
        Self { 
            meta,
            data,
            header_offset,
            header_size,
            data_size,
            data_offset,
        }
    }
}
impl Display for Graphic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.meta)
    }
}