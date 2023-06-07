# Changes by version

## 0.1.7
- Renamed all `sicd` struct members to snake_case
- Added simple `eval()` functions to `sicd` polynomials

## 0.1.6
- Added `nitf_rs::sicd` module
  - Structs using `serde_xml_rs` for `sicd` data parsing
    - Due to lack of good support for `.xml` specific syntax, may switch this later
  - Added `parse_sicd_meta()` to `sNitf`