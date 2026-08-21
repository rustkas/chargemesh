//! ChargeMesh Simulator
//!
//! Complete simulation environment for EV charging infrastructure.
//! Supports EV, EVSE, CSMS, OCPI, and Grid simulators with fault injection.

#![warn(missing_docs)]
#![warn(rust_2018_idioms)]

pub mod core;
pub mod ev;
pub mod evse;
pub mod csms;
pub mod ocpi;
pub mod grid;
pub mod faults;

pub use core::*;
pub use ev::*;
pub use evse::*;
pub use csms::*;
pub use ocpi::*;
pub use grid::*;
pub use faults::*;

use serde::{Deserialize, Serialize};

/// Simulator result type
pub type Result<T> = std::result::Result<T, SimulatorError>;

#[derive(Debug, thiserror::Error)]
pub enum SimulatorError {
    #[error("Simulation error: {0}")]
    Simulation(String),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Protocol error: {0}")]
    Protocol(String),

    #[error("Fault injection failed: {0}")]
    FaultInjection(String),

    #[error("Timeout: {0}")]
    Timeout(String),

    #[error("Internal error: {0}")]
    Internal(String),
}

/// Simulation mode
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum SimulationMode {
    Normal,
    FaultInjection,
    Stress,
    Compliance,
    Performance,
}

/// Simulation status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum SimulationStatus {
    Pending,
    Running,
    Paused,
    Completed,
    Failed,
}