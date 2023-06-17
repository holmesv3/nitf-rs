//! Sensor Independent Complex Data support
use std::fs::File;
use std::path::Path;
use quick_xml::de::from_str;

use crate::nitf::Nitf;

use self::sicd_types::SicdMeta;

// TODO: Write tests
pub mod sicd_types;

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
///     use nitf_rs::read_sicd;
///
///     let sicd_path = Path::new("../example.nitf");
///     let sicd = read_sicd(sicd_path);
pub fn read_sicd(path: &Path) -> Sicd {
    let mut reader = File::open(path).unwrap();
    return Sicd::from_file(&mut reader);
}

impl Sicd {
    pub fn from_file(reader: &mut File) -> Self {
        let nitf = Nitf::from_file(reader);
        let sicd_str = String::from_utf8(nitf.data_extension_segments[0].data[..].to_vec()).unwrap();
        let sicd_meta: SicdMeta = from_str(&sicd_str).unwrap();
        Self { nitf, sicd_meta }
    }
}
