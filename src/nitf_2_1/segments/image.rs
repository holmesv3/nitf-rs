//! Image  segment definition and associated functions
use std::fmt::Display;
use std::{io::Seek, fs::File, ops::Deref};
use std::io::SeekFrom::Start;
use memmap2::{Mmap, MmapOptions};
use num_complex::Complex32;
use ndarray::Array2;

use super::headers::{image_hdr::ImageHeader, NitfSegmentHeader};

#[derive(Debug)]
pub struct Image {
        /// Header fields defined in module
        pub meta: ImageHeader,
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

impl Image {
    pub fn initialize(reader: &mut File, header_size: u32, data_size: u64) -> Self {
        let header_offset = reader.stream_position().unwrap();
        let header_size = header_size;
        let data_size = data_size;
        let data_offset = header_offset + header_size as u64;
        let meta = ImageHeader::from_reader(reader);
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

    // TODO: Implement reading a slice of the data into an array, as opposed to the whole thing.
    // TODO: Support various data types based on header information
    /// Read image data from image segment into an array
    ///
    /// **Only supports `Complex32` data at the moment with `P` IMODE**
    /// 
    /// # Example
    ///
    ///     use std::path::Path;
    ///     use nitf_rs::{read_nitf, data_to_array};
    ///
    ///     let nitf_path = Path::new(<path-to-file>);
    ///     let nitf = read_nitf(nitf_path);
    ///     let data = data_to_array(&nitf.image_segments[0]);
    pub fn data_to_array(&self) -> Array2<Complex32> {
        let n_row: usize = self.meta.NROWS.string.parse().unwrap();
        let n_col: usize = self.meta.NCOLS.string.parse().unwrap();
    
        let mut arr = Array2::from_elem((n_row, n_col), Complex32::default());
    
        let mut real: [u8; 4] = [0u8; 4];
        let mut imag: [u8; 4] = [0u8; 4];
    

        let data_chunks = &mut self.data.chunks(4); // grab 4 bytes at a time
    
        for row in arr.rows_mut() {
            for elm in row {
                real.copy_from_slice(data_chunks.next().unwrap());
                imag.copy_from_slice(data_chunks.next().unwrap());
    
                *elm = Complex32 {
                    re: f32::from_be_bytes(real),
                    im: f32::from_be_bytes(imag),
                };
            }
        }
        return arr;
    }
}
    
impl Display for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.meta)
    }
}