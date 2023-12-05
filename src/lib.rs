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
//! let nitf_path = Path::new("../example.nitf");
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
//! let nitf_path = Path::new("../example.nitf");
//! let nitf = nitf_rs::read_nitf(&nitf_path).unwrap();
//! let im_seg = &nitf.image_segments[0];
//! let u8_slice = &im_seg.data[..];
//! ```
//! Most metadata elements are stored in a [NitfField] structure. This structure
//! stores the `bytes` which encode the value, a `string` representation, and a
//! `val` which holds on to native value of the field (i.e., the bytes parsed into a
//! u8, u16, String, enum, etc.)
//! ```no_run
//! // Read in a nitf and extract the...
//! use std::path::Path;
//! let nitf_path = Path::new("../example.nitf");
//! let nitf = nitf_rs::read_nitf(&nitf_path).unwrap();
//! // .. File title
//! let file_title = nitf.nitf_header.meta.ftitle.val;
//! // .. Number of image segments
//! let n_img_segments = nitf.nitf_header.meta.numi.val;
//! // .. and number of rows in the first image segment data
//! let n_rows = nitf.image_segments[0].meta.nrows.val;
//! ```
//!
//! If there is user-defined tagged-record-extension (TRE) data within a segment,
//! it is stored in an [ExtendedSubheader] for the user to parse accordingly.
use log::debug;
use std::fmt::Display;
use std::fs::File;
use std::path::Path;
use thiserror::Error;

pub type NitfResult<T> = Result<T, NitfError>;

#[derive(Error, Debug)]
pub enum NitfError {
    // Crate specific errors
    #[error("File does not appear to be a NITF. Expected file header \"NITF\", found \"{0}\"")]
    FileType(String),
    #[error("error parsing {0} enum")]
    EnumError(&'static str),
    #[error("Fatal error reading {0}")]
    Fatal(String),

    // Wrappers for built in errors
    #[error(transparent)]
    IOError(#[from] std::io::Error),
}

pub mod headers;
pub mod segments;
pub mod types;

// Convenience type-defs
use segments::NitfSegment;
type ImageSegment = NitfSegment<headers::ImageHeader>;
type GraphicSegment = NitfSegment<headers::GraphicHeader>;
type TextSegment = NitfSegment<headers::TextHeader>;
type DataExtensionSegment = NitfSegment<headers::DataExtensionHeader>;
type ReservedExtensionSegment = NitfSegment<headers::ReservedExtensionHeader>;

use segments::FileHeader;
#[allow(unused_imports)]
use types::{ExtendedSubheader, NitfField};

/// Top level NITF interface
#[derive(Default, Debug, Eq, PartialEq)]
pub struct Nitf {
    /// Nitf file header.
    pub nitf_header: FileHeader,

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
    let mut file = File::open(path)?;
    Nitf::from_file(&mut file)
}

impl Nitf {
    pub fn from_file(file: &mut File) -> NitfResult<Self> {
        let mut nitf = Self::default();
        debug!("Reading NITF file header");
        nitf.nitf_header.read(file)?;

        let mut n_seg = nitf.nitf_header.meta.numi.val as usize;
        for i_seg in 0..n_seg {
            let seg_info = &nitf.nitf_header.meta.imheaders[i_seg];
            let header_size = seg_info.subheader_size.val;
            let data_size = seg_info.item_size.val;
            let seg = ImageSegment::initialize(file, header_size, data_size)?;
            nitf.image_segments.push(seg);
        }

        n_seg = nitf.nitf_header.meta.nums.val as usize;
        for i_seg in 0..n_seg {
            let seg_info = &nitf.nitf_header.meta.graphheaders[i_seg];
            let header_size = seg_info.subheader_size.val;
            let data_size: u64 = seg_info.item_size.val;
            let seg = GraphicSegment::initialize(file, header_size, data_size)?;
            nitf.graphic_segments.push(seg);
        }

        n_seg = nitf.nitf_header.meta.numt.val as usize;
        for i_seg in 0..n_seg {
            let seg_info = &nitf.nitf_header.meta.textheaders[i_seg];
            let header_size = seg_info.subheader_size.val;
            let data_size: u64 = seg_info.item_size.val;
            let seg = TextSegment::initialize(file, header_size, data_size)?;
            nitf.text_segments.push(seg);
        }

        n_seg = nitf.nitf_header.meta.numdes.val as usize;
        for i_seg in 0..n_seg {
            let seg_info = &nitf.nitf_header.meta.dextheaders[i_seg];
            let header_size = seg_info.subheader_size.val;
            let data_size: u64 = seg_info.item_size.val;
            let seg = DataExtensionSegment::initialize(file, header_size, data_size)?;
            nitf.data_extension_segments.push(seg);
        }

        n_seg = nitf.nitf_header.meta.numres.val as usize;
        for i_seg in 0..n_seg {
            let seg_info = &nitf.nitf_header.meta.resheaders[i_seg];
            let header_size = seg_info.subheader_size.val;
            let data_size = seg_info.item_size.val;
            let seg = ReservedExtensionSegment::initialize(file, header_size, data_size)?;
            nitf.reserved_extension_segments.push(seg);
        }
        Ok(nitf)
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
