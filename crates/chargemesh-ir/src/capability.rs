//! Capability model

use super::*;
use std::collections::{HashMap, HashSet};

/// Capabilities supported by a station/EVSE/connector
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Capabilities {
    pub capabilities: HashSet<CapabilityType>,
    pub parameters: HashMap<CapabilityType, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CapabilityType {
    // Electrical
    SmartCharging,
    V2G,
    V2H,
    Bidirectional,
    LoadBalancing,

    // Protocol
    OCPP1_6,
    OCPP2_0_1,
    OCPP2_1,
    ISO15118,
    ISO15118_20,
    OCPI,
    VendorProtocol,

    // Security
    PlugAndCharge,
    BasicAuthentication,
    TLSAuthentication,
    CertificateManagement,

    // Management
    RemoteFirmwareUpdate,
    RemoteDiagnostics,
    RemoteReset,
    SelfTest,

    // User
    DisplayMessaging,
    Reservation,
    RFIDAuthorization,
    PINAuthorization,

    // Metering
    BidirectionalMetering,
    SignedMetering,
    RealTimeMetering,
    TariffManagement,

    // Smart
    ExternalConstraints,
    LocalGeneration,
    Scheduling,
    PriorityCharging,
    EnergyManagement,
    OCPIRoaming,
}

impl Capabilities {
    pub fn single(capability: CapabilityType) -> Self {
        let mut capabilities = HashSet::new();
        capabilities.insert(capability);
        Self {
            capabilities,
            parameters: HashMap::new(),
        }
    }

    pub fn add(&mut self, capability: CapabilityType) {
        self.capabilities.insert(capability);
    }

    pub fn remove(&mut self, capability: CapabilityType) {
        self.capabilities.remove(&capability);
    }

    pub fn has(&self, capability: CapabilityType) -> bool {
        self.capabilities.contains(&capability)
    }

    pub fn add_parameter(&mut self, capability: CapabilityType, value: serde_json::Value) {
        self.parameters.insert(capability, value);
    }

    pub fn get_parameter(&self, capability: CapabilityType) -> Option<&serde_json::Value> {
        self.parameters.get(&capability)
    }
}