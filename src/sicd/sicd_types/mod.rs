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

    /// Evaluate the polynomial at a point
    pub fn eval(&self, x: f64) -> f64 {
        let mut res = 0f64;
        for coef in &self.Coefs {
            res += coef.Value * x.powi(coef.exponent1 as i32)
        }
        res
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
    /// Evaluate the polynomial at a point
    pub fn eval(&self, x: f64, y: f64) -> f64 {
        let mut res = 0f64;
        for coef in &self.Coefs {
            res += coef.Value * x.powi(coef.exponent1 as i32) * y.powi(coef.exponent2 as i32)
        }
        res
    }
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct XyzPoly {
    pub X: Poly1D,
    pub Y: Poly1D,
    pub Z: Poly1D,
}
impl XyzPoly {
    pub fn eval(&self, t: f64) -> Vec<f64> {
        let x_pos = self.X.eval(t);
        let y_pos = self.Y.eval(t);
        let z_pos = self.Z.eval(t);
        vec![x_pos, y_pos, z_pos]
    }
}

pub type Parameter = Option<Vec<ParameterStruct>>;
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct ParameterStruct {
    pub name: String,
    #[serde(rename = "$value")]
    pub value: String,
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
