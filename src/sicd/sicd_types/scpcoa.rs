use serde::Deserialize;
use super::XYZ;

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