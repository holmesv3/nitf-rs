//! Field definitions for NITF header
use std::fmt::Display;

// FHDR
#[derive(Default, Clone, Hash, Debug)]
pub struct Fhdr {
    pub val: [u8; 4],
}
impl Display for Fhdr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Fhdr: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}
// FVER
#[derive(Default, Clone, Hash, Debug)]
pub struct Fver {
    pub val: [u8; 5],
}
impl Display for Fver {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Fver: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

// CLEVEL
#[derive(Default, Clone, Hash, Debug)]
pub struct Clevel {
    pub val: [u8; 2],
}
impl Display for Clevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Clevel: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

// STYPE
#[derive(Default, Clone, Hash, Debug)]
pub struct Stype {
    pub val: [u8; 4],
}
impl Display for Stype {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Stype: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

// OSTAID
#[derive(Default, Clone, Hash, Debug)]
pub struct Ostaid {
    pub val: [u8; 10],
}
impl Display for Ostaid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Ostaid: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

// FDT
#[derive(Default, Clone, Hash, Debug)]
pub struct Fdt {
    pub val: [u8; 14],
}
impl Display for Fdt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Fdt: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

// FTITLE
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

// Security Types

// FSCLAS
#[derive(Default, Clone, Hash, Debug)]
pub struct Fsclas {
    pub val: [u8; 1]
}
impl Display for Fsclas {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Fsclas: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

// FSCLSY
#[derive(Default, Clone, Hash, Debug)]
pub struct Fsclsy {
    pub val: [u8; 2]
}
impl Display for Fsclsy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Fsclsy: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

// FSCODE
#[derive(Default, Clone, Hash, Debug)]
pub struct Fscode {
    pub val: [u8; 11]
}
impl Display for Fscode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Fscode: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

// FSCTLH
#[derive(Default, Clone, Hash, Debug)]
pub struct Fsctlh {
    pub val: [u8; 2]
}
impl Display for Fsctlh {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Fsctlh: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

// FSREL
#[derive(Default, Clone, Hash, Debug)]
pub struct Fsrel {
    pub val: [u8; 20]
}
impl Display for Fsrel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Fsrel: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

// FSDCTP
#[derive(Default, Clone, Hash, Debug)]
pub struct Fsdctp {
    pub val: [u8; 2]
}
impl Display for Fsdctp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Fsdctp: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

// FSDCDT
#[derive(Default, Clone, Hash, Debug)]
pub struct Fsdcdt {
    pub val: [u8; 8]
}
impl Display for Fsdcdt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Fsdcdt: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

// FSDCXM
#[derive(Default, Clone, Hash, Debug)]
pub struct Fsdcxm {
    pub val: [u8; 4]
}
impl Display for Fsdcxm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Fsdcxm: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

// FSDG
#[derive(Default, Clone, Hash, Debug)]
pub struct Fsdg {
    pub val: [u8; 1]
}
impl Display for Fsdg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Fsdg: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

// FSDGDT
#[derive(Default, Clone, Hash, Debug)]
pub struct Fsdgdt {
    pub val: [u8; 8]
}
impl Display for Fsdgdt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Fsdgdt: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

// FSCLTX
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

// FSCATP
#[derive(Default, Clone, Hash, Debug)]
pub struct Fscatp {
    pub val: [u8; 1]
}
impl Display for Fscatp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Fscatp: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

// FSCAUT
#[derive(Clone, Hash, Debug)]
pub struct Fscaut {
    pub val: [u8; 40]
}
impl Display for Fscaut {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Fscaut: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

// FSCRSN
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

// FSSRDT
#[derive(Default, Clone, Hash, Debug)]
pub struct Fssrdt {
    pub val: [u8; 8]
}
impl Display for Fssrdt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Fssrdt: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

// FSCTLN
#[derive(Default, Clone, Hash, Debug)]
pub struct Fsctln {
    pub val: [u8; 15]
}
impl Display for Fsctln {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Fsctln: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

// FSCOP
#[derive(Default, Clone, Hash, Debug)]
pub struct Fscop {
    pub val: [u8; 5],
}
impl Display for Fscop {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Fscop: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

// FSCPYS
#[derive(Default, Clone, Hash, Debug)]
pub struct Fscpys {
    pub val: [u8; 5],
}
impl Display for Fscpys {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Fscpys: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

// ENCRYP
#[derive(Default, Clone, Hash, Debug)]
pub struct Encryp {
    pub val: [u8; 1],
}
impl Display for Encryp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Encryp: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

// FBKGC
#[derive(Default, Clone, Hash, Debug)]
pub struct Fbkgc {
    pub val: [u8; 3],
}
impl Display for Fbkgc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Fbkgc: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

// ONAME
#[derive(Default, Clone, Hash, Debug)]
pub struct Oname {
    pub val: [u8; 24],
}
impl Display for Oname {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Oname: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

// OPHONE
#[derive(Default, Clone, Hash, Debug)]
pub struct Ophone {
    pub val: [u8; 18],
}
impl Display for Ophone {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Ophone: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

// FL
#[derive(Default, Clone, Hash, Debug)]
pub struct Fl {
    pub val: [u8; 12],
}
impl Display for Fl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Fl: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

// HL
#[derive(Default, Clone, Hash, Debug)]
pub struct Hl {
    pub val: [u8; 6],
}
impl Display for Hl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Hl: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

// NUMIMAGESEGMENTS
#[derive(Default, Clone, Hash, Debug)]
pub struct NumImageSegments {
    pub val: [u8; 3],
}
impl Display for NumImageSegments {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NumImageSegments: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

// IMAGESEGMENTS
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

// IMAGESEGMENT
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

// NUMGRAPHICSEGMENT
#[derive(Default, Clone, Hash, Debug)]
pub struct NumGraphicSegment {
    pub val: [u8; 3],
}
impl Display for NumGraphicSegment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NumGraphicSegment: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

// GRAPHICSSEGMENTS
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

// GRAPHICSSEGMENT
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

// NUMX
#[derive(Default, Clone, Hash, Debug)]
pub struct Numx {
    pub val: [u8; 3],
}
impl Display for Numx {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Numx: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

// NUMTEXTFILES
#[derive(Default, Clone, Hash, Debug)]
pub struct NumTextFiles {
    pub val: [u8; 3],
}
impl Display for NumTextFiles {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NumTextFiles: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

// TEXTSEGMENTS
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

// TEXTSEGMENT
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

// NUMDES
#[derive(Default, Clone, Hash, Debug)]
pub struct Numdes {
    pub val: [u8; 3],
}
impl Display for Numdes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Numdes: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

// DATAEXTSEGMENTS
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

// DATAEXTSEGMENT
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

// NUMRES
#[derive(Default, Clone, Hash, Debug)]
pub struct Numres {
    pub val: [u8; 3],
}
impl Display for Numres {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Numres: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

// RESERVEDSEGMENTS
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

// RESERVEDSEGMENT
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

// UDHDL
#[derive(Default, Clone, Hash, Debug)]
pub struct Udhdl {
    pub val: [u8; 5],
}
impl Display for Udhdl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Udhdl: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

// XHDL
#[derive(Default, Clone, Hash, Debug)]
pub struct Xhdl {
    pub val: [u8; 5],
}
impl Display for Xhdl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Xhdl: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}
