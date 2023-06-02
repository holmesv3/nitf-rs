use serde::Deserialize;
use std::ops::Index;
use super::Poly1D;

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Timeline {
    pub CollectStart: String,
    pub CollectDuration: f64,
    pub IPP: Option<IppParams>,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct IppParams {
    pub Set: Vec<IppSet>,
}
impl Index<usize> for IppParams {
    type Output = IppSet;
    fn index(&self, index: usize) -> &Self::Output {
        &self.Set[index]
    }
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct IppSet {
    pub TStart: f64,
    pub TEnd: f64,
    pub IPPStart: u64,
    pub IPPEnd: u64,
    pub IPPPoly: Poly1D,
}

#[cfg(test)]
mod tests {
    use super::Timeline;
    use serde_xml_rs::from_str;
    #[test]
    fn test_timeline () {
        let xml_str = r#"<Timeline><CollectStart>0</CollectStart>
            <CollectDuration>0</CollectDuration><IPP size="1"><Set index="1">
            <TStart>0</TStart><TEnd>0</TEnd><IPPStart>0</IPPStart><IPPEnd>0
            </IPPEnd><IPPPoly order1="0"><Coef exponent1="0">0</Coef></IPPPoly>
            </Set></IPP></Timeline>"#;
        assert!(match from_str::<Timeline>(&xml_str) {
            Ok(_) => true, Err(_) => false,
        }) 
    }
}
