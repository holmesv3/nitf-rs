use std::fmt::Display;

/// File Part Type
#[derive(Default, Clone, Hash, Debug)]
pub struct SY {
    pub val: [u8; 2]
}
impl Display for SY {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SY: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// Graphic Identifier
#[derive(Default, Clone, Hash, Debug)]
pub struct SID {
    pub val: [u8;10]
}
impl Display for SID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SID: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// Graphic Name
#[derive(Default, Clone, Hash, Debug)]
pub struct SNAME {
    pub val: [u8; 20]
}
impl Display for SNAME {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SNAME: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// Graphic Security Classification
#[derive(Default, Clone, Hash, Debug)]
pub struct SSCLAS {
    pub val: [u8; 1]
}
impl Display for SSCLAS {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SSCLAS: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// Graphic Classification Security System
#[derive(Default, Clone, Hash, Debug)]
pub struct SSCLSY {
    pub val: [u8; 2]
}
impl Display for SSCLSY {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SSCLSY: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// Graphic Codewords
#[derive(Default, Clone, Hash, Debug)]
pub struct SSCODE {
    pub val: [u8; 11]
}
impl Display for SSCODE {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SSCODE: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// Graphic Control and Handling
#[derive(Default, Clone, Hash, Debug)]
pub struct SSCTLH {
    pub val: [u8; 2]
}
impl Display for SSCTLH {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SSCTLH: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// Graphic Releasing Instructions
#[derive(Default, Clone, Hash, Debug)]
pub struct SSREL {
    pub val: [u8; 20]
}
impl Display for SSREL {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SSREL: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// Graphic Declassification Type
#[derive(Default, Clone, Hash, Debug)]
pub struct SSDCTP {
    pub val: [u8; 2]
}
impl Display for SSDCTP {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SSDCTP: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// Graphic Declassification Date
#[derive(Default, Clone, Hash, Debug)]
pub struct SSDCDT {
    pub val: [u8; 8]
}
impl Display for SSDCDT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SSDCDT: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// Graphic Declassification Exemption
#[derive(Default, Clone, Hash, Debug)]
pub struct SSDCXM {
    pub val: [u8; 4]
}
impl Display for SSDCXM {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SSDCXM: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// Graphic Downgrade
#[derive(Default, Clone, Hash, Debug)]
pub struct SSDG {
    pub val: [u8; 1]
}
impl Display for SSDG {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SSDG: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// Graphic Downgrade Date
#[derive(Default, Clone, Hash, Debug)]
pub struct SSDGDT {
    pub val: [u8; 8]
}
impl Display for SSDGDT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SSDGDT: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// Graphic Classification Text
#[derive(Clone, Hash, Debug)]
pub struct SSCLTX {
    pub val: [u8; 43]
}
impl Display for SSCLTX {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SSCLTX: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}
impl Default for SSCLTX {
    fn default() -> Self {
        Self {val: [0u8; 43] }
    }
}

/// Graphic Classification Authority Type
#[derive(Default, Clone, Hash, Debug)]
pub struct SSCATP {
    pub val: [u8; 1]
}
impl Display for SSCATP {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SSCATP: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// Graphic Classification Authority
#[derive(Clone, Hash, Debug)]
pub struct SSCAUT {
    pub val: [u8; 40]
}
impl Display for SSCAUT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SSCAUT: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}
impl Default for SSCAUT {
    fn default() -> Self {
        Self {val: [0u8; 40] }
    }
}

/// Graphic Classification Reason
#[derive(Default, Clone, Hash, Debug)]
pub struct SSCRSN {
    pub val: [u8; 1]
}
impl Display for SSCRSN {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SSCRSN: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// Graphic Security Source Date
#[derive(Default, Clone, Hash, Debug)]
pub struct SSSRDT {
    pub val: [u8; 8]
}
impl Display for SSSRDT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SSSRDT: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// Graphic Security Control Number
#[derive(Default, Clone, Hash, Debug)]
pub struct SSCTLN {
    pub val: [u8; 15]
}
impl Display for SSCTLN {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SSCTLN: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// Encryption
#[derive(Default, Clone, Hash, Debug)]
pub struct ENCRYP {
    pub val: [u8; 1]
}
impl Display for ENCRYP {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ENCRYP: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// Graphic Type
#[derive(Default, Clone, Hash, Debug)]
pub struct SFMT {
    pub val: [u8; 1]
}
impl Display for SFMT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SFMT: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// Reserved for Future Use
#[derive(Default, Clone, Hash, Debug)]
pub struct SSTRUCT {
    pub val: [u8; 13]
}
impl Display for SSTRUCT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SSTRUCT: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// Graphic Display Level
#[derive(Default, Clone, Hash, Debug)]
pub struct SDLVL {
    pub val: [u8; 3]
}
impl Display for SDLVL {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SDLVL: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// Graphic Attachment Level
#[derive(Default, Clone, Hash, Debug)]
pub struct SALVL {
    pub val: [u8; 3]
}
impl Display for SALVL {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SALVL: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// Graphic Location
#[derive(Default, Clone, Hash, Debug)]
pub struct SLOC {
    pub val: [u8; 10]
}
impl Display for SLOC {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SLOC: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// First Graphic Bound Location
#[derive(Default, Clone, Hash, Debug)]
pub struct SBND1 {
    pub val: [u8; 10]
}
impl Display for SBND1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SBND1: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// Graphic Color
#[derive(Default, Clone, Hash, Debug)]
pub struct SCOLOR {
    pub val: [u8; 1]
}
impl Display for SCOLOR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SCOLOR: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// Second Graphic Bound Location
#[derive(Default, Clone, Hash, Debug)]
pub struct SBND2 {
    pub val: [u8; 10]
}
impl Display for SBND2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SBND2: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// Reserved for Future Use
#[derive(Default, Clone, Hash, Debug)]
pub struct SRES2 {
    pub val: [u8; 2]
}
impl Display for SRES2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SRES2: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// Graphic Extended Subheader Data Length
#[derive(Default, Clone, Hash, Debug)]
pub struct SXSHDL {
    pub val: [u8; 5]
}
impl Display for SXSHDL {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SXSHDL: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}