use super::{Poly1D, Poly2D, XYZ};
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
    pub polar_ang_poly: Poly1D,
    #[serde(rename = "SpatialFreqSFPoly")]
    pub spatial_freq_sf_poly: Poly1D,
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
    pub stds_phase_poly: Poly2D,
}
