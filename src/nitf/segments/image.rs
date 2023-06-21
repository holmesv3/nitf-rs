//! Image  segment definition and associated functions
use ndarray::{prelude::*, ArcArray1};
use memmap2::{Mmap, MmapOptions};
use num_complex::{Complex32, Complex64};
use std::fmt::Display;
use std::io::SeekFrom::Start;
use std::{fs::File, io::Seek, ops::Deref};

use crate::nitf::segments::headers::{
    image_hdr::{ImageHeader, Mode}, 
    NitfSegmentHeader};

use crate::nitf::segments::headers::image_hdr::PixelValueType as PVT;
use crate::nitf::error::NitfError;

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

#[derive(Clone, Debug)]
pub enum ImageDType {
    F32(f32),
    F64(f64),
    C32(Complex32),
    C64(Complex64),
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
            memmap_opts
                .offset(data_offset)
                .len(data_size as usize)
                .map(reader.deref())
                .unwrap()
        };
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

    pub fn read_band_data(&self) -> Result<ArcArray<ImageDType, Ix3>, NitfError> {
        match self.meta.imode.val {
            Mode::B => self.read_b_mode(),
            Mode::S => self.read_s_mode(),
            Mode::R => self.read_r_mode(),
            Mode::P => self.read_p_mode()
        }
    }
    
    fn read_b_mode(&self) -> Result<ArcArray<ImageDType, Ix3>, NitfError> { todo!() }
    fn read_s_mode(&self) -> Result<ArcArray<ImageDType, Ix3>, NitfError> { todo!() }
    fn read_r_mode(&self) -> Result<ArcArray<ImageDType, Ix3>, NitfError> { todo!() }
    
    /// Read data with bands interleaved by pixel.
    /// 
    /// Index ordering is band -> pixel -> block column -> block row
    fn read_p_mode(&self) -> Result<ArcArray<ImageDType, Ix3>, NitfError> {

        todo!("haven't finished this");

        // Out array dimensions
        let n_band = self.meta.nbands.val;
        let n_row = self.meta.nrows.val;
        let n_col = self.meta.ncols.val;

        let bit_per_px = self.meta.abpp.val;
        if bit_per_px % 8 != 0 {
            return Err(NitfError::DataBitError { 
                size: bit_per_px as usize })
        }
        let n_block_per_row = self.meta.nbpr.val;
        let n_block_per_col = self.meta.nbpc.val;
        if n_block_per_col != 1 || n_block_per_row != 1 {
            return Err(NitfError::NotImplementedError(
                String::from("reading bitwise image data from blocks")));
        }
        let _n_pix_per_block_h = 
            if self.meta.nppbh.val == 0 {n_col}
            else {self.meta.nppbh.val as u32};
        let _n_pix_per_block_v = 
            if self.meta.nppbv.val == 0 {n_row}
            else {self.meta.nppbv.val as u32};

        let total_elems = n_row * n_col * n_band as u32;
        // TODO: Maybe need to use the NBPP for something
        let arr_elem = match self.get_arr_elem(&bit_per_px) {
            Ok(elm) => elm,
            Err(e) => return Err(e),
        };

        // Try to just map and reshape the whole thing 
        let mut flat = ArcArray1::from_elem(total_elems as usize, arr_elem);

        
        let mut reshaped = flat.reshape((
            n_row as usize, 
            n_col as usize, 
            n_band as usize
        ));
        reshaped.swap_axes(2, 0);
        reshaped.swap_axes(1, 2);
        
        Ok(reshaped)
    }

    /// This is a rough one... 
    fn get_arr_elem(&self, bit_per_px: &u8) -> Result<ImageDType, NitfError> {
        match self.meta.pvtype.val {
            PVT::R => {
                match bit_per_px {
                    32 => Ok(ImageDType::F32(f32::default())),
                    64 => Ok(ImageDType::F64(f64::default())),
                    _  => Err(NitfError::NotImplementedError(
                        String::from("non exact bit-length data")))
                }
            },
            PVT::C => {
                match bit_per_px {
                    32 => Ok(ImageDType::C32(Complex32::default())),
                    64 => Ok(ImageDType::C64(Complex64::default())),
                    _  => Err(NitfError::NotImplementedError(
                        String::from("non exact bit-length data"))),
                }
            },
            PVT::SI => Err(NitfError::NotImplementedError(
                String::from("pixel value type {PVT::SI}"))),
            PVT::INT => Err(NitfError::NotImplementedError(
                String::from("pixel value type {PVT::INT}"))),
            PVT::B => Err(NitfError::NotImplementedError(
                String::from("pixel value type {PVT::B}"))),
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
    ///     use nitf_rs::nitf::read_nitf;
    ///
    ///     let nitf_path = Path::new("../example.nitf");
    ///     let nitf = read_nitf(nitf_path);
    ///     let data = nitf.image_segments[0].data_to_array();
    pub fn data_to_array(&self) -> Array2<ImageDType> {
        // TODO: implement other data types, modes
        self.get_c32_array()
    }

    fn get_c32_array(&self) -> Array2<ImageDType> {
        let n_row = self.meta.nrows.val as usize;
        let n_col = self.meta.ncols.val as usize;
        let mut arr = Array2::from_elem((n_row, n_col), ImageDType::C32(Complex32::default()));

        let mut real = [0u8; 4];
        let mut imag = [0u8; 4];

        let data_chunks = &mut self.data.chunks(4); // grab 4 bytes at a time

        for row in arr.rows_mut() {
            for elm in row {
                real.copy_from_slice(data_chunks.next().unwrap());
                imag.copy_from_slice(data_chunks.next().unwrap());
                let item = Complex32 {
                    re: f32::from_be_bytes(real),
                    im: f32::from_be_bytes(imag),
                };
                *elm = ImageDType::C32(item);
            }
        }
        arr
    }
}

impl Display for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.meta)
    }
}
