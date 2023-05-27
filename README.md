# nitf-rs

[![crates.io](https://img.shields.io/crates/v/nitf-rs)](https://crates.io/crates/nitf-rs)
[![docs](https://img.shields.io/docsrs/nitf-rs)](https://docs.rs/nitf-rs/latest/nitf_rs/)
[![Discord](https://img.shields.io/discord/1109246714721865810?label=discord&logo=discord&logoColor=white&color=blue)](https://discord.gg/Kg7NwN4XgS)

A rust NITF file interface

Current project goals are
- Finish implementing version 2.1 reading (maybe writing)
  - Verify header/subheader field entries (see [`NitfField`](https://docs.rs/nitf-rs/0.1.4/nitf_rs/nitf_2_1/types/struct.NitfField.html)
  - Implement logic for various header/subheader inputs 
      - For example, return data from an image segment as an array with the appropriate format (right now only `Complex32` supported)
- Expand with features for various applications of the standard (SICD as a first step)

This started as a personal project to learn the language better, and I work on 
it as I have time to. 

If you have questions, would like to contribute, or would like to request 
something be added, you can ask on [this Discord server](https://discord.gg/Kg7NwN4XgS), or create an issue.

## Example
```rust
use nitf_rs::read_nitf;
use std::path::Path;
// Define a string which is the path to some nitf file
let nitf_file: String = get_nitf_file();
let nitf_path = Path::new(&nitf_file);

// Read the file and parse all available metadata
let nitf = read_nitf(nitf_path);

// Print all metadata
println!("{}", &nitf);

// Read image data, check that the dimensions are what we expect
let im_seg = &nitf.image_segments[0];
let data = im_seg.data_to_array();
println!("Meta NROWS: {}, data.nrows(): {}", im_seg.meta.NROWS.val, data.nrows());
println!("Meta NCOLS: {}, data.ncols(): {}", im_seg.meta.NCOLS.val, data.ncols());
```

## Current Functionality

In my own testing, I have only been able to find files to verify parsing for:
- Header
- Image Segments
- Data Extension Segments

To the best of my knowledge, these function as expected.

There is no logic built around the inputs yet

## New from last version (that I remember)
- Removed `NitfFieldVec` and `NitfSubheaderVec` types
- Made `NitfField` generic for a parameter `V`
  - Can define types and `impl FromStr` to allow more complex parsing
  - Allows verification of fields upon read
  - Also helps with the idea of writing in the future
- Added minimal docstrings across all definitions

## Next in line
- Might rework the top-level segment definitions
  - Feels like the functionality of everything is too deep
  - Shift the current `subheader` and `Datasegment` stuff a level down, provide more utility at the `nitf.___` level
- Now that some basic `enum`/`struct`'s are in place, can begin to define within interface functions
  - First goal is going to be to read different types of data from an image segment for `P` IMODE value
  - ... Then think about how to do other values for `IMODE`
