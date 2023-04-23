use std::fmt::Display;

// File Part Type 
#[derive(Default, Clone, Hash, Debug)]
pub struct TE {
    pub val: [u8; 2]
}
impl Display for TE {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TE: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

// Text Identifier 
#[derive(Default, Clone, Hash, Debug)]
pub struct TextID {
    pub val: [u8; 7]
}
impl Display for TextID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TextID: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

// Text Attachment Level 
#[derive(Default, Clone, Hash, Debug)]
pub struct TxtALvl {
    pub val: [u8; 3]
}
impl Display for TxtALvl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TxtALvl: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

// Text Date and Time 
#[derive(Default, Clone, Hash, Debug)]
pub struct TxtDT {
    pub val: [u8; 14]
}
impl Display for TxtDT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TxtDT: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

// Text Title 
#[derive(Clone, Hash, Debug)]
pub struct TxtTitl {
    pub val: [u8; 80]
}
impl Display for TxtTitl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TxtTitl: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}
impl Default for TxtTitl {
    fn default() -> Self {
        Self { val: [0u8; 80] }
        
    }
}

// Text Security Classification 
#[derive(Default, Clone, Hash, Debug)]
pub struct TSClas {
    pub val: [u8; 1]
}
impl Display for TSClas {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TSClas: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

// Text Classification Security System 
#[derive(Default, Clone, Hash, Debug)]
pub struct TSClSy {
    pub val: [u8; 2]
}
impl Display for TSClSy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TSClSy: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

// Text Codewords 
#[derive(Default, Clone, Hash, Debug)]
pub struct TSCode {
    pub val: [u8; 11]
}
impl Display for TSCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TSCode: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

// Text Control and Handling 
#[derive(Default, Clone, Hash, Debug)]
pub struct TSCtlH {
    pub val: [u8; 2]
}
impl Display for TSCtlH {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TSCtlH: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

// Text Releasing Instructions 
#[derive(Default, Clone, Hash, Debug)]
pub struct TSRel {
    pub val: [u8; 20]
}
impl Display for TSRel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TSRel: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

// Text Declassification Type 
#[derive(Default, Clone, Hash, Debug)]
pub struct TSDcTp {
    pub val: [u8; 2]
}
impl Display for TSDcTp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TSDcTp: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

// Text Declassification Date 
#[derive(Default, Clone, Hash, Debug)]
pub struct TSDcDt {
    pub val: [u8; 8]
}
impl Display for TSDcDt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TSDcDt: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

// Text Declassification Exemption 
#[derive(Default, Clone, Hash, Debug)]
pub struct TSDcXm {
    pub val: [u8; 4]
}
impl Display for TSDcXm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TSDcXm: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

// Text Downgrade 
#[derive(Default, Clone, Hash, Debug)]
pub struct TSDg {
    pub val: [u8; 1]
}
impl Display for TSDg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TSDg: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

// Text Downgrade Date 
#[derive(Default, Clone, Hash, Debug)]
pub struct TSDgDt {
    pub val: [u8; 8]
}
impl Display for TSDgDt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TSDgDt: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

// Text Classification Text 
#[derive(Clone, Hash, Debug)]
pub struct TSCltTx {
    pub val: [u8; 43]
}
impl Display for TSCltTx {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TSCltTx: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}
impl Default for TSCltTx {
    fn default() -> Self {
        Self { val: [0u8; 43] }
        
    }
}

// Text Classification Authority Type 
#[derive(Default, Clone, Hash, Debug)]
pub struct TSCATp {
    pub val: [u8; 1]
}
impl Display for TSCATp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TSCATp: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

// Text Classification Authority 
#[derive(Clone, Hash, Debug)]
pub struct TSCAut {
    pub val: [u8; 40]
}
impl Display for TSCAut {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TSCAut: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}
impl Default for TSCAut {
    fn default() -> Self {
        Self { val: [0u8; 40] }
        
    }
}

// Text Classification Reason 
#[derive(Default, Clone, Hash, Debug)]
pub struct TSCsn {
    pub val: [u8; 1]
}
impl Display for TSCsn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TSCsn: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

// Text Security Source Date 
#[derive(Default, Clone, Hash, Debug)]
pub struct TSSrDt {
    pub val: [u8; 8]
}
impl Display for TSSrDt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TSSrDt: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

// Text Security Control Number 
#[derive(Default, Clone, Hash, Debug)]
pub struct TSCtlN {
    pub val: [u8; 15]
}
impl Display for TSCtlN {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TSCtlN: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

// Encryption 
#[derive(Default, Clone, Hash, Debug)]
pub struct Encryp {
    pub val: [u8; 1]
}
impl Display for Encryp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Encryp: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

// Text Format 
#[derive(Default, Clone, Hash, Debug)]
pub struct TxtFmt {
    pub val: [u8; 3]
}
impl Display for TxtFmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TxtFmt: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

// Text Extended Subheader Data Length 
#[derive(Default, Clone, Hash, Debug)]
pub struct TXSHDL {
    pub val: [u8; 5]
}
impl Display for TXSHDL {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TXSHDL: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}
