//! Minimal Crate for reading and manipulating `NITF` files

//! Interface for NITF version 2.1
//!
//! Constructing a [Nitf] object parses the file header and segment metadata.
//! Each segment in contains a `header` field which stores the respective
//! metadata defined in the file standard. The primary function for constructing a
//! [Nitf] is [Nitf::from_reader()]
//! ```no_run
//! // Read a nitf file and dump metadata to stdout
//! let mut nitf_file = std::fs::File::open("example.nitf").unwrap();
//! let nitf = nitf_rs::Nitf::from_reader(&mut nitf_file).unwrap();
//! println!("{nitf}");
//! ```
//!
//!
//! Aside from the `nitf_header`, all other segments use a generic [NitfSegment]
//! to provide metadata and access to the segment data.
//! ```no_run
//! // Get the bytes from the first image segment
//! let mut nitf_file = std::fs::File::open("example.nitf").unwrap();
//! let nitf = nitf_rs::Nitf::from_reader(&mut nitf_file).unwrap();
//! let im_seg = &nitf.image_segments[0];
//! let im_seg_hdr = &im_seg.header;
//! let im_seg_data = &im_seg.get_data_map(&mut nitf_file).unwrap();
//! ```
//!
//! Most metadata elements are stored in a [NitfField](types::NitfField) structure.
//! This structure has a `val` which holds on to native value of the field
//! (i.e., the bytes parsed into a u8, u16, String, enum, etc.), as well as the
//! length (in bytes) and name of the field.
//! ```no_run
//! let mut nitf_file = std::fs::File::open("example.nitf").unwrap();
//! let nitf = nitf_rs::Nitf::from_reader(&mut nitf_file).unwrap();
//! let file_title = nitf.nitf_header.ftitle.val;
//! let n_img_segments = nitf.nitf_header.numi.val;
//! let n_rows = nitf.image_segments[0].header.nrows.val;
//! ```
//!
//! If there is user-defined tagged-record-extension (TRE) data within a segment,
//! it is stored in an [ExtendedSubheader] for the user to parse accordingly.
pub mod headers;
pub mod types;

use log::{debug, trace};
use std::fmt::Display;
use std::io::{Read, Write, Seek};
use thiserror::Error;

use headers::file_hdr::Segment::*;
use headers::*;
use types::NitfSegment;

pub type NitfResult<T> = Result<T, NitfError>;

// Crate specific errors
#[derive(Error, Debug)]
pub enum NitfError {
    #[error("error parsing {0}")]
    ParseError(String),
    #[error("{0}")]
    Fatal(String),
    #[error("Fatal error reading {0}")]
    ReadFatal(String),
    #[error("Cannot write without first providing a file")]
    FileFatal,
    #[error("value of {0} does not match")]
    Value(String),
    #[error("Couldn't update header values")]
    Update(),
    // Wrappers for built in errors
    #[error(transparent)]
    IOError(#[from] std::io::Error),
}

// Convenience type-defs
pub type ImageSegment = NitfSegment<ImageHeader>;
pub type GraphicSegment = NitfSegment<GraphicHeader>;
pub type TextSegment = NitfSegment<TextHeader>;
pub type DataExtensionSegment = NitfSegment<DataExtensionHeader>;
pub type ReservedExtensionSegment = NitfSegment<ReservedExtensionHeader>;

#[allow(unused_imports)]
use types::ExtendedSubheader;

/// Top level NITF interface
#[derive(Default, Debug)]
pub struct Nitf {
    /// Nitf file header.
    pub nitf_header: NitfHeader,

    /// Vector of image segments.
    pub image_segments: Vec<ImageSegment>,

    /// Vector of graphics segments.
    pub graphic_segments: Vec<GraphicSegment>,

    /// Vector of text segments.
    pub text_segments: Vec<TextSegment>,

    /// Vector of data extension segments.
    pub data_extension_segments: Vec<DataExtensionSegment>,

