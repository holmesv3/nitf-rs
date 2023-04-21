//! Functions to interface with NITF Header

pub mod nitf_header;
pub mod elements;
pub mod image_segment;
pub mod graphic_segment;
pub mod prelude;

use nitf_header::NitfHeader;

use std::io::Read;
use std::fmt::Display;
use std::string::FromUtf8Error;

impl Nitf {
    pub fn from_reader(reader: &mut impl Read) -> Result<Self, FromUtf8Error> {
        let header = NitfHeader::from_reader(reader).unwrap();
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