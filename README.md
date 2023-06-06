# nitf-rs

[![crates.io](https://img.shields.io/crates/v/nitf-rs)](https://crates.io/crates/nitf-rs)
[![docs](https://img.shields.io/docsrs/nitf-rs)](https://docs.rs/nitf-rs/latest/nitf_rs/)
[![Discord](https://img.shields.io/discord/1109246714721865810?label=discord&logo=discord&logoColor=white&color=blue)](https://discord.gg/Kg7NwN4XgS)

A rust NITF file interface

Current project goals are
- Finish implementing version 2.1 reading (maybe writing)
  - Verify header/subheader field entries (see [`NitfField`](https://docs.rs/nitf-rs/0.1.4/nitf_rs/nitf_2_1/types/struct.NitfField.html))
  - Implement logic for various header/subheader inputs
      - For example, return data from an image segment as an array with the appropriate format (right now only `Complex32` supported)
- Expand with features for various applications of the standard (SICD as a first step)


## Example
In your `Cargo.toml`, include `nitf-rs` as a dependency, or, for the 'latest-and-greatest' features, pull it from the `main` repository branch
```toml
nitf-rs = {git="https://github.com/holmesv3/nitf-rs.git"}
```
### Usage
```rust
use nitf_rs::read_nitf;
use std::path::Path;
// Define a string which is the path to some nitf file
let nitf_file: String = get_nitf_file();
let nitf_path = Path::new(&nitf_file);

// Read the file and print all metadata
let nitf = read_nitf(nitf_path);
println!("{}", &nitf);

// Read image data, check that the dimensions are what we expect
let im_seg = &nitf.image_segments[0];
let data = im_seg.data_to_array();
println!("Meta NROWS: {}, data.nrows(): {}", im_seg.meta.NROWS.val, data.nrows());
println!("Meta NCOLS: {}, data.ncols(): {}", im_seg.meta.NCOLS.val, data.ncols());

// If you have a SICD file, parse and print the metadata
let sicd_meta = &nitf.parse_sicd_meta().unwrap();
println!("{:#?}", sicd_meta);
```

## Current Functionality

In my own testing, I have only been able to find files to verify parsing for:
- Header
- Image Segments
- Data Extension Segments
  - Sicd xml metadata parsing

To the best of my knowledge, these function as expected.

There is no logic built around the inputs yet (in progress)

## New from last version (that I remember)
- Added a `sicd` metadata parsing capability
- Added basic `eval` methods on `SICD` polynomial objects (1D, 2D, and Xyz)

If you have questions, would like to contribute, or would like to request
something be added, you can ask on [this Discord server](https://discord.gg/Kg7NwN4XgS), or create an issue.

## Next in line
- Improve documentation, examples
- Now that some basic `enum`/`struct`'s are in place, can begin to define within interface functions
  - First goal is going to be to read different types of data from an image segment for `P` IMODE value
  - ... Then think about how to do other values for `IMODE`
