//! Firmware model

use super::*;
use std::collections::HashMap;

/// Firmware information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Firmware {
    pub id: Id,
    pub station_id: StationId,
    pub version: String,
    pub available_version: Option<String>,
    pub firmware_uri: Option<String>,
    pub size: Option<u64>,
    pub installed_at: Option<Timestamp>,
    pub last_check: Option<Timestamp>,
    pub update_status: FirmwareUpdateStatus,
    pub checksum: Option<String>,
    pub release_notes: Option<String>,
    pub metadata: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum FirmwareUpdateStatus {
    UpToDate,
    UpdateAvailable,
    Downloading,
    Verifying,
    Installing,
    InstallComplete,
    InstallFailed,
    Rollback,
}

impl Firmware {
    pub fn new(station_id: StationId, version: String) -> Self {
        Self {
            id: Id::new(),
            station_id,
            version,
            available_version: None,
            firmware_uri: None,
            size: None,
            installed_at: None,
            last_check: None,
            update_status: FirmwareUpdateStatus::UpToDate,
            checksum: None,
            release_notes: None,
            metadata: HashMap::new(),
        }
    }
}