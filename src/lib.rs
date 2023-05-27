#![allow(clippy::needless_return)]

//! Crate for reading and manipulating NITF files
use std::fs::File;
use std::path::Path;

pub mod nitf_2_1;

use nitf_2_1::Nitf;

/// Construct a `Nitf` object from a file `path`.
///
/// # Example
///
///     use std::path::Path;
///     use nitf_rs::read_nitf;
///
///     let nitf_path = Path::new(<path-to-file>);
///     let nitf = read_nitf(nitf_path);
pub fn read_nitf(path: &Path) -> Nitf {
    let mut reader = File::open(path).unwrap();
    return Nitf::from_file(&mut reader);
}




// UNIT TESTS
#[cfg(test)]
mod tests {
    // TODO - Figure out how to test this
}

