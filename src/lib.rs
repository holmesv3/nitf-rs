//! Crate for reading and manipulating NITF files
//!

// TODO:
// Need to implement reading optional segments
// Meaning: after the main header, each sub-segment starts with 2 characters.
// Need to implement reading these in some arbitrary order, storing starting bytes, offsets, etc.

use std::fs::File;
use std::path::Path;

use ndarray::Array2;
use nitf_2_1::headers::image::ImageHeader;
use nitf_2_1::types::DataSegment;
use num_complex::Complex32;

pub mod nitf_2_1;

use nitf_2_1::Nitf;

/// Construct a `Nitf` object from a file `path`.
///
/// If `path` is `None`, returns `Nitf::default()`
///
/// # Example
///
///     use std::path::Path;
///     use nitf_rs::read_nitf;
///
///     let nitf_path = Path::new(<path-to-file>);
///     let nitf = read_nitf(Some(nitf_path));
pub fn read_nitf(path: Option<&Path>) -> Nitf {
    match path {
        Some(path) => {
            let mut reader = File::open(path).unwrap();
            return Nitf::from_file(&mut reader);
        }
        None => return Nitf::default(),
    }
}

// TODO: Implement reading a slice of the data into an array, as opposed to the whole thing.
/// Read image data from `image_header` into an array
///
/// Only supports `Complex32` data
pub fn data_to_array(image_header: &DataSegment<ImageHeader>) -> Array2<Complex32> {
    let n_row: usize = image_header.meta.NROWS.string.parse().unwrap();
    let n_col: usize = image_header.meta.NCOLS.string.parse().unwrap();

    let mut arr = Array2::from_elem((n_row, n_col), Complex32::default());

    let mut real: [u8; 4] = [0u8; 4];
    let mut imag: [u8; 4] = [0u8; 4];

    let data = image_header.read_data_bytes(..); // read all the data
    let data_chunks = &mut data.chunks(4); // grab 4 bytes at a time

    for row in arr.rows_mut() {
        for elm in row {
            real.copy_from_slice(data_chunks.next().unwrap());
            imag.copy_from_slice(data_chunks.next().unwrap());

            *elm = Complex32 {
                re: f32::from_be_bytes(real),
                im: f32::from_be_bytes(imag),
            };
        }
    }
    return arr;
}

// UNIT TESTS
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_nitf() {
        let _res = read_nitf(None);
    }
}
