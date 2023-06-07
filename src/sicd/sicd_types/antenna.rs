use super::{Poly1D, Poly2D, XyzPoly};
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Antenna {
    #[serde(rename = "Tx")]
    pub tx: Option<AntennaType>,
    #[serde(rename = "Rcv")]
    pub rcv: Option<AntennaType>,
    #[serde(rename = "TwoWay")]
    pub two_way: Option<AntennaType>,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct AntennaType {
    #[serde(rename = "XAxisPoly")]
    pub x_axis_poly: XyzPoly,
    #[serde(rename = "YAxisPoly")]
    pub y_axis_poly: XyzPoly,
    #[serde(rename = "FreqZero")]
    pub freq_zero: f64,
    #[serde(rename = "EB")]
    pub eb: Option<EB>,
    #[serde(rename = "Array")]
    pub array: Array,
    #[serde(rename = "Elem")]
    pub elem: Option<Elem>,
    #[serde(rename = "GainBSPoly")]
    pub gain_bs_poly: Option<Poly1D>,
    #[serde(rename = "EBFreqShift")]
    pub eb_freq_shift: Option<bool>,
    #[serde(rename = "MLFreqDilation")]
    pub ml_freq_dilation: Option<bool>,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct EB {
    #[serde(rename = "DCXPoly")]
    pub dcx_poly: Poly1D,
    #[serde(rename = "DCYPoly")]
    pub dcy_poly: Poly1D,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Array {
    #[serde(rename = "GainPoly")]
    pub gain_poly: Poly2D,
    #[serde(rename = "PhasePoly")]
    pub phase_poly: Poly2D,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Elem {
    #[serde(rename = "GainPoly")]
    pub gain_poly: Poly2D,
    #[serde(rename = "PhasePoly")]
    pub phase_poly: Poly2D,
}

#[cfg(test)]
mod tests {
    use super::Antenna;
    use serde_xml_rs::from_str;
    #[test]
    fn test_antenna() {
        let xml_str = r#"<Antenna><Tx><XAxisPoly><X order1="1">
            <Coef exponent1="0">0</Coef><Coef exponent1="1">0</Coef></X>
            <Y order1="1"><Coef exponent1="0">0</Coef><Coef exponent1="1">
            0</Coef></Y><Z order1="1"><Coef exponent1="0">0</Coef>
            <Coef exponent1="1">0</Coef></Z></XAxisPoly><YAxisPoly>
            <X order1="1"><Coef exponent1="0">0</Coef><Coef exponent1="1">0
            </Coef></X><Y order1="1"><Coef exponent1="0">0</Coef>
            <Coef exponent1="1">0</Coef></Y><Z order1="1"><Coef exponent1="0">0
            </Coef><Coef exponent1="1">0</Coef></Z></YAxisPoly><FreqZero>0
            </FreqZero><EB><DCXPoly order1="0"><Coef exponent1="0">0</Coef>
            </DCXPoly><DCYPoly order1="0"><Coef exponent1="0">0</Coef></DCYPoly>
            </EB><Array><GainPoly order1="1" order2="1">
            <Coef exponent1="0" exponent2="0">0</Coef>
            <Coef exponent1="0" exponent2="1">0</Coef>
            <Coef exponent1="1" exponent2="0">0</Coef>
            <Coef exponent1="1" exponent2="1">0</Coef></GainPoly>
            <PhasePoly order1="1" order2="1"><Coef exponent1="0" exponent2="0">0
            </Coef><Coef exponent1="0" exponent2="1">0</Coef>
            <Coef exponent1="1" exponent2="0">0</Coef>
            <Coef exponent1="1" exponent2="1">0</Coef></PhasePoly></Array><Elem>
            <GainPoly order1="0" order2="0"><Coef exponent1="0" exponent2="0">0
            </Coef></GainPoly><PhasePoly order1="0" order2="0">
            <Coef exponent1="0" exponent2="0">0</Coef></PhasePoly></Elem>
            <GainBSPoly order1="0"><Coef exponent1="0">0</Coef></GainBSPoly>
            <EBFreqShift>false</EBFreqShift><MLFreqDilation>true
            </MLFreqDilation></Tx></Antenna>"#;
        assert!(match from_str::<Antenna>(&xml_str) {
            Ok(_) => true,
            Err(_) => false,
        })
    }
}