    /// Vector of reserved extension segments.
    pub reserved_extension_segments: Vec<ReservedExtensionSegment>,
}

impl Nitf {
    pub fn from_reader(reader: &mut (impl Read + Seek)) -> NitfResult<Self> {
        let mut nitf = Nitf::default();

        debug!("Reading NITF file header");
        nitf.nitf_header.read(reader)?;

        let mut n_seg = nitf.nitf_header.numi.val as usize;
        for i_seg in 0..n_seg {
            let seg_info = &nitf.nitf_header.imheaders[i_seg];
            let data_size = seg_info.item_size.val;
            let seg = ImageSegment::read(reader, data_size)?;
            nitf.image_segments.push(seg);
        }

        n_seg = nitf.nitf_header.nums.val as usize;
        for i_seg in 0..n_seg {
            let seg_info = &nitf.nitf_header.graphheaders[i_seg];
            let data_size: u64 = seg_info.item_size.val;
            let seg = GraphicSegment::read(reader, data_size)?;
            nitf.graphic_segments.push(seg);
        }

        n_seg = nitf.nitf_header.numt.val as usize;
        for i_seg in 0..n_seg {
            let seg_info = &nitf.nitf_header.textheaders[i_seg];
            let data_size: u64 = seg_info.item_size.val;
            let seg = TextSegment::read(reader, data_size)?;
            nitf.text_segments.push(seg);
        }

        n_seg = nitf.nitf_header.numdes.val as usize;
        for i_seg in 0..n_seg {
            let seg_info = &nitf.nitf_header.dextheaders[i_seg];
            let data_size: u64 = seg_info.item_size.val;
            let seg = DataExtensionSegment::read(reader, data_size)?;
            nitf.data_extension_segments.push(seg);
        }

        n_seg = nitf.nitf_header.numres.val as usize;
        for i_seg in 0..n_seg {
            let seg_info = &nitf.nitf_header.resheaders[i_seg];
            let data_size = seg_info.item_size.val;
            let seg = ReservedExtensionSegment::read(reader, data_size)?;
            nitf.reserved_extension_segments.push(seg);
        }
        Ok(nitf)
    }

    /// Write the header information for all segments to a file
    pub fn write_headers(&mut self, writer: &mut (impl Write + Seek)) -> NitfResult<usize> {
        let mut bytes_written = 0;

        let file_length = self.length() as u64;
        bytes_written += self.nitf_header.write_header(writer, file_length)?;
        for seg in self.image_segments.iter_mut() {
            bytes_written += seg.write_header(writer)?;
        }
        for seg in self.graphic_segments.iter_mut() {
            bytes_written += seg.write_header(writer)?;
        }
        for seg in self.text_segments.iter_mut() {
            bytes_written += seg.write_header(writer)?;
        }
        for seg in self.data_extension_segments.iter_mut() {
            bytes_written += seg.write_header(writer)?;
        }
        for seg in self.reserved_extension_segments.iter_mut() {
            bytes_written += seg.write_header(writer)?;
        }
        Ok(bytes_written)
    }
    /// Get the length of the [Nitf] file in bytes
    pub fn length(&self) -> usize {
        let mut length = 0;
        length += self.nitf_header.length();
        length += self
            .image_segments
            .iter()
            .map(|seg| seg.length())
            .sum::<usize>();
        length += self
            .graphic_segments
            .iter()
            .map(|seg| seg.length())
            .sum::<usize>();
        length += self
            .text_segments
            .iter()
            .map(|seg| seg.length())
            .sum::<usize>();
        length += self
            .data_extension_segments
            .iter()
            .map(|seg| seg.length())
            .sum::<usize>();
        length += self
            .reserved_extension_segments
            .iter()
            .map(|seg| seg.length())
            .sum::<usize>();
        length
    }

    // After changing something with the size or number of segments, need to update internal info

