use super::Parameter;
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct MatchInfo {
    #[serde(rename = "@size")]
    pub size: usize,
    #[serde(rename = "Collect")]
    pub collect: Vec<Collect>,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Collect {
    #[serde(rename = "@index")]
    pub index: usize,
    #[serde(rename = "CollectorName")]
    pub collector_name: String,
    #[serde(rename = "IlluminatorName")]
    pub illuminator_name: Option<String>,
    #[serde(rename = "CoreName")]
    pub core_name: String,
    #[serde(rename = "MatchType")]
    pub match_type: Option<Vec<String>>,
    #[serde(rename = "Parameter")]
    pub parameter: Parameter,
}
