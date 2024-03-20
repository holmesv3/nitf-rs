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
use thiserror::Error;

pub mod headers;
mod nitf;
pub mod types;

pub use nitf::*;

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
