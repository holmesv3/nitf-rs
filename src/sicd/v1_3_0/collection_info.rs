use serde::Deserialize;

use super::Parameter;
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct CollectionInfo {
    #[serde(rename = "CollectorName")]
    pub collector_name: String,
    #[serde(rename = "IlluminatorName")]
    pub illuminator_name: Option<String>,
    #[serde(rename = "CoreName")]
    pub core_name: String,
    #[serde(rename = "CollectType")]
    pub collect_type: Option<CollectType>,
    #[serde(rename = "RadarMode")]
    pub radar_mode: RadarMode,
    #[serde(default = "default_classification")]
    #[serde(rename = "Classification")]
    pub classification: String,
    #[serde(rename = "CountryCode")]
    pub country_code: Option<Vec<String>>,
    #[serde(rename = "Parameter")]
    pub parameters: Option<Vec<Parameter>>,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct CollectType {
    #[serde(rename = "$text")]
    pub value: CollectTypeEnum,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub enum CollectTypeEnum {
    MONOSTATIC,
    BISTATIC,
}
fn default_classification() -> String {
    String::from("UNCLASSIFIED")
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct RadarMode {
    #[serde(rename = "ModeType")]
    pub mode_type: ModeType,
    #[serde(rename = "ModeID")]
    pub mode_id: Option<String>,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct ModeType {
    #[serde(rename = "$text")]
    pub value: ModeTypeEnum,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub enum ModeTypeEnum {
    SPOTLIGHT,
    STRIPMAP,
    #[serde(rename = "DYNAMIC STRIPMAP")]
    DYNAMICSTRIPMAP,
}

#[cfg(test)]
mod tests {
    use super::CollectionInfo;
    use quick_xml::de::from_str;

    #[test]
    fn test_collection_info() {
        let xml_str = r#"<CollectionInfo><CollectorName></CollectorName>
            <CoreName></CoreName><RadarMode><ModeType>SPOTLIGHT</ModeType>
            </RadarMode><Classification>UNCLASSIFIED</Classification>
            <Parameter name="param1">value</Parameter></CollectionInfo>"#;
        assert!(match from_str::<CollectionInfo>(&xml_str) {
            Ok(_) => true,
            Err(_) => false,
        });
    }
}
