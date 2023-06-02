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

#[cfg(test)]
mod tests {
    use super::Position;
    use serde_xml_rs::from_str;

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
                Ok(_) => true, Err(_) => false,
            })     
        }
}