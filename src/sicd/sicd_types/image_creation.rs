use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct ImageCreation {
    pub Application: Option<String>,
    pub DateTime: Option<String>,
    pub Site: Option<String>,
    pub Profile: Option<String>,
}