    /// Update internal state of file header and offsets.
    ///
    /// I don't think this needs to be part of the public interface..
    fn update_offsets(&mut self) {
        let mut offset = self.nitf_header.length();
        let mut trace_string = "Updated offsets: \n".to_string();
        trace_string += &format!("\tFile Header length: {offset}\n");
        for (i_seg, seg) in self.image_segments.iter_mut().enumerate() {
            seg.header_offset = offset as u64;
            offset += seg.header.length();
            trace_string += &format!(
                "\tImage segment {i_seg} header offset: {}\n",
                seg.header_offset
            );
            trace_string += &format!(
                "\tImage segment {i_seg} header length: {}\n",
                seg.header.length()
            );
            seg.data_offset = offset as u64;
            offset += seg.data_size as usize;
            trace_string += &format!("\tImage segment {i_seg} data offset: {}\n", seg.data_offset);
            trace_string += &format!("\tImage segment {i_seg} data length: {}\n", seg.data_size);
        }
        for (i_seg, seg) in self.graphic_segments.iter_mut().enumerate() {
            seg.header_offset = offset as u64;
            offset += seg.header.length();
            trace_string += &format!(
                "\tGraphic segment {i_seg} header offset: {}\n",
                seg.header_offset
            );
            trace_string += &format!(
                "\tGraphic segment {i_seg} header length: {}\n",
                seg.header.length()
            );
            seg.data_offset = offset as u64;
            offset += seg.data_size as usize;
            trace_string += &format!(
                "\tGraphic segment {i_seg} data offset: {}\n",
                seg.data_offset
            );
            trace_string += &format!("\tGraphic segment {i_seg} data length: {}\n", seg.data_size);
        }
        for (i_seg, seg) in self.text_segments.iter_mut().enumerate() {
            seg.header_offset = offset as u64;
            offset += seg.header.length();
            trace_string += &format!(
                "\tText segment {i_seg} header offset: {}\n",
                seg.header_offset
            );
            trace_string += &format!(
                "\tText segment {i_seg} header length: {}\n",
                seg.header.length()
            );
            seg.data_offset = offset as u64;
            offset += seg.data_size as usize;
            trace_string += &format!("\tText segment {i_seg} data offset: {}\n", seg.data_offset);
            trace_string += &format!("\tText segment {i_seg} data length: {}\n", seg.data_size);
        }
        for (i_seg, seg) in self.data_extension_segments.iter_mut().enumerate() {
            seg.header_offset = offset as u64;
            offset += seg.header.length();
            trace_string += &format!(
                "\tData Extension segment {i_seg} header offset: {}\n",
                seg.header_offset
            );
            trace_string += &format!(
                "\tData Extension segment {i_seg} header length: {}\n",
                seg.header.length()
            );
            seg.data_offset = offset as u64;
            offset += seg.data_size as usize;
            trace_string += &format!(
                "\tData Extension segment {i_seg} data offset: {}\n",
                seg.data_offset
            );
            trace_string += &format!(
                "\tData Extension segment {i_seg} data length: {}\n",
                seg.data_size
            );
        }
        for (i_seg, seg) in self.reserved_extension_segments.iter_mut().enumerate() {
            seg.header_offset = offset as u64;
            offset += seg.header.length();
            trace_string += &format!(
                "\tReserved Extension segment {i_seg} header offset: {}\n",
                seg.header_offset
            );
            trace_string += &format!(
                "\tReserved Extension segment {i_seg} header length: {}\n",
                seg.header.length()
            );
            seg.data_offset = offset as u64;
            offset += seg.data_size as usize;
            trace_string += &format!(
                "\tReserved Extension segment {i_seg} data offset: {}\n",
                seg.data_offset
            );
            trace_string += &format!(
                "\tReserved Extension segment {i_seg} data length: {}\n",
                seg.data_size
            );
        }
        trace!("{trace_string}");
    }

    // I could  wrap these in an enum to match off the type and have one function,
    // but I think the more explicit funcntions makes for a cleaner interface..

