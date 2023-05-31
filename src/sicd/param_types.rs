//! Sicd metadata type definitions
pub use serde::{Deserialize, Serialize};
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct RC {
    Row: u64,
    Col: u64,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct CMPLX {
    Real: f64,
    Imag: f64,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct XYZ {
    X: f64,
    Y: f64,
    Z: f64,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct LLH {
    Lat: f64,
    Lon: f64,
    HAE: f64,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct LL {
    Lat: f64,
    Lon: f64,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Coef1D {
    exponent1: u8,
    #[serde(rename="$value")]
    Value: f64,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Poly1D {
    order1: u8,
    #[serde(rename="$value")]
    Coefs: Vec<Coef1D>,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Coef2D {
    exponent1: u8,
    exponent2: u8,
    #[serde(rename="$value")]
    Value: f64,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Poly2D {
    order1: u8,
    order2: u8,
    #[serde(rename="$value")]
    Coefs: Vec<Coef2D>,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct XyzPoly {
    X: Poly1D,
    Y: Poly1D,
    Z: Poly1D,
}