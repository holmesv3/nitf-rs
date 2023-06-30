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
    #[serde(rename = "HPBW")]
    pub hpbw: Option<HPBW>,
    #[serde(rename = "Array")]
    pub array: Option<Array>,
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
pub struct HPBW {
    #[serde(rename = "DCX")]
    pub dcx: f64,
    #[serde(rename = "DCY")]
    pub dcy: f64,
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
