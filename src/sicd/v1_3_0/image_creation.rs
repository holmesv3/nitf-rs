use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct ImageCreation {
    #[serde(rename = "Application")]
    pub application: Option<String>,
    #[serde(rename = "DateTime")]
    pub date_time: Option<String>,
    #[serde(rename = "Site")]
    pub site: Option<String>,
    #[serde(rename = "Profile")]
    pub profile: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::ImageCreation;
    use quick_xml::de::from_str;
    #[test]
    fn test_imagecreation() {
        let xml_str = r#"<Grid><ImagePlane>SLANT</ImagePlane><Type>RGAZIM</Type>
            <TimeCOAPoly order1="0" order2="0"><Coef exponent1="0" exponent2="0"
            >0</Coef></TimeCOAPoly><Row><UVectECF><X>0</X><Y>0</Y><Z>0</Z>
            </UVectECF><SS>0</SS><ImpRespWid>0</ImpRespWid><Sgn>-1</Sgn>
            <ImpRespBW>0</ImpRespBW><KCtr>0</KCtr><DeltaK1>0</DeltaK1><DeltaK2>
            0</DeltaK2><DeltaKCOAPoly order1="0" order2="0"><Coef exponent1="0" 
            exponent2="0">-0</Coef></DeltaKCOAPoly></Row><Col><UVectECF><X>0</X>
            <Y>0</Y><Z>0</Z></UVectECF><SS>0</SS><ImpRespWid>0</ImpRespWid><Sgn>
            -1</Sgn><ImpRespBW>0</ImpRespBW><KCtr>0</KCtr><DeltaK1>0</DeltaK1>
            <DeltaK2>0</DeltaK2><DeltaKCOAPoly order1="0" order2="0">
            <Coef exponent1="0" exponent2="0">-0</Coef></DeltaKCOAPoly></Col>
            </Grid>"#;
        assert!(match from_str::<ImageCreation>(&xml_str) {
            Ok(_) => true,
            Err(_) => false,
        })
    }
}
