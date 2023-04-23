use std::fmt::Display;

/// File Part Type
#[derive(Default, Clone, Hash, Debug)]
pub struct Sy {
    pub val: [u8; 2]
}
impl Display for Sy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// Graphic Identifier
#[derive(Default, Clone, Hash, Debug)]
pub struct SId {
    pub val: [u8;10]
}
impl Display for SId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// Graphic Name
#[derive(Default, Clone, Hash, Debug)]
pub struct SName {
    pub val: [u8; 20]
}
impl Display for SName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// Graphic Security Classification
#[derive(Default, Clone, Hash, Debug)]
pub struct SSClas {
    pub val: [u8; 1]
}
impl Display for SSClas {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// Graphic Classification Security System
#[derive(Default, Clone, Hash, Debug)]
pub struct SSClSy {
    pub val: [u8; 2]
}
impl Display for SSClSy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// Graphic Codewords
#[derive(Default, Clone, Hash, Debug)]
pub struct SSCode {
    pub val: [u8; 11]
}
impl Display for SSCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// Graphic Control and Handling
#[derive(Default, Clone, Hash, Debug)]
pub struct SSCtlH {
    pub val: [u8; 2]
}
impl Display for SSCtlH {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// Graphic Releasing Instructions
#[derive(Default, Clone, Hash, Debug)]
pub struct SSRel {
    pub val: [u8; 20]
}
impl Display for SSRel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// Graphic Declassification Type
#[derive(Default, Clone, Hash, Debug)]
pub struct SSDcTp {
    pub val: [u8; 2]
}
impl Display for SSDcTp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// Graphic Declassification Date
#[derive(Default, Clone, Hash, Debug)]
pub struct SSDcDt {
    pub val: [u8; 8]
}
impl Display for SSDcDt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// Graphic Declassification Exemption
#[derive(Default, Clone, Hash, Debug)]
pub struct SSDcXm {
    pub val: [u8; 4]
}
impl Display for SSDcXm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// Graphic Downgrade
#[derive(Default, Clone, Hash, Debug)]
pub struct SSDg {
    pub val: [u8; 1]
}
impl Display for SSDg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// Graphic Downgrade Date
#[derive(Default, Clone, Hash, Debug)]
pub struct SSDgDt {
    pub val: [u8; 8]
}
impl Display for SSDgDt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// Graphic Classification Text
#[derive(Clone, Hash, Debug)]
pub struct SSClTx {
    pub val: [u8; 43]
}
impl Display for SSClTx {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}
impl Default for SSClTx {
    fn default() -> Self {
        Self {val: [0u8; 43] }
    }
}

/// Graphic Classification Authority Type
#[derive(Default, Clone, Hash, Debug)]
pub struct SSCATp {
    pub val: [u8; 1]
}
impl Display for SSCATp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// Graphic Classification Authority
#[derive(Clone, Hash, Debug)]
pub struct SSCAut {
    pub val: [u8; 40]
}
impl Display for SSCAut {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}
impl Default for SSCAut {
    fn default() -> Self {
        Self {val: [0u8; 40] }
    }
}

/// Graphic Classification Reason
#[derive(Default, Clone, Hash, Debug)]
pub struct SSCRsn {
    pub val: [u8; 1]
}
impl Display for SSCRsn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// Graphic Security Source Date
#[derive(Default, Clone, Hash, Debug)]
pub struct SSSrDt {
    pub val: [u8; 8]
}
impl Display for SSSrDt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// Graphic Security Control Number
#[derive(Default, Clone, Hash, Debug)]
pub struct Ssctln {
    pub val: [u8; 15]
}
impl Display for Ssctln {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// Encryption
#[derive(Default, Clone, Hash, Debug)]
pub struct Encryp {
    pub val: [u8; 1]
}
impl Display for Encryp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// Graphic Type
#[derive(Default, Clone, Hash, Debug)]
pub struct SFmt {
    pub val: [u8; 1]
}
impl Display for SFmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// Reserved for Future Use
#[derive(Default, Clone, Hash, Debug)]
pub struct SStruct {
    pub val: [u8; 13]
}
impl Display for SStruct {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// Graphic Display Level
#[derive(Default, Clone, Hash, Debug)]
pub struct SDLvl {
    pub val: [u8; 3]
}
impl Display for SDLvl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// Graphic Attachment Level
#[derive(Default, Clone, Hash, Debug)]
pub struct SALvl {
    pub val: [u8; 3]
}
impl Display for SALvl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// Graphic Location
#[derive(Default, Clone, Hash, Debug)]
pub struct SLoc {
    pub val: [u8; 10]
}
impl Display for SLoc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// First Graphic Bound Location
#[derive(Default, Clone, Hash, Debug)]
pub struct SBnd1 {
    pub val: [u8; 10]
}
impl Display for SBnd1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// Graphic Color
#[derive(Default, Clone, Hash, Debug)]
pub struct SColor {
    pub val: [u8; 1]
}
impl Display for SColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// Second Graphic Bound Location
#[derive(Default, Clone, Hash, Debug)]
pub struct SBnd2 {
    pub val: [u8; 10]
}
impl Display for SBnd2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// Reserved for Future Use
#[derive(Default, Clone, Hash, Debug)]
pub struct SRes2 {
    pub val: [u8; 2]
}
impl Display for SRes2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// Graphic Extended Subheader Data Length
#[derive(Default, Clone, Hash, Debug)]
pub struct SXSHDL {
    pub val: [u8; 5]
}
impl Display for SXSHDL {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}