use super::RowCol;
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct ImageData {
    pub PixelType: PixelType,
    pub AmpTable: Option<AmpTable>,
    pub NumRows: u64,
    pub NumCols: u64,
    pub FirstRow: u64,
    pub FirstCol: u64,
    pub FullImage: FullImage,
    pub SCPPixel: RowCol,
    pub ValidData: Option<ValidDataRC>,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub enum PixelType {
    RE32F_IM32F,
    RE16I_IM16I,
    AMP8I_PHS8I,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct AmpTable {
    pub size: u16,
    pub Amplitude: Vec<Amplitude>,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Amplitude {
    pub index: u8,
    #[serde(rename = "$value")]
    pub value: f64,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct FullImage {
    pub NumRows: u64,
    pub NumCols: u64,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct ValidDataRC {
    pub size: u64,
    pub Vertex: Vec<VertexRC>,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct VertexRC {
    pub index: usize,
    pub Row: u64,
    pub Col: u64,
}

#[cfg(test)]
mod tests {
    use super::ImageData;
    use serde_xml_rs::from_str;

    #[test]
    fn test_image_data() {
        let xml_str = r#"<ImageData><PixelType>RE32F_IM32F</PixelType><NumRows>0
            </NumRows><NumCols>10077</NumCols><FirstRow>0</FirstRow><FirstCol>0
            </FirstCol><FullImage><NumRows>0</NumRows><NumCols>0</NumCols>
            </FullImage><SCPPixel><Row>0</Row><Col>0</Col></SCPPixel>
            <ValidData size="2"><Vertex index="1"><Row>0</Row><Col>0</Col>
            </Vertex><Vertex index="2"><Row>0</Row><Col>0</Col></Vertex>
            </ValidData></ImageData>"#;
        assert!(match from_str::<ImageData>(xml_str) {
            Ok(_) => true,
            Err(_) => false,
        })
    }
}
