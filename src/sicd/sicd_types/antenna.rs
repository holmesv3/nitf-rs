
use serde::Deserialize;
use super::{XyzPoly, Poly1D, Poly2D};


#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Antenna {
    pub Tx: Option<AntennaType>,
    pub Rcv: Option<AntennaType>,
    pub TwoWay: Option<AntennaType>,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct AntennaType {
    pub XAxisPoly: XyzPoly,
    pub YAxisPoly: XyzPoly,
    pub FreqZero: f64,
    pub EB: Option<EB>,
    pub Array: Array,
    pub Elem: Option<Elem>,
    pub GainBSPoly: Option<Poly1D>,
    pub EBFreqShift: Option<bool>,
    pub MLFreqDilation: Option<bool>,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct EB {
    pub DCXPoly: Poly1D,
    pub DCYPoly: Poly1D,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Array {
    pub GainPoly: Poly2D,
    pub PhasePoly: Poly2D,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Elem {
    pub GainPoly: Poly2D,
    pub PhasePoly: Poly2D,
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
            Ok(_) => true, Err(_) => false
        })
    }
}