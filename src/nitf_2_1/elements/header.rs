//! Field definitions for NITF header
use std::fmt::Display;

use super::NitfElement;
/// File Profile Name
#[derive(Default, Clone, Hash, Debug)]
pub struct FHDR (pub NitfElement);

/// File Version
#[derive(Default, Clone, Hash, Debug)]
pub struct FVER ( pub [u8; 5]);
impl Display for FVER {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FVER: {}", String::from_utf8(self.0.to_vec()).unwrap())
    }
}

/// Complexity Level
#[derive(Default, Clone, Hash, Debug)]
pub struct CLEVEL ( pub [u8; 2]);
impl Display for CLEVEL {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CLEVEL: {}", String::from_utf8(self.0.to_vec()).unwrap())
    }
}

/// Standard Type
#[derive(Default, Clone, Hash, Debug)]
pub struct STYPE ( pub [u8; 4]);
impl Display for STYPE {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "STYPE: {}", String::from_utf8(self.0.to_vec()).unwrap())
    }
}

/// Originating Station ID
#[derive(Default, Clone, Hash, Debug)]
pub struct OSTAID ( pub [u8; 10]);
impl Display for OSTAID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "OSTAID: {}", String::from_utf8(self.0.to_vec()).unwrap())
    }
}

/// File Date and Time
#[derive(Default, Clone, Hash, Debug)]
pub struct FDT ( pub [u8; 14]);
impl Display for FDT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FDT: {}", String::from_utf8(self.0.to_vec()).unwrap())
    }
}

/// File Title
#[derive(Clone, Hash, Debug)]
pub struct FTITLE ( pub [u8; 80]);impl Display for FTITLE {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FTITLE: {}", String::from_utf8(self.0.to_vec()).unwrap())
    }
}impl Default for FTITLE {
    fn default() -> Self {
        Self { 0: [0u8; 80] }
    }
}


/// File Security Classification
#[derive(Default, Clone, Hash, Debug)]
pub struct FSCLAS ( pub [u8; 1]);
impl Display for FSCLAS {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FSCLAS: {}", String::from_utf8(self.0.to_vec()).unwrap())
    }
}

/// File Classification Security System
#[derive(Default, Clone, Hash, Debug)]
pub struct FSCLSY ( pub [u8; 2]);
impl Display for FSCLSY {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FSCLSY: {}", String::from_utf8(self.0.to_vec()).unwrap())
    }
}

/// File Codewords
#[derive(Default, Clone, Hash, Debug)]
pub struct FSCODE ( pub [u8; 11]);
impl Display for FSCODE {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FSCODE: {}", String::from_utf8(self.0.to_vec()).unwrap())
    }
}

/// File Control and Handling
#[derive(Default, Clone, Hash, Debug)]
pub struct FSCTLH ( pub [u8; 2]);
impl Display for FSCTLH {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FSCTLH: {}", String::from_utf8(self.0.to_vec()).unwrap())
    }
}

/// File Releasing Instructions
#[derive(Default, Clone, Hash, Debug)]
pub struct FSREL ( pub [u8; 20]);
impl Display for FSREL {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FSREL: {}", String::from_utf8(self.0.to_vec()).unwrap())
    }
}

/// File Declassification Type
#[derive(Default, Clone, Hash, Debug)]
pub struct FSDCTP ( pub [u8; 2]);
impl Display for FSDCTP {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FSDCTP: {}", String::from_utf8(self.0.to_vec()).unwrap())
    }
}

/// File Declassification Date
#[derive(Default, Clone, Hash, Debug)]
pub struct FSDCDT ( pub [u8; 8]);
impl Display for FSDCDT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FSDCDT: {}", String::from_utf8(self.0.to_vec()).unwrap())
    }
}

/// File Declassification Exemption
#[derive(Default, Clone, Hash, Debug)]
pub struct FSDCXM ( pub [u8; 4]);
impl Display for FSDCXM {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FSDCXM: {}", String::from_utf8(self.0.to_vec()).unwrap())
    }
}

/// File Downgrade
#[derive(Default, Clone, Hash, Debug)]
pub struct FSDG ( pub [u8; 1]);
impl Display for FSDG {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FSDG: {}", String::from_utf8(self.0.to_vec()).unwrap())
    }
}

/// File Downgrade Date
#[derive(Default, Clone, Hash, Debug)]
pub struct FSDGDT ( pub [u8; 8]);
impl Display for FSDGDT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FSDGDT: {}", String::from_utf8(self.0.to_vec()).unwrap())
    }
}

/// File Classification Text
#[derive(Clone, Hash, Debug)]
pub struct FSCLTX ( pub [u8; 43]);
impl Display for FSCLTX {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FSCLTX: {}", String::from_utf8(self.0.to_vec()).unwrap())
    }
}
impl Default for FSCLTX {
    fn default() -> Self {
        Self { 0: ([0u8; 43]) }
    }
}

/// File Classification Authority Type
#[derive(Default, Clone, Hash, Debug)]
pub struct FSCATP ( pub [u8; 1]);
impl Display for FSCATP {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FSCATP: {}", String::from_utf8(self.0.to_vec()).unwrap())
    }
}

