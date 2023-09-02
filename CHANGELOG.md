# Changes by version

## 0.2.0
- Removed all SICD support to a separate crate, [sicd-rs](https://crates.io/crates/sicd-rs)

## 0.1.8
- Moved `read_<nitf format>` functions inside of respective modules
  - e.g, `use nitf_rs::{nitf::read_nitf, sicd::read_sicd};`
- For the SICD file format
  - Updated to 1.3.0 standard found [here](https://nsgreg.nga.mil/doc/view?i=5381&month=6&day=25&year=2023)
  - Added limited pre-version 1.0 compatibility (currently version 0.5.0 and 0.4.0, untested)
  - Added intermediate step of SICD parsing to determine proper version
  
## 0.1.7
- Switched to using `quick_xml` for better `.xml` syntax support
- Added `eval()` functions to `Sicd` polynomials
- Renamed all `Sicd` struct members to snake_case
- Added unit tests for basic sicd types (RowCol, LLH, etc.)

## 0.1.6
- Added `nitf_rs::sicd` module
  - Structs using `serde_xml_rs` for `Sicd` data parsing
  - Added `parse_sicd_meta()` to `Nitf`