    /// Add a [ImageSegment] to the object.
    ///
    /// Takes ownership of the segment to indicate that the metadata should not
    /// be extensively modified after adding. Some fields can be changed without
    /// adverse affect, but it should be done with moderate prejudice.
    pub fn add_im(&mut self, seg: ImageSegment) {
        let segment_type = Image;
        let subheader_size = seg.header.length() as u32;
        let item_size = seg.data_size;
        self.nitf_header
            .add_subheader(segment_type, subheader_size, item_size);
        self.image_segments.push(seg);
        self.update_offsets();
        debug!("Added Image Segment to NITF");
    }
    /// Add a [GraphicSegment] to the object.
    ///
    /// Takes ownership of the segment to indicate that the metadata should not
    /// be extensively modified after adding. Some fields can be changed without
    /// adverse affect, but it should be done with moderate prejudice.
    pub fn add_sy(&mut self, seg: GraphicSegment) {
        let segment_type = Graphic;
        let subheader_size = seg.header.length() as u32;
        let item_size = seg.data_size;
        self.nitf_header
            .add_subheader(segment_type, subheader_size, item_size);
        self.graphic_segments.push(seg);
        self.update_offsets();
        debug!("Added Graphic Segment to NITF");
    }
    /// Add a [TextSegment] to the object.
    ///
    /// Takes ownership of the segment to indicate that the metadata should not
    /// be extensively modified after adding. Some fields can be changed without
    /// adverse affect, but it should be done with moderate prejudice.
    pub fn add_te(&mut self, seg: TextSegment) {
        let segment_type = Text;
        let subheader_size = seg.header.length() as u32;
        let item_size = seg.data_size;
        self.nitf_header
            .add_subheader(segment_type, subheader_size, item_size);
        self.text_segments.push(seg);
        self.update_offsets();
        debug!("Added Text Segment to NITF");
    }
    /// Add a [DataExtensionSegment] to the object.
    ///
    /// Takes ownership of the segment to indicate that the metadata should not
    /// be extensively modified after adding. Some fields can be changed without
    /// adverse affect, but it should be done with moderate prejudice.
    pub fn add_de(&mut self, seg: DataExtensionSegment) {
        let segment_type = DataExtension;
        let subheader_size = seg.header.length() as u32;
        let item_size = seg.data_size;
        self.nitf_header
            .add_subheader(segment_type, subheader_size, item_size);
        self.data_extension_segments.push(seg);
        self.update_offsets();
        debug!("Added Data Extension Segment to NITF");
    }
    /// Add a [ReservedExtensionSegment] to the object.
    ///
    /// Takes ownership of the segment to indicate that the metadata should not
    /// be extensively modified after adding. Some fields can be changed without
    /// adverse affect, but it should be done with moderate prejudice.
    pub fn add_re(&mut self, seg: ReservedExtensionSegment) {
        let segment_type = ReservedExtension;
        let subheader_size = seg.header.length() as u32;
        let item_size = seg.data_size;
        self.nitf_header
            .add_subheader(segment_type, subheader_size, item_size);
        self.reserved_extension_segments.push(seg);
        self.update_offsets();
        debug!("Added Reserved Extension Segment to NITF");
    }
}

impl Display for Nitf {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut out_str = String::default();
        out_str += format!("{}", self.nitf_header).as_ref();
        for segment in &self.image_segments {
            out_str += format!("{}, ", segment).as_ref();
        }
        for segment in &self.graphic_segments {
            out_str += format!("{}, ", segment).as_ref();
        }
        for segment in &self.text_segments {
            out_str += format!("{}, ", segment).as_ref();
        }
        for segment in &self.data_extension_segments {
            out_str += format!("{}, ", segment).as_ref();
        }
        for segment in &self.reserved_extension_segments {
            out_str += format!("{}, ", segment).as_ref();
        }
        write!(f, "{}", out_str)
    }
}
impl PartialEq for Nitf {
    fn eq(&self, other: &Self) -> bool {
        self.nitf_header == other.nitf_header
            && self.image_segments == other.image_segments
            && self.graphic_segments == other.graphic_segments
            && self.text_segments == other.text_segments
            && self.data_extension_segments == other.data_extension_segments
            && self.reserved_extension_segments == other.reserved_extension_segments
    }
}
impl Eq for Nitf {}
