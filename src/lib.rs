#![allow(clippy::needless_return)]
//! Crate for reading and manipulating `NITF` files

/// General `NIFT` interface functions and types
pub mod nitf;

/// Functions and types specific to `SICD` files
#[cfg(feature = "sicd")]
pub mod sicd;

// UNIT TESTS
#[cfg(test)]
mod tests {
    // TODO - Figure out how to test this
}
