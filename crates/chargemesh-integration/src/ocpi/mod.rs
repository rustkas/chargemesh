//! OCPI (Open Charge Point Interface) implementation

mod client;
mod server;
mod models;
mod roaming;
mod endpoints;

pub use client::*;
pub use server::*;
pub use models::*;
pub use roaming::*;
pub use endpoints::*;

use super::*;
use std::collections::HashMap;

/// OCPI version
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum OcpiVersion {
    V2_0,
    V2_1,
    V2_2,
    V2_2_1,
}

impl std::fmt::Display for OcpiVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OcpiVersion::V2_0 => write!(f, "2.0"),
            OcpiVersion::V2_1 => write!(f, "2.1"),
            OcpiVersion::V2_2 => write!(f, "2.2"),
            OcpiVersion::V2_2_1 => write!(f, "2.2.1"),
        }
    }
}

/// OCPI role
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum OcpiRole {
    CPO,
    EMSP,
    NAP,
    Hub,
}

/// OCPI endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OcpiEndpoint {
    pub id: String,
    pub url: String,
    pub version: OcpiVersion,
    pub role: OcpiRole,
    pub country_code: String,
    pub party_id: String,
    pub token: Option<String>,
    pub status: IntegrationStatus,
}