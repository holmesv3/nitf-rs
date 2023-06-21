//! Interface for NITF version 2.1

pub mod segments;
pub mod types;
pub(crate) mod error;

use std::fmt::Display;
use std::fs::File;
use std::path::Path;

use segments::{DataExtension, FileHeader, Graphic, Image, ReservedExtension, Text};

/// Top level NITF interface
#[derive(Default, Debug)]
pub struct Nitf {
    /// Nitf file header.
    ///     
    /// See [NitfHeader](segments::headers::nitf_file_hdr) for `meta` fields
    pub nitf_header: FileHeader,

    /// Vector of image segments.
    ///     
    /// See [ImageHeader](segments::headers::image_hdr) for `meta` fields
    pub image_segments: Vec<Image>,

    /// Vector of graphics segments.
    ///     
    /// See [GraphicHeader](segments::headers::graphic_hdr) for `meta` fields
    pub graphic_segments: Vec<Graphic>,

    /// Vector of text segments.
    ///     
    /// See [TextHeader](segments::headers::text_hdr) for `meta` fields
    pub text_segments: Vec<Text>,

    /// Vector of data extension segments.
    ///     
    /// See [DataExtensionHeader](segments::headers::data_extension_hdr) for `meta` fields
    pub data_extension_segments: Vec<DataExtension>,

    /// Vector of reserved extension segments.
    ///     
    /// See [ReservedExtensionHeader](segments::headers::reserved_extension_hdr) for `meta` fields
    pub reserved_extension_segments: Vec<ReservedExtension>,
}

/// Construct a [Nitf] object from a file `path`.
///
/// # Example
///
///     use std::path::Path;
///     use nitf_rs::nitf::read_nitf;
///
///     let nitf_path = Path::new("../example.nitf");
///     let nitf = read_nitf(nitf_path);
pub fn read_nitf(path: &Path) -> Nitf {
    let mut file = File::open(path).unwrap();
    Nitf::from_file(&mut file)
}

impl Nitf {
    pub fn from_file(file: &mut File) -> Self {
        let mut nitf = Self::default();
        nitf.nitf_header.read(file);

        let mut n_seg = nitf.nitf_header.meta.numi.val as usize;
        for i_seg in 0..n_seg {
            let seg_info = &nitf.nitf_header.meta.imheaders[i_seg];
            let header_size = seg_info.subheader_size.val;
            let data_size = seg_info.item_size.val;
            let seg = Image::initialize(file, header_size, data_size);
            nitf.image_segments.push(seg);
        }

        n_seg = nitf.nitf_header.meta.nums.val as usize;
        for i_seg in 0..n_seg {
            let seg_info = &nitf.nitf_header.meta.graphheaders[i_seg];
            let header_size = seg_info.subheader_size.val;
            let data_size: u64 = seg_info.item_size.val;
            let seg = Graphic::initialize(file, header_size, data_size);
            nitf.graphic_segments.push(seg);
        }

        n_seg = nitf.nitf_header.meta.numt.val as usize;
        for i_seg in 0..n_seg {
            let seg_info = &nitf.nitf_header.meta.textheaders[i_seg];
            let header_size = seg_info.subheader_size.val;
            let data_size: u64 = seg_info.item_size.val;
            let seg = Text::initialize(file, header_size, data_size);
            nitf.text_segments.push(seg);
        }

        n_seg = nitf.nitf_header.meta.numdes.val as usize;
        for i_seg in 0..n_seg {
            let seg_info = &nitf.nitf_header.meta.dextheaders[i_seg];
            let header_size = seg_info.subheader_size.val;
            let data_size: u64 = seg_info.item_size.val;
            let seg = DataExtension::initialize(file, header_size, data_size);
            nitf.data_extension_segments.push(seg);
        }

        n_seg = nitf.nitf_header.meta.numres.val as usize;
        for i_seg in 0..n_seg {
            let seg_info = &nitf.nitf_header.meta.resheaders[i_seg];
            let header_size = seg_info.subheader_size.val;
            let data_size = seg_info.item_size.val;
            let seg = ReservedExtension::initialize(file, header_size, data_size);
            nitf.reserved_extension_segments.push(seg);
        }
        nitf
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
