use serde::Deserialize;
use super::Parameter;

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct MatchInfo {
    pub NumMatchTypes: u64,
    pub MatchType: Vec<MatchType>,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct MatchType {
    pub index: usize,
    pub TypeID: String,
    pub CurrentIndex: Option<usize>,
    pub NumMatchCollections: u64,
    pub MatchCollection: Option<Vec<MatchCollection>>,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct MatchCollection {
    pub index: usize,
    pub CoreName: String,
    pub MatchIndex: Option<usize>,
    pub Parameter: Parameter,
}