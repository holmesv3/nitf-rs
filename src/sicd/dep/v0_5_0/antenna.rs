use super::{Poly1d, Poly2d, XyzPoly};
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
    pub gain_bs_poly: Option<Poly1d>,
    #[serde(rename = "EBFreqShift")]
    pub eb_freq_shift: Option<bool>,
    #[serde(rename = "MLFreqDilation")]
    pub ml_freq_dilation: Option<bool>,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct EB {
    #[serde(rename = "DCXPoly")]
    pub dcx_poly: Poly1d,
    #[serde(rename = "DCYPoly")]
    pub dcy_poly: Poly1d,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Array {
    #[serde(rename = "GainPoly")]
    pub gain_poly: Poly2d,
    #[serde(rename = "PhasePoly")]
    pub phase_poly: Poly2d,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Elem {
    #[serde(rename = "GainPoly")]
    pub gain_poly: Poly2d,
    #[serde(rename = "PhasePoly")]
    pub phase_poly: Poly2d,
}

