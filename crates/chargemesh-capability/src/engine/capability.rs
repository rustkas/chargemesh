//! Core capability models

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Capability definition — describes what a capability is
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityDefinition {
    /// Unique capability identifier
    pub id: CapabilityType,

    /// Human-readable name
    pub name: String,

    /// Description
    pub description: String,

    /// Category
    pub category: CapabilityCategory,

    /// Dependencies (other capabilities required)
    pub dependencies: Vec<CapabilityType>,

    /// Required protocol versions
    pub required_protocols: Vec<ProtocolRequirement>,

    /// Required vendor/model combinations
    pub required_hardware: Vec<HardwareRequirement>,

    /// Minimum firmware version
    pub min_firmware_version: Option<String>,

    /// Configuration parameters
    pub config_params: Vec<ConfigParam>,

    /// Default state if not explicitly determined
    pub default_state: CapabilityState,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum CapabilityType {
    // ========== Core ==========
    BasicCharging,
    SmartCharging,
    FastCharging,
    V2G,
    V2H,
    Bidirectional,

    // ========== Protocol ==========
    OCPP1_6,
    OCPP2_0_1,
    OCPP2_1,
    ISO15118,
    ISO15118_20,
    OCPI,
    PlugAndCharge,

    // ========== Management ==========
    RemoteDiagnostics,
    RemoteFirmwareUpdate,
    RemoteReset,
    SelfTest,
    LogRetrieval,
    ConfigurationManagement,

    // ========== User ==========
    DisplayMessaging,
    Reservation,
    RFIDAuthorization,
    PINAuthorization,

    // ========== Metering ==========
    BidirectionalMetering,
    SignedMetering,
    RealTimeMetering,
    TariffManagement,

    // ========== Smart ==========
    LoadBalancing,
    ExternalConstraints,
    LocalGeneration,
    Scheduling,
    PriorityCharging,
    EnergyManagement,
    OCPIRoaming,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum CapabilityCategory {
    Core,
    Protocol,
    Management,
    User,
    Metering,
    Smart,
    Security,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtocolRequirement {
    pub protocol: ProtocolName,
    pub min_version: Option<String>,
    pub max_version: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HardwareRequirement {
    pub vendor: String,
    pub models: Vec<String>,
    pub hardware_versions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigParam {
    pub key: String,
    pub description: String,
    pub required: bool,
    pub default_value: Option<serde_json::Value>,
}

/// Capability state — describes if and how a capability is available
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "state")]
pub enum CapabilityState {
    /// Fully supported
    Supported {
        /// Additional parameters
        parameters: HashMap<String, serde_json::Value>,
    },

    /// Partially supported or limited
    Limited {
        /// Reason for limitation
        reason: String,
        /// Parameters describing the limitation
        parameters: HashMap<String, serde_json::Value>,
    },

    /// Not supported
    NotSupported {
        /// Reason (optional)
        reason: Option<String>,
    },

    /// Not available due to runtime conditions
    NotAvailable {
        /// Reason
        reason: String,
    },

    /// Unknown (need to discover)
    Unknown,
}

impl CapabilityState {
    pub fn is_available(&self) -> bool {
        matches!(self, CapabilityState::Supported { .. })
    }

    pub fn is_limited(&self) -> bool {
        matches!(self, CapabilityState::Limited { .. })
    }

    pub fn is_supported(&self) -> bool {
        matches!(self, CapabilityState::Supported { .. })
    }

    pub fn is_available_or_limited(&self) -> bool {
        matches!(
            self,
            CapabilityState::Supported { .. } | CapabilityState::Limited { .. }
        )
    }
}

/// Capability set — collection of capability states
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CapabilitySet {
    pub capabilities: HashMap<CapabilityType, CapabilityState>,
    pub detected_at: chrono::DateTime<chrono::Utc>,
    pub source: CapabilitySource,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum CapabilitySource {
    ProtocolDiscovery,
    VendorProfile,
    RuleEvaluation,
    ManualOverride,
    Cached,
}

impl CapabilitySet {
    pub fn new() -> Self {
        Self {
            capabilities: HashMap::new(),
            detected_at: chrono::Utc::now(),
            source: CapabilitySource::ProtocolDiscovery,
        }
    }

    pub fn with_source(mut self, source: CapabilitySource) -> Self {
        self.source = source;
        self
    }

    pub fn add_capability(&mut self, capability: CapabilityType, state: CapabilityState) {
        self.capabilities.insert(capability, state);
    }

    pub fn set_capability(&mut self, capability: CapabilityType, state: CapabilityState) {
        self.capabilities.insert(capability, state);
    }

    pub fn get_capability(&self, capability: &CapabilityType) -> Option<&CapabilityState> {
        self.capabilities.get(capability)
    }

    pub fn has_capability(&self, capability: &CapabilityType) -> bool {
        self.capabilities.contains_key(capability)
    }

    pub fn is_supported(&self, capability: &CapabilityType) -> bool {
        self.capabilities
            .get(capability)
            .map(|state| state.is_supported())
            .unwrap_or(false)
    }

    pub fn is_available(&self, capability: &CapabilityType) -> bool {
        self.capabilities
            .get(capability)
            .map(|state| state.is_available())
            .unwrap_or(false)
    }

    pub fn is_available_or_limited(&self, capability: &CapabilityType) -> bool {
        self.capabilities
            .get(capability)
            .map(|state| state.is_available_or_limited())
            .unwrap_or(false)
    }

    pub fn to_json(&self) -> serde_json::Value {
        let mut map = serde_json::Map::new();
        for (cap, state) in &self.capabilities {
            let key = format!("{:?}", cap).to_lowercase();
            let value = match state {
                CapabilityState::Supported { parameters } => {
                    let mut obj = serde_json::json!({ "supported": true });
                    if !parameters.is_empty() {
                        obj["parameters"] = serde_json::to_value(parameters).unwrap();
                    }
                    obj
                }
                CapabilityState::Limited { reason, parameters } => {
                    let mut obj = serde_json::json!({ 
                        "supported": true,
                        "limited": true,
                        "reason": reason 
                    });
                    if !parameters.is_empty() {
                        obj["parameters"] = serde_json::to_value(parameters).unwrap();
                    }
                    obj
                }
                CapabilityState::NotSupported { reason } => {
                    let mut obj = serde_json::json!({ "supported": false });
                    if let Some(r) = reason {
                        obj["reason"] = serde_json::Value::String(r.clone());
                    }
                    obj
                }
                CapabilityState::NotAvailable { reason } => {
                    serde_json::json!({ 
                        "supported": false,
                        "available": false,
                        "reason": reason 
                    })
                }
                CapabilityState::Unknown => {
                    serde_json::json!({ "supported": false, "unknown": true })
                }
            };
            map.insert(key, value);
        }
        serde_json::Value::Object(map)
    }
}