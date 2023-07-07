use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct ImageCreation {
    #[serde(rename = "Application")]
    pub application: Option<String>,
    #[serde(rename = "DateTime")]
    pub date_time: Option<String>,
    #[serde(rename = "Site")]
    pub site: Option<String>,
    #[serde(rename = "Profile")]
    pub profile: Option<String>,
}
