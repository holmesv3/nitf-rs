use serde::Deserialize;
use super::{XYZ, Poly1D, Poly2D};

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Pfa {
    pub FPN: XYZ,
    pub IPN: XYZ,
    pub PolarAngRefTime: f64,
    pub PolarAngPoly: Poly1D,
    pub SpatialFreqSFPoly: Poly1D,
    pub Krg1: f64,
    pub Krg2: f64,
    pub Kaz1: f64,
    pub Kaz2: f64,
    pub STDeskew: Option<STDeskew>,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct STDeskew {
    pub Applied: bool,
    pub STDSPhasePoly: Poly2D,
}

#[cfg(test)]
mod tests {
    use super::Pfa;
    use serde_xml_rs::from_str;

    #[test]
    fn test_pfa() {
        let xml_str = r#"<PFA><FPN><X>0</X><Y>0</Y><Z>0</Z></FPN><IPN><X>0</X>
            <Y>0</Y><Z>0</Z></IPN><PolarAngRefTime>0</PolarAngRefTime>
            <PolarAngPoly order1="0"><Coef exponent1="0">0</Coef></PolarAngPoly>
            <SpatialFreqSFPoly order1="0"><Coef exponent1="0">0</Coef>
            </SpatialFreqSFPoly><Krg1>0</Krg1><Krg2>0</Krg2><Kaz1>0</Kaz1><Kaz2>
            0</Kaz2></PFA>"#;
        assert!(match from_str::<Pfa>(xml_str) {
            Ok(_) => true, Err(_) => false
        })
    }
}