//! Header metadata definitions

use std::fs::File;

pub mod data_extension_hdr;
pub mod graphic_hdr;
pub mod image_hdr;
pub mod nitf_file_hdr;
pub mod reserved_extension_hdr;
pub mod text_hdr;

pub use data_extension_hdr::DataExtensionHeader;
pub use graphic_hdr::GraphicHeader;
pub use image_hdr::ImageHeader;
pub use nitf_file_hdr::NitfHeader;
pub use reserved_extension_hdr::ReservedExtensionHeader;
pub use text_hdr::TextHeader;

use crate::NitfResult;

/// Nitf segment header interface definition
///
/// Provide implementation for `read()`, `from_reader` defined automatically.
pub trait NitfSegmentHeader
where
    Self: Sized + Default,
{
    /// Read the segment info from stream
    ///
    /// # Parameters
    ///
    /// reader: Stream from which to read header information
    #[allow(unused)]
    fn read(&mut self, reader: &mut File) -> NitfResult<()>;
    
    /// Write the segment info to stream
    ///
    /// # Parameters
    ///
    /// writer: Stream from which to write header information
    #[allow(unused)]
    fn write(&self, writer: &mut File) -> NitfResult<usize>;
    
    /// Get the length of the segment
    #[allow(unused)]
    fn length(&self) -> usize;


    fn from_reader(reader: &mut File) -> NitfResult<Self> {
        let mut hdr = Self::default();
        hdr.read(reader)?;
        Ok(hdr)
    }
}
