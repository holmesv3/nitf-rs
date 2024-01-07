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
//! let file_title = nitf.nitf_header.meta.ftitle.val();
//! // .. Number of image segments
//! let n_img_segments = nitf.nitf_header.meta.numi.val();
//! // .. and number of rows in the first image segment data
//! let n_rows = nitf.image_segments[0].meta.nrows.val();
//! ```
//!
//! If there is user-defined tagged-record-extension (TRE) data within a segment,
//! it is stored in an [ExtendedSubheader] for the user to parse accordingly.
use headers::NitfSegmentHeader;
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
    #[error("value of {0} does not match")]
    Value(String),
    #[error("Couldn't update header values")]
    Update(),
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

        let mut n_seg = nitf.nitf_header.meta.numi.val().clone() as usize;
        for i_seg in 0..n_seg {
            let seg_info = &nitf.nitf_header.meta.imheaders[i_seg];
            let header_size = seg_info.subheader_size.val().clone();
            let data_size = seg_info.item_size.val().clone();
            let seg = ImageSegment::initialize(file, header_size, data_size)?;
            nitf.image_segments.push(seg);
        }

        n_seg = nitf.nitf_header.meta.nums.val().clone() as usize;
        for i_seg in 0..n_seg {
            let seg_info = &nitf.nitf_header.meta.graphheaders[i_seg];
            let header_size = seg_info.subheader_size.val().clone();
            let data_size: u64 = seg_info.item_size.val().clone();
            let seg = GraphicSegment::initialize(file, header_size, data_size)?;
            nitf.graphic_segments.push(seg);
        }

        n_seg = nitf.nitf_header.meta.numt.val().clone() as usize;
        for i_seg in 0..n_seg {
            let seg_info = &nitf.nitf_header.meta.textheaders[i_seg];
            let header_size = seg_info.subheader_size.val().clone();
            let data_size: u64 = seg_info.item_size.val().clone();
            let seg = TextSegment::initialize(file, header_size, data_size)?;
            nitf.text_segments.push(seg);
        }

        n_seg = nitf.nitf_header.meta.numdes.val().clone() as usize;
        for i_seg in 0..n_seg {
            let seg_info = &nitf.nitf_header.meta.dextheaders[i_seg];
            let header_size = seg_info.subheader_size.val().clone();
            let data_size: u64 = seg_info.item_size.val().clone();
            let seg = DataExtensionSegment::initialize(file, header_size, data_size)?;
            nitf.data_extension_segments.push(seg);
        }

        n_seg = nitf.nitf_header.meta.numres.val().clone() as usize;
        for i_seg in 0..n_seg {
            let seg_info = &nitf.nitf_header.meta.resheaders[i_seg];
            let header_size = seg_info.subheader_size.val().clone();
            let data_size = seg_info.item_size.val().clone();
            let seg = ReservedExtensionSegment::initialize(file, header_size, data_size)?;
            nitf.reserved_extension_segments.push(seg);
        }
        Ok(nitf)
    }
    
    /// Write the nitf object to a file
    pub fn write(&mut self, path: &Path) -> NitfResult<usize> {
        debug!("Writing NITF file header");
        let mut bytes_written = 0;
        // Verify that the header values match 
        match self.check_headers() {
            // Try to update, if it fails, error
            Err(_) => self.update_headers().or(Err(NitfError::Update()))?,
            _ => true,
        }; 
        
        let mut writer = File::create(path)?;
        writer.set_len(self.length() as u64)?;
        
        bytes_written += self.nitf_header.write(&mut writer)?;
        for seg in self.image_segments.iter() {
            bytes_written += seg.write(&mut writer)?;
        }
        for seg in self.graphic_segments.iter() {
            bytes_written += seg.write(&mut writer)?;
        }
        for seg in self.text_segments.iter() {
            bytes_written += seg.write(&mut writer)?;
        }
        for seg in self.data_extension_segments.iter() {
            bytes_written += seg.write(&mut writer)?;
        }
        for seg in self.reserved_extension_segments.iter() {
            bytes_written += seg.write(&mut writer)?;
        }
        Ok(bytes_written)
    }
    
    /// Check the header values are accurate
    fn check_headers(&self) -> NitfResult<bool> {
        debug!("Checking header values for consistency");
        let hdr = &self.nitf_header.meta;
        // Check image segments
        let n_img = hdr.numi.val().clone() as usize;
        if self.image_segments.len() != n_img {
            return Err(NitfError::Value("NUMI".to_string()))
        };
        for (seg, hdrval) in self.image_segments.iter().zip(hdr.imheaders.iter()) {
            if seg.meta.length() != hdrval.subheader_size.length() {
                return Err(NitfError::Value("Subheader length".to_string()))
            };
            if seg.data.len() != hdrval.item_size.length() {
                return Err(NitfError::Value("Item length".to_string()))
            };
        }
        
        // Check graphic segments
        let n_gph = hdr.nums.val().clone() as usize;
        if self.graphic_segments.len() != n_gph {
            return Err(NitfError::Value("NUMS".to_string()))
        };
        for (seg, hdrval) in self.graphic_segments.iter().zip(hdr.graphheaders.iter()) {
            if seg.meta.length() != hdrval.subheader_size.length() {
                return Err(NitfError::Value("Subheader length".to_string()))
            };
            if seg.data.len() != hdrval.item_size.length() {
                return Err(NitfError::Value("Item length".to_string()))
            };
        }
        
        // Check text segments
        let n_txt = hdr.numt.val().clone() as usize;
        if self.text_segments.len() != n_txt {
            return Err(NitfError::Value("NUMT".to_string()))
        };
        for (seg, hdrval) in self.text_segments.iter().zip(hdr.textheaders.iter()) {
            if seg.meta.length() != hdrval.subheader_size.length() {
                return Err(NitfError::Value("Subheader length".to_string()))
            };
            if seg.data.len() != hdrval.item_size.length() {
                return Err(NitfError::Value("Item length".to_string()))
            };
        }
        
        // Check data extension segments
        let n_dex = hdr.numdes.val().clone() as usize;
        if self.data_extension_segments.len() != n_dex {
            return Err(NitfError::Value("NUMDES".to_string()))
        };
        for (seg, hdrval) in self.data_extension_segments.iter().zip(hdr.dextheaders.iter()) {
            if seg.meta.length() != hdrval.subheader_size.length() {
                return Err(NitfError::Value("Subheader length".to_string()))
            };
            if seg.data.len() != hdrval.item_size.length() {
                return Err(NitfError::Value("Item length".to_string()))
            };
        }
        
        // Check reserved extension segments
        let n_rex = hdr.numres.val().clone() as usize;
        if self.reserved_extension_segments.len() != n_rex {
            return Err(NitfError::Value("NUMRES".to_string()))
        };
        for (seg, hdrval) in self.reserved_extension_segments.iter().zip(hdr.resheaders.iter()) {
            if seg.meta.length() != hdrval.subheader_size.length() {
                return Err(NitfError::Value("Subheader length".to_string()))
            };
            if seg.data.len() != hdrval.item_size.length() {
                return Err(NitfError::Value("Item length".to_string()))
            };
        }
        Ok(true)
    }
    
    pub fn update_headers(&mut self) -> NitfResult<bool> {
        debug!("Updating header values");
        Ok(true)
    }
    pub fn length(&self) -> usize {
        let mut length = 0;
        length += self.nitf_header.meta.length();
        length += self.image_segments.iter().map(|seg| seg.length()).sum::<usize>();
        length += self.graphic_segments.iter().map(|seg| seg.length()).sum::<usize>();
        length += self.text_segments.iter().map(|seg| seg.length()).sum::<usize>();
        length += self.data_extension_segments.iter().map(|seg| seg.length()).sum::<usize>();
        length += self.reserved_extension_segments.iter().map(|seg| seg.length()).sum::<usize>();
        length
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
