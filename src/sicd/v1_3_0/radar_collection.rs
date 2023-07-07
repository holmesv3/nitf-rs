use super::{DualPolarization, IdxLLH, Parameter, SinglePolarization, XYZ};
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct RadarCollection {
    #[serde(rename = "TxFrequency")]
    pub tx_frequency: TxFrequency,
    #[serde(rename = "RefFreqIndex")]
    pub ref_freq_index: Option<u64>,
    #[serde(rename = "Waveform")]
    pub waveform: Option<Waveform>,
    #[serde(rename = "TxPolarization")]
    pub tx_polarization: TxPolarization,
    #[serde(rename = "TxSequence")]
    pub tx_sequence: Option<TxSequence>,
    #[serde(rename = "RcvChannels")]
    pub rcv_channels: RcvChannels,
    #[serde(rename = "Area")]
    pub area: Option<Area>,
    #[serde(rename = "Parameter")]
    pub parameters: Option<Vec<Parameter>>,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct TxFrequency {
    #[serde(rename = "Min")]
    pub min: f64,
    #[serde(rename = "Max")]
    pub max: f64,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Waveform {
    #[serde(rename = "@size")]
    pub size: u64,
    #[serde(rename = "WFParameters")]
    pub wf_parameters: Vec<WFParameters>,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct WFParameters {
    #[serde(rename = "@index")]
    pub index: usize,
    #[serde(rename = "TxPulseLength")]
    pub tx_pulse_length: Option<f64>,
    #[serde(rename = "TxRFBandwidth")]
    pub tx_rf_bandwidth: Option<f64>,
    #[serde(rename = "TxFreqStart")]
    pub tx_freq_start: Option<f64>,
    #[serde(rename = "TxFMRate")]
    pub tx_fm_rate: Option<f64>,
    #[serde(rename = "RcvDemodType")]
    pub rcv_demod_type: Option<RcvDemodType>,
    #[serde(rename = "RcvWindowLength")]
    pub rcv_window_length: Option<f64>,
    #[serde(rename = "ADCSampleRate")]
    pub adc_sample_rate: Option<f64>,
    #[serde(rename = "RcvIFBandwidth")]
    pub rcv_if_bandwidth: Option<f64>,
    #[serde(rename = "RcvFreqStart")]
    pub rcv_freq_start: Option<f64>,
    #[serde(rename = "RcvFMRate")]
    pub rcv_fm_rate: Option<f64>,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct RcvDemodType {
    #[serde(rename = "$text")]
    pub value: RcvDemodTypeEnum,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub enum RcvDemodTypeEnum {
    STRETCH,
    CHIRP,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct TxPolarization {
    #[serde(rename = "$text")]
    pub value: SinglePolarization,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct TxSequence {
    #[serde(rename = "@size")]
    pub size: u64,
    #[serde(rename = "TxStep")]
    pub tx_step: Vec<TxStep>,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct TxStep {
    #[serde(rename = "@index")]
    pub index: usize,
    #[serde(rename = "WFIndex")]
    pub wf_index: Option<usize>,
    #[serde(rename = "TxPolarization")]
    pub tx_polarization: Option<TxStepPolarization>,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct TxStepPolarization {
    #[serde(rename = "$text")]
    pub value: TxStepPolarizationEnum,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub enum TxStepPolarizationEnum {
    V,
    H,
    RHC,
    LHC,
    OTHER,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct RcvChannels {
    #[serde(rename = "@size")]
    pub size: u64,
    #[serde(rename = "ChanParameters")]
    pub chan_parameters: Vec<ChanParameters>,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct ChanParameters {
    #[serde(rename = "@index")]
    pub index: usize,
    #[serde(rename = "TxRcvPolarization")]
    pub tx_rcv_polarization: TxRcvPolarization,
    #[serde(rename = "RcvAPCIndex")]
    pub rcv_apc_index: Option<u64>,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct TxRcvPolarization {
    #[serde(rename = "$text")]
    pub value: DualPolarization,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Area {
    #[serde(rename = "Corner")]
    pub corner: Corner,
    #[serde(rename = "Plane")]
    pub plane: Option<Plane>,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Corner {
    #[serde(rename = "$value")]
    pub acp: Vec<IdxLLH>,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Plane {
    #[serde(rename = "RefPt")]
    pub ref_pt: RefPt,
    #[serde(rename = "XDir")]
    pub x_dir: XDir,
    #[serde(rename = "YDir")]
    pub y_dir: YDir,
    #[serde(rename = "SegmentList")]
    pub segment_list: Option<SegmentList>,
    #[serde(rename = "Orientation")]
    pub orientation: Option<Orientation>,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct RefPt {
    #[serde(rename = "@name")]
    pub name: Option<String>,
    #[serde(rename = "ECF")]
    pub ecf: XYZ,
    #[serde(rename = "Line")]
    pub line: f64,
    #[serde(rename = "Sample")]
    pub sample: f64,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct XDir {
    #[serde(rename = "UVectECF")]
    pub u_vect_ecf: XYZ,
    #[serde(rename = "LineSpacing")]
    pub line_spacing: f64,
    #[serde(rename = "NumLines")]
    pub num_lines: u64,
    #[serde(rename = "FirstLine")]
    pub first_line: u64,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct YDir {
    #[serde(rename = "UVectECF")]
    pub u_vect_ecf: XYZ,
    #[serde(rename = "SampleSpacing")]
    pub sample_spacing: f64,
    #[serde(rename = "NumSamples")]
    pub num_samples: u64,
    #[serde(rename = "FirstSample")]
    pub first_sample: u64,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct SegmentList {
    #[serde(rename = "@size")]
    pub size: Option<String>,
    #[serde(rename = "Segment")]
    pub segment: Vec<Segment>,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Segment {
    #[serde(rename = "@index")]
    pub index: usize,
    #[serde(rename = "StartLine")]
    pub start_line: u64,
    #[serde(rename = "StartSample")]
    pub start_sample: u64,
    #[serde(rename = "EndLine")]
    pub end_line: u64,
    #[serde(rename = "EndSample")]
    pub end_sample: u64,
    #[serde(rename = "Identifier")]
    pub identifier: String,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Orientation {
    #[serde(rename = "$text")]
    pub value: OrientationEnum,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub enum OrientationEnum {
    UP,
    DOWN,
    LEFT,
    RIGHT,
    ARBITRARY,
}

#[cfg(test)]
mod tests {
    use super::RadarCollection;
    use quick_xml::de::from_str;

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
            OTHER</TxRcvPolarization><RcvAPCIndex>1</RcvAPCIndex></ChanParameters>
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
