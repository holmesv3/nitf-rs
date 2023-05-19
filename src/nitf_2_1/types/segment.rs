use std::io::{Read, Seek};
use std::ops::RangeBounds;
use std::ops::Bound;
use std::fs::File;
use std::fmt::Display;

use std::string::FromUtf8Error;

use memmap2::Mmap;

/// Nitf segment header interface definition
/// 
/// Provide implementation for `read()`, `from_reader` defined automatically.
pub trait NitfSegmentHeader where Self: Sized + Default {
    
    /// Read the segment info from stream
    /// 
    /// # Parameters
    /// 
    /// reader: Stream from which to read header information
    #[allow(unused)]
    fn read(&mut self, reader: &mut (impl Read + Seek)) {todo!("Didn't implement read() method")}
    
    fn from_reader(reader: &mut (impl Read + Seek)) -> Self {
        let mut hdr = Self::default();
        hdr.read(reader);
        return hdr
    }
}

/// Nitf segment data interface definition
pub trait NitfSegmentData where Self: Sized {
    fn read(&mut self, reader: &mut (impl Read + Seek));
}

impl NitfSegmentData for Vec<u8> {
    #[allow(unused)]
    fn read(&mut self, reader: &mut (impl Read + Seek)) {
        todo!()
    }
}
impl NitfSegmentData for Mmap {
    #[allow(unused)]
    fn read(&mut self, reader: &mut (impl Read + Seek)) {
        todo!()
    }
}

/// Segment structure definition
/// 
///     // Header metadata fields defined in module
///     pub meta: T
///     // Segment data
///     data: Vec<u8>
///     // Byte offset of header start
///     pub header_offset: u64
///     // Byte size of header
///     pub header_size: usize
///     // Data byte offset
///     pub data_offset: u64
///     // Data size in bytes
///     pub data_size: usize
#[derive(Default, Debug)]
pub struct Segment<T, U> {
    /// Header fields defined in module
    pub meta: T,
    /// Segment data, must define function interface for access
    data: U,
    /// Byte offset of header start
    pub header_offset: u64,
    /// Byte size of header
    pub header_size: usize,
    /// Data byte offset
    pub data_offset: u64,
    /// Data size in bytes
    pub data_size: usize,
} 
impl<T, U> Display for Segment<T, U> 
where 
    T: NitfSegmentHeader + Display,
    U: NitfSegmentData
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.meta)
    }
} 
impl<T> Segment<T, Vec<u8>> 
where 
    T: NitfSegmentHeader + Default,
{
    pub fn from_reader(reader: &mut (impl Read + Seek), header_size: usize, data_size: usize,) -> Result<Self, FromUtf8Error> {
        let mut seg = Self::default();
        seg.header_size = header_size;
        seg.data_size = data_size;
        seg.header_offset = reader.stream_position().unwrap();
        seg.data_offset = seg.header_offset + (header_size as u64);

        seg.meta.read(reader);
        // seg.data = 
        Ok(seg)
    }
    pub fn read(&mut self, reader: &mut (impl Read + Seek), header_size: usize, data_size: usize) {
        self.header_size = header_size;
        self.data_size = data_size;
        self.header_offset = reader.stream_position().unwrap();
        self.data_offset = self.header_offset + (header_size as u64);
        self.meta.read(reader);
        if header_size == 0 {
            self.header_size = (reader.stream_position().unwrap() - self.header_offset) as usize; 
        }
    }
}
impl<T> Segment<T, Mmap> 
where 
    T: NitfSegmentHeader + Default,
{  
    pub fn from_file(reader: &mut File, header_size: usize, data_size: usize,) -> Result<Self, FromUtf8Error> {
        let header_offset = reader.stream_position().unwrap();
        let data_offset = header_offset + (header_size as u64);
        let seg = Self { 
            meta: T::from_reader(reader), 
            data: Self::make_mmap(reader), 
            header_offset, 
            header_size, 
            data_offset, 
            data_size, 
        };
        return Ok(seg)
    }
    pub fn read(&mut self, reader: &mut (impl Read + Seek), header_size: usize, data_size: usize) {
        self.header_size = header_size;
        self.data_size = data_size;
        self.header_offset = reader.stream_position().unwrap();
        self.data_offset = self.header_offset + (header_size as u64);
        self.meta.read(reader);
        if header_size == 0 {
            self.header_size = (reader.stream_position().unwrap() - self.header_offset) as usize; 
            self.data_offset = reader.stream_position().unwrap(); 
        }
    }
    fn make_mmap(file: &File) -> Mmap {
        unsafe {Mmap::map(file).unwrap()}
    }
    pub fn read_data_bytes(&self, index: impl RangeBounds<usize>) -> &[u8] {
        let data_start = self.data_offset as usize;
        let data_end = self.data_size + data_start;

        // Get the bounds from input range, parse and apply to mapped memory
        let idx_start: usize = match index.start_bound() {
            Bound::Included(x) => *x + data_start,
            Bound::Excluded(x) => *x + data_start,
            Bound::Unbounded => data_start,
        };
        let idx_end = match index.end_bound() {
            Bound::Included(x) => *x + data_start-1,
            Bound::Excluded(x) => *x + data_start,
            Bound::Unbounded => data_end,
        };

        return &self.data[idx_start..idx_end]
    }
}
