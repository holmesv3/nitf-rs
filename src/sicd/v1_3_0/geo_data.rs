use super::{IdxLL, LL, LLH, XYZ};
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct GeoData {
    #[serde(rename = "EarthModel")]
    pub earth_model: EarthModel,
    #[serde(rename = "SCP")]
    pub scp: SCP,
    #[serde(rename = "ImageCorners")]
    pub image_corners: ImageCorners,
    #[serde(rename = "ValidData")]
    pub valid_data: Option<ValidDataLL>,
    #[serde(rename = "GeoInfo")]
    pub geo_info: Option<Vec<GeoInfo>>,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct EarthModel {
    #[serde(rename = "$text")]
    pub value: EarthModelEnum,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub enum EarthModelEnum {
    #[serde(rename = "WGS_84")]
    WGS84,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct SCP {
    #[serde(rename = "ECF")]
    pub ecf: XYZ,
    #[serde(rename = "LLH")]
    pub llh: LLH,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct ImageCorners {
    #[serde(rename = "ICP")]
    pub icp: Vec<ICP>,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct ICP {
    #[serde(rename = "@index")]
    pub index: String,
    #[serde(rename = "Lat")]
    pub lat: f64,
    #[serde(rename = "Lon")]
    pub lon: f64,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct ValidDataLL {
    #[serde(rename = "@size")]
    pub size: u64,
    #[serde(rename = "Vertex")]
    pub vertex: Vec<IdxLL>,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct GeoInfo {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "Desc")]
    pub desc: Option<Vec<Desc>>,
    #[serde(rename = "Point")]
    pub point: Option<LL>,
    #[serde(rename = "Line")]
    pub line: Option<Line>,
    #[serde(rename = "Polygon")]
    pub polygon: Option<Polygon>,
    #[serde(rename = "GeoInfo")]
    pub geo_info: Option<Vec<GeoInfo>>,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Desc {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "$value")]
    pub value: String,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Line {
    #[serde(rename = "@size")]
    pub size: u64,
    #[serde(rename = "Endpoint")]
    pub endpoint: Vec<IdxLL>,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Polygon {
    #[serde(rename = "@size")]
    pub size: u64,
    #[serde(rename = "Vertex")]
    pub vertex: Vec<IdxLL>,
}

#[cfg(test)]
mod tests {
    use super::GeoData;
    use quick_xml::de::from_str;
    #[test]
    fn test_geo_data() {
        let xml_str = r#"<GeoData><EarthModel>WGS_84</EarthModel>
            <SCP><ECF><X>0</X><Y>0</Y><Z>0</Z></ECF><LLH><Lat>0</Lat><Lon>0
            </Lon><HAE>0</HAE></LLH></SCP><ImageCorners><ICP index="1:FRFC">
            <Lat>0</Lat><Lon>0</Lon></ICP><ICP index="2:FRLC"><Lat>0</Lat><Lon>
            0</Lon></ICP><ICP index="3:LRLC"><Lat>0</Lat><Lon>0</Lon></ICP>
            <ICP index="4:LRFC"><Lat>0</Lat><Lon>0</Lon></ICP></ImageCorners>
            <ValidData size="1"><Vertex index="1"><Lat>0</Lat><Lon>0</Lon>
            </Vertex></ValidData></GeoData>"#;
        assert!(match from_str::<GeoData>(&xml_str) {
            Ok(_) => true,
            Err(_) => false,
        })
    }
}
