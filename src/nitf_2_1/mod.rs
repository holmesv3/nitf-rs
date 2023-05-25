//! Interface for NITF version 2.1

pub mod types;
pub mod subheaders;
pub mod nitf_header;

use std::{fmt::Display, str::FromStr};
use std::fs::File;
use ndarray::Array2;
use num_complex::Complex;

use subheaders::*;
use data_extension::DataExtensionHeader;
use graphic::GraphicHeader;
use image::{ImageHeader, PixelValueType};
use nitf_header::NitfHeader;
use reserved_extension::ReservedExtensionHeader;
use text::TextHeader;
use types::{Segment, DataSegment};

/// Top level NITF interface
///
/// Collection of [Segment] and [DataSegment] objects
#[derive(Default, Debug)]
pub struct Nitf {
    /// Nitf file header. See [NitfHeader] for `meta` fields
    pub nitf_header: Segment<NitfHeader>,
    /// Vector of image segments. See [ImageHeader] for `meta` fields
    pub image_segments: Vec<DataSegment<ImageHeader>>,
    /// Vector of graphics segments. See [GraphicHeader] for `meta` fields
    pub graphic_segments: Vec<Segment<GraphicHeader>>,
    /// Vector of text segments. See [TextHeader] for `meta` fields
    pub text_segments: Vec<Segment<TextHeader>>,
    /// Vector of data extension segments. See [DataExtensionHeader] for `meta` fields
    pub data_extension_segments: Vec<Segment<DataExtensionHeader>>,
    /// Vector of reserved extension segments. See [ReservedExtensionHeader] for `meta` fields
    pub reserved_extension_segments: Vec<DataSegment<ReservedExtensionHeader>>,
}


impl Nitf {
    pub fn from_file(reader: &mut File) -> Self {
        let mut nitf = Self::default();
        nitf.nitf_header.read_header(reader, 0);

        let mut n_seg: usize = nitf.nitf_header.meta.NUMI.string.parse().unwrap();
        for i_seg in 0..n_seg {
            let seg_info = &nitf.nitf_header.meta.IMHEADERS.val[i_seg];
            let header_size = seg_info.subheader_size.string.parse().unwrap();
            let data_size = seg_info.item_size.string.parse().unwrap();
            let seg: DataSegment<ImageHeader> =
                DataSegment::from_file(reader, header_size, data_size).unwrap();
            nitf.image_segments.push(seg);
        }

        n_seg = nitf.nitf_header.meta.NUMS.string.parse().unwrap();
        for i_seg in 0..n_seg {
            let seg_info = &nitf.nitf_header.meta.GRAPHHEADERS.val[i_seg];
            let header_size = seg_info.subheader_size.string.parse().unwrap();
            let _data_size: u64 = seg_info.item_size.string.parse().unwrap();
            let seg: Segment<GraphicHeader> =
                Segment::from_reader(reader, header_size).unwrap();
            nitf.graphic_segments.push(seg);
        }

        n_seg = nitf.nitf_header.meta.NUMT.string.parse().unwrap();
        for i_seg in 0..n_seg {
            let seg_info = &nitf.nitf_header.meta.TEXTHEADERS.val[i_seg];
            let header_size = seg_info.subheader_size.string.parse().unwrap();
            let _data_size: u64 = seg_info.item_size.string.parse().unwrap();
            let seg: Segment<TextHeader> =
                Segment::from_reader(reader, header_size).unwrap();
            nitf.text_segments.push(seg);
        }

        n_seg = nitf.nitf_header.meta.NUMDES.string.parse().unwrap();
        for i_seg in 0..n_seg {
            let seg_info = &nitf.nitf_header.meta.DEXTHEADERS.val[i_seg];
            let header_size = seg_info.subheader_size.string.parse().unwrap();
            let _data_size: u64 = seg_info.item_size.string.parse().unwrap();
            let seg: Segment<DataExtensionHeader> =
                Segment::from_reader(reader, header_size).unwrap();
            nitf.data_extension_segments.push(seg);
        }

        n_seg = nitf.nitf_header.meta.NUMRES.string.parse().unwrap();
        for i_seg in 0..n_seg {
            let seg_info = &nitf.nitf_header.meta.RESHEADERS.val[i_seg];
            let header_size = seg_info.subheader_size.string.parse().unwrap();
            let data_size = seg_info.item_size.string.parse().unwrap();
            let seg: DataSegment<ReservedExtensionHeader> =
                DataSegment::from_file(reader, header_size, data_size).unwrap();
            nitf.reserved_extension_segments.push(seg);
        }
        return nitf;
    }

    pub fn image_data_to_array(&self, image_idx: usize) -> Array2<_> {
        todo!();
        let image_segment = &self.image_segments[image_idx];
        let n_row: usize = image_segment.meta.NROWS.string.parse().unwrap();
        let n_col: usize = image_segment.meta.NCOLS.string.parse().unwrap();

        let n_byte_per_pixel: u8 = image_segment.meta.NBPP.string.parse().unwrap();

        let dtype = PixelValueType::from_str(&image_segment.meta.PVTYPE.string).unwrap();

        let mut arr = Array2::from_elem((n_row, n_col), Complex32::default());

        let mut real: [u8; 4] = [0u8; 4];
        let mut imag: [u8; 4] = [0u8; 4];

        let data = image_segment.read_data_bytes(..); // read all the data
        let data_chunks = &mut data.chunks(4); // grab 4 bytes at a time

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

impl Display for Nitf {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut out_str = String::default();
        out_str += format!("[{}]", self.nitf_header).as_ref();
        for segment in &self.image_segments {
            out_str += format!("[{}]", segment).as_ref();
        }
        for segment in &self.graphic_segments {
            out_str += format!("[{}]", segment).as_ref();
        }
        for segment in &self.text_segments {
            out_str += format!("[{}]", segment).as_ref();
        }
        for segment in &self.data_extension_segments {
            out_str += format!("[{}]", segment).as_ref();
        }
        for segment in &self.reserved_extension_segments {
            out_str += format!("[{}]", segment).as_ref();
        }
        write!(f, "{}", out_str)
    }
}
