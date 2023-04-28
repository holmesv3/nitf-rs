use std::io::Read;

pub mod header;
pub mod image;
pub mod graphic;
pub mod text;

pub trait NitfElement {
    fn read(val: &mut [u8], reader: &mut impl Read) -> Result<usize, std::io::Error>;
}