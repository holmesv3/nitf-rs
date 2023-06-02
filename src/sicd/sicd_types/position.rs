use serde::Deserialize;
use super::{Poly1D, XyzPoly};

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Position {
    pub ARPPoly: XyzPoly,
    pub GRPPoly: Option<XyzPoly>,
    pub TxAPCPoly: Option<XyzPoly>,
    pub RcvApc: Option<RcvAPC>,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct RcvAPC {
    pub size: usize,
    pub RcvAPCPoly: RcvAPCPoly,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct RcvAPCPoly {
    pub index: usize,
    pub X: Poly1D,
    pub Y: Poly1D,
    pub Z: Poly1D,
}
