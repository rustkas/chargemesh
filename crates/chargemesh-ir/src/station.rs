//! Charging Station model

use super::*;
use std::collections::HashMap;

/// Main charging station entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Station {
    /// Unique identifier (OCPP charge point ID)
    pub id: StationId,
    
    /// Vendor/manufacturer name
    pub vendor: String,
    
    /// Model name/number
    pub model: String,
    
    /// Serial number
    pub serial_number: Option<String>,
    
    /// Firmware version
    pub firmware_version: Option<String>,
    
    /// Current state
    pub state: StationState,
    
    /// Protocol information
    pub protocol: ProtocolInfo,
    
    /// List of EVSEs
    pub evses: Vec<Evse>,
    
    /// Capabilities supported
    pub capabilities: Capabilities,
    
    /// Physical location
    pub location: Option<GeoLocation>,
    
    /// Last heartbeat timestamp
    pub last_heartbeat: Option<Timestamp>,
    
    /// Boot timestamp
    pub boot_time: Option<Timestamp>,
    
    /// Custom metadata
    pub metadata: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum StationState {
    Offline,
    Booted,
    Enabled,
    Disabled,
    Faulted,
    Maintenance,
    Updating,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtocolInfo {
    pub name: ProtocolName,
    pub version: String,
    pub transport: TransportType,
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TransportType {
    WebSocket,
    HTTP,
    HTTPS,
    TCP,
    MQTT,
    Other,
}

impl Station {
    pub fn new(id: StationId, vendor: String, model: String) -> Self {
        Self {
            id,
            vendor,
            model,
            serial_number: None,
            firmware_version: None,
            state: StationState::Offline,
            protocol: ProtocolInfo {
                name: ProtocolName::OCPP,
                version: "1.6".to_string(),
                transport: TransportType::WebSocket,
                security_profile: "Basic".to_string(),
            },
            evses: Vec::new(),
            capabilities: Capabilities::default(),
            location: None,
            last_heartbeat: None,
            boot_time: None,
            metadata: HashMap::new(),
        }
    }
    
    pub fn add_evse(&mut self, evse: Evse) {
        self.evses.push(evse);
    }
    
    pub fn get_evse(&self, id: &str) -> Option<&Evse> {
        self.evses.iter().find(|e| e.id.0 == id)
    }
    
    pub fn get_connector(&self, id: &str) -> Option<&Connector> {
        for evse in &self.evses {
            if let Some(conn) = evse.get_connector(id) {
                return Some(conn);
            }
        }
        None
    }
    
    pub fn has_capability(&self, capability: CapabilityType) -> bool {
        self.capabilities.has(capability)
    }
}
