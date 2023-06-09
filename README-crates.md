# nitf-rs

A rust NITF file interface

Current project goals are
- Finish implementing version 2.1 reading (maybe writing)
  - Verify header/subheader field entries (see [`NitfField`](https://docs.rs/nitf-rs/0.1.4/nitf_rs/nitf_2_1/types/struct.NitfField.html))
  - Implement logic for various header/subheader inputs
      - For example, return data from an image segment as an array with the appropriate format (right now only two-channel complex float supported supported)
- Expand with features for various applications of the standard (SICD as a first step)


## Example

```rust
use nitf_rs::nitf::read_nitf;
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

// If you have a SICD file, parse and print the metadata
use nitf_rs::sicd::read_sicd;
let sicd_meta = read_sicd(nitf_path);
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

If you have questions, would like to contribute, or would like to request
something be added, you can ask on [this Discord server](https://discord.gg/Kg7NwN4XgS), or create an issue.