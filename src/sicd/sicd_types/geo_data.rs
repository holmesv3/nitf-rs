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



#[cfg(test)]
mod tests {
    use super::GeoData;
    use serde_xml_rs::from_str;
    #[test]
    fn test_geo_data () {
        let xml_str = r#"<GeoData><EarthModel>WGS_84</EarthModel>
            <SCP><ECF><X>0</X><Y>0</Y><Z>0</Z></ECF><LLH><Lat>0</Lat><Lon>0
            </Lon><HAE>0</HAE></LLH></SCP><ImageCorners><ICP index="1:FRFC">
            <Lat>0</Lat><Lon>0</Lon></ICP><ICP index="2:FRLC"><Lat>0</Lat><Lon>
            0</Lon></ICP><ICP index="3:LRLC"><Lat>0</Lat><Lon>0</Lon></ICP>
            <ICP index="4:LRFC"><Lat>0</Lat><Lon>0</Lon></ICP></ImageCorners>
            <ValidData size="1"><Vertex index="1"><Lat>0</Lat><Lon>0</Lon>
            </Vertex></ValidData></GeoData>"#;
        assert!(match from_str::<GeoData>(&xml_str) {
            Ok(_) => true, Err(_) => false,
        }) 
    }
}