use super::{IdxRowCol, RowCol};
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct ImageData {
    #[serde(rename = "PixelType")]
    pub pixel_type: PixelType,
    #[serde(rename = "AmpTable")]
    pub amp_table: Option<AmpTable>,
    #[serde(rename = "NumRows")]
    pub num_rows: u64,
    #[serde(rename = "NumCols")]
    pub num_cols: u64,
    #[serde(rename = "FirstRow")]
    pub first_row: u64,
    #[serde(rename = "FirstCol")]
    pub first_col: u64,
    #[serde(rename = "FullImage")]
    pub full_image: FullImage,
    #[serde(rename = "SCPPixel")]
    pub scp_pixel: RowCol,
    #[serde(rename = "ValidData")]
    pub valid_data: Option<ValidDataRC>,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct PixelType {
    #[serde(rename = "$text")]
    pub value: PixelTypeEnum,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub enum PixelTypeEnum {
    #[serde(rename = "RE32F_IM32F")]
    RE32FIM32F,
    #[serde(rename = "RE16I_IM16I")]
    RE16IIM16I,
    #[serde(rename = "AMP8I_PHS8I")]
    AMP8IPHS8I,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct AmpTable {
    #[serde(rename = "@size")]
    pub size: u16, // 256
    #[serde(rename = "Amplitude")]
    pub amplitude: Vec<Amplitude>,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Amplitude {
    #[serde(rename = "@index")]
    pub index: u8, // [0, 255]
    #[serde(rename = "$value")]
    pub value: f64,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct FullImage {
    #[serde(rename = "NumRows")]
    pub num_rows: u64,
    #[serde(rename = "NumCols")]
    pub num_cols: u64,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct ValidDataRC {
    #[serde(rename = "@size")]
    pub size: u64,
    #[serde(rename = "Vertex")]
    pub vertex: Vec<IdxRowCol>,
}