/// File Classification Authority
#[derive(Clone, Hash, Debug)]
pub struct FSCAUT ( pub [u8; 40]);
impl Display for FSCAUT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FSCAUT: {}", String::from_utf8(self.0.to_vec()).unwrap())
    }
}

/// File Classification Reason
#[derive(Default, Clone, Hash, Debug)]
pub struct FSCRSN ( pub [u8; 1]);
impl Display for FSCRSN {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FSCRSN: {}", String::from_utf8(self.0.to_vec()).unwrap())
    }
}
impl Default for FSCAUT {
    fn default() -> Self {
        Self { 0: [0u8; 40] }
    }
}

/// File Security Source Date
#[derive(Default, Clone, Hash, Debug)]
pub struct FSSRDT ( pub [u8; 8]);
impl Display for FSSRDT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FSSRDT: {}", String::from_utf8(self.0.to_vec()).unwrap())
    }
}

/// File Security Control Number
#[derive(Default, Clone, Hash, Debug)]
pub struct FSCTLN ( pub [u8; 15]);
impl Display for FSCTLN {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FSCTLN: {}", String::from_utf8(self.0.to_vec()).unwrap())
    }
}

/// File Copy Number
#[derive(Default, Clone, Hash, Debug)]
pub struct FSCOP ( pub [u8; 5]);
impl Display for FSCOP {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FSCOP: {}", String::from_utf8(self.0.to_vec()).unwrap())
    }
}

/// File Number of Copies
#[derive(Default, Clone, Hash, Debug)]
pub struct FSCPYS ( pub [u8; 5]);
impl Display for FSCPYS {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FSCPYS: {}", String::from_utf8(self.0.to_vec()).unwrap())
    }
}

/// Encryption
#[derive(Default, Clone, Hash, Debug)]
pub struct ENCRYP ( pub [u8; 1]);
impl Display for ENCRYP {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ENCRYP: {}", String::from_utf8(self.0.to_vec()).unwrap())
    }
}

/// File Background Color
#[derive(Default, Clone, Hash, Debug)]
pub struct FBKGC ( pub [u8; 3]);
impl Display for FBKGC {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FBKGC: {}", String::from_utf8(self.0.to_vec()).unwrap())
    }
}

/// Originator's Name
#[derive(Default, Clone, Hash, Debug)]
pub struct ONAME ( pub [u8; 24]);
impl Display for ONAME {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ONAME: {}", String::from_utf8(self.0.to_vec()).unwrap())
    }
}

/// Originator's Phone Number
#[derive(Default, Clone, Hash, Debug)]
pub struct OPHONE ( pub [u8; 18]);
impl Display for OPHONE {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "OPHONE: {}", String::from_utf8(self.0.to_vec()).unwrap())
    }
}

/// File Length
#[derive(Default, Clone, Hash, Debug)]
pub struct FL ( pub [u8; 12]);
impl Display for FL {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FL: {}", String::from_utf8(self.0.to_vec()).unwrap())
    }
}

/// NITF File Header Length
#[derive(Default, Clone, Hash, Debug)]
pub struct HL ( pub [u8; 6]);
impl Display for HL {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "HL: {}", String::from_utf8(self.0.to_vec()).unwrap())
    }
}

/// Number of Image Segments
#[derive(Default, Clone, Hash, Debug)]
pub struct NUMIMAGESEGMENTS ( pub [u8; 3]);
impl Display for NUMIMAGESEGMENTS {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NUMIMAGESEGMENTS: {}", String::from_utf8(self.0.to_vec()).unwrap())
    }
}

/// Image Segments
#[derive(Default, Clone, Hash, Debug)]
pub struct IMAGESEGMENTS ( pub Vec<IMAGESEGMENTELEM>,);
impl Display for IMAGESEGMENTS {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out_str = String::default();
        for seg in self.0.iter() {
            out_str += format!("\t{}", seg).as_ref()
        }
        write!(f, "{}", out_str)
    }
}

#[derive(Default, Clone, Hash, Debug)]
pub struct IMAGESEGMENTELEM {
    pub subheader_size: [u8; 6],
    pub item_size: [u8; 10],
}
impl Display for IMAGESEGMENTELEM {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str1 = String::from_utf8(self.subheader_size.to_vec()).unwrap();
        let str2 = String::from_utf8(self.item_size.to_vec()).unwrap();
        write!(f, "[{}, {}]", str1, str2)
    }
}

/// Number of Graphics Segments
#[derive(Default, Clone, Hash, Debug)]
pub struct NUMGRAPHICSEGMENT ( pub [u8; 3]);
impl Display for NUMGRAPHICSEGMENT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NUMGRAPHICSEGMENT: {}", String::from_utf8(self.0.to_vec()).unwrap())
    }
}

