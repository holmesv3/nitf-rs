//! Header definitions for each segment
pub mod data_extension_hdr;
pub mod graphic_hdr;
pub mod image_hdr;
pub mod nitf_file_hdr;
pub mod reserved_extension_hdr;
pub mod text_hdr;

use std::io::{Read, Seek};

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
    fn read(&mut self, reader: &mut (impl Read + Seek)) {
        todo!("Didn't implement read() method")
    }

    fn from_reader(reader: &mut (impl Read + Seek)) -> Self {
        let mut hdr = Self::default();
        hdr.read(reader);
        return hdr;
    }
}
