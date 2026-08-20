//! EVSE model

use super::*;
use std::collections::HashMap;

/// Electric Vehicle Supply Equipment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EVSE {
    /// EVSE identifier (within station)
    pub id: EvseId,

    /// Parent station ID
    pub station_id: StationId,

    /// Current state
    pub state: EVSEState,

    /// Maximum power capacity (watts)
    pub max_power: Power,

    /// Connectors
    pub connectors: Vec<Connector>,

    /// Capabilities
    pub capabilities: Capabilities,

    /// Energy meter
    pub meter: Option<Meter>,

    /// Installation timestamp
    pub installed_at: Option<Timestamp>,

    /// Custom metadata
    pub metadata: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum EVSEState {
    Available,
    Occupied,
    Charging,
    Faulted,
}

impl EVSE {
    pub fn new(id: EvseId, station_id: StationId, max_power: Power) -> Self {
        Self {
            id,
            station_id,
            state: EVSEState::Available,
            max_power,
            connectors: Vec::new(),
            capabilities: Capabilities::default(),
            meter: None,
            installed_at: None,
            metadata: HashMap::new(),
        }
    }

    pub fn add_connector(&mut self, connector: Connector) {
        self.connectors.push(connector);
    }

    pub fn get_connector(&self, connector_id: &str) -> Option<&Connector> {
        self.connectors.iter().find(|c| c.id.0 == connector_id)
    }

    pub fn get_connector_mut(&mut self, connector_id: &str) -> Option<&mut Connector> {
        self.connectors.iter_mut().find(|c| c.id.0 == connector_id)
    }

    pub fn active_session(&self) -> Option<&ChargingSession> {
        for connector in &self.connectors {
            if let Some(session) = connector.active_session() {
                return Some(session);
            }
        }
        None
    }

    pub fn is_charging(&self) -> bool {
        self.connectors.iter().any(|c| c.is_charging())
    }
}