/// Graphic Segments
#[derive(Default, Clone, Hash, Debug)]
pub struct GRAPHICSSEGMENTS ( pub Vec<GRAPHICSSEGMENTELEM>,);
impl Display for GRAPHICSSEGMENTS {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out_str = String::default();
        for seg in self.0.iter() {
            out_str += format!("\t{}", seg).as_ref()
        }
        write!(f, "{}", out_str)
    }
}

#[derive(Default, Clone, Hash, Debug)]
pub struct GRAPHICSSEGMENTELEM {
    pub subheader_size: [u8; 4],
    pub item_size: [u8; 6],
}
impl Display for GRAPHICSSEGMENTELEM {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str1 = String::from_utf8(self.subheader_size.to_vec()).unwrap();
        let str2 = String::from_utf8(self.item_size.to_vec()).unwrap();
        write!(f, "[{}, {}]", str1, str2)
    }
}

/// Reserved for future use
#[derive(Default, Clone, Hash, Debug)]
pub struct NUMX ( pub [u8; 3]);
impl Display for NUMX {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NUMX: {}", String::from_utf8(self.0.to_vec()).unwrap())
    }
}

/// Number of Text Files
#[derive(Default, Clone, Hash, Debug)]
pub struct NUMTEXTFILES ( pub [u8; 3]);
impl Display for NUMTEXTFILES {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NUMTEXTFILES: {}", String::from_utf8(self.0.to_vec()).unwrap())
    }
}

/// Text Segments
#[derive(Default, Clone, Hash, Debug)]
pub struct TEXTSEGMENTS ( pub Vec<TEXTSEGMENTELEM>,);
impl Display for TEXTSEGMENTS {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out_str = String::default();
        for seg in self.0.iter() {
            out_str += format!("\t{}", seg).as_ref()
        }
        write!(f, "{}", out_str)
    }
}

#[derive(Default, Clone, Hash, Debug)]
pub struct TEXTSEGMENTELEM {
    pub subheader_size: [u8; 4],
    pub item_size: [u8; 5],
}
impl Display for TEXTSEGMENTELEM {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str1 = String::from_utf8(self.subheader_size.to_vec()).unwrap();
        let str2 = String::from_utf8(self.item_size.to_vec()).unwrap();
        write!(f, "[{}, {}]", str1, str2)
    }
}

/// Number of Data Extension Segments
#[derive(Default, Clone, Hash, Debug)]
pub struct NUMDES ( pub [u8; 3]);
impl Display for NUMDES {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NUMDES: {}", String::from_utf8(self.0.to_vec()).unwrap())
    }
}

/// Data Extenstion Segments
#[derive(Default, Clone, Hash, Debug)]
pub struct DATAEXTSEGMENTS ( pub Vec<DATAEXTSEGMENTELEM>,);
impl Display for DATAEXTSEGMENTS {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out_str = String::default();
        for seg in self.0.iter() {
            out_str += format!("\t{}", seg).as_ref()
        }
        write!(f, "{}", out_str)
    }
}

#[derive(Default, Clone, Hash, Debug)]
pub struct DATAEXTSEGMENTELEM {
    pub subheader_size: [u8; 4],
    pub item_size: [u8; 9],
}
impl Display for DATAEXTSEGMENTELEM {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str1 = String::from_utf8(self.subheader_size.to_vec()).unwrap();
        let str2 = String::from_utf8(self.item_size.to_vec()).unwrap();
        write!(f, "[{}, {}]", str1, str2)
    }
}

/// Number of Reserved Extension Segments
#[derive(Default, Clone, Hash, Debug)]
pub struct NUMRES ( pub [u8; 3]);
impl Display for NUMRES {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NUMRES: {}", String::from_utf8(self.0.to_vec()).unwrap())
    }
}

/// Reserved Extension Segments
#[derive(Default, Clone, Hash, Debug)]
pub struct RESERVEDSEGMENTS ( pub Vec<RESERVEDSEGMENTELEM>,);
impl Display for RESERVEDSEGMENTS {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out_str = String::default();
        for seg in self.0.iter() {
            out_str += format!("\t{}", seg).as_ref()
        }
        write!(f, "{}", out_str)
    }
}

#[derive(Default, Clone, Hash, Debug)]
pub struct RESERVEDSEGMENTELEM {
    pub subheader_size: [u8; 4],
    pub item_size: [u8; 7],
}
impl Display for RESERVEDSEGMENTELEM {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str1 = String::from_utf8(self.subheader_size.to_vec()).unwrap();
        let str2 = String::from_utf8(self.item_size.to_vec()).unwrap();
        write!(f, "[{}, {}]", str1, str2)
    }
}

/// User Defined Header Data Length
#[derive(Default, Clone, Hash, Debug)]
pub struct UDHDL ( pub [u8; 5]);
impl Display for UDHDL {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "UDHDL: {}", String::from_utf8(self.0.to_vec()).unwrap())
    }
}

/// Extended Header Data Length
#[derive(Default, Clone, Hash, Debug)]
pub struct XHDL ( pub [u8; 5]);
impl Display for XHDL {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "XHDL: {}", String::from_utf8(self.0.to_vec()).unwrap())
    }
}


