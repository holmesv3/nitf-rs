use serde::Deserialize;
use super::{Parameter, XYZ, CMPLX, Poly1D, Poly2D};
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct ImageFormation {
    pub RcvChanProc: RcvChanProc,
    pub TxRcvPolarizationProc: String, // TODO: implement this enum
    pub TStartProc: f64,
    pub TEndProc: f64,
    pub TxFrequencyProc: TxFrequencyProc,
    pub SegmentIdentifier: Option<String>,
    pub ImageFormAlgo: ImageFormAlgo,
    pub STBeamComp: STBeamComp,
    pub ImageBeamComp: ImageBeamComp,
    pub AzAutofocus: AzAutofocus,
    pub RgAutofocus: RgAutofocus,
    pub Processing: Option<Vec<Processing>>,
    pub PolarizationCalibration: Option<PolCal>,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct RcvChanProc {
    pub NumChanProc: u64,
    pub PRFScaleFactor: Option<f64>,
    pub ChanIndex: usize,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct TxFrequencyProc {
    pub MinProc: f64,
    pub MaxProc: f64,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub enum ImageFormAlgo {
    PFA,
    RMA,
    RGAZCOMP,
    OTHER,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub enum STBeamComp {
    NO,
    GLOBAL,
    SV,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub enum ImageBeamComp {
    NO,
    SV,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub enum AzAutofocus {
    NO,
    GLOBAL,
    SV,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub enum RgAutofocus {
    NO,
    GLOBAL,
    SV,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Processing {
    pub Type: String,
    pub Applied: bool,
    pub Parameter: Parameter,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct PolCal {
    pub DistortCorrectionApplied: bool,
    pub Distortion: Distortion,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Distortion {
    pub CalibrationDate: Option<String>,
    pub A: f64,
    pub F1: CMPLX,
    pub Q1: CMPLX,
    pub Q2: CMPLX,
    pub F2: CMPLX,
    pub Q3: CMPLX,
    pub Q4: CMPLX,
    pub GainErrorA: Option<f64>,
    pub GainErrorF1: Option<f64>,
    pub GainErrorF2: Option<f64>,
    pub PhaseErrorF1: Option<f64>,
    pub PhaseErrorF2: Option<f64>,
}


#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct RgAzComp {
    pub AzSF: f64,
    pub KazPoly: Poly1D,
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

#[cfg(test)]
mod tests {
    use super::{ImageFormation};
    use serde_xml_rs::from_str;

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
                Ok(_) => true, Err(_) => false,
        }) 
    }
    // TODO: Test RgAzComp, RMA
}