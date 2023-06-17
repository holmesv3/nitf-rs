# Changes by version

## 0.1.8
- Moved `read_<nitf format>` functions inside of respective modules
  - e.g, `use nitf_rs::{nitf::read_nitf, sicd::read_sicd};`
- Added `sicd` feature flag with optional `serde` and `quick-xml` dependencies
- 

## 0.1.7
- Switched to using `quick_xml` for better `.xml` syntax support
- Added `eval()` functions to `Sicd` polynomials
- Renamed all `Sicd` struct members to snake_case
- Added unit tests for basic sicd types (RowCol, LLH, etc.)

## 0.1.6
- Added `nitf_rs::sicd` module
  - Structs using `serde_xml_rs` for `Sicd` data parsing
  - Added `parse_sicd_meta()` to `Nitf`
