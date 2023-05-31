pub mod data_extension;
pub mod graphic;
pub mod headers;
pub mod image;
pub mod nitf_file;
pub mod reserved_extension;
pub mod text;

pub use data_extension::DataExtension;
pub use graphic::Graphic;
pub use image::Image;
pub use nitf_file::FileHeader;
pub use reserved_extension::ReservedExtension;
pub use text::Text;

use memmap2::Mmap;
use std::fmt::Display;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom::Current};
use std::ops::Deref;
use std::string::FromUtf8Error;

use headers::NitfSegmentHeader;
/// Segment structure definition
///
///
///     pub meta: T  // header metadata fields defined in module
///     pub header_offset: u64  // byte offset of header start
///     pub header_size: usize  // byte size of header
///
#[derive(Default, Debug)]
pub struct Segment<T> {
    /// Header fields defined in module
    pub meta: T,
    /// Byte offset of header start
    pub header_offset: u64,
    /// Byte size of header
    pub header_size: u64,
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
        header_size: u64, // TODO refactor to not usize
    ) -> Result<Self, FromUtf8Error> {
        let mut seg = Self::default();
        seg.read_header(reader, header_size);
        Ok(seg)
    }
    pub fn read_header(&mut self, reader: &mut (impl Read + Seek), header_size: u64) {
        self.header_size = header_size;
        self.header_offset = reader.stream_position().unwrap();
        self.meta.read(reader);
        if header_size == 0 {
            self.header_size = reader.stream_position().unwrap() - self.header_offset;
        }
    }
}

/// Segment with memmaped data
#[derive(Debug)]
pub struct DataSegment<T> {
    /// Header fields defined in module
    pub meta: T,
    /// Segment data, must define function interface for access
    pub data: Mmap,
    /// Byte offset of header start
    pub header_offset: u64,
    /// Byte size of header
    pub header_size: u64,
    /// Data byte offset
    pub data_offset: u64,
    /// Data size in bytes
    pub data_size: u64,
}

/// Nitf segment data interface definition
pub trait NitfSegmentData
where
    Self: Sized,
{
    fn read_segment_data(&mut self, reader: &mut (impl Read + Seek), n_bytes: u64);
}

impl NitfSegmentData for Mmap {
    #[allow(unused)]
    fn read_segment_data(&mut self, reader: &mut (impl Read + Seek), n_bytes: u64) {
        todo!()
    }
}
impl<T: NitfSegmentHeader + Display> Display for DataSegment<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.meta)
    }
}
impl<T: NitfSegmentHeader + Default> DataSegment<T> {
    pub fn from_file(
        reader: &mut File,
        header_size: u64,
        data_size: u64,
    ) -> Result<Self, FromUtf8Error> {
        let header_offset = reader.stream_position().unwrap();
        let data_offset = header_offset + header_size;
        let seg = Self {
            meta: T::from_reader(reader),
            data: unsafe { Mmap::map(reader.deref()).unwrap() },
            header_offset,
            header_size,
            data_offset,
            data_size,
        };
        // Move pointer to end of data
        reader.seek(Current(data_size as i64)).unwrap();
        return Ok(seg);
    }
    pub fn read(&mut self, reader: &mut (impl Read + Seek), header_size: u64, data_size: u64) {
        self.header_size = header_size;
        self.data_size = data_size;
        self.header_offset = reader.stream_position().unwrap();
        self.data_offset = self.header_offset + header_size;
        self.meta.read(reader);
        if header_size == 0 {
            self.header_size = reader.stream_position().unwrap() - self.header_offset;
            self.data_offset = reader.stream_position().unwrap();
        }
    }
}
