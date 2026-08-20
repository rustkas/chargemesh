//! OCPP Implementation for ChargeMesh
//!
//! This crate provides OCPP 1.6, 2.0.1, and 2.1 implementations
//! for the ChargeMesh project.

#![warn(missing_docs)]
#![warn(rust_2018_idioms)]

pub mod common;
pub mod v16;
pub mod v201;
pub mod v21;

pub use common::*;
pub use v16::*;
pub use v201::*;
pub use v21::*;

/// OCPP version
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
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

/// OCPP message direction
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MessageDirection {
    Incoming,
    Outgoing,
}