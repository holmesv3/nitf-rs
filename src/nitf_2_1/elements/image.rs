//! Field definitions for NITF image segment subheader
use std::fmt::Display;

#[derive(Default, Clone, Hash, Debug)]
pub struct FilePartType {
    pub val: [u8; 2],
}
impl Display for FilePartType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FilePart: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

#[derive(Default, Clone, Hash, Debug)]
pub struct Iid1 {
    pub val: [u8; 10],
}
impl Display for Iid1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Iid1: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

#[derive(Default, Clone, Hash, Debug)]
pub struct IDaTim {
    pub val: [u8; 14],
}
impl Display for IDaTim {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "IDaTim: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

#[derive(Default, Clone, Hash, Debug)]
pub struct TgtID {
    pub val: [u8; 17],
}
impl Display for TgtID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TgtID: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

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

#[derive(Clone, Hash, Debug)]
pub struct ISClas {
    pub val: [u8; 1],
}
impl Display for ISClas {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ISClas: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

#[derive(Default, Clone, Hash, Debug)]
pub struct ISClSy {
    pub val: [u8; 2],
}
impl Display for ISClSy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ISClSy: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

#[derive(Default, Clone, Hash, Debug)]
pub struct ISCode {
    pub val: [u8; 11],
}
impl Display for ISCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ISCode: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

#[derive(Default, Clone, Hash, Debug)]
pub struct ISCtlH {
    pub val: [u8; 2],
}
impl Display for ISCtlH {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ISCtlH: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

#[derive(Default, Clone, Hash, Debug)]
pub struct ISRel {
    pub val: [u8; 20],
}
impl Display for ISRel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ISRel: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

#[derive(Default, Clone, Hash, Debug)]
pub struct ISDcTp {
    pub val: [u8; 2],
}
impl Display for ISDcTp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ISDcTp: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

#[derive(Default, Clone, Hash, Debug)]
pub struct IsDcDt {
    pub val: [u8; 8],
}
impl Display for IsDcDt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "IsDcDt: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

#[derive(Default, Clone, Hash, Debug)]
pub struct IsDcXm {
    pub val: [u8; 4],
}
impl Display for IsDcXm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "IsDcXm: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

#[derive(Default, Clone, Hash, Debug)]
pub struct IsDg {
    pub val: [u8; 1],
}
impl Display for IsDg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "IsDg: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

#[derive(Default, Clone, Hash, Debug)]
pub struct IsDgDt {
    pub val: [u8; 8],
}
impl Display for IsDgDt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "IsDgDt: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

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

#[derive(Clone, Hash, Debug)]
pub struct ISCATp {
    pub val: [u8; 1],
}
impl Display for ISCATp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ISCATp: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

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

#[derive(Default, Clone, Hash, Debug)]
pub struct ISCRsn {
    pub val: [u8; 1],
}
impl Display for ISCRsn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ISCRsn: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

#[derive(Default, Clone, Hash, Debug)]
pub struct ISSrDt {
    pub val: [u8; 8],
}
impl Display for ISSrDt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ISSrDt: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

#[derive(Default, Clone, Hash, Debug)]
pub struct ISCtlN {
    pub val: [u8; 15],
}
impl Display for ISCtlN {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ISCtlN: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

#[derive(Default, Clone, Hash, Debug)]
pub struct ENCRYP {
    pub val: [u8; 1],
}
impl Display for ENCRYP {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ENCRYP: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

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

#[derive(Default, Clone, Hash, Debug)]
pub struct NRows {
    pub val: [u8; 8],
}
impl Display for NRows {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NRows: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

#[derive(Default, Clone, Hash, Debug)]
pub struct NCols {
    pub val: [u8; 8],
}
impl Display for NCols {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NCols: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

#[derive(Default, Clone, Hash, Debug)]
pub struct PVType {
    pub val: [u8; 3],
}
impl Display for PVType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PVType: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

#[derive(Default, Clone, Hash, Debug)]
pub struct IRep {
    pub val: [u8; 8],
}
impl Display for IRep {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "IRep: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

// End of pg 45