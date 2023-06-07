use super::{LL, LLH, XYZ, IdxLL};
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
pub enum EarthModel {
    WGS_84,
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
    pub index: String,
    #[serde(rename = "Lat")]
    pub lat: f64,
    #[serde(rename = "Lon")]
    pub lon: f64,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct ValidDataLL {
    pub size: u64,
    #[serde(rename = "Vertex")]
    pub vertex: Vec<IdxLL>,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct GeoInfo {
    pub name: String,
    #[serde(rename = "Desc")]
    pub desc: Option<Vec<String>>,
    #[serde(rename = "Point")]
    pub point: Option<LL>,
    #[serde(rename = "Line")]
    pub line: Option<Line>,
    #[serde(rename = "Polygon")]
    pub polygon: Option<Polygon>,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Line {
    pub size: u64,
    #[serde(rename = "Endpoint")]
    pub endpoint: Vec<IdxLL>,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Polygon {
    pub size: u64,
    #[serde(rename = "Vertex")]
    pub vertex: Vec<IdxLL>,
}

#[cfg(test)]
mod tests {
    use super::GeoData;
    use serde_xml_rs::from_str;
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
