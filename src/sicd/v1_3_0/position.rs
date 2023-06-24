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

#[cfg(test)]
mod tests {
    use super::Position;
    use quick_xml::de::from_str;

    #[test]
    fn test_position() {
        let xml_str = r#"<Position><ARPPoly><X order1="0"><Coef exponent1="0">0
            </Coef></X><Y order1="0"><Coef exponent1="0">0</Coef></Y>
            <Z order1="0"><Coef exponent1="0">0</Coef></Z></ARPPoly><GRPPoly>
            <X order1="0"><Coef exponent1="0">0</Coef></X><Y order1="0">
            <Coef exponent1="0">0</Coef></Y><Z order1="0"><Coef exponent1="0">0
            </Coef></Z></GRPPoly><TxAPCPoly><X order1="0"><Coef exponent1="0">0
            </Coef></X><Y order1="0"><Coef exponent1="0">0</Coef></Y>
            <Z order1="0"><Coef exponent1="0">0</Coef></Z></TxAPCPoly>
            <RcvAPC size="1"><RcvAPCPoly index="1"><X order1="0">
            <Coef exponent1="0">0</Coef></X><Y order1="0"><Coef exponent1="0">0
            </Coef></Y><Z order1="0"><Coef exponent1="0">0</Coef></Z>
            </RcvAPCPoly></RcvAPC></Position>"#;
        assert!(match from_str::<Position>(&xml_str) {
            Ok(_) => true,
            Err(_) => false,
        })
    }
}
