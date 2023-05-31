#![allow(non_camel_case_types)]
use std::ops::Index;

use super::param_types::{Poly1D, Poly2D, XYZ, RowCol};
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Sicd {
    pub CollectionInfo: CollectionInfo,
    pub ImageCreation: Option<ImageCreation>,
    pub ImageData: ImageData,
    pub GeoData: GeoData,
    pub Grid: Grid,
    pub Timeline: Timeline, // Done
    pub Position: Position,
    pub RadarCollection: RadarCollection,
    pub ImageFormation: ImageFormation,
    pub SCPCOA: ScpCoa,
    pub Radiometric: Option<Radiometric>,
    pub Antenna: Option<Antenna>,
    pub ErrorStatistics: Option<ErrorStatistics>,
    pub MatchInfo: Option<MatchInfo>,
    pub RgAzComp: Option<RgAzComp>,
    pub Pfa: Option<Pfa>, // Done
    pub Rma: Option<Rma>,
}


#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct CollectionInfo {
    pub CollectorName: String,
    pub IlluminatorName: Option<String>,
    pub CoreName: String,
    pub CollectType: Option<CollectType>,
    pub RadarMode: RadarMode,
    #[serde(default = "default_class")]
    pub Classification: String,
    pub CountryCode: Option<Vec<String>>,
    pub Parameter: Option<Vec<Parameter>>,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub enum CollectType {
    MONOSTATIC,
    BISTATIC
}
fn default_class() -> String {
    String::from("UNCLASSIFIED")
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct RadarMode {
    pub ModeType: ModeType,
    pub ModeID: Option<String>,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub enum ModeType {
    SPOTLIGHT,
    STRIPMAP,
    #[serde(rename = "DYNAMIC STRIPMAP")]
    DYNAMIC_STRIPMAP,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Parameter {
    pub name: String,
    #[serde(rename = "$value")]
    pub value: String
}


#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct ImageCreation {
    pub Application: Option<String>,
    pub DateTime: Option<String>,
    pub Site: Option<String>,
    pub Profile: Option<String>,
}


#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct ImageData {
    pub PixelType: PixelType,
    pub AmpTable: Option<AmpTable>,
    pub NumRows: u64,
    pub NumCols: u64,
    pub FirstRow: u64,
    pub FirstCol: u64,
    pub FullImage: FullImage,
    pub SCPPixel: RowCol,
    pub ValidData: Option<ValidData>
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub enum PixelType {
    RE32F_IM32F,
    RE16I_IM16I,
    AMP8I_PHS8I,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct AmpTable {
    pub size: u16,
    pub Amplitude: Vec<Amplitude>,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Amplitude {
    pub index: u8,
    #[serde(rename = "$value")]
    pub value: f64,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct FullImage {
    pub NumRows: u64,
    pub NumCols: u64,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct ValidData {
    pub size: u64,
    pub Vertex: Vec<Vertex>,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Vertex {
    pub index: usize,
    pub Row: u64,
    pub Col: u64
}


#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct GeoData {}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Grid {}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Timeline {
    pub CollectStart: String,
    pub CollectDuration: f64,
    pub IPP: Option<IppParams>,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct IppParams {
    pub Set: Vec<IppSet>,
}
impl Index<usize> for IppParams {
    type Output = IppSet;
    fn index(&self, index: usize) -> &Self::Output {
        &self.Set[index]
    }
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
    pub STDeskew: Option<STDeskew>,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct STDeskew {
    Applied: bool,
    STDSPhasePoly: Poly2D,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Rma {}
