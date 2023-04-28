use std::fmt::Display;

/// File Part Type 
#[derive(Default, Clone, Hash, Debug)]
pub struct TE {
    pub val: [u8; 2]
}
impl Display for TE {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TE: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// Text Identifier 
#[derive(Default, Clone, Hash, Debug)]
pub struct TEXTID {
    pub val: [u8; 7]
}
impl Display for TEXTID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TEXTID: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// Text Attachment Level 
#[derive(Default, Clone, Hash, Debug)]
pub struct TXTALVL {
    pub val: [u8; 3]
}
impl Display for TXTALVL {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TXTALVL: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// Text Date and Time 
#[derive(Default, Clone, Hash, Debug)]
pub struct TXTDT {
    pub val: [u8; 14]
}
impl Display for TXTDT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TXTDT: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// Text Title 
#[derive(Clone, Hash, Debug)]
pub struct TXTTITL {
    pub val: [u8; 80]
}
impl Display for TXTTITL {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TXTTITL: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}
impl Default for TXTTITL {
    fn default() -> Self {
        Self { val: [0u8; 80] }
        
    }
}

/// Text Security Classification 
#[derive(Default, Clone, Hash, Debug)]
pub struct TSCLAS {
    pub val: [u8; 1]
}
impl Display for TSCLAS {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TSCLAS: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// Text Classification Security System 
#[derive(Default, Clone, Hash, Debug)]
pub struct TSCLSY {
    pub val: [u8; 2]
}
impl Display for TSCLSY {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TSCLSY: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// Text Codewords 
#[derive(Default, Clone, Hash, Debug)]
pub struct TSCODE {
    pub val: [u8; 11]
}
impl Display for TSCODE {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TSCODE: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// Text Control and Handling 
#[derive(Default, Clone, Hash, Debug)]
pub struct TSCTLH {
    pub val: [u8; 2]
}
impl Display for TSCTLH {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TSCTLH: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// Text Releasing Instructions 
#[derive(Default, Clone, Hash, Debug)]
pub struct TSREL {
    pub val: [u8; 20]
}
impl Display for TSREL {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TSREL: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// Text Declassification Type 
#[derive(Default, Clone, Hash, Debug)]
pub struct TSDCTP {
    pub val: [u8; 2]
}
impl Display for TSDCTP {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TSDCTP: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// Text Declassification Date 
#[derive(Default, Clone, Hash, Debug)]
pub struct TSDCDT {
    pub val: [u8; 8]
}
impl Display for TSDCDT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TSDCDT: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// Text Declassification Exemption 
#[derive(Default, Clone, Hash, Debug)]
pub struct TSDCXM {
    pub val: [u8; 4]
}
impl Display for TSDCXM {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TSDCXM: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// Text Downgrade 
#[derive(Default, Clone, Hash, Debug)]
pub struct TSDG {
    pub val: [u8; 1]
}
impl Display for TSDG {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TSDG: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// Text Downgrade Date 
#[derive(Default, Clone, Hash, Debug)]
pub struct TSDGDT {
    pub val: [u8; 8]
}
impl Display for TSDGDT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TSDGDT: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// Text Classification Text 
#[derive(Clone, Hash, Debug)]
pub struct TSCLTTX {
    pub val: [u8; 43]
}
impl Display for TSCLTTX {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TSCLTTX: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}
impl Default for TSCLTTX {
    fn default() -> Self {
        Self { val: [0u8; 43] }
        
    }
}

/// Text Classification Authority Type 
#[derive(Default, Clone, Hash, Debug)]
pub struct TSCATP {
    pub val: [u8; 1]
}
impl Display for TSCATP {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TSCATP: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// Text Classification Authority 
#[derive(Clone, Hash, Debug)]
pub struct TSCAUT {
    pub val: [u8; 40]
}
impl Display for TSCAUT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TSCAUT: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}
impl Default for TSCAUT {
    fn default() -> Self {
        Self { val: [0u8; 40] }
        
    }
}

/// Text Classification Reason 
#[derive(Default, Clone, Hash, Debug)]
pub struct TSCSN {
    pub val: [u8; 1]
}
impl Display for TSCSN {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TSCSN: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// Text Security Source Date 
#[derive(Default, Clone, Hash, Debug)]
pub struct TSSRDT {
    pub val: [u8; 8]
}
impl Display for TSSRDT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TSSRDT: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// Text Security Control Number 
#[derive(Default, Clone, Hash, Debug)]
pub struct TSCTLN {
    pub val: [u8; 15]
}
impl Display for TSCTLN {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TSCTLN: {}", String::from_utf8(self.val.to_vec()).unwrap())
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

/// Text Format 
#[derive(Default, Clone, Hash, Debug)]
pub struct TXTFMT {
    pub val: [u8; 3]
}
impl Display for TXTFMT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TXTFMT: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}

/// Text Extended Subheader Data Length 
#[derive(Default, Clone, Hash, Debug)]
pub struct TXSHDL {
    pub val: [u8; 5]
}
impl Display for TXSHDL {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TXSHDL: {}", String::from_utf8(self.val.to_vec()).unwrap())
    }
}
