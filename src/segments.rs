//! File header and generic segment definition
use memmap2::{Mmap, MmapOptions};
use std::fmt::Display;
use std::fs::File;
use std::io::{Seek, SeekFrom::Start};
use std::ops::Deref;

use crate::headers::{NitfHeader, NitfSegmentHeader};
use crate::NitfError;

#[derive(Default, Debug, Clone, Eq, PartialEq)]
pub struct FileHeader {
    /// Header fields defined in module
    pub meta: NitfHeader,
    /// Byte size of header
    pub header_size: u64,
}
impl FileHeader {
    pub fn read(&mut self, reader: &mut File) -> Result<(), NitfError> {
        self.meta.read(reader)?;
        // Crash if cursor error
        self.header_size = reader.stream_position().or(Err(NitfError::IOError))?;
        Ok(())
    }
}
impl Display for FileHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.meta)
    }
}

#[derive(Debug)]
pub struct NitfSegment<T: NitfSegmentHeader> {
    /// Header fields defined in module
    pub meta: T,
    /// Segment data
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
impl<T: NitfSegmentHeader> NitfSegment<T> {
    pub fn initialize(
        reader: &mut File,
        header_size: u32,
        data_size: u64,
    ) -> Result<Self, NitfError> {
        // Crash if cursor error
        let header_offset = reader.stream_position().or(Err(NitfError::IOError))?;
        let header_size = header_size;
        let data_size = data_size;
        let data_offset = header_offset + header_size as u64;
        let meta = T::from_reader(reader)?;
        let mut memmap_opts = MmapOptions::new();
        let data = unsafe {
            memmap_opts
                .offset(data_offset)
                .len(data_size as usize)
                .map(reader.deref())
                .or(Err(NitfError::IOError))?
        };
        // Seek to end of data for next segment to be read
        // Crash if cursor error
        reader
            .seek(Start(data_offset + data_size))
            .or(Err(NitfError::IOError))?;
        Ok(Self {
            meta,
            data,
            header_offset,
            header_size,
            data_size,
            data_offset,
        })
    }
}
impl<T: NitfSegmentHeader + Display> Display for NitfSegment<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.meta)
    }
}
impl<T: NitfSegmentHeader + PartialEq> PartialEq for NitfSegment<T> {
    fn eq(&self, other: &Self) -> bool {
        let b0 = self.meta == other.meta;
        let b1 = self.header_offset == other.header_offset;
        let b2 = self.header_size == other.header_size;
        let b3 = self.data_offset == other.data_offset;
        let b4 = self.data_size == other.data_size;
        b0 & b1 & b2 & b3 & b4
    }
}
impl<T: NitfSegmentHeader + Eq> Eq for NitfSegment<T> {}
