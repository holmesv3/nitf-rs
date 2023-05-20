//! Functions to interface with NITF version 2.1

/// TODO
pub mod types {
    // This style makes all of the structs and traits
    // visible, without the module in the middle
    mod field;
    mod security;
    mod segment;
    mod subheader;

    pub use field::*;
    pub use security::*;
    pub use segment::*;
    pub use subheader::*;
}

pub mod data_segment;
pub mod graphic_segment;
pub mod image_segment;
pub mod nitf_header;
pub mod reserved_segment;
pub mod text_segment;

use std::fmt::Display;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom::Current};
use std::string::FromUtf8Error;

use memmap2::Mmap;

use data_segment::DataExtensionSegmentHeader;
use graphic_segment::GraphicSegmentHeader;
use image_segment::ImageSegmentHeader;
use nitf_header::NitfHeader;
use reserved_segment::ReservedExtensionSegmentHeader;
use text_segment::TextSegmentHeader;
use types::Segment;

/// Top level NITF interface
///
/// Collection of [Segment] objects
///  
#[derive(Default, Debug)]
pub struct Nitf {
    /// Nitf file header. See [NitfHeader] for `meta` fields
    pub nitf_header: Segment<NitfHeader, Vec<u8>>,
    /// Vector of image segments. See [ImageSegmentHeader] for `meta` fields
    pub image_segments: Vec<Segment<ImageSegmentHeader, Mmap>>,
    /// Vector of graphics segments. See [GraphicSegmentHeader] for `meta` fields
    pub graphics_segments: Vec<Segment<GraphicSegmentHeader, Vec<u8>>>,
    /// Vector of text segments. See [TextSegmentHeader] for `meta` fields
    pub text_segments: Vec<Segment<TextSegmentHeader, Vec<u8>>>,
    /// Vector of data extension segments. See [DataExtensionSegmentHeader] for `meta` fields
    pub data_extension_segments: Vec<Segment<DataExtensionSegmentHeader, Vec<u8>>>,
    /// Vector of reserved extension segments. See [ReservedExtensionSegmentHeader] for `meta` fields
    pub reserved_extension_segments: Vec<Segment<ReservedExtensionSegmentHeader, Vec<u8>>>,
}

impl From<&mut File> for Nitf {
    fn from(value: &mut File) -> Self {
        Self::from_file(value)
    }
}
impl Nitf {
    pub fn from_file(reader: &mut File) -> Self {
        let mut nitf = Self::default();
        nitf.nitf_header.read(reader, 0, 0);

        let mut n_seg: usize = nitf.nitf_header.meta.NUMI.string.parse().unwrap();
        for i_seg in 0..n_seg {
            let seg_info = &nitf.nitf_header.meta.IMHEADERS.val[i_seg];
            let header_size = seg_info.subheader_size.string.parse().unwrap();
            let data_size = seg_info.item_size.string.parse().unwrap();
            let seg: Segment<ImageSegmentHeader, Mmap> =
                Segment::from_file(reader, header_size, data_size).unwrap();
            nitf.image_segments.push(seg);
        }

        let mut n_seg: usize = nitf.nitf_header.meta.NUMS.string.parse().unwrap();
        for i_seg in 0..n_seg {
            let seg_info = &nitf.nitf_header.meta.GRAPHHEADERS.val[i_seg];
            let header_size = seg_info.subheader_size.string.parse().unwrap();
            let data_size = seg_info.item_size.string.parse().unwrap();
            let seg: Segment<GraphicSegmentHeader, Vec<u8>> =
                Segment::from_reader(reader, header_size, data_size).unwrap();
            nitf.graphics_segments.push(seg);
        }

        n_seg = nitf.nitf_header.meta.NUMT.string.parse().unwrap();
        for i_seg in 0..n_seg {
            let seg_info = &nitf.nitf_header.meta.TEXTHEADERS.val[i_seg];
            let header_size = seg_info.subheader_size.string.parse().unwrap();
            let data_size = seg_info.item_size.string.parse().unwrap();
            let seg: Segment<TextSegmentHeader, Vec<u8>> =
                Segment::from_reader(reader, header_size, data_size).unwrap();
            nitf.text_segments.push(seg);
        }

        n_seg = nitf.nitf_header.meta.NUMDES.string.parse().unwrap();
        for i_seg in 0..n_seg {
            let seg_info = &nitf.nitf_header.meta.DEXTHEADERS.val[i_seg];
            let header_size = seg_info.subheader_size.string.parse().unwrap();
            let data_size = seg_info.item_size.string.parse().unwrap();
            let seg: Segment<DataExtensionSegmentHeader, Vec<u8>> =
                Segment::from_reader(reader, header_size, data_size).unwrap();
            nitf.data_extension_segments.push(seg);
        }

        n_seg = nitf.nitf_header.meta.NUMDES.string.parse().unwrap();
        for i_seg in 0..n_seg {
            let seg_info = &nitf.nitf_header.meta.RESHEADERS.val[i_seg];
            let header_size = seg_info.subheader_size.string.parse().unwrap();
            let data_size = seg_info.item_size.string.parse().unwrap();
            let seg: Segment<ReservedExtensionSegmentHeader, Vec<u8>> =
                Segment::from_reader(reader, header_size, data_size).unwrap();
            nitf.reserved_extension_segments.push(seg);
        }

        return nitf;
    }
}

impl Display for Nitf {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut out_str = String::default();
        out_str += format!("[{}]", self.nitf_header).as_ref();
        for segment in &self.image_segments {
            out_str += format!("[{}]", segment).as_ref();
        }
        for segment in &self.graphics_segments {
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
