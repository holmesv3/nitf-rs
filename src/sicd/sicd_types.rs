use serde::Deserialize;
use super::param_types::{Poly1D, Poly2D, XYZ};

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Sicd {
    pub CollectionInfo: CollectionInfo,
    pub ImageCreation: Option<ImageCreation>, 
    pub ImageData: ImageData,
    pub GeoData: GeoData,
    pub Grid: Grid,
    pub Timeline: Timeline,  // Done
    pub Position: Position, 
    pub RadarCollection: RadarCollection,
    pub ImageFormation: ImageFormation,
    pub SCPCOA: ScpCoa,
    pub Radiometric: Option<Radiometric>,
    pub Antenna: Option<Antenna>,
    pub ErrorStatistics: Option<ErrorStatistics>,
    pub MatchInfo: Option<MatchInfo>,
    pub RgAzComp: Option<RgAzComp>,
    pub Pfa: Option<Pfa> , // Done
    pub Rma: Option<Rma>,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct CollectionInfo {}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct ImageCreation {}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct ImageData {}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct GeoData {}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Grid {}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Timeline {
    pub CollectStart: String,
    pub CollectDuration: f64,
    pub IPP: Option<IppParams>
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct IppParams {
    pub Set: Vec<IppSet>,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct IppSet {
    pub TStart: f64,
    pub TEnd: f64, 
    pub IPPStart: u64,
    pub IPPEnd: u64,
    pub IPPPoly: Poly1D,
}


#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Position {}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct RadarCollection {}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct ImageFormation {}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct ScpCoa {}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Radiometric {}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Antenna {}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct ErrorStatistics {}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct MatchInfo {}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct RgAzComp {}

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
    pub STDeskew: Option<STDeskew>
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct STDeskew {
    Applied: bool,
    STDSPhasePoly: Poly2D
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Rma {}
