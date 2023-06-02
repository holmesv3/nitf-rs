use serde::Deserialize;
use super::{Poly2D, XYZ, Parameter};
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Grid {
    pub ImagePlane: ImagePlane,
    pub Type: Type,
    pub TimeCOAPoly: Poly2D,
    pub Row: DirectionParams,
    pub Col: DirectionParams,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub enum ImagePlane {
    GROUND,
    SLANT,
    OTHER,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub enum Type {
    RGAZIM,
    RGZERO,
    XRGYCR,
    XCTYAT,
    PLANE,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct DirectionParams {
    pub UVectECF: XYZ,
    pub SS: f64,
    pub ImpRespWid: f64,
    pub Sgn: i8, // TODO: Maybe use an actual enum here
    pub ImpRespBW: f64,
    pub KCtr: f64,
    pub DeltaK1: f64,
    pub DeltaK2: f64,
    pub DeltaKCOAPoly: Option<Poly2D>,
    pub WgtType: Option<WgtType>,
    pub WgtFunct: Option<WgtFunct>,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct WgtType {
    pub WindowName: String,
    pub Parameter: Parameter,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct WgtFunct {
    pub size: u64,
    pub Wgt: Vec<Wgt>,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Wgt {
    pub index: usize,
    pub Wgt: f64,
}

#[cfg(test)]
mod tests {
    use super::Grid;
    use serde_xml_rs::from_str;
    #[test]
    fn test_grid () {
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
            Ok(_) => true, Err(_) => false,
        }) 
    }
}