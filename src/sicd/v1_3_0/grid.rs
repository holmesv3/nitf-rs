use super::{Parameter, Poly2d, XYZ};
use serde::Deserialize;
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Grid {
    #[serde(rename = "ImagePlane")]
    pub image_plane: ImagePlane,
    #[serde(rename = "Type")]
    pub type_grid: GridType,
    #[serde(rename = "TimeCOAPoly")]
    pub time_coa_poly: Poly2d,
    #[serde(rename = "Row")]
    pub row: DirectionParams,
    #[serde(rename = "Col")]
    pub col: DirectionParams,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct ImagePlane {
    #[serde(rename = "$text")]
    pub value: ImagePlaneEnum,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub enum ImagePlaneEnum {
    GROUND,
    SLANT,
    OTHER,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct GridType {
    #[serde(rename = "$text")]
    pub value: GridTypeEnum,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub enum GridTypeEnum {
    RGAZIM,
    RGZERO,
    XRGYCR,
    XCTYAT,
    PLANE,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct DirectionParams {
    #[serde(rename = "UVectECF")]
    pub u_vect_ecf: XYZ,
    #[serde(rename = "SS")]
    pub ss: f64,
    #[serde(rename = "ImpRespWid")]
    pub imp_resp_wid: f64,
    #[serde(rename = "Sgn")]
    pub sgn: i8, // TODO: Maybe use an actual enum here
    #[serde(rename = "ImpRespBW")]
    pub imp_resp_bw: f64,
    #[serde(rename = "KCtr")]
    pub k_ctr: f64,
    #[serde(rename = "DeltaK1")]
    pub delta_k1: f64,
    #[serde(rename = "DeltaK2")]
    pub delta_k2: f64,
    #[serde(rename = "DeltaKCOAPoly")]
    pub delta_kcoa_poly: Option<Poly2d>,
    #[serde(rename = "WgtType")]
    pub wgt_type: Option<WgtType>,
    #[serde(rename = "WgtFunct")]
    pub wgt_funct: Option<WgtFunct>,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct WgtType {
    #[serde(rename = "WindowName")]
    pub window_name: String,
    #[serde(rename = "Parameter")]
    pub parameters: Option<Vec<Parameter>>,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct WgtFunct {
    #[serde(rename = "@size")]
    pub size: u64,
    #[serde(rename = "Wgt")]
    pub wgt: Vec<Wgt>,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Wgt {
    #[serde(rename = "@index")]
    pub index: usize,
    #[serde(rename = "$value")]
    pub value: f64,
}

#[cfg(test)]
mod tests {
    use super::Grid;
    use quick_xml::de::from_str;
    #[test]
    fn test_grid() {
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
        assert!(match from_str::<Grid>(&xml_str) {
            Ok(_) => true,
            Err(_) => false,
        })
    }
}
