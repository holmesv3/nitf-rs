# nitf-rs

[![crates.io](https://img.shields.io/crates/v/nitf-rs)](https://crates.io/crates/nitf-rs)
[![docs](https://img.shields.io/docsrs/nitf-rs)](https://docs.rs/nitf-rs/latest/nitf_rs/)

A minimal rust NITF file interface

### Note: 
Formerly, there was `Sicd` functionality built into this crate. That has been moved into a separate crate [sicd-rs](https://github.com/holmesv3/sicd-rs)
## Example

```rust
// Read a nitf file and dump metadata to stdout
use std::path::Path;
let nitf_path = Path::new("../example.nitf");
let nitf = nitf_rs::read_nitf(&nitf_path);
println!("{nitf}");

// Get the bytes from the first image segment
let im_seg = &nitf.image_segments[0];
let u8_slice = &im_seg.data[..];

// Extract metadata values for the...
// .. File title
let file_title = nitf.nitf_header.meta.ftitle.val;
// .. Number of image segments
let n_img_segments = nitf.nitf_header.meta.numi.val;
// .. and number of rows in the first image segment data
let n_rows = nitf.image_segments[0].meta.nrows.val;
```
## Current Functionality

In my own testing, I have only been able to find files to verify parsing for:
- Header
- Image Segments
- Data Extension Segments

To the best of my knowledge, these function as expected.

If you have questions, would like to contribute, or would like to request
something be added, please create an issue.

