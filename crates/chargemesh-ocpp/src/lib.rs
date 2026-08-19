//! OCPP Implementation for ChargeMesh
//!
//! Supports OCPP 1.6, 2.0.1, and 2.1

pub mod v16;
pub mod v201;
pub mod v21;
pub mod common;

pub use v16::*;
pub use v201::*;
pub use v21::*;
pub use common::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OcppVersion {
    V16,
    V201,
    V21,
}

impl std::fmt::Display for OcppVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OcppVersion::V16 => write!(f, "1.6"),
            OcppVersion::V201 => write!(f, "2.0.1"),
            OcppVersion::V21 => write!(f, "2.1"),
        }
    }
}
