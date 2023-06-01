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
    pub Grid: Grid,  // TODO
    pub Timeline: Timeline,
    pub Position: Position,
    pub RadarCollection: RadarCollection,  // TODO
    pub ImageFormation: ImageFormation,  // TODO
    pub SCPCOA: ScpCoa,
    pub Radiometric: Option<Radiometric>,
    pub Antenna: Option<Antenna>,
    pub ErrorStatistics: Option<ErrorStatistics>,
    pub MatchInfo: Option<MatchInfo>,
    pub RgAzComp: Option<RgAzComp>,
    pub PFA: Option<Pfa>,
    pub RMA: Option<Rma>,
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
pub struct Grid {
    pub ImagePlane: ImagePlane,
    pub Type: Type,
    pub TimeCOAPoly: Poly2D,
    pub Row: DirectionParams,
    pub Col: DirectionParams,


}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub enum ImagePlane {
    GROUND,
    SLANT,
    OTHER
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub enum Type {
    RGAZIM,
    RGZERO,
    XRGYCR,
    XCTYAT,
    PLANE
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct DirectionParams {
    UVectECF: XYZ,
    SS: f64,
    ImpRespWid: f64,
    Sgn: i8,  // TODO: Maybe use an actual enum here
    ImpRespBW: f64,
    KCtr: f64,
    DeltaK1: f64,
    DeltaK2: f64,
    DeltaKCOAPoly: Option<Poly2D>,
    WgtType: Option<WgtType>,
    WgtFunct: Option<WgtFunct>,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct WgtType {
    WindowName: String,
    Parameter: Option<Vec<Parameter>>,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct WgtFunct {
    size: u64,
    Wgt: Vec<Wgt>,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Wgt {
    index: usize,
    Wgt: f64,
}


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
pub struct ErrorStatistics {
    pub CompositeSCP: Option<CompositeSCP>,
    pub Components: Option<Components>,
    pub AdditionalParams: Option<Vec<Parameter>>,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct CompositeSCP {
    pub Rg: f64,
    pub Az: f64,
    pub RgAz: f64,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Components {
    pub PosVelErr: PosVelErr,
    pub RadarSensor: RadarSensor,
    pub TropoErro: Option<TropoError>,
    pub IonoError: Option<IonoError>,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct PosVelErr {
    pub Frame: Frame,
    pub P1: f64,
    pub P2: f64,
    pub P3: f64,
    pub V1: f64,
    pub V2: f64,
    pub V3: f64,
    pub CorrCoefs: Option<CorrCoefs>,
    pub PositionDecorr: Option<Decorr>,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub enum Frame {
    ECF,
    RIC_ECF,
    RIC_ECI,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct CorrCoefs {
    pub P1P2: f64,
    pub P1P3: f64,
    pub P1V1: f64,
    pub P1V2: f64,
    pub P1V3: f64,
    pub P2P3: f64,
    pub P2V1: f64,
    pub P2V2: f64,
    pub P2V3: f64,
    pub P3V1: f64,
    pub P3V2: f64,
    pub P3V3: f64,
    pub V1V2: f64,
    pub V1V3: f64,
    pub V2V3: f64,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct RadarSensor {
    pub RangeBias: f64,
    pub ClockFreqSF: Option<f64>,
    pub TransmitFreqSF: Option<f64>,
    pub RangeBiasDecorr: Option<Decorr>,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct TropoError {
    pub TropoRangeVertical: Option<f64>,
    pub TropoRangeSlant: Option<f64>,
    pub TropoRangeDecorr: Option<Decorr>,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct IonoError {
    pub IonoRangeVertical: Option<f64>,
    pub IonoRangeRateVertical: Option<f64>,
    pub IonoRgRgRateCC: f64,
    pub IonoRangeVertDecorr: Option<Decorr>,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Decorr {
    pub CorrCoefZero: f64,
    pub DecorrRate: f64,
}


#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct MatchInfo {
    pub NumMatchTypes: u64,
    pub MatchType: Vec<MatchType>,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct MatchType {
    pub index: usize,
    pub TypeID: String,
    pub CurrentIndex: Option<usize>,
    pub NumMatchCollections: u64,
    pub MatchCollection: Option<Vec<MatchCollection>>
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct MatchCollection {
    pub index: usize,
    pub CoreName: String,
    pub MatchIndex: Option<usize>,
    pub Parameter: Option<Vec<Parameter>>
}


#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct RgAzComp {
    pub AzSF: f64,
    pub KazPoly: Poly1D,
}

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
pub struct Rma {
    pub RMAlgoType: RMAlgoType,
    pub ImageType: ImageType,
    pub RMAT: Option<RMAlgo>,
    pub RMCR: Option<RMAlgo>,
    pub INCA: Option<INCA>,

}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub enum RMAlgoType {
    OMEGA_K,
    CSA,
    RG_DOP,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub enum ImageType {
    RMAT,
    RMCR,
    INCA,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct RMAlgo {
    pub PosRef: XYZ,
    pub VelRef: XYZ,
    pub DopConeAngRef: f64,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct INCA {
    pub TimeCAPoly: Poly1D,
    pub R_CA_SCP: f64,
    pub FreqZero: f64,
    pub DRateSFPoly: Poly2D,
    pub DopCentroidPoly: Option<Poly2D>,
    pub DopCentroidCOA: Option<bool>,
}
