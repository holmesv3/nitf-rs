use std::fmt::Display;

#[derive(Default, Clone, Hash, Debug)]
pub struct Sy {
    pub val: [u8; 2]
}
impl Display for Sy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

#[derive(Default, Clone, Hash, Debug)]
pub struct SId {
    pub val: [u8;10]
}
impl Display for SId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

#[derive(Default, Clone, Hash, Debug)]
pub struct SName {
    pub val: [u8; 20]
}
impl Display for SName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

#[derive(Default, Clone, Hash, Debug)]
pub struct SSClas {
    pub val: [u8; 1]
}
impl Display for SSClas {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

#[derive(Default, Clone, Hash, Debug)]
pub struct SSClSy {
    pub val: [u8; 2]
}
impl Display for SSClSy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

#[derive(Default, Clone, Hash, Debug)]
pub struct SSCode {
    pub val: [u8; 11]
}
impl Display for SSCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

#[derive(Default, Clone, Hash, Debug)]
pub struct SSCtlH {
    pub val: [u8; 2]
}
impl Display for SSCtlH {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

#[derive(Default, Clone, Hash, Debug)]
pub struct SSRel {
    pub val: [u8; 20]
}
impl Display for SSRel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

#[derive(Default, Clone, Hash, Debug)]
pub struct SSDcTp {
    pub val: [u8; 2]
}
impl Display for SSDcTp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

#[derive(Default, Clone, Hash, Debug)]
pub struct SSDcDt {
    pub val: [u8; 8]
}
impl Display for SSDcDt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

#[derive(Default, Clone, Hash, Debug)]
pub struct SSDcXm {
    pub val: [u8; 4]
}
impl Display for SSDcXm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

#[derive(Default, Clone, Hash, Debug)]
pub struct SSDg {
    pub val: [u8; 1]
}
impl Display for SSDg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

#[derive(Default, Clone, Hash, Debug)]
pub struct SSDgDt {
    pub val: [u8; 8]
}
impl Display for SSDgDt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

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

#[derive(Default, Clone, Hash, Debug)]
pub struct SSCATp {
    pub val: [u8; 1]
}
impl Display for SSCATp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

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

#[derive(Default, Clone, Hash, Debug)]
pub struct SSCRsn {
    pub val: [u8; 1]
}
impl Display for SSCRsn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

#[derive(Default, Clone, Hash, Debug)]
pub struct SSSrDt {
    pub val: [u8; 8]
}
impl Display for SSSrDt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

#[derive(Default, Clone, Hash, Debug)]
pub struct Ssctln {
    pub val: [u8; 15]
}
impl Display for Ssctln {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

#[derive(Default, Clone, Hash, Debug)]
pub struct Encryp {
    pub val: [u8; 1]
}
impl Display for Encryp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

#[derive(Default, Clone, Hash, Debug)]
pub struct SFmt {
    pub val: [u8; 1]
}
impl Display for SFmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

#[derive(Default, Clone, Hash, Debug)]
pub struct SStruct {
    pub val: [u8; 13]
}
impl Display for SStruct {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

#[derive(Default, Clone, Hash, Debug)]
pub struct SDLvl {
    pub val: [u8; 3]
}
impl Display for SDLvl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

#[derive(Default, Clone, Hash, Debug)]
pub struct SALvl {
    pub val: [u8; 3]
}
impl Display for SALvl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

#[derive(Default, Clone, Hash, Debug)]
pub struct SLoc {
    pub val: [u8; 10]
}
impl Display for SLoc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

#[derive(Default, Clone, Hash, Debug)]
pub struct SBnd1 {
    pub val: [u8; 10]
}
impl Display for SBnd1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

#[derive(Default, Clone, Hash, Debug)]
pub struct SColor {
    pub val: [u8; 1]
}
impl Display for SColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

#[derive(Default, Clone, Hash, Debug)]
pub struct SBnd2 {
    pub val: [u8; 10]
}
impl Display for SBnd2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

#[derive(Default, Clone, Hash, Debug)]
pub struct SRes2 {
    pub val: [u8; 2]
}
impl Display for SRes2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

#[derive(Default, Clone, Hash, Debug)]
pub struct SXSHDL {
    pub val: [u8; 5]
}
impl Display for SXSHDL {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}