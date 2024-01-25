//! Minimal Crate for reading and manipulating `NITF` files

//! Interface for NITF version 2.1
//!
//! Constructing a [Nitf] object parses the header and subheader information.
//! Each segment in contains a `meta` field which stores the respective
//! fields defined in the file standard. The primary function for constructing a
//! [Nitf] is [read_nitf()]
//! ```no_run
//! // Read a nitf file and dump metadata to stdout
//! use std::path::Path;
//! let nitf_path = Path::new("example.nitf");
//! let nitf = nitf_rs::read_nitf(&nitf_path).unwrap();
//! println!("{nitf:?}");
//! ```
//!
//! The main feature of the [FileHeader] is its `meta` field (see (NitfHeader)
//! [headers::NitfHeader]).
//! All other segments use the generic [NitfSegment] to provide header fields and
//! a memory-map of the segment data.
//! ```no_run
//! // Get the bytes from the first image segment
//! use std::path::Path;
//! let nitf_path = Path::new("example.nitf");
//! let nitf = nitf_rs::read_nitf(&nitf_path).unwrap();
//! let im_seg = &nitf.image_segments[0];
//! let u8_slice = &im_seg.data[..];
//! ```
//! Most metadata elements are stored in a [NitfField](types::NitfField) structure.
//! This structure hold onto the `val` which holds on to native value of the field
//! (i.e., the bytes parsed into a u8, u16, String, enum, etc.)
//! ```no_run
//! // Read in a nitf and extract the...
//! use std::path::Path;
//! let nitf_path = Path::new("example.nitf");
//! let nitf = nitf_rs::read_nitf(&nitf_path).unwrap();
//! // .. File title
//! let file_title = nitf.nitf_header.ftitle.val;
//! // .. Number of image segments
//! let n_img_segments = nitf.nitf_header.numi.val;
//! // .. and number of rows in the first image segment data
//! let n_rows = nitf.image_segments[0].meta.nrows.val;
//! ```
//!
//! If there is user-defined tagged-record-extension (TRE) data within a segment,
//! it is stored in an [ExtendedSubheader] for the user to parse accordingly.
//!
pub mod headers;
pub mod types;

use log::{debug, trace};
use std::fmt::Display;
use std::fs::File;
use std::path::Path;
use thiserror::Error;

use headers::file_hdr::Segment::*;
use headers::*;
use types::NitfSegment;

pub type NitfResult<T> = Result<T, NitfError>;

#[derive(Error, Debug)]
pub enum NitfError {
    // Crate specific errors
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
    pub file: Option<File>,
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

/// Construct a [Nitf] object from a file `path`.
///
/// # Example
/// ```no_run
/// use std::path::Path;
/// let nitf_path = Path::new("../example.nitf");
/// let nitf = nitf_rs::read_nitf(nitf_path).unwrap();
/// ```
pub fn read_nitf(path: &Path) -> NitfResult<Nitf> {
    // Crash if failure to open file
    let file = File::open(path)?;
    Nitf::from_file(file)
}

impl Nitf {
    pub fn from_file(file: File) -> NitfResult<Self> {
        let mut nitf = Self::default();
        nitf.file = Some(file);
        let mut reader = nitf.file.as_mut().unwrap();
        debug!("Reading NITF file header");
        nitf.nitf_header.read(&mut reader)?;

        let mut n_seg = nitf.nitf_header.numi.val as usize;
        for i_seg in 0..n_seg {
            let seg_info = &nitf.nitf_header.imheaders[i_seg];
            let data_size = seg_info.item_size.val;
            let seg = ImageSegment::from_reader(&mut reader, data_size)?;
            nitf.image_segments.push(seg);
        }

        n_seg = nitf.nitf_header.nums.val as usize;
        for i_seg in 0..n_seg {
            let seg_info = &nitf.nitf_header.graphheaders[i_seg];
            let data_size: u64 = seg_info.item_size.val;
            let seg = GraphicSegment::from_reader(&mut reader, data_size)?;
            nitf.graphic_segments.push(seg);
        }

        n_seg = nitf.nitf_header.numt.val as usize;
        for i_seg in 0..n_seg {
            let seg_info = &nitf.nitf_header.textheaders[i_seg];
            let data_size: u64 = seg_info.item_size.val;
            let seg = TextSegment::from_reader(&mut reader, data_size)?;
            nitf.text_segments.push(seg);
        }

        n_seg = nitf.nitf_header.numdes.val as usize;
        for i_seg in 0..n_seg {
            let seg_info = &nitf.nitf_header.dextheaders[i_seg];
            let data_size: u64 = seg_info.item_size.val;
            let seg = DataExtensionSegment::from_reader(&mut reader, data_size)?;
            nitf.data_extension_segments.push(seg);
        }

        n_seg = nitf.nitf_header.numres.val as usize;
        for i_seg in 0..n_seg {
            let seg_info = &nitf.nitf_header.resheaders[i_seg];
            let data_size = seg_info.item_size.val;
            let seg = ReservedExtensionSegment::from_reader(&mut reader, data_size)?;
            nitf.reserved_extension_segments.push(seg);
        }
        Ok(nitf)
    }

