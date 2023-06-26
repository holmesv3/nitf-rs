//! Sensor Independent Complex Data support
use quick_xml::de::from_str;
use std::fs::File;
use std::path::Path;
use std::str::from_utf8;

use crate::nitf::Nitf;

use self::v1_3_0::SicdMeta;

pub mod v1_3_0;
pub mod dep;

/// SICD file structure
#[derive(Debug)]
pub struct Sicd {
    /// Nitf file object and associated metadata
    pub nitf: Nitf,
    /// Parsed SICD xml metadata
    pub sicd_meta: SicdMeta,
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
    let mut reader = File::open(path).unwrap();
    Sicd::from_file(&mut reader)
}

impl Sicd {
    pub fn from_file(reader: &mut File) -> Self {
        let nitf = Nitf::from_file(reader);
        let sicd_str = from_utf8(&nitf.data_extension_segments[0].data[..]).unwrap();
        let sicd_meta: SicdMeta = from_str(sicd_str).unwrap();
        Self { nitf, sicd_meta }
    }
}
