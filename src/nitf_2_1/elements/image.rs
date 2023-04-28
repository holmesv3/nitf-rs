//! Field definitions for NITF image segment subheader
use std::fmt::Display;

/// File Part Type
#[derive(Default, Clone, Hash, Debug)]
pub struct FILEPARTTYPE {
    pub val: [u8; 2],
}
impl Display for FILEPARTTYPE {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FILEPART: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// Image Identifier 1
#[derive(Default, Clone, Hash, Debug)]
pub struct IID1 {
    pub val: [u8; 10],
}
impl Display for IID1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "IID1: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// Image Date and Time
#[derive(Default, Clone, Hash, Debug)]
pub struct IDATIM {
    pub val: [u8; 14],
}
impl Display for IDATIM {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "IDATIM: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// Target Identifier
#[derive(Default, Clone, Hash, Debug)]
pub struct TGTID {
    pub val: [u8; 17],
}
impl Display for TGTID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TGTID: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// Image Identifier 2
#[derive(Clone, Hash, Debug)]
pub struct IID2 {
    pub val: [u8; 80],
}
impl Display for IID2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "IID2: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}
impl Default for IID2 {
    fn default() -> Self {
        Self { val: [0u8; 80]}
    }
}

/// Image Security Classification
#[derive(Default, Clone, Hash, Debug)]
pub struct ISCLAS {
    pub val: [u8; 1],
}
impl Display for ISCLAS {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ISCLAS: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// Image Classification Security System
#[derive(Default, Clone, Hash, Debug)]
pub struct ISCLSY {
    pub val: [u8; 2],
}
impl Display for ISCLSY {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ISCLSY: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// Image Codewords
#[derive(Default, Clone, Hash, Debug)]
pub struct ISCODE {
    pub val: [u8; 11],
}
impl Display for ISCODE {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ISCODE: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// Image Control and Handling
#[derive(Default, Clone, Hash, Debug)]
pub struct ISCTLH {
    pub val: [u8; 2],
}
impl Display for ISCTLH {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ISCTLH: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// Image Releasing Instructions
#[derive(Default, Clone, Hash, Debug)]
pub struct ISREL {
    pub val: [u8; 20],
}
impl Display for ISREL {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ISREL: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// Image Declassification Type
#[derive(Default, Clone, Hash, Debug)]
pub struct ISDCTP {
    pub val: [u8; 2],
}
impl Display for ISDCTP {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ISDCTP: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// Image Declassification Date
#[derive(Default, Clone, Hash, Debug)]
pub struct ISDCDT {
    pub val: [u8; 8],
}
impl Display for ISDCDT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ISDCDT: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// Image Declassification Exemption
#[derive(Default, Clone, Hash, Debug)]
pub struct ISDCXM {
    pub val: [u8; 4],
}
impl Display for ISDCXM {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ISDCXM: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// Image Downgrade
#[derive(Default, Clone, Hash, Debug)]
pub struct ISDG {
    pub val: [u8; 1],
}
impl Display for ISDG {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ISDG: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// Image Downgrade Date
#[derive(Default, Clone, Hash, Debug)]
pub struct ISDGDT {
    pub val: [u8; 8],
}
impl Display for ISDGDT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ISDGDT: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// Image Classification Text
#[derive(Clone, Hash, Debug)]
pub struct ISCLTX {
    pub val: [u8; 43],
}
impl Display for ISCLTX {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ISCLTX: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}
impl Default for ISCLTX {
    fn default() -> Self {
        Self { val: [0u8; 43]}
    }
}

/// Image Classification Authority Type
#[derive(Default, Clone, Hash, Debug)]
pub struct ISCATP {
    pub val: [u8; 1],
}
impl Display for ISCATP {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ISCATP: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// Image Classification Authority
#[derive(Clone, Hash, Debug)]
pub struct ISCAUT {
    pub val: [u8; 40],
}
impl Display for ISCAUT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ISCAUT: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}
impl Default for ISCAUT { 
    fn default() -> Self {
        Self {val: [0u8; 40]}
    }
}

/// Image Classification Reason
#[derive(Default, Clone, Hash, Debug)]
pub struct ISCRSN {
    pub val: [u8; 1],
}
impl Display for ISCRSN {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ISCRSN: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// Image Security Source Date
#[derive(Default, Clone, Hash, Debug)]
pub struct ISSRDT {
    pub val: [u8; 8],
}
impl Display for ISSRDT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ISSRDT: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// Image Security Control Number
#[derive(Default, Clone, Hash, Debug)]
pub struct ISCTLN {
    pub val: [u8; 15],
}
impl Display for ISCTLN {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ISCTLN: {}", String::from_utf8(self.val.to_vec()).unwrap())
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
pub struct ISORCE {
    pub val: [u8; 42],
}
impl Display for ISORCE {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ISORCE: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}
impl Default for ISORCE {
    fn default() -> Self {
        Self { val: [0u8; 42]}
    }
}

/// Number of Significant Rows in image
#[derive(Default, Clone, Hash, Debug)]
pub struct NROWS {
    pub val: [u8; 8],
}
impl Display for NROWS {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NROWS: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// Number of Significant Columns in image
#[derive(Default, Clone, Hash, Debug)]
pub struct NCOLS {
    pub val: [u8; 8],
}
impl Display for NCOLS {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NCOLS: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// Pixel Value Type
#[derive(Default, Clone, Hash, Debug)]
pub struct PVTYPE {
    pub val: [u8; 3],
}
impl Display for PVTYPE {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PVTYPE: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// Image Representation
#[derive(Default, Clone, Hash, Debug)]
pub struct IREP {
    pub val: [u8; 8],
}
impl Display for IREP {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "IREP: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// Image Category
#[derive(Default, Clone, Hash, Debug)]
pub struct ICAT {
    pub val: [u8; 8],
}
impl Display for ICAT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ICAT: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// Actual Bits-Per-Pixel Per Band
#[derive(Default, Clone, Hash, Debug)]
pub struct ABPP {
    pub val: [u8; 2],
}
impl Display for ABPP {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ABPP: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// Pixel Justification
#[derive(Default, Clone, Hash, Debug)]
pub struct PJUST {
    pub val: [u8; 1],
}
impl Display for PJUST {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PJUST: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// Image Coordinate Representation
#[derive(Default, Clone, Hash, Debug)]
pub struct ICORDS {
    pub val: [u8; 1],
}
impl Display for ICORDS {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ICORDS: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// Number of Image Comments
#[derive(Default, Clone, Hash, Debug)]
pub struct NICOM {
    pub val: [u8; 1],
}
impl Display for NICOM {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NICOM: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// Image Comments
#[derive(Default, Clone, Hash, Debug)]
pub struct ICOMS {
    pub val: Vec<ICOM>,
}
impl Display for ICOMS {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out_str = String::default();
        for com in self.val.iter() {
            out_str += format!("\t{}", com).as_ref()
        }
        write!(f, "IComs: [{}]", out_str)    }
}

/// Image Comment
#[derive(Clone, Hash, Debug)]
pub struct ICOM {
    pub val: [u8; 80],
}
impl Display for ICOM {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ICOM: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}
impl Default for ICOM {
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
pub struct NBANDS {
    pub val: [u8; 1],
}
impl Display for NBANDS {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NBANDS: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// 1st Band Representation
#[derive(Default, Clone, Hash, Debug)]
pub struct IREPBAND1 {
    pub val: [u8; 2],
}
impl Display for IREPBAND1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "IREPBAND1: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// 1st Band Subcategory
#[derive(Default, Clone, Hash, Debug)]
pub struct ISUBCAT1 {
    pub val: [u8; 6],
}
impl Display for ISUBCAT1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ISUBCAT1: {}", String::from_utf8(self.val.to_vec()).unwrap())
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
pub struct IMFLT1 {
    pub val: [u8; 3],
}
impl Display for IMFLT1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "IMFLT1: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// Number of LUTs for the 1st Image Band
#[derive(Default, Clone, Hash, Debug)]
pub struct NLUTS1 {
    pub val: [u8; 1],
}
impl Display for NLUTS1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NLUTS1: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// Image Sync Code
#[derive(Default, Clone, Hash, Debug)]
pub struct ISYNC {
    pub val: [u8; 1],
}
impl Display for ISYNC {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ISYNC: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// Image Mode
#[derive(Default, Clone, Hash, Debug)]
pub struct IMODE {
    pub val: [u8; 1],
}
impl Display for IMODE {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "IMODE: {}", String::from_utf8(self.val.to_vec()).unwrap())
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
pub struct IDLVL {
    pub val: [u8; 3],
}
impl Display for IDLVL {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "IDLVL: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// Image Attachment Level
#[derive(Default, Clone, Hash, Debug)]
pub struct IALVL {
    pub val: [u8; 3],
}
impl Display for IALVL {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "IALVL: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// Image Location
#[derive(Default, Clone, Hash, Debug)]
pub struct ILOC {
    pub val: [u8; 10],
}
impl Display for ILOC {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ILOC: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// Image Magnification
#[derive(Default, Clone, Hash, Debug)]
pub struct IMAG {
    pub val: [u8; 4],
}
impl Display for IMAG {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "IMAG: {}", String::from_utf8(self.val.to_vec()).unwrap())
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