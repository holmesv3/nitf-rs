//! Functions to interface with NITF Header

pub mod types;
pub mod nitf_header;
pub mod image_segment;
pub mod graphic_segment;
pub mod text_segment;
pub mod data_segment;
pub mod reserved_segment;

use nitf_header::NitfHeader;
use image_segment::ImageSegment;
use graphic_segment::GraphicSegment;
use text_segment::TextSegment;
use data_segment::DataExtensionSegment;
use reserved_segment::ReservedExtensionSegment;

use types::{Segment, NitfSegmentData};

use std::io::{Read, Seek, SeekFrom};
use std::fmt::Display;
use std::string::FromUtf8Error;

#[derive(Default, Clone, Hash, Debug)]
pub struct Nitf {
    pub nitf_header: Segment<NitfHeader, Vec<u8>>,
    pub image_headers: Vec<Segment<ImageSegment, Vec<u8>>>,
    pub graphics_headers: Vec<Segment<GraphicSegment, Vec<u8>>>,
    pub text_header: Vec<Segment<TextSegment, Vec<u8>>>,
    pub data_extension_headers: Vec<Segment<DataExtensionSegment, Vec<u8>>>,
    pub reserved_extension_header: Vec<Segment<ReservedExtensionSegment, Vec<u8>>>,
}

impl Nitf {
    pub fn from_reader(reader: &mut (impl Read + Seek)) -> Result<Self, FromUtf8Error> {
        let mut nitf = Self::default();
        nitf.nitf_header.read(reader, 0, 0);
        let n_image: usize = nitf.nitf_header.meta.NUMI.string.parse().unwrap();
        for i_seg in 0..n_image {
            let seg_info = &nitf.nitf_header.meta.IMHEADERS.val[i_seg];
            let header_size = seg_info.subheader_size.string.parse().unwrap();
            let data_size = seg_info.item_size.string.parse().unwrap();
            let seg: Segment<ImageSegment, Vec<u8>> = Segment::from_reader(reader, header_size, data_size).unwrap();
            nitf.image_headers.push(seg);
        }
        Ok(nitf)
    }   
}

impl Display for Nitf {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut out_str = String::default();
        out_str += format!("[{}]", self.nitf_header).as_ref();
        for segment in &self.image_headers {
            out_str += format!("[{}]", segment).as_ref();
        }
        for segment in &self.graphics_headers {
            out_str += format!("[{}]", segment).as_ref();
        }
        for segment in &self.text_header {
            out_str += format!("[{}]", segment).as_ref();
        }
        for segment in &self.data_extension_headers {
            out_str += format!("[{}]", segment).as_ref();
        }
        for segment in &self.reserved_extension_header {
            out_str += format!("[{}]", segment).as_ref();
        }
        write!(f, "{}", out_str)
    }
}

impl NitfSegmentData for Vec<u8> {
    #[allow(unused)]
    fn read(&mut self, reader: &mut (impl Read + Seek)) {
        todo!()
    }
}

/// Read first two bytes of next segment,  rewind pointer
fn get_next_segment(reader: &mut (impl Read + Seek)) -> String {
    let mut val = vec![0u8, 0u8];
    // Read two bytes
    reader.read(&mut val).unwrap();
    // Rewind the two bytes
    reader.seek(SeekFrom::Current(-2)).unwrap();
    String::from_utf8(val).unwrap().parse().unwrap()
}