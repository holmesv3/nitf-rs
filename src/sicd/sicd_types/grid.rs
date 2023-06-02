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