//! ChargeMesh Integration Layer
//!
//! OCPI + Energy Integration for EV charging ecosystem:
//! - OCPI roaming (CPO ↔ EMSP)
//! - Energy Management (EMS, DER, BESS)
//! - Smart Charging optimization
//! - V2G (Vehicle-to-Grid)
//! - Grid constraints and renewable integration

#![warn(missing_docs)]
#![warn(rust_2018_idioms)]

pub mod ocpi;
pub mod energy;
pub mod smart_charging;
pub mod iso15118;

pub use ocpi::*;
pub use energy::*;
pub use smart_charging::*;
pub use iso15118::*;

use serde::{Deserialize, Serialize};

/// Integration result type
pub type Result<T> = std::result::Result<T, IntegrationError>;

#[derive(Debug, thiserror::Error)]
pub enum IntegrationError {
    #[error("OCPI error: {0}")]
    Ocpi(String),

    #[error("Energy management error: {0}")]
    Energy(String),

    #[error("Smart charging error: {0}")]
    SmartCharging(String),

    #[error("ISO 15118 error: {0}")]
    Iso15118(String),

    #[error("Orchestration error: {0}")]
    Orchestration(String),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Timeout: {0}")]
    Timeout(String),

    #[error("Internal error: {0}")]
    Internal(String),
}

/// Integration mode
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum IntegrationMode {
    Roaming,
    Energy,
    SmartCharging,
    Full,
    Standalone,
}

/// Integration status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum IntegrationStatus {
    Disconnected,
    Connecting,
    Connected,
    Synchronizing,
    Faulted,
}