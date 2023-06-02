use serde::Deserialize;
use super::{XYZ, LL, LLH};

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct GeoData {
    pub EarthModel: EarthModel,
    pub SCP: SCP,
    pub ImageCorners: ImageCorners,
    pub ValidData: Option<ValidDataLL>,
    pub GeoInfo: Option<Vec<GeoInfo>>,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub enum EarthModel {
    WGS_84,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct SCP {
    pub ECF: XYZ,
    pub LLH: LLH,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct ImageCorners {
    pub ICP: Vec<ICP>,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct ICP {
    pub index: String,
    pub Lat: f64,
    pub Lon: f64,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct ValidDataLL {
    pub size: u64,
    pub Vertex: Vec<VertexLL>,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct VertexLL {
    pub index: usize,
    pub Lat: f64,
    pub Lon: f64,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct GeoInfo {
    pub name: String,
    pub Desc: Option<Vec<String>>,
    pub Point: Option<LL>,
    pub Line: Option<Line>,
    pub Polygon: Option<Polygon>,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Line {
    pub size: u64,
    pub Endpoint: Vec<Endpoint>,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Endpoint {
    pub index: usize,
    pub Lat: f64,
    pub Lon: f64,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Polygon {
    pub size: u64,
    pub Vertex: Vec<VertexLL>,
}