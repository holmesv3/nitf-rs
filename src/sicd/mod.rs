//! Sensor Independent Complex Data support
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use serde::Deserialize;

// TODO: Write tests
pub mod sicd_types;
use sicd_types::{
    collection_info::CollectionInfo,
    image_creation::ImageCreation,
    image_data::ImageData,
    geo_data::GeoData,
    grid::Grid,
    timeline::Timeline,
    position::Position,
    radar_collection::RadarCollection,
    image_formation::{ImageFormation, RgAzComp, Rma},
    scpcoa::ScpCoa,
    radiometric::Radiometric,
    antenna::Antenna,
    error_statistics::ErrorStatistics,
    match_info::MatchInfo,
    pfa::Pfa,
};

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Sicd {
    pub CollectionInfo: CollectionInfo,
    pub ImageCreation: Option<ImageCreation>,
    pub ImageData: ImageData,
    pub GeoData: GeoData,
    pub Grid: Grid,
    pub Timeline: Timeline,
    pub Position: Position,
    pub RadarCollection: RadarCollection,
    pub ImageFormation: ImageFormation,
    pub SCPCOA: ScpCoa,
    pub Radiometric: Option<Radiometric>,
    pub Antenna: Option<Antenna>,
    pub ErrorStatistics: Option<ErrorStatistics>,
    pub MatchInfo: Option<MatchInfo>,
    pub RgAzComp: Option<RgAzComp>,
    pub PFA: Option<Pfa>,
    pub RMA: Option<Rma>,
}

