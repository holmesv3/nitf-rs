use serde::Deserialize;
use super::Poly2D;

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Radiometric {
    pub NoiseLevel: Option<NoiseLevel>,
    pub RCSSFPoly: Option<Poly2D>,
    pub SigmaZeroSFPoly: Option<Poly2D>,
    pub BetaZeroSFPoly: Option<Poly2D>,
    pub GammaZeroSFPoly: Option<Poly2D>,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct NoiseLevel {
    pub NoiseLevelType: NoiseLevelType,
    pub NoisePoly: Poly2D,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub enum NoiseLevelType {
    ABSOLUTE,
    RELATIVE,
}