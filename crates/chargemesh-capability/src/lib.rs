//! ChargeMesh Capability Engine
//!
//! This crate provides dynamic capability detection and evaluation
//! for EV charging stations based on protocol, vendor, model,
//! firmware, configuration, and runtime state.

#![warn(missing_docs)]
#![warn(rust_2018_idioms)]

pub mod engine;
pub mod detectors;
pub mod profiles;
pub mod rules;

pub use engine::*;
pub use detectors::*;
pub use profiles::*;
pub use rules::*;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Core result type
pub type Result<T> = std::result::Result<T, CapabilityError>;

/// Capability error types
#[derive(Debug, thiserror::Error)]
pub enum CapabilityError {
    #[error("Station not found: {0}")]
    StationNotFound(String),

    #[error("Unknown protocol: {0}")]
    UnknownProtocol(String),

    #[error("Unknown vendor: {0}")]
    UnknownVendor(String),

    #[error("Capability not supported: {0}")]
    CapabilityNotSupported(String),

    #[error("Rule evaluation failed: {0}")]
    RuleEvaluationFailed(String),

    #[error("Configuration error: {0}")]
    ConfigurationError(String),

    #[error("Internal error: {0}")]
    Internal(String),
}

/// Capability context — all information about a station
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityContext {
    /// Station identifier
    pub station_id: String,

    /// Protocol information
    pub protocol: ProtocolInfo,

    /// Vendor information
    pub vendor: VendorInfo,

    /// Firmware information
    pub firmware: FirmwareInfo,

    /// Configuration
    pub configuration: HashMap<String, serde_json::Value>,

    /// Runtime state
    pub runtime: RuntimeState,

    /// Hardware model
    pub model: String,

    /// Hardware version
    pub hardware_version: Option<String>,

    /// Serial number
    pub serial_number: Option<String>,
}

/// Protocol information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtocolInfo {
    pub name: ProtocolName,
    pub version: String,
    pub transport: String,
    pub security_profile: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ProtocolName {
    OCPP,
    ISO15118,
    OCPI,
    Vendor,
    Unknown,
}

/// Vendor information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VendorInfo {
    pub name: String,
    pub id: Option<String>,
    pub known_models: Vec<String>,
}

/// Firmware information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FirmwareInfo {
    pub version: String,
    pub build_date: Option<String>,
    pub checksum: Option<String>,
    pub compatibility: Vec<String>,
}

/// Runtime state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeState {
    pub is_online: bool,
    pub is_booted: bool,
    pub active_sessions: u32,
    pub total_energy_delivered: u64,
    pub uptime_seconds: u64,
    pub temperature: Option<f32>,
    pub load_percentage: Option<u8>,
}