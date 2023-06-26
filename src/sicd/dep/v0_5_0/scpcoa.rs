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
