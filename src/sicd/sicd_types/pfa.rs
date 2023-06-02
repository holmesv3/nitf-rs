use serde::Deserialize;
use super::{XYZ, Poly1D, Poly2D};

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Pfa {
    pub FPN: XYZ,
    pub IPN: XYZ,
    pub PolarAngRefTime: f64,
    pub PolarAngPoly: Poly1D,
    pub SpatialFreqSFPoly: Poly1D,
    pub Krg1: f64,
    pub Krg2: f64,
    pub Kaz1: f64,
    pub Kaz2: f64,
    pub STDeskew: Option<STDeskew>,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct STDeskew {
    pub Applied: bool,
    pub STDSPhasePoly: Poly2D,
}