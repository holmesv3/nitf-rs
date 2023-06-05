use super::XYZ;
use serde::Deserialize;

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

#[cfg(test)]
mod tests {
    use super::ScpCoa;
    use serde_xml_rs::from_str;

    #[test]
    fn test_image_formation() {
        let xml_str = r#"<SCPCOA><SCPTime>0</SCPTime><ARPPos><X>0</X><Y>0</Y><Z>
            0</Z></ARPPos><ARPVel><X>0</X><Y>0</Y><Z>0</Z></ARPVel><ARPAcc><X>0
            </X><Y>0</Y><Z>0</Z></ARPAcc><SideOfTrack>L</SideOfTrack>
            <SlantRange>0</SlantRange><GroundRange>0</GroundRange>
            <DopplerConeAng>0</DopplerConeAng><GrazeAng>0</GrazeAng>
            <IncidenceAng>0</IncidenceAng><TwistAng>0</TwistAng><SlopeAng>0
            </SlopeAng><AzimAng>0</AzimAng><LayoverAng>0</LayoverAng></SCPCOA>"#;
        assert!(match from_str::<ScpCoa>(&xml_str) {
            Ok(_) => true,
            Err(_) => false,
        })
    }
}
