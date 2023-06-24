use super::Parameter;
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct ErrorStatistics {
    #[serde(rename = "CompositeSCP")]
    pub composite_scp: Option<CompositeSCP>,
    #[serde(rename = "Components")]
    pub components: Option<Components>,
    #[serde(rename = "Unmodeled")]
    pub unmodeled: Option<Unmodeled>,
    #[serde(rename = "AdditionalParams")]
    pub additional_params: Option<Vec<Parameter>>,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct CompositeSCP {
    #[serde(rename = "Rg")]
    pub rg: f64,
    #[serde(rename = "Az")]
    pub az: f64,
    #[serde(rename = "RgAz")]
    pub rg_az: f64,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Components {
    #[serde(rename = "PosVelErr")]
    pub pos_vel_err: PosVelErr,
    #[serde(rename = "RadarSensor")]
    pub radar_sensor: RadarSensor,
    #[serde(rename = "TropoErro")]
    pub tropo_erro: Option<TropoError>,
    #[serde(rename = "IonoError")]
    pub iono_error: Option<IonoError>,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct PosVelErr {
    #[serde(rename = "Frame")]
    pub frame: Frame,
    #[serde(rename = "P1")]
    pub p1: f64,
    #[serde(rename = "P2")]
    pub p2: f64,
    #[serde(rename = "P3")]
    pub p3: f64,
    #[serde(rename = "V1")]
    pub v1: f64,
    #[serde(rename = "V2")]
    pub v2: f64,
    #[serde(rename = "V3")]
    pub v3: f64,
    #[serde(rename = "CorrCoefs")]
    pub corr_coefs: Option<CorrCoefs>,
    #[serde(rename = "PositionDecorr")]
    pub position_decorr: Option<Decorr>,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Frame {
    #[serde(rename = "$text")]
    pub value: FrameEnum,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub enum FrameEnum {
    ECF,
    #[serde(rename = "RIC_ECF")]
    RICECF,
    #[serde(rename = "RIC_ECI")]
    RICECI,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct CorrCoefs {
    #[serde(rename = "P1P2")]
    pub p1p2: f64,
    #[serde(rename = "P1P3")]
    pub p1p3: f64,
    #[serde(rename = "P1V1")]
    pub p1v1: f64,
    #[serde(rename = "P1V2")]
    pub p1v2: f64,
    #[serde(rename = "P1V3")]
    pub p1v3: f64,
    #[serde(rename = "P2P3")]
    pub p2p3: f64,
    #[serde(rename = "P2V1")]
    pub p2v1: f64,
    #[serde(rename = "P2V2")]
    pub p2v2: f64,
    #[serde(rename = "P2V3")]
    pub p2v3: f64,
    #[serde(rename = "P3V1")]
    pub p3v1: f64,
    #[serde(rename = "P3V2")]
    pub p3v2: f64,
    #[serde(rename = "P3V3")]
    pub p3v3: f64,
    #[serde(rename = "V1V2")]
    pub v1v2: f64,
    #[serde(rename = "V1V3")]
    pub v1v3: f64,
    #[serde(rename = "V2V3")]
    pub v2v3: f64,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct RadarSensor {
    #[serde(rename = "RangeBias")]
    pub range_bias: f64,
    #[serde(rename = "ClockFreqSF")]
    pub clock_freq_sf: Option<f64>,
    #[serde(rename = "TransmitFreqSF")]
    pub transmit_freq_sf: Option<f64>,
    #[serde(rename = "RangeBiasDecorr")]
    pub range_bias_decorr: Option<Decorr>,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct TropoError {
    #[serde(rename = "TropoRangeVertical")]
    pub tropo_range_vertical: Option<f64>,
    #[serde(rename = "TropoRangeSlant")]
    pub tropo_range_slant: Option<f64>,
    #[serde(rename = "TropoRangeDecorr")]
    pub tropo_range_decorr: Option<Decorr>,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct IonoError {
    #[serde(rename = "IonoRangeVertical")]
    pub iono_range_vertical: Option<f64>,
    #[serde(rename = "IonoRangeRateVertical")]
    pub iono_range_rate_vertical: Option<f64>,
    #[serde(rename = "IonoRgRgRateCC")]
    pub iono_rg_rg_rate_cc: f64,
    #[serde(rename = "IonoRangeVertDecorr")]
    pub iono_range_vert_decorr: Option<Decorr>,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Unmodeled {
    #[serde(rename = "Xrow")]
    pub xrow: f64,
    #[serde(rename = "Ycol")]
    pub ycol: f64,
    #[serde(rename = "XrowYcol")]
    pub xrow_ycol: f64,
    #[serde(rename = "UnmodeledDecorr")]
    pub unmodeled_decorr: Option<UnmodeledDecorr>,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct UnmodeledDecorr {
    #[serde(rename = "Xrow")]
    pub xrow: Decorr,
    #[serde(rename = "Ycol")]
    pub ycol: Decorr,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Decorr {
    #[serde(rename = "CorrCoefZero")]
    pub corr_coef_zero: f64,
    #[serde(rename = "DecorrRate")]
    pub decorr_rate: f64,
}

#[cfg(test)]
mod tests {
    use super::ErrorStatistics;
    use quick_xml::de::from_str;

    #[test]
    fn test_errror_statistics() {
        let xml_str = r#"<ErrorStatistics><Components><PosVelErr><Frame>RIC_ECI
            </Frame><P1>0</P1><P2>0</P2><P3>0</P3><V1>0</V1><V2>0</V2><V3>0</V3>
            <CorrCoefs><P1P2>0</P1P2><P1P3>0</P1P3><P1V1>0</P1V1><P1V2>0</P1V2>
            <P1V3>0</P1V3><P2P3>0</P2P3><P2V1>0</P2V1><P2V2>0</P2V2><P2V3>0
            </P2V3><P3V1>0</P3V1><P3V2>0</P3V2><P3V3>0</P3V3><V1V2>0</V1V2>
            <V1V3>0</V1V3><V2V3>0</V2V3></CorrCoefs></PosVelErr><RadarSensor>
            <RangeBias>0</RangeBias><ClockFreqSF>0</ClockFreqSF><TransmitFreqSF>
            0</TransmitFreqSF></RadarSensor><TropoError><TropoRangeVertical>0
            </TropoRangeVertical><TropoRangeSlant>0</TropoRangeSlant>
            </TropoError></Components></ErrorStatistics>"#;
        assert!(match from_str::<ErrorStatistics>(&xml_str) {
            Ok(_) => true,
            Err(_) => false,
        })
    }
}
