use super::{IdxXyzPoly, XyzPoly};
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Position {
    #[serde(rename = "ARPPoly")]
    pub arp_poly: XyzPoly,
    #[serde(rename = "GRPPoly")]
    pub grp_poly: Option<XyzPoly>,
    #[serde(rename = "TxAPCPoly")]
    pub tx_apc_poly: Option<XyzPoly>,
    #[serde(rename = "RcvApc")]
    pub rcv_apc: Option<RcvAPC>,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct RcvAPC {
    #[serde(rename = "@size")]
    pub size: usize,
    #[serde(rename = "RcvAPCPoly")]
    pub rcv_apc_poly: Vec<IdxXyzPoly>,
}
