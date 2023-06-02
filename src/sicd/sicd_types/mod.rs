//! Common types and metadata definition for SICD structure
use serde::Deserialize;
use ndarray::{Array1, Array2};

pub mod collection_info;
pub mod image_creation;
pub mod image_data;
pub mod geo_data;
pub mod grid;
pub mod timeline;
pub mod position;
pub mod radar_collection;
pub mod image_formation;
pub mod scpcoa;
pub mod radiometric;
pub mod antenna;
pub mod ErrorStatistics;
pub mod match_info;
pub mod pfa;

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct RowCol {
    pub Row: u64,
    pub Col: u64,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct CMPLX {
    pub Real: f64,
    pub Imag: f64,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct XYZ {
    pub X: f64,
    pub Y: f64,
    pub Z: f64,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct LLH {
    pub Lat: f64,
    pub Lon: f64,
    pub HAE: f64,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct LL {
    pub Lat: f64,
    pub Lon: f64,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Coef1D {
    pub exponent1: usize,
    #[serde(rename = "$value")]
    pub Value: f64,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Poly1D {
    pub order1: usize,
    #[serde(rename = "$value")]
    pub Coefs: Vec<Coef1D>,
}
impl Poly1D {
    /// Parse the data in the polynomial to an array object
    pub fn to_array(&self) -> Array1<f64> {
        let mut poly = Array1::zeros(self.order1 + 1);
        for coef in &self.Coefs {
            let term = coef.exponent1;
            poly[term] = coef.Value;
        }
        poly
    }
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Coef2D {
    pub exponent1: usize,
    pub exponent2: usize,
    #[serde(rename = "$value")]
    pub Value: f64,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Poly2D {
    pub order1: usize,
    pub order2: usize,
    #[serde(rename = "$value")]
    pub Coefs: Vec<Coef2D>,
}
impl Poly2D {
    /// Parse the data in the polynomial to an array object
    pub fn to_array(&self) -> Array2<f64> {
        let mut poly = Array2::zeros((self.order1 + 1, self.order2 + 1));
        for coef in &self.Coefs {
            let term1 = coef.exponent1;
            let term2 = coef.exponent2;
            poly[[term1, term2]] = coef.Value;
        }
        poly
    }
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct XyzPoly {
    pub X: Poly1D,
    pub Y: Poly1D,
    pub Z: Poly1D,
}

pub type Parameter = Option<Vec<ParameterStruct>>;
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct ParameterStruct {
    pub name: String,
    #[serde(rename = "$value")]
    pub value: String,
}

