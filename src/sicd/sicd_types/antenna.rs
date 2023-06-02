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
    
    
    #[test]
    fn test_antenna() {
        let antenna_str = r#"
            <Antenna>
            <Tx>
                <XAxisPoly>
                    <X order1="1">
                        <Coef exponent1="0">-0.043040129758302216</Coef>
                        <Coef exponent1="1">8.3894258665007037e-05</Coef>
                    </X>
                    <Y order1="1">
                        <Coef exponent1="0">0.86650241157354035</Coef>
                        <Coef exponent1="1">0.00041253146944526949</Coef>
                    </Y>
                    <Z order1="1">
                        <Coef exponent1="0">-0.49731414772296534</Coef>
                        <Coef exponent1="1">0.00071303340343231247</Coef>
                    </Z>
                </XAxisPoly>
                <YAxisPoly>
                    <X order1="1">
                        <Coef exponent1="0">-0.40691335445134125</Coef>
                        <Coef exponent1="1">0.0092656464455340044</Coef>
                    </X>
                    <Y order1="1">
                        <Coef exponent1="0">0.43942506084592903</Coef>
                        <Coef exponent1="1">0.0017055904605369268</Coef>
                    </Y>
                    <Z order1="1">
                        <Coef exponent1="0">0.80085207377130485</Coef>
                        <Coef exponent1="1">0.0036318762853149731</Coef>
                    </Z>
                </YAxisPoly>
                <FreqZero>9800000000</FreqZero>
                <EB>
                    <DCXPoly order1="0">
                        <Coef exponent1="0">0</Coef>
                    </DCXPoly>
                    <DCYPoly order1="0">
                        <Coef exponent1="0">0</Coef>
                    </DCYPoly>
                </EB>
                <Array>
                    <GainPoly order1="1" order2="1">
                        <Coef exponent1="0" exponent2="0">0</Coef>
                        <Coef exponent1="0" exponent2="1">33.079682693497979</Coef>
                        <Coef exponent1="1" exponent2="0">86.259306544101662</Coef>
                        <Coef exponent1="1" exponent2="1">-4389.7717865765271</Coef>
                    </GainPoly>
                    <PhasePoly order1="1" order2="1">
                        <Coef exponent1="0" exponent2="0">0</Coef>
                        <Coef exponent1="0" exponent2="1">-0.0040833372598498311</Coef>
                        <Coef exponent1="1" exponent2="0">-0.41936793258034533</Coef>
                        <Coef exponent1="1" exponent2="1">95.234873249052086</Coef>
                    </PhasePoly>
                </Array>
                <Elem>
                    <GainPoly order1="0" order2="0">
                        <Coef exponent1="0" exponent2="0">0</Coef>
                    </GainPoly>
                    <PhasePoly order1="0" order2="0">
                        <Coef exponent1="0" exponent2="0">0</Coef>
                    </PhasePoly>
                </Elem>
                <GainBSPoly order1="0">
                    <Coef exponent1="0">0</Coef>
                </GainBSPoly>
                <EBFreqShift>false</EBFreqShift>
                <MLFreqDilation>true</MLFreqDilation>
            </Tx>
            <Rcv>
                <XAxisPoly>
                    <X order1="0">
                        <Coef exponent1="0">-0.043040129758302216</Coef>
                    </X>
                    <Y order1="0">
                        <Coef exponent1="0">0.86650241157354035</Coef>
                    </Y>
                    <Z order1="0">
                        <Coef exponent1="0">-0.49731414772296534</Coef>
                    </Z>
                </XAxisPoly>
                <YAxisPoly>
                    <X order1="0">
                        <Coef exponent1="0">-0.40691335445134125</Coef>
                    </X>
                    <Y order1="0">
                        <Coef exponent1="0">0.43942506084592903</Coef>
                    </Y>
                    <Z order1="0">
                        <Coef exponent1="0">0.80085207377130485</Coef>
                    </Z>
                </YAxisPoly>
                <FreqZero>9800000000</FreqZero>
                <EB>
                    <DCXPoly order1="0">
                        <Coef exponent1="0">0</Coef>
                    </DCXPoly>
                    <DCYPoly order1="0">
                        <Coef exponent1="0">0</Coef>
                    </DCYPoly>
                </EB>
                <Array>
                    <GainPoly order1="1" order2="1">
                        <Coef exponent1="0" exponent2="0">0</Coef>
                        <Coef exponent1="0" exponent2="1">33.079682693497979</Coef>
                        <Coef exponent1="1" exponent2="0">86.259306544101662</Coef>
                        <Coef exponent1="1" exponent2="1">-4389.7717865765271</Coef>
                    </GainPoly>
                    <PhasePoly order1="1" order2="1">
                        <Coef exponent1="0" exponent2="0">0</Coef>
                        <Coef exponent1="0" exponent2="1">-0.0040833372598498311</Coef>
                        <Coef exponent1="1" exponent2="0">-0.41936793258034533</Coef>
                        <Coef exponent1="1" exponent2="1">95.234873249052086</Coef>
                    </PhasePoly>
                </Array>
                <Elem>
                    <GainPoly order1="0" order2="0">
                        <Coef exponent1="0" exponent2="0">0</Coef>
                    </GainPoly>
                    <PhasePoly order1="0" order2="0">
                        <Coef exponent1="0" exponent2="0">0</Coef>
                    </PhasePoly>
                </Elem>
                <GainBSPoly order1="0">
                    <Coef exponent1="0">0</Coef>
                </GainBSPoly>
                <EBFreqShift>false</EBFreqShift>
                <MLFreqDilation>true</MLFreqDilation>
            </Rcv>
            <TwoWay>
                <XAxisPoly>
                    <X order1="1">
                        <Coef exponent1="0">-0.043040129758302216</Coef>
                        <Coef exponent1="1">8.3894258665007037e-05</Coef>
                    </X>
                    <Y order1="1">
                        <Coef exponent1="0">0.86650241157354035</Coef>
                        <Coef exponent1="1">0.00041253146944526949</Coef>
                    </Y>
                    <Z order1="1">
                        <Coef exponent1="0">-0.49731414772296534</Coef>
                        <Coef exponent1="1">0.00071303340343231247</Coef>
                    </Z>
                </XAxisPoly>
                <YAxisPoly>
                    <X order1="1">
                        <Coef exponent1="0">-0.40691335445134125</Coef>
                        <Coef exponent1="1">0.0092656464455340044</Coef>
                    </X>
                    <Y order1="1">
                        <Coef exponent1="0">0.43942506084592903</Coef>
                        <Coef exponent1="1">0.0017055904605369268</Coef>
                    </Y>
                    <Z order1="1">
                        <Coef exponent1="0">0.80085207377130485</Coef>
                        <Coef exponent1="1">0.0036318762853149731</Coef>
                    </Z>
                </YAxisPoly>
                <FreqZero>9800000000</FreqZero>
                <EB>
                    <DCXPoly order1="0">
                        <Coef exponent1="0">0</Coef>
                    </DCXPoly>
                    <DCYPoly order1="0">
                        <Coef exponent1="0">0</Coef>
                    </DCYPoly>
                </EB>
                <Array>
                    <GainPoly order1="1" order2="1">
                        <Coef exponent1="0" exponent2="0">0</Coef>
                        <Coef exponent1="0" exponent2="1">33.079682693497979</Coef>
                        <Coef exponent1="1" exponent2="0">86.259306544101662</Coef>
                        <Coef exponent1="1" exponent2="1">-4389.7717865765271</Coef>
                    </GainPoly>
                    <PhasePoly order1="1" order2="1">
                        <Coef exponent1="0" exponent2="0">0</Coef>
                        <Coef exponent1="0" exponent2="1">-0.0040833372598498311</Coef>
                        <Coef exponent1="1" exponent2="0">-0.41936793258034533</Coef>
                        <Coef exponent1="1" exponent2="1">95.234873249052086</Coef>
                    </PhasePoly>
                </Array>
                <Elem>
                    <GainPoly order1="0" order2="0">
                        <Coef exponent1="0" exponent2="0">0</Coef>
                    </GainPoly>
                    <PhasePoly order1="0" order2="0">
                        <Coef exponent1="0" exponent2="0">0</Coef>
                    </PhasePoly>
                </Elem>
                <GainBSPoly order1="0">
                    <Coef exponent1="0">0</Coef>
                </GainBSPoly>
                <EBFreqShift>false</EBFreqShift>
                <MLFreqDilation>true</MLFreqDilation>
            </TwoWay>
        </Antenna>"#;
        let antenna: Antenna;
    }
}