    /// Write the header information for all segments to a file
    pub fn write_headers(&mut self) -> NitfResult<usize> {
        if self.file.is_none() {
            Err(NitfError::Fatal("Must set 'file' before writing".to_string()))?;
        }
        debug!("Writing NITF file header");
        let mut bytes_written = 0;

        let length = self.length();
        let mut writer = self.file.as_mut().ok_or(NitfError::FileFatal)?;
        writer.set_len(length as u64)?;
        bytes_written += self.nitf_header.write(&mut writer)?;
        for seg in self.image_segments.iter_mut() {
            bytes_written += seg.write_header(&mut writer)?;
        }
        for seg in self.graphic_segments.iter_mut() {
            bytes_written += seg.write_header(&mut writer)?;
        }
        for seg in self.text_segments.iter_mut() {
            bytes_written += seg.write_header(&mut writer)?;
        }
        for seg in self.data_extension_segments.iter_mut() {
            bytes_written += seg.write_header(&mut writer)?;
        }
        for seg in self.reserved_extension_segments.iter_mut() {
            bytes_written += seg.write_header(&mut writer)?;
        }
        Ok(bytes_written)
    }

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

    /// After changing something with the size or number of segments, need to update internal info
    fn update_offsets(&mut self) {
        let mut offset = self.nitf_header.length();
        let mut trace_string = "Updated offsets: \n".to_string();
        trace_string += &format!("\tFile Header length: {offset}\n");
        for (i_seg, seg) in self.image_segments.iter_mut().enumerate() {
            seg.header_offset = offset as u64;
            offset += seg.header.length();
            trace_string += &format!("\tImage segment {i_seg} header offset: {}\n", seg.header_offset);
            trace_string += &format!("\tImage segment {i_seg} header length: {}\n", seg.header.length());
            seg.data_offset = offset as u64;
            offset += seg.data_size as usize;
            trace_string += &format!("\tImage segment {i_seg} data offset: {}\n", seg.data_offset);
            trace_string += &format!("\tImage segment {i_seg} data length: {}\n", seg.data_size);
        }
        for (i_seg, seg) in self.graphic_segments.iter_mut().enumerate() {
            seg.header_offset = offset as u64;
            offset += seg.header.length();
            trace_string += &format!("\tGraphic segment {i_seg} header offset: {}\n", seg.header_offset);
            trace_string += &format!("\tGraphic segment {i_seg} header length: {}\n", seg.header.length());
            seg.data_offset = offset as u64;
            offset += seg.data_size as usize;
            trace_string += &format!("\tGraphic segment {i_seg} data offset: {}\n", seg.data_offset);
            trace_string += &format!("\tGraphic segment {i_seg} data length: {}\n", seg.data_size);
        }
        for (i_seg, seg) in self.text_segments.iter_mut().enumerate() {
            seg.header_offset = offset as u64;
            offset += seg.header.length();
            trace_string += &format!("\tText segment {i_seg} header offset: {}\n", seg.header_offset);
            trace_string += &format!("\tText segment {i_seg} header length: {}\n", seg.header.length());
            seg.data_offset = offset as u64;
            offset += seg.data_size as usize;
            trace_string += &format!("\tText segment {i_seg} data offset: {}\n", seg.data_offset);
            trace_string += &format!("\tText segment {i_seg} data length: {}\n", seg.data_size);
        }
        for (i_seg, seg) in self.data_extension_segments.iter_mut().enumerate() {
            seg.header_offset = offset as u64;
            offset += seg.header.length();
            trace_string += &format!("\tData Extension segment {i_seg} header offset: {}\n", seg.header_offset);
            trace_string += &format!("\tData Extension segment {i_seg} header length: {}\n", seg.header.length());
            seg.data_offset = offset as u64;
            offset += seg.data_size as usize;
            trace_string += &format!("\tData Extension segment {i_seg} data offset: {}\n", seg.data_offset);
            trace_string += &format!("\tData Extension segment {i_seg} data length: {}\n", seg.data_size);
        }
        for (i_seg, seg) in self.reserved_extension_segments.iter_mut().enumerate() {
            seg.header_offset = offset as u64;
            offset += seg.header.length();
            trace_string += &format!("\tReserved Extension segment {i_seg} header offset: {}\n", seg.header_offset);
            trace_string += &format!("\tReserved Extension segment {i_seg} header length: {}\n", seg.header.length());
            seg.data_offset = offset as u64;
            offset += seg.data_size as usize;
            trace_string += &format!("\tReserved Extension segment {i_seg} data offset: {}\n", seg.data_offset);
            trace_string += &format!("\tReserved Extension segment {i_seg} data length: {}\n", seg.data_size);
        }
        trace!("{trace_string}");
    }
    // I could  wrap these in an enum to match off the type and have one function,
    // but I think the more explicit funcntions makes for a cleaner interface..
    /// Add a [ImageSegment] to the object
    pub fn add_im(&mut self, seg: ImageSegment) {
        let segment_type = Image;
        let subheader_size = seg.header.length() as u32;
        let item_size = seg.data_size;
        self.nitf_header
            .add_subheader(segment_type, subheader_size, item_size);
        self.image_segments.push(seg);
        // update offsets
        self.update_offsets();
        debug!("Added Image Segment to NITF");
    }
    /// Add a [GraphicSegment] to the object
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
    /// Add a [TextSegment] to the object
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
    /// Add a [DataExtensionSegment] to the object
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
    /// Add a [ReservedExtensionSegment] to the object
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
