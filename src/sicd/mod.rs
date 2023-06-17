//! Sensor Independent Complex Data support

use serde::Deserialize;

// TODO: Write tests
pub mod sicd_types;
use sicd_types::{
    antenna::Antenna,
    collection_info::CollectionInfo,
    error_statistics::ErrorStatistics,
    geo_data::GeoData,
    grid::Grid,
    image_creation::ImageCreation,
    image_data::ImageData,
    image_formation::{ImageFormation, RgAzComp, Rma},
    match_info::MatchInfo,
    pfa::Pfa,
    position::Position,
    radar_collection::RadarCollection,
    radiometric::Radiometric,
    scpcoa::ScpCoa,
    timeline::Timeline,
};

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Sicd {
    #[serde(rename = "CollectionInfo")]
    pub collection_info: CollectionInfo,
    #[serde(rename = "ImageCreation")]
    pub image_creation: Option<ImageCreation>,
    #[serde(rename = "ImageData")]
    pub image_data: ImageData,
    #[serde(rename = "GeoData")]
    pub geo_data: GeoData,
    #[serde(rename = "Grid")]
    pub grid: Grid,
    #[serde(rename = "Timeline")]
    pub timeline: Timeline,
    #[serde(rename = "Position")]
    pub position: Position,
    #[serde(rename = "RadarCollection")]
    pub radar_collection: RadarCollection,
    #[serde(rename = "ImageFormation")]
    pub image_formation: ImageFormation,
    #[serde(rename = "SCPCOA")]
    pub scpcoa: ScpCoa,
    #[serde(rename = "Radiometric")]
    pub radiometric: Option<Radiometric>,
    #[serde(rename = "Antenna")]
    pub antenna: Option<Antenna>,
    #[serde(rename = "ErrorStatistics")]
    pub error_statistics: Option<ErrorStatistics>,
    #[serde(rename = "MatchInfo")]
    pub match_info: Option<MatchInfo>,
    #[serde(rename = "RgAzComp")]
    pub rg_az_comp: Option<RgAzComp>,
    #[serde(rename = "PFA")]
    pub pfa: Option<Pfa>,
    #[serde(rename = "RMA")]
    pub rma: Option<Rma>,
}
