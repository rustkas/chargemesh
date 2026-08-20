//! ChargeMesh IR — Universal EV Charging Intermediate Representation
//!
//! This crate provides the canonical data model for all EV charging entities.
//! It is protocol-independent and serves as the foundation for the entire platform.

#![warn(missing_docs)]
#![warn(rust_2018_idioms)]

pub mod station;
pub mod evse;
pub mod connector;
pub mod vehicle;
pub mod session;
pub mod transaction;
pub mod meter;
pub mod tariff;
pub mod authorization;
pub mod reservation;
pub mod profile;
pub mod capability;
pub mod error;
pub mod firmware;
pub mod energy;
pub mod network;
pub mod state_machine;

// Re-export all core types
pub use station::*;
pub use evse::*;
pub use connector::*;
pub mod prelude {
    pub use super::*;
}

/// IR version
pub const IR_VERSION: &str = "0.1.0";