//! Charging Station model

use super::*;
use std::collections::HashMap;

/// Main charging station entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChargingStation {
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
    pub evses: Vec<EVSE>,

    /// Capabilities supported by this station
    pub capabilities: Capabilities,

    /// Physical location
    pub location: Option<GeoLocation>,

    /// Address
    pub address: Option<Address>,

    /// Last heartbeat timestamp
    pub last_heartbeat: Option<Timestamp>,

    /// Boot timestamp
    pub boot_time: Option<Timestamp>,

    /// Custom metadata
    pub metadata: HashMap<String, serde_json::Value>,

    /// Configuration
    pub configuration: StationConfiguration,
}

/// Station state lifecycle
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

/// Protocol information
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

/// Station configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StationConfiguration {
    pub max_power: Option<Power>,
    pub connector_count: u8,
    pub connector_types: Vec<ConnectorType>,
    pub network: NetworkConfig,
    pub security: SecurityConfig,
}

/// Network configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NetworkConfig {
    pub ip_address: Option<String>,
    pub mac_address: Option<String>,
    pub gateway: Option<String>,
    pub dns_servers: Vec<String>,
    pub network_type: NetworkType,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum NetworkType {
    #[default]
    Ethernet,
    WiFi,
    Cellular,
    Other(String),
}

/// Security configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SecurityConfig {
    pub tls_enabled: bool,
    pub tls_version: Option<String>,
    pub security_profile: SecurityProfile,
    pub certificate_info: Option<CertificateInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum SecurityProfile {
    #[default]
    Basic,
    TLS,
    TLSWithCertificates,
    Advanced,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CertificateInfo {
    pub issuer: String,
    pub subject: String,
    pub valid_from: Timestamp,
    pub valid_to: Timestamp,
    pub serial_number: String,
}

/// Geographic location
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeoLocation {
    pub latitude: f64,
    pub longitude: f64,
    pub altitude: Option<f64>,
}

/// Physical address
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Address {
    pub street: String,
    pub city: String,
    pub state: Option<String>,
    pub postal_code: Option<String>,
    pub country: String,
}

impl ChargingStation {
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
            address: None,
            last_heartbeat: None,
            boot_time: None,
            metadata: HashMap::new(),
            configuration: StationConfiguration::default(),
        }
    }

    pub fn add_evse(&mut self, evse: EVSE) {
        self.evses.push(evse);
    }

    pub fn get_evse(&self, id: &str) -> Option<&EVSE> {
        self.evses.iter().find(|e| e.id.0 == id)
    }

    pub fn get_evse_mut(&mut self, id: &str) -> Option<&mut EVSE> {
        self.evses.iter_mut().find(|e| e.id.0 == id)
    }

    pub fn get_connector(&self, connector_id: &str) -> Option<&Connector> {
        for evse in &self.evses {
            if let Some(connector) = evse.get_connector(connector_id) {
                return Some(connector);
            }
        }
        None
    }

    pub fn get_connector_mut(&mut self, connector_id: &str) -> Option<&mut Connector> {
        for evse in &mut self.evses {
            if let Some(connector) = evse.get_connector_mut(connector_id) {
                return Some(connector);
            }
        }
        None
    }

    pub fn has_capability(&self, capability: CapabilityType) -> bool {
        self.capabilities.has(capability)
    }

    pub fn total_connectors(&self) -> usize {
        self.evses.iter().map(|e| e.connectors.len()).sum()
    }
}