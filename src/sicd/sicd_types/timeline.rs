use super::Poly1d;
use serde::Deserialize;
use std::ops::Index;

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Timeline {
    #[serde(rename = "CollectStart")]
    pub collect_start: String,
    #[serde(rename = "CollectDuration")]
    pub collect_duration: f64,
    #[serde(rename = "IPP")]
    pub ipp: Option<IppParams>,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct IppParams {
    #[serde(rename = "Set")]
    pub set: Vec<IppSet>,
}
impl Index<usize> for IppParams {
    type Output = IppSet;
    fn index(&self, index: usize) -> &Self::Output {
        &self.set[index]
    }
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct IppSet {
    #[serde(rename = "TStart")]
    pub t_start: f64,
    #[serde(rename = "TEnd")]
    pub t_end: f64,
    #[serde(rename = "IPPStart")]
    pub ipp_start: u64,
    #[serde(rename = "IPPEnd")]
    pub ipp_end: u64,
    #[serde(rename = "IPPPoly")]
    pub ipp_poly: Poly1d,
}

#[cfg(test)]
mod tests {
    use super::Timeline;
    use quick_xml::de::from_str;
    #[test]
    fn test_timeline() {
        let xml_str = r#"<Timeline><CollectStart>0</CollectStart>
            <CollectDuration>0</CollectDuration><IPP size="1"><Set index="1">
            <TStart>0</TStart><TEnd>0</TEnd><IPPStart>0</IPPStart><IPPEnd>0
            </IPPEnd><IPPPoly order1="0"><Coef exponent1="0">0</Coef></IPPPoly>
            </Set></IPP></Timeline>"#;
        assert!(match from_str::<Timeline>(&xml_str) {
            Ok(_) => true,
            Err(_) => false,
        })
    }
}
