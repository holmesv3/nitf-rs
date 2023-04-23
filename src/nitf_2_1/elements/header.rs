//! Field definitions for NITF header
use std::fmt::Display;

/// File Profile Name
#[derive(Default, Clone, Hash, Debug)]
pub struct Fhdr {
    pub val: [u8; 4],
}
impl Display for Fhdr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Fhdr: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}
/// File Version
#[derive(Default, Clone, Hash, Debug)]
pub struct Fver {
    pub val: [u8; 5],
}
impl Display for Fver {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Fver: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// Complexity Level
#[derive(Default, Clone, Hash, Debug)]
pub struct Clevel {
    pub val: [u8; 2],
}
impl Display for Clevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Clevel: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// Standard Type
#[derive(Default, Clone, Hash, Debug)]
pub struct Stype {
    pub val: [u8; 4],
}
impl Display for Stype {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Stype: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// Originating Station ID
#[derive(Default, Clone, Hash, Debug)]
pub struct Ostaid {
    pub val: [u8; 10],
}
impl Display for Ostaid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Ostaid: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// File Date and Time
#[derive(Default, Clone, Hash, Debug)]
pub struct Fdt {
    pub val: [u8; 14],
}
impl Display for Fdt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Fdt: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// File Title
#[derive(Clone, Hash, Debug)]
pub struct Ftitle {
    pub val: [u8; 80],
}impl Display for Ftitle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Ftitle: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}impl Default for Ftitle {
    fn default() -> Self {
        Self { val: [0u8; 80] }
    }
}


/// File Security Classification
#[derive(Default, Clone, Hash, Debug)]
pub struct Fsclas {
    pub val: [u8; 1]
}
impl Display for Fsclas {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Fsclas: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// File Classification Security System
#[derive(Default, Clone, Hash, Debug)]
pub struct Fsclsy {
    pub val: [u8; 2]
}
impl Display for Fsclsy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Fsclsy: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// File Codewords
#[derive(Default, Clone, Hash, Debug)]
pub struct Fscode {
    pub val: [u8; 11]
}
impl Display for Fscode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Fscode: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// File Control and Handling
#[derive(Default, Clone, Hash, Debug)]
pub struct Fsctlh {
    pub val: [u8; 2]
}
impl Display for Fsctlh {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Fsctlh: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// File Releasing Instructions
#[derive(Default, Clone, Hash, Debug)]
pub struct Fsrel {
    pub val: [u8; 20]
}
impl Display for Fsrel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Fsrel: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// File Declassification Type
#[derive(Default, Clone, Hash, Debug)]
pub struct Fsdctp {
    pub val: [u8; 2]
}
impl Display for Fsdctp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Fsdctp: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// File Declassification Date
#[derive(Default, Clone, Hash, Debug)]
pub struct Fsdcdt {
    pub val: [u8; 8]
}
impl Display for Fsdcdt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Fsdcdt: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// File Declassification Exemption
#[derive(Default, Clone, Hash, Debug)]
pub struct Fsdcxm {
    pub val: [u8; 4]
}
impl Display for Fsdcxm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Fsdcxm: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// File Downgrade
#[derive(Default, Clone, Hash, Debug)]
pub struct Fsdg {
    pub val: [u8; 1]
}
impl Display for Fsdg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Fsdg: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// File Downgrade Date
#[derive(Default, Clone, Hash, Debug)]
pub struct Fsdgdt {
    pub val: [u8; 8]
}
impl Display for Fsdgdt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Fsdgdt: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// File Classification Text
#[derive(Clone, Hash, Debug)]
pub struct Fscltx {
    pub val: [u8; 43]
}
impl Display for Fscltx {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Fscltx: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}
impl Default for Fscltx {
    fn default() -> Self {
        Self { val: ([0u8; 43]) }
    }
}

/// File Classification Authority Type
#[derive(Default, Clone, Hash, Debug)]
pub struct Fscatp {
    pub val: [u8; 1]
}
impl Display for Fscatp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Fscatp: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// File Classification Authority
#[derive(Clone, Hash, Debug)]
pub struct Fscaut {
    pub val: [u8; 40]
}
impl Display for Fscaut {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Fscaut: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// File Classification Reason
#[derive(Default, Clone, Hash, Debug)]
pub struct Fscrsn {
    pub val: [u8; 1]
}
impl Display for Fscrsn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Fscrsn: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}
impl Default for Fscaut {
    fn default() -> Self {
        Self { val: ([0u8; 40]) }
    }
}

/// File Security Source Date
#[derive(Default, Clone, Hash, Debug)]
pub struct Fssrdt {
    pub val: [u8; 8]
}
impl Display for Fssrdt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Fssrdt: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// File Security Control Number
#[derive(Default, Clone, Hash, Debug)]
pub struct Fsctln {
    pub val: [u8; 15]
}
impl Display for Fsctln {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Fsctln: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// File Copy Number
#[derive(Default, Clone, Hash, Debug)]
pub struct Fscop {
    pub val: [u8; 5],
}
impl Display for Fscop {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Fscop: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// File Number of Copies
#[derive(Default, Clone, Hash, Debug)]
pub struct Fscpys {
    pub val: [u8; 5],
}
impl Display for Fscpys {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Fscpys: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// Encryption
#[derive(Default, Clone, Hash, Debug)]
pub struct Encryp {
    pub val: [u8; 1],
}
impl Display for Encryp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Encryp: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// File Background Color
#[derive(Default, Clone, Hash, Debug)]
pub struct Fbkgc {
    pub val: [u8; 3],
}
impl Display for Fbkgc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Fbkgc: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// Originator's Name
#[derive(Default, Clone, Hash, Debug)]
pub struct Oname {
    pub val: [u8; 24],
}
impl Display for Oname {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Oname: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// Originator's Phone Number
#[derive(Default, Clone, Hash, Debug)]
pub struct Ophone {
    pub val: [u8; 18],
}
impl Display for Ophone {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Ophone: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// File Length
#[derive(Default, Clone, Hash, Debug)]
pub struct Fl {
    pub val: [u8; 12],
}
impl Display for Fl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Fl: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// NITF File Header Length
#[derive(Default, Clone, Hash, Debug)]
pub struct Hl {
    pub val: [u8; 6],
}
impl Display for Hl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Hl: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// Number of Image Segments
#[derive(Default, Clone, Hash, Debug)]
pub struct NumImageSegments {
    pub val: [u8; 3],
}
impl Display for NumImageSegments {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NumImageSegments: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// Image Segments
#[derive(Default, Clone, Hash, Debug)]
pub struct ImageSegments {
    pub val: Vec<ImageSegment>,
}
impl Display for ImageSegments {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out_str = String::default();
        for seg in self.val.iter() {
            out_str += format!("\t{}", seg).as_ref()
        }
        write!(f, "{}", out_str)
    }
}

/// Image Segment Info
#[derive(Default, Clone, Hash, Debug)]
pub struct ImageSegment {
    pub subheader_size: [u8; 6],
    pub item_size: [u8; 10],
}
impl Display for ImageSegment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str1 = String::from_utf8(self.subheader_size.to_vec()).unwrap();
        let str2 = String::from_utf8(self.item_size.to_vec()).unwrap();
        write!(f, "[{}, {}]", str1, str2)
    }
}

/// Number of Graphics Segments
#[derive(Default, Clone, Hash, Debug)]
pub struct NumGraphicSegment {
    pub val: [u8; 3],
}
impl Display for NumGraphicSegment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NumGraphicSegment: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// Graphic Segments
#[derive(Default, Clone, Hash, Debug)]
pub struct GraphicsSegments {
    pub val: Vec<GraphicsSegment>,
}
impl Display for GraphicsSegments {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out_str = String::default();
        for seg in self.val.iter() {
            out_str += format!("\t{}", seg).as_ref()
        }
        write!(f, "{}", out_str)
    }
}

/// Graphic Segment Info
#[derive(Default, Clone, Hash, Debug)]
pub struct GraphicsSegment {
    pub subheader_size: [u8; 4],
    pub item_size: [u8; 6],
}
impl Display for GraphicsSegment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str1 = String::from_utf8(self.subheader_size.to_vec()).unwrap();
        let str2 = String::from_utf8(self.item_size.to_vec()).unwrap();
        write!(f, "[{}, {}]", str1, str2)
    }
}

/// Reserved for future use
#[derive(Default, Clone, Hash, Debug)]
pub struct Numx {
    pub val: [u8; 3],
}
impl Display for Numx {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Numx: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// Number of Text Files
#[derive(Default, Clone, Hash, Debug)]
pub struct NumTextFiles {
    pub val: [u8; 3],
}
impl Display for NumTextFiles {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NumTextFiles: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// Text Segments
#[derive(Default, Clone, Hash, Debug)]
pub struct TextSegments {
    pub val: Vec<TextSegment>,
}
impl Display for TextSegments {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out_str = String::default();
        for seg in self.val.iter() {
            out_str += format!("\t{}", seg).as_ref()
        }
        write!(f, "{}", out_str)
    }
}

/// Text Segment Info
#[derive(Default, Clone, Hash, Debug)]
pub struct TextSegment {
    pub subheader_size: [u8; 4],
    pub item_size: [u8; 5],
}
impl Display for TextSegment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str1 = String::from_utf8(self.subheader_size.to_vec()).unwrap();
        let str2 = String::from_utf8(self.item_size.to_vec()).unwrap();
        write!(f, "[{}, {}]", str1, str2)
    }
}

/// Number of Data Extension Segments
#[derive(Default, Clone, Hash, Debug)]
pub struct Numdes {
    pub val: [u8; 3],
}
impl Display for Numdes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Numdes: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// Data Extenstion Segments
#[derive(Default, Clone, Hash, Debug)]
pub struct DataExtSegments {
    pub val: Vec<DataExtSegment>,
}
impl Display for DataExtSegments {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out_str = String::default();
        for seg in self.val.iter() {
            out_str += format!("\t{}", seg).as_ref()
        }
        write!(f, "{}", out_str)
    }
}

/// Data Extenstion Segment Info
#[derive(Default, Clone, Hash, Debug)]
pub struct DataExtSegment {
    pub subheader_size: [u8; 4],
    pub item_size: [u8; 9],
}
impl Display for DataExtSegment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str1 = String::from_utf8(self.subheader_size.to_vec()).unwrap();
        let str2 = String::from_utf8(self.item_size.to_vec()).unwrap();
        write!(f, "[{}, {}]", str1, str2)
    }
}

