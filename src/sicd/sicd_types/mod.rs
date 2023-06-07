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
    #[serde(rename = "@index")]
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
    #[serde(rename = "@index")]
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
    #[serde(rename = "@index")]
    pub index: usize,
    #[serde(rename = "Lat")]
    pub lat: f64,
    #[serde(rename = "Lon")]
    pub lon: f64,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Coef1d {
    #[serde(rename = "@exponent1")]
    pub exponent1: usize,
    #[serde(rename = "$value")]
    pub value: f64,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Poly1d {
    #[serde(rename = "@order1")]
    pub order1: usize,
    #[serde(rename = "$value")]
    pub coefs: Vec<Coef1d>,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Coef2d {
    #[serde(rename = "@exponent1")]
    pub exponent1: usize,
    #[serde(rename = "@exponent2")]
    pub exponent2: usize,
    #[serde(rename = "$value")]
    pub value: f64,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Poly2d {
    #[serde(rename = "@order1")]
    pub order1: usize,
    #[serde(rename = "@order2")]
    pub order2: usize,
    #[serde(rename = "$value")]
    pub coefs: Vec<Coef2d>,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct XyzPoly {
    #[serde(rename = "X")]
    pub x: Poly1d,
    #[serde(rename = "Y")]
    pub y: Poly1d,
    #[serde(rename = "Z")]
    pub z: Poly1d,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct IdxXyzPoly {
    #[serde(rename = "@index")]
    pub index: usize,
    #[serde(rename = "X")]
    pub x: Poly1d,
    #[serde(rename = "Y")]
    pub y: Poly1d,
    #[serde(rename = "Z")]
    pub z: Poly1d,
}

pub type Parameter = Option<Vec<ParameterStruct>>;
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct ParameterStruct {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "$value")]
    pub value: String,
}

impl Poly1d {
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
impl Poly2d {
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

#[cfg(test)]
mod tests {
    use super::*;
    use quick_xml::de::from_str;
    
    #[test]
    fn test_row_col() {
        let xml_str = r#"
        <RowCol>
            <Row>0</Row>
            <Col>0</Col>
        </RowCol>
        "#;
        assert!(match from_str::<RowCol>(xml_str) {
            Ok(_) => true, Err(_) => false
        })
    }
    #[test]
    fn test_idx_row_col() {
        let xml_str = r#"
        <IdxRowCol index = "0">
            <Row>0</Row>
            <Col>0</Col>
        </IdxRowCol>
        "#;
        assert!(match from_str::<IdxRowCol>(xml_str) {
            Ok(_) => true, Err(_) => false
        })
    }
    #[test]
    fn test_cmplx() {
        let xml_str = r#"
        <CMPLX>
            <Real>0</Real>
            <Imag>0</Imag>
        </CMPLX>
        "#;
        assert!(match from_str::<CMPLX>(xml_str) {
            Ok(_) => true, Err(_) => false
        })
    }
    #[test]
    fn test_xyz() {
        let xml_str = r#"
        <XYZ>
            <X>0</X>
            <Y>0</Y>
            <Z>0</Z>
        </XYZ>
        "#;
        assert!(match from_str::<XYZ>(xml_str) {
            Ok(_) => true, Err(_) => false
        })
    }
    #[test]
    fn test_llh() {
        let xml_str = r#"
        <LLH>
            <Lat>0</Lat>
            <Lon>0</Lon>
            <HAE>0</HAE>
        </LLH>
        "#;
        assert!(match from_str::<LLH>(xml_str) {
            Ok(_) => true, Err(_) => false
        })
    }
    #[test]
    fn test_idx_llh() {
        let xml_str = r#"
        <IdxLLH index = "0">
            <Lat>0</Lat>
            <Lon>0</Lon>
            <HAE>0</HAE>
        </IdxLLH>
        "#;
        assert!(match from_str::<IdxLLH>(xml_str) {
            Ok(_) => true, Err(_) => false
        })
    }
    #[test]
    fn test_ll() {
        let xml_str = r#"
        <LL>
            <Lat>0</Lat>
            <Lon>0</Lon>
        </LL>
        "#;
        assert!(match from_str::<LL>(xml_str) {
            Ok(_) => true, Err(_) => false
        })
    }
    #[test]
    fn test_idx_ll() {
        let xml_str = r#"
        <IdxLL index = "0">
            <Lat>0</Lat>
            <Lon>0</Lon>
        </IdxLL>
        "#;
        assert!(match from_str::<IdxLL>(xml_str) {
            Ok(_) => true, Err(_) => false
        })
    }
    #[test]
    fn test_poly1d() {
        let xml_str = r#"
        <Poly1d order1="1">
            <Coef1d exponent1="0">0</Coef1d>
            <Coef1d exponent1="1">0</Coef1d>
        </Poly1d>
        "#;
        assert!(match from_str::<Poly1d>(xml_str) {
            Ok(_) => true, Err(_) => false
        })
    }
    #[test]
    fn test_poly2d() {
        let xml_str = r#"
        <Poly2d order1 = "1" order2 = "1">
            <Coef2d exponent1="0" exponent2="0">0</Coef2d>
            <Coef2d exponent1="1" exponent2="0">0</Coef2d>
            <Coef2d exponent1="0" exponent2="1">0</Coef2d>
            <Coef2d exponent1="1" exponent2="1">0</Coef2d>
        </Poly2d>
        "#;
        assert!(match from_str::<Poly2d>(xml_str) {
            Ok(_) => true, Err(_) => false
        })
    }
    #[test]
    fn test_xyz_poly() {
        let xml_str = r#"
        <XyzPoly>
            <X order1="0">
                <Coef1d exponent1="0">0</Coef1d>
            </X>
            <Y order1="0">
                <Coef1d exponent1="0">0</Coef1d>
            </Y>
            <Z order1="0">
                <Coef1d exponent1="0">0</Coef1d>
            </Z>
        </XyzPoly>
        "#;
        assert!(match from_str::<XyzPoly>(xml_str) {
            Ok(_) => true, Err(_) => false
        })
    }
    #[test]
    fn test_idx_xyz_poly() {
        let xml_str = r#"
        <IdxXyzPoly index="0">
            <X order1="0">
                <Coef1d exponent1="0">0</Coef1d>
            </X>
            <Y order1="0">
                <Coef1d exponent1="0">0</Coef1d>
            </Y>
            <Z order1="0">
                <Coef1d exponent1="0">0</Coef1d>
            </Z>
        </IdxXyzPoly>
        "#;
        assert!(match from_str::<IdxXyzPoly>(xml_str) {
            Ok(_) => true, Err(_) => false
        })
    }
    #[test]
    fn test_parameter() {
        let xml_str = r#"
            <Parameter name="Param0">TestP0</Parameter>
            <Parameter name="Param1">TestP1</Parameter>
        "#;
        assert!(match from_str::<Parameter>(xml_str) {
            Ok(_) => true, Err(_) => false
        })
    }
}
