use serde::Deserialize;
use super::Parameter;

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct ErrorStatistics {
    pub CompositeSCP: Option<CompositeSCP>,
    pub Components: Option<Components>,
    pub AdditionalParams: Parameter,
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

#[cfg(test)]
mod tests {
    use super::ErrorStatistics;
    use serde_xml_rs::from_str;

    #[test]
    fn test_errror_statistics () {
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
            Ok(_) => true, Err(_) => false,
        }) 
    }
}