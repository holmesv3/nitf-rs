use super::XYZ;
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct ScpCoa {
    #[serde(rename = "SCPTime")]
    pub scp_time: f64,
    #[serde(rename = "ARPPos")]
    pub arp_pos: XYZ,
    #[serde(rename = "ARPVel")]
    pub arp_vel: XYZ,
    #[serde(rename = "ARPAcc")]
    pub arp_acc: XYZ,
    #[serde(rename = "SideOfTrack")]
    pub side_of_track: SideOfTrack,
    #[serde(rename = "SlantRange")]
    pub slant_range: f64,
    #[serde(rename = "GroundRange")]
    pub ground_range: f64,
    #[serde(rename = "DopplerConeAng")]
    pub doppler_cone_ang: f64,
    #[serde(rename = "GrazeAng")]
    pub graze_ang: f64,
    #[serde(rename = "IncidenceAng")]
    pub incidence_ang: f64,
    #[serde(rename = "TwistAng")]
    pub twist_ang: f64,
    #[serde(rename = "SlopeAng")]
    pub slope_ang: f64,
    #[serde(rename = "AzimAng")]
    pub azim_ang: f64,
    #[serde(rename = "LayoverAng")]
    pub layover_ang: f64,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct SideOfTrack {
    #[serde(rename = "$text")]
    pub value: SideOfTrackEnum,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub enum SideOfTrackEnum {
    L,
    R,
}

#[cfg(test)]
mod tests {
    use super::ScpCoa;
    use quick_xml::de::from_str;

    #[test]
    fn test_scpcoa() {
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
