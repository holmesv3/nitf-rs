use serde::Deserialize;
use super::RowCol;

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