//! Functions to interface with NITF Header

pub mod nitf_header;
pub mod image_segment;
pub mod graphic_segment;
pub mod text_segment;
pub mod elements;
pub mod prelude;

use nitf_header::NitfHeader;

use std::io::Read;
use std::fmt::Display;
use std::string::FromUtf8Error;

impl NitfSegment for Nitf {
    fn read(reader: &mut impl Read) -> Result<Nitf, FromUtf8Error> {
        let res = NitfHeader::from_reader(reader);
        let header = match res {
            Ok(header) => header,
            Err(e) => return Err(e),
        };
        let nitf = Self { header };
        return Ok(nitf)
    }
}

#[derive(Default, Clone, Hash, Debug)]
pub struct Nitf {
    pub header: NitfHeader,
}
impl Display for Nitf {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.header)
    }
}

pub trait NitfSegment                                                                                    {
    fn read(reader: &mut impl Read) -> Result<usize, std::io::Error>;
}