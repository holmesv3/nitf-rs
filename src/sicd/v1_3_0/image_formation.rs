use super::{DualPolarization, Parameter, Poly1d, Poly2d, CMPLX, XYZ};
use serde::Deserialize;
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct ImageFormation {
    #[serde(rename = "RcvChanProc")]
    pub rcv_chan_proc: RcvChanProc,
    #[serde(rename = "TxRcvPolarizationProc")]
    pub tx_rcv_polarization_proc: TxRcvPolarizationProc,
    #[serde(rename = "TStartProc")]
    pub t_start_proc: f64,
    #[serde(rename = "TEndProc")]
    pub t_end_proc: f64,
    #[serde(rename = "TxFrequencyProc")]
    pub tx_frequency_proc: TxFrequencyProc,
    #[serde(rename = "SegmentIdentifier")]
    pub segment_identifier: Option<String>,
    #[serde(rename = "ImageFormAlgo")]
    pub image_form_algo: ImageFormAlgo,
    #[serde(rename = "STBeamComp")]
    pub st_beam_comp: STBeamComp,
    #[serde(rename = "ImageBeamComp")]
    pub image_beam_comp: ImageBeamComp,
    #[serde(rename = "AzAutofocus")]
    pub az_autofocus: AzAutofocus,
    #[serde(rename = "RgAutofocus")]
    pub rg_autofocus: RgAutofocus,
    #[serde(rename = "Processing")]
    pub processing: Option<Vec<Processing>>,
    #[serde(rename = "PolarizationCalibration")]
    pub polarization_calibration: Option<PolCal>,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct TxRcvPolarizationProc {
    #[serde(rename = "$text")]
    pub value: DualPolarization,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct RcvChanProc {
    #[serde(rename = "NumChanProc")]
    pub num_chan_proc: u64,
    #[serde(rename = "PRFScaleFactor")]
    pub prf_scale_factor: Option<f64>,
    #[serde(rename = "ChanIndex")]
    pub chan_index: usize,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct TxFrequencyProc {
    #[serde(rename = "MinProc")]
    pub min_proc: f64,
    #[serde(rename = "MaxProc")]
    pub max_proc: f64,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct ImageFormAlgo {
    #[serde(rename = "$text")]
    pub value: ImageFormAlgoEnum,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub enum ImageFormAlgoEnum {
    PFA,
    RMA,
    RGAZCOMP,
    OTHER,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct STBeamComp {
    #[serde(rename = "$text")]
    pub value: STBeamCompEnum,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub enum STBeamCompEnum {
    NO,
    GLOBAL,
    SV,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct ImageBeamComp {
    #[serde(rename = "$text")]
    pub value: ImageBeamCompEnum,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub enum ImageBeamCompEnum {
    NO,
    SV,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct AzAutofocus {
    #[serde(rename = "$text")]
    pub value: AzAutofocusEnum,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub enum AzAutofocusEnum {
    NO,
    GLOBAL,
    SV,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct RgAutofocus {
    #[serde(rename = "$text")]
    pub value: RgAutofocusEnum,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub enum RgAutofocusEnum {
    NO,
    GLOBAL,
    SV,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Processing {
    #[serde(rename = "Type")]
    pub type_proc: String,
    #[serde(rename = "Applied")]
    pub applied: bool,
    #[serde(rename = "Parameter")]
    pub parameters: Option<Vec<Parameter>>,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct PolCal {
    #[serde(rename = "DistortCorrectionApplied")]
    pub distort_correction_applied: bool,
    #[serde(rename = "Distortion")]
    pub distortion: Distortion,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Distortion {
    #[serde(rename = "CalibrationDate")]
    pub calibration_date: Option<String>,
    #[serde(rename = "A")]
    pub a: f64,
    #[serde(rename = "F1")]
    pub f1: CMPLX,
    #[serde(rename = "Q1")]
    pub q1: CMPLX,
    #[serde(rename = "Q2")]
    pub q2: CMPLX,
    #[serde(rename = "F2")]
    pub f2: CMPLX,
    #[serde(rename = "Q3")]
    pub q3: CMPLX,
    #[serde(rename = "Q4")]
    pub q4: CMPLX,
    #[serde(rename = "GainErrorA")]
    pub gain_error_a: Option<f64>,
    #[serde(rename = "GainErrorF1")]
    pub gain_error_f1: Option<f64>,
    #[serde(rename = "GainErrorF2")]
    pub gain_error_f2: Option<f64>,
    #[serde(rename = "PhaseErrorF1")]
    pub phase_error_f1: Option<f64>,
    #[serde(rename = "PhaseErrorF2")]
    pub phase_error_f2: Option<f64>,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct RgAzComp {
    #[serde(rename = "AzSF")]
    pub az_sf: f64,
    #[serde(rename = "KazPoly")]
    pub kaz_poly: Poly1d,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Rma {
    #[serde(rename = "RMAlgoType")]
    pub rm_algo_type: RMAlgoType,
    #[serde(rename = "ImageType")]
    pub image_type: ImageType,
    #[serde(rename = "RMAT")]
    pub rmat: Option<RMAlgo>,
    #[serde(rename = "RMCR")]
    pub rmcr: Option<RMAlgo>,
    #[serde(rename = "INCA")]
    pub inca: Option<INCA>,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct RMAlgoType {
    #[serde(rename = "$text")]
    pub value: RMAlgoTypeEnum,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub enum RMAlgoTypeEnum {
    #[serde(rename = "OMEGA_K")]
    OMEGAK,
    CSA,
    #[serde(rename = "RG_DOP")]
    RGDOP,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct ImageType {
    #[serde(rename = "$text")]
    pub value: ImageTypeEnum,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub enum ImageTypeEnum {
    RMAT,
    RMCR,
    INCA,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct RMAlgo {
    #[serde(rename = "PosRef")]
    pub pos_ref: XYZ,
    #[serde(rename = "VelRef")]
    pub vel_ref: XYZ,
    #[serde(rename = "DopConeAngRef")]
    pub dop_cone_ang_ref: f64,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct INCA {
    #[serde(rename = "TimeCAPoly")]
    pub time_ca_poly: Poly1d,
    #[serde(rename = "R_CA_SCP")]
    pub r_ca_scp: f64,
    #[serde(rename = "FreqZero")]
    pub freq_zero: f64,
    #[serde(rename = "DRateSFPoly")]
    pub d_rate_sf_poly: Poly2d,
    #[serde(rename = "DopCentroidPoly")]
    pub dop_centroid_poly: Option<Poly2d>,
    #[serde(rename = "DopCentroidCOA")]
    pub dop_centroid_coa: Option<bool>,
}

#[cfg(test)]
mod tests {
    use super::ImageFormation;
    use quick_xml::de::from_str;

    #[test]
    fn test_image_formation() {
        let xml_str = r#"<ImageFormation><RcvChanProc><NumChanProc>1
            </NumChanProc><ChanIndex>1</ChanIndex></RcvChanProc>
            <TxRcvPolarizationProc>V:V</TxRcvPolarizationProc><TStartProc>0
            </TStartProc><TEndProc>0</TEndProc><TxFrequencyProc><MinProc>0
            </MinProc><MaxProc>0</MaxProc></TxFrequencyProc><ImageFormAlgo>
            PFA</ImageFormAlgo><STBeamComp>NO</STBeamComp><ImageBeamComp>NO
            </ImageBeamComp><AzAutofocus>NO</AzAutofocus><RgAutofocus>NO
            </RgAutofocus><Processing><Type>Processing</Type><Applied>true
            </Applied><Parameter name="param">true</Parameter></Processing>
            </ImageFormation>"#;
        assert!(match from_str::<ImageFormation>(&xml_str) {
            Ok(_) => true,
            Err(_) => false,
        })
    }
    // TODO: Test RgAzComp, RMA
}
