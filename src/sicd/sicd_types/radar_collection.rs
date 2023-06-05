use super::{Parameter, XYZ};
use serde::Deserialize;

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

#[cfg(test)]
mod tests {
    use super::RadarCollection;
    use serde_xml_rs::from_str;

    #[test]
    fn test_radar_collection() {
        let xml_str = r#"<RadarCollection><TxFrequency><Min>0</Min><Max>0</Max>
            </TxFrequency><Waveform size="1"><WFParameters index="1">
            <TxPulseLength>0</TxPulseLength><TxRFBandwidth>0</TxRFBandwidth>
            <TxFreqStart>0</TxFreqStart><TxFMRate>0</TxFMRate><RcvDemodType>
            CHIRP</RcvDemodType><RcvWindowLength>0</RcvWindowLength>
            <ADCSampleRate>0</ADCSampleRate><RcvIFBandwidth>0</RcvIFBandwidth>
            <RcvFreqStart>0</RcvFreqStart><RcvFMRate>0</RcvFMRate>
            </WFParameters></Waveform><TxPolarization>V</TxPolarization>
            <RcvChannels size="1"><ChanParameters index="1"><TxRcvPolarization>
            POL</TxRcvPolarization><RcvAPCIndex>1</RcvAPCIndex></ChanParameters>
            </RcvChannels><Area><Corner><ACP index="1"><Lat>0</Lat><Lon>0</Lon>
            <HAE>0</HAE></ACP></Corner><Plane><RefPt><ECF><X>0</X><Y>0</Y><Z>0
            </Z></ECF><Line>0</Line><Sample>0</Sample></RefPt><XDir><UVectECF>
            <X>0</X><Y>0</Y><Z>0</Z></UVectECF><LineSpacing>0</LineSpacing>
            <NumLines>0</NumLines><FirstLine>0</FirstLine></XDir><YDir>
            <UVectECF><X>0</X><Y>0</Y><Z>0</Z></UVectECF><SampleSpacing>0
            </SampleSpacing><NumSamples>0</NumSamples><FirstSample>0
            </FirstSample></YDir></Plane></Area></RadarCollection>"#;
        assert!(match from_str::<RadarCollection>(&xml_str) {
            Ok(_) => true,
            Err(_) => false,
        })
    }
}
