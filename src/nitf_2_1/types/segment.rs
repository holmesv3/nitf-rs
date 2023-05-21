//! Segment without memmaped data

use std::fmt::Display;
use std::io::{Read, Seek};

use std::string::FromUtf8Error;

use memmap2::Mmap;

/// Nitf segment header interface definition
///
/// Provide implementation for `read()`, `from_reader` defined automatically.
pub trait NitfSegmentHeader
where
    Self: Sized + Default,
{
    /// Read the segment info from stream
    ///
    /// # Parameters
    ///
    /// reader: Stream from which to read header information
    #[allow(unused)]
    fn read(&mut self, reader: &mut (impl Read + Seek)) {
        todo!("Didn't implement read() method")
    }

    fn from_reader(reader: &mut (impl Read + Seek)) -> Self {
        let mut hdr = Self::default();
        hdr.read(reader);
        return hdr;
    }
}

/// Segment structure definition
///
///     // Header metadata fields defined in module
///     pub meta: T
///     // Byte offset of header start
///     pub header_offset: u64
///     // Byte size of header
///     pub header_size: usize
#[derive(Default, Debug)]
pub struct Segment<T> {
    /// Header fields defined in module
    pub meta: T,
    /// Byte offset of header start
    pub header_offset: usize,
    /// Byte size of header
    pub header_size: usize,
}

impl<T> Display for Segment<T>
where
    T: NitfSegmentHeader + Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.meta)
    }
}

impl<T: NitfSegmentHeader + Default> Segment<T> {
    pub fn from_reader(
        reader: &mut (impl Read + Seek),
        header_size: usize,  // TODO refactor to not usize
        data_size: usize,
    ) -> Result<Self, FromUtf8Error> {
        let mut seg = Self::default();
        seg.read_header(reader, header_size, data_size);
        Ok(seg)
    }
    pub fn read_header(&mut self, reader: &mut (impl Read + Seek), header_size: usize, data_size: usize) {
        self.header_size = header_size;
        self.header_offset = reader.stream_position().unwrap() as usize;
        self.meta.read(reader);
        if header_size == 0 {
            self.header_size = reader.stream_position().unwrap() as usize - self.header_offset;
        }
    }
}
