//! Crate for reading and manipulating NITF files
//! 
use std::fs::File;
use std::path::Path;
use std::string::FromUtf8Error;

pub mod nitf_2_1;

use nitf_2_1::nitf_header::NitfHeader;
use nitf_2_1::Nitf;

pub fn read_nitf_header(path: &Path) -> Result<NitfHeader, FromUtf8Error> {
    let mut reader = File::open(path).unwrap();
    NitfHeader::from_reader(&mut reader)
}

pub fn read_nitf(path: &Path) -> Result<Nitf, FromUtf8Error> {
    let mut reader = File::open(path).unwrap();
    Nitf::from_reader(&mut reader)
}