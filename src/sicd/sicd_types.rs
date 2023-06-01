#![allow(non_camel_case_types)]
use std::ops::Index;

use super::param_types::*;
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Sicd {
    pub CollectionInfo: CollectionInfo,
    pub ImageCreation: Option<ImageCreation>,
    pub ImageData: ImageData,
    pub GeoData: GeoData,
    pub Grid: Grid,
    pub Timeline: Timeline,
    pub Position: Position,
    pub RadarCollection: RadarCollection,
    pub ImageFormation: ImageFormation,
    pub SCPCOA: ScpCoa,
    pub Radiometric: Option<Radiometric>,
    pub Antenna: Option<Antenna>,
    pub ErrorStatistics: Option<ErrorStatistics>,
    pub MatchInfo: Option<MatchInfo>,
    pub RgAzComp: Option<RgAzComp>,
    pub Pfa: Option<Pfa>,
    pub Rma: Option<Rma>,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct CollectionInfo {
    pub CollectorName: String,
    pub IlluminatorName: Option<String>,
    pub CoreName: String,
    pub CollectType: Option<CollectType>,
    pub RadarMode: RadarMode,
    #[serde(default = "default_classification")]
    pub Classification: String,
    pub CountryCode: Option<Vec<String>>,
    pub Parameter: Option<Vec<Parameter>>,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub enum CollectType {
    MONOSTATIC,
    BISTATIC,
}
fn default_classification() -> String {
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
    pub value: String,
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
    pub ValidData: Option<ValidDataRC>,
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
pub struct ValidDataRC {
    pub size: u64,
    pub Vertex: Vec<VertexRC>,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct VertexRC {
    pub index: usize,
    pub Row: u64,
    pub Col: u64,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct GeoData {
    pub EarthModel: EarthModel,
    pub SCP: SCP,
    pub ImageCorners: ImageCorners,
    pub ValidData: Option<ValidDataLL>,
    pub GeoInfo: Option<Vec<GeoInfo>>,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub enum EarthModel {
    WGS_84,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct SCP {
    pub ECF: XYZ,
    pub LLH: LLH,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct ImageCorners {
    pub ICP: Vec<ICP>,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct ICP {
    pub index: String,
    pub Lat: f64,
    pub Lon: f64,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct ValidDataLL {
    pub size: u64,
    pub Vertex: Vec<VertexLL>,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct VertexLL {
    pub index: usize,
    pub Lat: f64,
    pub Lon: f64,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct GeoInfo {
    pub name: String,
    pub Desc: Option<Vec<String>>,
    pub Point: Option<LL>,
    pub Line: Option<Line>,
    pub Polygon: Option<Polygon>,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Line {
    pub size: u64,
    pub Endpoint: Vec<Endpoint>,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Endpoint {
    pub index: usize,
    pub Lat: f64,
    pub Lon: f64,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Polygon {
    pub size: u64,
    pub Vertex: Vec<VertexLL>,
}

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
pub struct Position {
    pub ARPPoly: XyzPoly,
    pub GRPPoly: Option<XyzPoly>,
    pub TxAPCPoly: Option<XyzPoly>,
    pub RcvApc: Option<RcvAPC>,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct RcvAPC {
    pub size: usize,
    pub RcvAPCPoly: RcvAPCPoly,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct RcvAPCPoly{
    pub index: usize,
    pub X: Poly1D,
    pub Y: Poly1D,
    pub Z: Poly1D,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct RadarCollection {}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct ImageFormation {}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct ScpCoa {
    pub SCPTime: f64,
    pub ARPPos: XYZ,
    pub ARPVel: XYZ,
    pub ARPAcc: XYZ,
    pub SideOfTrack: SideOfTrack,
    pub SlantRange: f64,
    pub GroundRange: f64,
    pub DopplerConeAng: f64,
    pub GrazeAng: f64,
    pub IncidenceAng: f64,
    pub TwistAng: f64,
    pub SlopeAng: f64,
    pub AzimAng: f64,
    pub LayoverAng: f64,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub enum SideOfTrack {
    L,
    R,
}

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


#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Antenna {
    pub Tx: Option<AntennaType>,
    pub Rcv: Option<AntennaType>,
    pub TwoWay: Option<AntennaType>,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct AntennaType {
    pub XAxisPoly: XyzPoly,
    pub YAxisPoly: XyzPoly,
    pub FreqZero: f64,
    pub EB: Option<EB>,
    pub Array: Array,
    pub Elem: Option<Elem>,
    pub GainBSPoly: Option<Poly1D>,
    pub EBFreqShift: Option<bool>,
    pub MLFreqDilation: Option<bool>,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct EB {
    pub DCXPoly: Poly1D,
    pub DCYPoly: Poly1D,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Array {
    pub GainPoly: Poly2D,
    pub PhasePoly: Poly2D,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Elem {
    pub GainPoly: Poly2D,
    pub PhasePoly: Poly2D,
}


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
    pub Applied: bool,
    pub STDSPhasePoly: Poly2D,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Rma {}
