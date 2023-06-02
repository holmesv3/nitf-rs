use serde::Deserialize;
use super::Parameter;
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct CollectionInfo {
    pub CollectorName: String,
    pub IlluminatorName: Option<String>,
    pub CoreName: String,
    pub CollectType: Option<CollectType>,
    pub RadarMode: RadarMode,
    #[serde(default = "default_classification")]
    pub Classification: String,
    pub CountryCode: Option<Vec<String>>,
    pub Parameter: Parameter,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub enum CollectType {
    MONOSTATIC,
    BISTATIC,
}
fn default_classification() -> String {
    String::from("UNCLASSIFIED")
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct RadarMode {
    pub ModeType: ModeType,
    pub ModeID: Option<String>,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub enum ModeType {
    SPOTLIGHT,
    STRIPMAP,
    #[serde(rename = "DYNAMIC STRIPMAP")]
    DYNAMIC_STRIPMAP,
}

