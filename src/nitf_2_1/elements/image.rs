//! Field definitions for NITF image segment subheader
use std::fmt::Display;

/// File Part Type
#[derive(Default, Clone, Hash, Debug)]
pub struct FilePartType {
    pub val: [u8; 2],
}
impl Display for FilePartType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FilePart: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// Image Identifier 1
#[derive(Default, Clone, Hash, Debug)]
pub struct Iid1 {
    pub val: [u8; 10],
}
impl Display for Iid1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Iid1: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// Image Date and Time
#[derive(Default, Clone, Hash, Debug)]
pub struct IDaTim {
    pub val: [u8; 14],
}
impl Display for IDaTim {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "IDaTim: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// Target Identifier
#[derive(Default, Clone, Hash, Debug)]
pub struct TgtID {
    pub val: [u8; 17],
}
impl Display for TgtID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TgtID: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// Image Identifier 2
#[derive(Clone, Hash, Debug)]
pub struct Iid2 {
    pub val: [u8; 80],
}
impl Display for Iid2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Iid2: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}
impl Default for Iid2 {
    fn default() -> Self {
        Self { val: [0u8; 80]}
    }
}

/// Image Security Classification
#[derive(Default, Clone, Hash, Debug)]
pub struct ISClas {
    pub val: [u8; 1],
}
impl Display for ISClas {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ISClas: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// Image Classification Security System
#[derive(Default, Clone, Hash, Debug)]
pub struct ISClSy {
    pub val: [u8; 2],
}
impl Display for ISClSy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ISClSy: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// Image Codewords
#[derive(Default, Clone, Hash, Debug)]
pub struct ISCode {
    pub val: [u8; 11],
}
impl Display for ISCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ISCode: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// Image Control and Handling
#[derive(Default, Clone, Hash, Debug)]
pub struct ISCtlH {
    pub val: [u8; 2],
}
impl Display for ISCtlH {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ISCtlH: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// Image Releasing Instructions
#[derive(Default, Clone, Hash, Debug)]
pub struct ISRel {
    pub val: [u8; 20],
}
impl Display for ISRel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ISRel: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// Image Declassification Type
#[derive(Default, Clone, Hash, Debug)]
pub struct ISDcTp {
    pub val: [u8; 2],
}
impl Display for ISDcTp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ISDcTp: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// Image Declassification Date
#[derive(Default, Clone, Hash, Debug)]
pub struct IsDcDt {
    pub val: [u8; 8],
}
impl Display for IsDcDt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "IsDcDt: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// Image Declassification Exemption
#[derive(Default, Clone, Hash, Debug)]
pub struct IsDcXm {
    pub val: [u8; 4],
}
impl Display for IsDcXm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "IsDcXm: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// Image Downgrade
#[derive(Default, Clone, Hash, Debug)]
pub struct IsDg {
    pub val: [u8; 1],
}
impl Display for IsDg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "IsDg: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// Image Downgrade Date
#[derive(Default, Clone, Hash, Debug)]
pub struct IsDgDt {
    pub val: [u8; 8],
}
impl Display for IsDgDt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "IsDgDt: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// Image Classification Text
#[derive(Clone, Hash, Debug)]
pub struct IsClTx {
    pub val: [u8; 43],
}
impl Display for IsClTx {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "IsClTx: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}
impl Default for IsClTx {
    fn default() -> Self {
        Self { val: [0u8; 43]}
    }
}

/// Image Classification Authority Type
#[derive(Default, Clone, Hash, Debug)]
pub struct ISCATp {
    pub val: [u8; 1],
}
impl Display for ISCATp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ISCATp: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// Image Classification Authority
#[derive(Clone, Hash, Debug)]
pub struct ISCAut {
    pub val: [u8; 40],
}
impl Display for ISCAut {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ISCAut: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}
impl Default for ISCAut { 
    fn default() -> Self {
        Self {val: [0u8; 40]}
    }
}

/// Image Classification Reason
#[derive(Default, Clone, Hash, Debug)]
pub struct ISCRsn {
    pub val: [u8; 1],
}
impl Display for ISCRsn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ISCRsn: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// Image Security Source Date
#[derive(Default, Clone, Hash, Debug)]
pub struct ISSrDt {
    pub val: [u8; 8],
}
impl Display for ISSrDt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ISSrDt: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// Image Security Control Number
#[derive(Default, Clone, Hash, Debug)]
pub struct ISCtlN {
    pub val: [u8; 15],
}
impl Display for ISCtlN {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ISCtlN: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// Encryption
#[derive(Default, Clone, Hash, Debug)]
pub struct ENCRYP {
    pub val: [u8; 1],
}
impl Display for ENCRYP {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ENCRYP: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// Image Source
#[derive(Clone, Hash, Debug)]
pub struct ISorce {
    pub val: [u8; 42],
}
impl Display for ISorce {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ISorce: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}
impl Default for ISorce {
    fn default() -> Self {
        Self { val: [0u8; 42]}
    }
}

/// Number of Significant Rows in image
#[derive(Default, Clone, Hash, Debug)]
pub struct NRows {
    pub val: [u8; 8],
}
impl Display for NRows {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NRows: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// Number of Significant Columns in image
#[derive(Default, Clone, Hash, Debug)]
pub struct NCols {
    pub val: [u8; 8],
}
impl Display for NCols {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NCols: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// Pixel Value Type
#[derive(Default, Clone, Hash, Debug)]
pub struct PVType {
    pub val: [u8; 3],
}
impl Display for PVType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PVType: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// Image Representation
#[derive(Default, Clone, Hash, Debug)]
pub struct IRep {
    pub val: [u8; 8],
}
impl Display for IRep {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "IRep: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// Image Category
#[derive(Default, Clone, Hash, Debug)]
pub struct ICat {
    pub val: [u8; 8],
}
impl Display for ICat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ICat: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// Actual Bits-Per-Pixel Per Band
#[derive(Default, Clone, Hash, Debug)]
pub struct ABPp {
    pub val: [u8; 2],
}
impl Display for ABPp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ABPp: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// Pixel Justification
#[derive(Default, Clone, Hash, Debug)]
pub struct PJust {
    pub val: [u8; 1],
}
impl Display for PJust {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PJust: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// Image Coordinate Representation
#[derive(Default, Clone, Hash, Debug)]
pub struct ICords {
    pub val: [u8; 1],
}
impl Display for ICords {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ICords: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// Number of Image Comments
#[derive(Default, Clone, Hash, Debug)]
pub struct NICom {
    pub val: [u8; 1],
}
impl Display for NICom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NICom: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// Image Comments
#[derive(Default, Clone, Hash, Debug)]
pub struct IComs {
    pub val: Vec<ICom>,
}
impl Display for IComs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out_str = String::default();
        for com in self.val.iter() {
            out_str += format!("\t{}", com).as_ref()
        }
        write!(f, "IComs: [{}]", out_str)    }
}

/// Image Comment
#[derive(Clone, Hash, Debug)]
pub struct ICom {
    pub val: [u8; 80],
}
impl Display for ICom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ICom: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}
impl Default for ICom {
    fn default() -> Self {
        Self {val: [0u8; 80]}
    }
}

/// Image Compression
#[derive(Default, Clone, Hash, Debug)]
pub struct IC {
    pub val: [u8; 2],
}
impl Display for IC {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "IC: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// Number of Bands
#[derive(Default, Clone, Hash, Debug)]
pub struct NBands {
    pub val: [u8; 1],
}
impl Display for NBands {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NBands: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// 1st Band Representation
#[derive(Default, Clone, Hash, Debug)]
pub struct IRepBand1 {
    pub val: [u8; 2],
}
impl Display for IRepBand1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "IRepBand1: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// 1st Band Subcategory
#[derive(Default, Clone, Hash, Debug)]
pub struct ISubCat1 {
    pub val: [u8; 6],
}
impl Display for ISubCat1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ISubCat1: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// 1st Band Image Filter Condition
#[derive(Default, Clone, Hash, Debug)]
pub struct IFC1 {
    pub val: [u8; 1],
}
impl Display for IFC1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "IFC1: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// 1st Band Standard Image Filter Code
#[derive(Default, Clone, Hash, Debug)]
pub struct IMFlt1 {
    pub val: [u8; 3],
}
impl Display for IMFlt1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "IMFlt1: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// Number of LUTs for the 1st Image Band
#[derive(Default, Clone, Hash, Debug)]
pub struct NLUTs1 {
    pub val: [u8; 1],
}
impl Display for NLUTs1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NLUTs1: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// Image Sync Code
#[derive(Default, Clone, Hash, Debug)]
pub struct ISync {
    pub val: [u8; 1],
}
impl Display for ISync {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ISync: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// Image Mode
#[derive(Default, Clone, Hash, Debug)]
pub struct IMode {
    pub val: [u8; 1],
}
impl Display for IMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "IMode: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// Number of Blocks per Row
#[derive(Default, Clone, Hash, Debug)]
pub struct NBPR {
    pub val: [u8; 4],
}
impl Display for NBPR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NBPR: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// Number of Blocks per Column
#[derive(Default, Clone, Hash, Debug)]
pub struct NBPC {
    pub val: [u8; 4],
}
impl Display for NBPC {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NBPC: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// Number of Pixels Per Block Horizontal
#[derive(Default, Clone, Hash, Debug)]
pub struct NPPBH {
    pub val: [u8; 4],
}
impl Display for NPPBH {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NPPBH: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// Number of Pixels Per Block Vertical
#[derive(Default, Clone, Hash, Debug)]
pub struct NPPBV {
    pub val: [u8; 4],
}
impl Display for NPPBV {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NPPBV: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// Number of Bits Per Pixel
#[derive(Default, Clone, Hash, Debug)]
pub struct NBPP {
    pub val: [u8; 2],
}
impl Display for NBPP {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NBPP: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// Image Display Level
#[derive(Default, Clone, Hash, Debug)]
pub struct IDLvl {
    pub val: [u8; 3],
}
impl Display for IDLvl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "IDLvl: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// Image Attachment Level
#[derive(Default, Clone, Hash, Debug)]
pub struct IALvl {
    pub val: [u8; 3],
}
impl Display for IALvl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "IALvl: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// Image Location
#[derive(Default, Clone, Hash, Debug)]
pub struct ILoc {
    pub val: [u8; 10],
}
impl Display for ILoc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ILoc: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// Image Magnification
#[derive(Default, Clone, Hash, Debug)]
pub struct IMag {
    pub val: [u8; 4],
}
impl Display for IMag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "IMag: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// User Defined Image Data Length
#[derive(Default, Clone, Hash, Debug)]
pub struct UDIDL {
    pub val: [u8; 5],
}
impl Display for UDIDL {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "UDIDL: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// Image Extended Subheader Data Length
#[derive(Default, Clone, Hash, Debug)]
pub struct IXSHDL {
    pub val: [u8; 5],
}
impl Display for IXSHDL {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "IXSHDL: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}