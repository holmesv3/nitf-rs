use super::{Parameter, Poly2D, XYZ};
use serde::Deserialize;
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Grid {
    #[serde(rename = "ImagePlane")]
    pub image_plane: ImagePlane,
    #[serde(rename = "Type")]
    pub type_grid: GridType,
    #[serde(rename = "TimeCOAPoly")]
    pub time_coa_poly: Poly2D,
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
    pub delta_kcoa_poly: Option<Poly2D>,
    #[serde(rename = "WgtType")]
    pub wgt_type: Option<String>,
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
