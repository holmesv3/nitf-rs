use serde::Deserialize;
use super::{Parameter, XYZ};

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct RadarCollection {
    pub TxFrequency: TxFrequency,
    pub RefFreqIndex: Option<u64>,
    pub Waveform: Option<Waveform>,
    pub TxPolarization: TxPolarization,
    pub TxSequence: Option<TxSequence>,
    pub RcvChannels: RcvChannels,
    pub Area: Option<Area>,
    pub Parameter: Parameter,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct TxFrequency {
    pub Min: f64,
    pub Max: f64,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Waveform {
    pub size: u64,
    pub WFParameters: Vec<WFParameters>,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct WFParameters {
    pub index: usize,
    pub TxPulseLength: Option<f64>,
    pub TxRFBandwidth: Option<f64>,
    pub TxFreqStart: Option<f64>,
    pub TxFMRate: Option<f64>,
    pub RcvDemodType: Option<RcvDemodType>,
    pub RcvWindowLength: Option<f64>,
    pub ADCSampleRate: Option<f64>,
    pub RcvIFBandwidth: Option<f64>,
    pub RcvFreqStart: Option<f64>,
    pub RcvFMRate: Option<f64>,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub enum RcvDemodType {
    STRETCH,
    CHIRP,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub enum TxPolarization {
    V,
    H,
    RHC,
    LHC,
    OTHER,
    UNKNOWN,
    SEQUENCE,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct TxSequence {
    pub size: u64,
    pub TxStep: Vec<TxStep>,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct TxStep {
    pub index: usize,
    pub WFIndex: Option<usize>,
    pub TxPolarization: Option<TxStepPolarization>,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub enum TxStepPolarization {
    V,
    H,
    RHC,
    LHC,
    OTHER,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct RcvChannels {
    pub size: u64,
    pub ChanParameters: Vec<ChanParameters>,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct ChanParameters {
    pub index: usize,
    pub TxRcvPolarization: String, // TODO: Implement this enum
    pub RcvAPCIndex: Option<u64>,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Area {
    pub Corner: Corner,
    pub Plane: Option<Plane>,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Corner {
    #[serde(rename = "$value")]
    pub ACP: Vec<ACP>,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct ACP {
    pub index: usize,
    pub Lat: f64,
    pub Lon: f64,
    pub HAE: f64,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Plane {
    pub RefPt: RefPt,
    pub XDir: XDir,
    pub YDir: YDir,
    pub SegmentList: Option<SegmentList>,
    pub Orientation: Option<Orientation>,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct RefPt {
    pub name: Option<String>,
    pub ECF: XYZ,
    pub Line: f64,
    pub Sample: f64,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct XDir {
    pub UVectECF: XYZ,
    pub LineSpacing: f64,
    pub NumLines: u64,
    pub FirstLine: u64,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct YDir {
    pub UVectECF: XYZ,
    pub SampleSpacing: f64,
    pub NumSamples: u64,
    pub FirstSample: u64,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct SegmentList {
    pub size: Option<String>,
    pub Segment: Vec<Segment>,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Segment {
    pub index: usize,
    pub StartLine: u64,
    pub StartSample: u64,
    pub EndLine: u64,
    pub EndSample: u64,
    pub Identifier: String,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub enum Orientation {
    UP,
    DOWN,
    LEFT,
    RIGHT,
    ARBITRARY,
}