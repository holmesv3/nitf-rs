use super::Poly2D;
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Radiometric {
    #[serde(rename = "NoiseLevel")]
    pub noise_level: Option<NoiseLevel>,
    #[serde(rename = "NoisePoly")]
    pub noise_poly: Option<Poly2D>,
    #[serde(rename = "RCSSFPoly")]
    pub rcssf_poly: Option<Poly2D>,
    #[serde(rename = "SigmaZeroSFPoly")]
    pub sigma_zero_sf_poly: Option<Poly2D>,
    #[serde(rename = "SigmaZeroSFIncidenceMap")]
    pub sigma_zero_sf_incidence_map: Option<IncidenceMap>,
    #[serde(rename = "BetaZeroSFPoly")]
    pub beta_zero_sf_poly: Option<Poly2D>,
    #[serde(rename = "GammaZeroSFPoly")]
    pub gamma_zero_sf_poly: Option<Poly2D>,
    #[serde(rename = "GammaZeroSFIncidenceMap")]
    pub gamma_zero_sf_incidence_map: Option<IncidenceMap>,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct NoiseLevel {
    #[serde(rename = "$text")]
    pub value: NoiseLevelType,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub enum NoiseLevelType {
    ABSOLUTE,
    RELATIVE,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct IncidenceMap {
    #[serde(rename = "$text")]
    pub value: AppliedType,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub enum AppliedType {
    APPLIED,
    #[serde(rename = "NOT_APPLIED")]
    NOTAPPLIED,
}
