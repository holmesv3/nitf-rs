use super::{Poly1d, Poly2d, XYZ};
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Pfa {
    #[serde(rename = "FPN")]
    pub fpn: XYZ,
    #[serde(rename = "IPN")]
    pub ipn: XYZ,
    #[serde(rename = "PolarAngRefTime")]
    pub polar_ang_ref_time: f64,
    #[serde(rename = "PolarAngPoly")]
    pub polar_ang_poly: Poly1d,
    #[serde(rename = "SpatialFreqSFPoly")]
    pub spatial_freq_sf_poly: Poly1d,
    #[serde(rename = "Krg1")]
    pub krg1: f64,
    #[serde(rename = "Krg2")]
    pub krg2: f64,
    #[serde(rename = "Kaz1")]
    pub kaz1: f64,
    #[serde(rename = "Kaz2")]
    pub kaz2: f64,
    #[serde(rename = "STDeskew")]
    pub st_deskew: Option<STDeskew>,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct STDeskew {
    #[serde(rename = "Applied")]
    pub applied: bool,
    #[serde(rename = "STDSPhasePoly")]
    pub stds_phase_poly: Poly2d,
}

#[cfg(test)]
mod tests {
    use super::Pfa;
    use quick_xml::de::from_str;

    #[test]
    fn test_pfa() {
        let xml_str = r#"<PFA><FPN><X>0</X><Y>0</Y><Z>0</Z></FPN><IPN><X>0</X>
            <Y>0</Y><Z>0</Z></IPN><PolarAngRefTime>0</PolarAngRefTime>
            <PolarAngPoly order1="0"><Coef exponent1="0">0</Coef></PolarAngPoly>
            <SpatialFreqSFPoly order1="0"><Coef exponent1="0">0</Coef>
            </SpatialFreqSFPoly><Krg1>0</Krg1><Krg2>0</Krg2><Kaz1>0</Kaz1><Kaz2>
            0</Kaz2></PFA>"#;
        assert!(match from_str::<Pfa>(xml_str) {
            Ok(_) => true,
            Err(_) => false,
        })
    }
}
