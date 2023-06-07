//! Common types and metadata definition for SICD structure
use ndarray::{Array1, Array2};
use serde::Deserialize;

pub mod antenna;
pub mod collection_info;
pub mod error_statistics;
pub mod geo_data;
pub mod grid;
pub mod image_creation;
pub mod image_data;
pub mod image_formation;
pub mod match_info;
pub mod pfa;
pub mod position;
pub mod radar_collection;
pub mod radiometric;
pub mod scpcoa;
pub mod timeline;

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct RowCol {
    #[serde(rename = "Row")]
    pub row: u64,
    #[serde(rename = "Col")]
    pub col: u64,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct IdxRowCol {
    pub index: usize,
    #[serde(rename = "Row")]
    pub row: u64,
    #[serde(rename = "Col")]
    pub col: u64,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct CMPLX {
    #[serde(rename = "Real")]
    pub real: f64,
    #[serde(rename = "Imag")]
    pub imag: f64,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct XYZ {
    #[serde(rename = "X")]
    pub x: f64,
    #[serde(rename = "Y")]
    pub y: f64,
    #[serde(rename = "Z")]
    pub z: f64,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct LLH {
    #[serde(rename = "Lat")]
    pub lat: f64,
    #[serde(rename = "Lon")]
    pub lon: f64,
    #[serde(rename = "HAE")]
    pub hae: f64,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct IdxLLH {
    pub index: usize,
    #[serde(rename = "Lat")]
    pub lat: f64,
    #[serde(rename = "Lon")]
    pub lon: f64,
    #[serde(rename = "HAE")]
    pub hae: f64,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct LL {
    #[serde(rename = "Lat")]
    pub lat: f64,
    #[serde(rename = "Lon")]
    pub lon: f64,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct IdxLL {
    pub index: usize,
    #[serde(rename = "Lat")]
    pub lat: f64,
    #[serde(rename = "Lon")]
    pub lon: f64,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Coef1D {
    pub exponent1: usize,
    #[serde(rename = "$value")]
    pub value: f64,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Poly1D {
    pub order1: usize,
    #[serde(rename = "$value")]
    pub coefs: Vec<Coef1D>,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Coef2D {
    pub exponent1: usize,
    pub exponent2: usize,
    #[serde(rename = "$value")]
    pub value: f64,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Poly2D {
    pub order1: usize,
    pub order2: usize,
    #[serde(rename = "$value")]
    pub coefs: Vec<Coef2D>,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct XyzPoly {
    #[serde(rename = "X")]
    pub x: Poly1D,
    #[serde(rename = "Y")]
    pub y: Poly1D,
    #[serde(rename = "Z")]
    pub z: Poly1D,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct IdxXyzPoly {
    pub index: usize,
    #[serde(rename = "X")]
    pub x: Poly1D,
    #[serde(rename = "Y")]
    pub y: Poly1D,
    #[serde(rename = "Z")]
    pub z: Poly1D,
}

pub type Parameter = Option<Vec<ParameterStruct>>;
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct ParameterStruct {
    pub name: String,
    #[serde(rename = "$value")]
    pub value: String,
}


impl Poly1D {
    /// Parse the data in the polynomial to an array object
    pub fn to_array(&self) -> Array1<f64> {
        let mut poly = Array1::zeros(self.order1 + 1);
        for coef in &self.coefs {
            let term = coef.exponent1;
            poly[term] = coef.value;
        }
        poly
    }

    /// Evaluate the polynomial at a point
    pub fn eval(&self, x: f64) -> f64 {
        let mut res = 0f64;
        for coef in &self.coefs {
            res += coef.value * x.powi(coef.exponent1 as i32)
        }
        res
    }
}
impl Poly2D {
    /// Parse the data in the polynomial to an array object
    pub fn to_array(&self) -> Array2<f64> {
        let mut poly = Array2::zeros((self.order1 + 1, self.order2 + 1));
        for coef in &self.coefs {
            let term1 = coef.exponent1;
            let term2 = coef.exponent2;
            poly[[term1, term2]] = coef.value;
        }
        poly
    }
    /// Evaluate the polynomial at a point
    pub fn eval(&self, x: f64, y: f64) -> f64 {
        let mut res = 0f64;
        for coef in &self.coefs {
            res += coef.value * x.powi(coef.exponent1 as i32) * y.powi(coef.exponent2 as i32)
        }
        res
    }
}
impl XyzPoly {
    pub fn eval(&self, t: f64) -> Vec<f64> {
        let x_pos = self.x.eval(t);
        let y_pos = self.y.eval(t);
        let z_pos = self.z.eval(t);
        vec![x_pos, y_pos, z_pos]
    }
}



// #[cfg(test)]
// mod tests {

//     #[test]
//     fn test_rowcol() {}
//     #[test]
//     fn test_cmplx() {}
//     #[test]
//     fn test_xyz() {}
//     #[test]
//     fn test_llh() {}
//     #[test]
//     fn test_ll() {}
//     #[test]
//     fn test_coef1d() {}
//     #[test]
//     fn test_poly1d() {}
//     #[test]
//     fn test_coef2d() {}
//     #[test]
//     fn test_poly2d() {}
//     #[test]
//     fn test_xyzpoly() {}
//     #[test]
//     fn test_parameterstruct() {}
// }
