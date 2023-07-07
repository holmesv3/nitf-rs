//! Sensor Independent Complex Data support
use quick_xml::de::from_str;
use serde::Deserialize;
use std::fs::File;
use std::path::Path;
use std::str::{from_utf8, FromStr};
use thiserror::Error;

use crate::nitf::Nitf;

pub mod dep;
pub mod v1_3_0;

#[derive(Error, Debug)]
pub enum SicdError {
    #[error("unknown sicd version {0}")]
    VersionError(String),
    #[error("metadata for version {0} is not implemented")]
    Unimpl(String),
}

/// SICD file structure
#[derive(Debug)]
pub struct Sicd {
    /// Nitf file object and associated metadata
    pub nitf: Nitf,
    /// Parsed SICD xml metadata
    pub meta: SicdMeta,
    /// SICD Version
    pub version: SicdVersion,
}

#[derive(Debug, Deserialize)]
pub enum SicdVersion {
    V0_3_1,
    V0_4_0,
    V0_4_1,
    V0_5_0,
    V1_0_0,
    V1_0_1,
    V1_1_0,
    V1_2_0,
    V1_2_1,
    V1_3_0,
}

impl FromStr for SicdVersion {
    type Err = SicdError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split("urn:SICD:").collect::<String>().as_str() {
            "0.3.1" => Ok(SicdVersion::V0_3_1),
            "0.4.0" => Ok(SicdVersion::V0_4_0),
            "0.4.1" => Ok(SicdVersion::V0_4_1),
            "0.5.0" => Ok(SicdVersion::V0_5_0),
            "1.0.0" => Ok(SicdVersion::V1_0_0),
            "1.0.1" => Ok(SicdVersion::V1_0_1),
            "1.1.0" => Ok(SicdVersion::V1_1_0),
            "1.2.0" => Ok(SicdVersion::V1_2_0),
            "1.2.1" => Ok(SicdVersion::V1_2_1),
            "1.3.0" => Ok(SicdVersion::V1_3_0),
            _ => Err(SicdError::VersionError(s.to_string())),
        }
    }
}

#[derive(Debug)]
pub enum SicdMeta {
    V0_3_1,
    V0_4_0(dep::v0_4_0::SicdMeta),
    V0_4_1,
    V0_5_0(dep::v0_5_0::SicdMeta),
    V1(v1_3_0::SicdMeta),
}

/// Construct a [Sicd] object from a file `path`.
///
/// # Example
///
///     use std::path::Path;
///     use nitf_rs::sicd::read_sicd;
///
///     let sicd_path = Path::new("../example.nitf");
///     let sicd = read_sicd(sicd_path);
pub fn read_sicd(path: &Path) -> Sicd {
    let mut file = File::open(path).unwrap();
    Sicd::from_file(&mut file)
}

impl Sicd {
    pub fn from_file(file: &mut File) -> Self {
        let nitf = Nitf::from_file(file);
        let sicd_str = from_utf8(&nitf.data_extension_segments[0].data[..]).unwrap();
        let (version, meta) = parse_sicd(sicd_str).unwrap();
        Self {
            nitf,
            meta,
            version,
        }
    }
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
struct VersionGetter {
    #[serde(rename = "@xmlns")]
    pub version: String,
}

fn parse_sicd(sicd_str: &str) -> Result<(SicdVersion, SicdMeta), SicdError> {
    // This feels bad
    let tmp: VersionGetter = from_str(&sicd_str).unwrap();
    let sicd_version = SicdVersion::from_str(&tmp.version).unwrap();
    use SicdError::Unimpl;
    match sicd_version {
        SicdVersion::V0_3_1 => Err(Unimpl("V0_3_1".to_string())),
        SicdVersion::V0_4_0 => Ok((
            SicdVersion::V0_4_0,
            SicdMeta::V0_4_0(from_str(sicd_str).unwrap()),
        )),
        SicdVersion::V0_4_1 => Err(Unimpl("V0_4_1".to_string())),
        SicdVersion::V0_5_0 => Ok((
            SicdVersion::V0_5_0,
            SicdMeta::V0_5_0(from_str(sicd_str).unwrap()),
        )),
        // Don't need to worry about anything else, all versions past 1.0 are backwards compatible
        other => Ok((other, SicdMeta::V1(from_str(sicd_str).unwrap()))),
    }
}
