# Changes by version

## 0.1.7 (In development)
- Switched to using `quick_xml` for better `.xml` syntax support
- Added `eval()` functions to `sicd` polynomials
- Renamed all `sicd` struct members to snake_case
- Added unit tests for basic sicd types (RowCol, LLH, etc.)

## 0.1.6
- Added `nitf_rs::sicd` module
  - Structs using `serde_xml_rs` for `sicd` data parsing
  - Added `parse_sicd_meta()` to `sNitf`