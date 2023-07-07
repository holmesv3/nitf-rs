use super::Parameter;
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct MatchInfo {
    #[serde(rename = "NumMatchTypes")]
    pub num_match_types: u64,
    #[serde(rename = "MatchType")]
    pub match_type: Vec<MatchType>,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct MatchType {
    #[serde(rename = "@index")]
    pub index: usize,
    #[serde(rename = "TypeID")]
    pub type_id: String,
    #[serde(rename = "CurrentIndex")]
    pub current_index: Option<usize>,
    #[serde(rename = "NumMatchCollections")]
    pub num_match_collections: u64,
    #[serde(rename = "MatchCollection")]
    pub match_collection: Option<Vec<MatchCollection>>,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct MatchCollection {
    #[serde(rename = "@index")]
    pub index: usize,
    #[serde(rename = "CoreName")]
    pub core_name: String,
    #[serde(rename = "MatchIndex")]
    pub match_index: Option<usize>,
    #[serde(rename = "Parameter")]
    pub parameters: Option<Vec<Parameter>>,
}
