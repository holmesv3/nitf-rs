//! Interface for NITF version 2.1

pub mod segments;
pub mod types;

use quick_xml::{de::from_str, DeError};
use std::fmt::Display;
use std::fs::File;

use crate::sicd::Sicd;
use segments::{DataExtension, FileHeader, Graphic, Image, ReservedExtension, Text};

/// Top level NITF interface
///
/// Collection of [Segment] and [DataSegment] objects
#[derive(Default, Debug)]
pub struct Nitf {
    /// Nitf file header. See [NitfHeader] for `meta` fields
    pub nitf_header: FileHeader,
    /// Vector of image segments. See [ImageHeader] for `meta` fields
    pub image_segments: Vec<Image>,
    /// Vector of graphics segments. See [GraphicHeader] for `meta` fields
    pub graphic_segments: Vec<Graphic>,
    /// Vector of text segments. See [TextHeader] for `meta` fields
    pub text_segments: Vec<Text>,
    /// Vector of data extension segments. See [DataExtensionHeader] for `meta` fields
    pub data_extension_segments: Vec<DataExtension>,
    /// Vector of reserved extension segments. See [ReservedExtensionHeader] for `meta` fields
    pub reserved_extension_segments: Vec<ReservedExtension>,
}

impl Nitf {
    pub fn from_file(reader: &mut File) -> Self {
        let mut nitf = Self::default();
        nitf.nitf_header.read(reader);

        let mut n_seg: usize = nitf.nitf_header.meta.NUMI.string.parse().unwrap();
        for i_seg in 0..n_seg {
            let seg_info = &nitf.nitf_header.meta.IMHEADERS[i_seg];
            let header_size = seg_info.subheader_size.string.parse().unwrap();
            let data_size = seg_info.item_size.string.parse().unwrap();
            let seg = Image::initialize(reader, header_size, data_size);
            nitf.image_segments.push(seg);
        }

        n_seg = nitf.nitf_header.meta.NUMS.string.parse().unwrap();
        for i_seg in 0..n_seg {
            let seg_info = &nitf.nitf_header.meta.GRAPHHEADERS[i_seg];
            let header_size = seg_info.subheader_size.string.parse().unwrap();
            let data_size: u64 = seg_info.item_size.string.parse().unwrap();
            let seg = Graphic::initialize(reader, header_size, data_size);
            nitf.graphic_segments.push(seg);
        }

        n_seg = nitf.nitf_header.meta.NUMT.string.parse().unwrap();
        for i_seg in 0..n_seg {
            let seg_info = &nitf.nitf_header.meta.TEXTHEADERS[i_seg];
            let header_size = seg_info.subheader_size.string.parse().unwrap();
            let data_size: u64 = seg_info.item_size.string.parse().unwrap();
            let seg = Text::initialize(reader, header_size, data_size);
            nitf.text_segments.push(seg);
        }

        n_seg = nitf.nitf_header.meta.NUMDES.string.parse().unwrap();
        for i_seg in 0..n_seg {
            let seg_info = &nitf.nitf_header.meta.DEXTHEADERS[i_seg];
            let header_size = seg_info.subheader_size.string.parse().unwrap();
            let data_size: u64 = seg_info.item_size.string.parse().unwrap();
            let seg = DataExtension::initialize(reader, header_size, data_size);
            nitf.data_extension_segments.push(seg);
        }

        n_seg = nitf.nitf_header.meta.NUMRES.string.parse().unwrap();
        for i_seg in 0..n_seg {
            let seg_info = &nitf.nitf_header.meta.RESHEADERS[i_seg];
            let header_size = seg_info.subheader_size.string.parse().unwrap();
            let data_size = seg_info.item_size.string.parse().unwrap();
            let seg = ReservedExtension::initialize(reader, header_size, data_size);
            nitf.reserved_extension_segments.push(seg);
        }
        return nitf;
    }

    pub fn parse_sicd_meta(&self) -> Result<Sicd, DeError> {
        let xml_str = String::from_utf8(self.data_extension_segments[0].data[..].to_vec()).unwrap();
        from_str(&xml_str)
    }
}

impl Display for Nitf {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut out_str = String::default();
        out_str += format!("{}", self.nitf_header).as_ref();
        for segment in &self.image_segments {
            out_str += format!("{}", segment).as_ref();
        }
        for segment in &self.graphic_segments {
            out_str += format!("{}", segment).as_ref();
        }
        for segment in &self.text_segments {
            out_str += format!("{}", segment).as_ref();
        }
        for segment in &self.data_extension_segments {
            out_str += format!("{}", segment).as_ref();
        }
        for segment in &self.reserved_extension_segments {
            out_str += format!("{}", segment).as_ref();
        }
        write!(f, "{}", out_str)
    }
}
