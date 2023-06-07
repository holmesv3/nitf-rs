use super::Poly2d;
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Radiometric {
    #[serde(rename = "NoiseLevel")]
    pub noise_level: Option<NoiseLevel>,
    #[serde(rename = "RCSSFPoly")]
    pub rcssf_poly: Option<Poly2d>,
    #[serde(rename = "SigmaZeroSFPoly")]
    pub sigma_zero_sf_poly: Option<Poly2d>,
    #[serde(rename = "BetaZeroSFPoly")]
    pub beta_zero_sf_poly: Option<Poly2d>,
    #[serde(rename = "GammaZeroSFPoly")]
    pub gamma_zero_sf_poly: Option<Poly2d>,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct NoiseLevel {
    #[serde(rename = "NoiseLevelType")]
    pub noise_level_type: NoiseLevelType,
    #[serde(rename = "NoisePoly")]
    pub noise_poly: Poly2d,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub enum NoiseLevelType {
    ABSOLUTE,
    RELATIVE,
}
