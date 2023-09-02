# nitf-rs

[![crates.io](https://img.shields.io/crates/v/nitf-rs)](https://crates.io/crates/nitf-rs)
[![docs](https://img.shields.io/docsrs/nitf-rs)](https://docs.rs/nitf-rs/latest/nitf_rs/)

A rust NITF file interface

Current project goals are
- Finish implementing version 2.1 reading (maybe writing)
  - Verify header/subheader field entries (see [`NitfField`](https://docs.rs/nitf-rs/0.1.4/nitf_rs/nitf_2_1/types/struct.NitfField.html))
  - Implement logic for various header/subheader inputs
      - For example, return data from an image segment as an array with the appropriate format (right now only two-channel complex float supported supported)

### Note: 
Formerly, there was `Sicd` functionality built into this crate. That has been moved into a separate crate, [sicd-rs](https://github.com/holmesv3/sicd-rs)
## Example

```rust
use nitf_rs::read_nitf;
use std::path::Path;
// Define a string which is the path to some nitf file
let nitf_file: String = get_nitf_file();
let nitf_path = Path::new(&nitf_file);

// Read the file and print all metadata
let nitf = read_nitf(nitf_path);
println!("{}", &nitf);

// Read image data from a segment, check that the dimensions are what we expect
let im_seg = &nitf.image_segments[0];
let data = im_seg.data_to_array();
println!("Meta NROWS: {}, data.nrows(): {}", im_seg.meta.nrows.val, data.nrows());
println!("Meta NCOLS: {}, data.ncols(): {}", im_seg.meta.nrows.val, data.ncols());
```

## Current Functionality

In my own testing, I have only been able to find files to verify parsing for:
- Header
- Image Segments
- Data Extension Segments

To the best of my knowledge, these function as expected.

There is no logic built around the inputs yet (in progress)

If you have questions, would like to contribute, or would like to request
something be added, please create an issue.