/// Number of Reserved Extension Segments
#[derive(Default, Clone, Hash, Debug)]
pub struct Numres {
    pub val: [u8; 3],
}
impl Display for Numres {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Numres: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// Reserved Extension Segments
#[derive(Default, Clone, Hash, Debug)]
pub struct ReservedSegments {
    pub val: Vec<ReservedSegment>,
}
impl Display for ReservedSegments {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out_str = String::default();
        for seg in self.val.iter() {
            out_str += format!("\t{}", seg).as_ref()
        }
        write!(f, "{}", out_str)
    }
}

/// Reserved Extension Segment Info
#[derive(Default, Clone, Hash, Debug)]
pub struct ReservedSegment {
    pub subheader_size: [u8; 4],
    pub item_size: [u8; 7],
}
impl Display for ReservedSegment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str1 = String::from_utf8(self.subheader_size.to_vec()).unwrap();
        let str2 = String::from_utf8(self.item_size.to_vec()).unwrap();
        write!(f, "[{}, {}]", str1, str2)
    }
}

/// User Defined Header Data Length
#[derive(Default, Clone, Hash, Debug)]
pub struct Udhdl {
    pub val: [u8; 5],
}
impl Display for Udhdl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Udhdl: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// Extended Header Data Length
#[derive(Default, Clone, Hash, Debug)]
pub struct Xhdl {
    pub val: [u8; 5],
}
impl Display for Xhdl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Xhdl: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}


