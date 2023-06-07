use super::Poly2D;
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Radiometric {
    #[serde(rename = "NoiseLevel")]
    pub noise_level: Option<NoiseLevel>,
    #[serde(rename = "RCSSFPoly")]
    pub rcssf_poly: Option<Poly2D>,
    #[serde(rename = "SigmaZeroSFPoly")]
    pub sigma_zero_sf_poly: Option<Poly2D>,
    #[serde(rename = "BetaZeroSFPoly")]
    pub beta_zero_sf_poly: Option<Poly2D>,
    #[serde(rename = "GammaZeroSFPoly")]
    pub gamma_zero_sf_poly: Option<Poly2D>,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct NoiseLevel {
    #[serde(rename = "NoiseLevelType")]
    pub noise_level_type: NoiseLevelType,
    #[serde(rename = "NoisePoly")]
    pub noise_poly: Poly2D,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub enum NoiseLevelType {
    ABSOLUTE,
    RELATIVE,
}
