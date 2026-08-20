//! Connector model

use super::*;
use std::collections::HashMap;

/// Physical connector (plug)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Connector {
    /// Connector identifier
    pub id: ConnectorId,

    /// Connector type
    pub connector_type: ConnectorType,

    /// Current state
    pub state: ConnectorState,

    /// Parent EVSE ID
    pub evse_id: EvseId,

    /// Maximum power (watts)
    pub max_power: Power,

    /// Current power delivery (watts)
    pub current_power: Option<Power>,

    /// Active charging session (if any)
    pub active_session: Option<ChargingSession>,

    /// Meter readings
    pub meter: Option<Meter>,

    /// Capabilities
    pub capabilities: Capabilities,

    /// Physical status
    pub physical_status: ConnectorPhysicalStatus,

    /// Custom metadata
    pub metadata: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ConnectorType {
    Type1,
    Type2,
    CCS,
    CHAdeMO,
    GB_T,
    Tesla,
    Other(String),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ConnectorState {
    Available,
    Preparing,
    Charging,
    Suspended,
    Faulted,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectorPhysicalStatus {
    pub cable_temperature: Option<Temperature>,
    pub lock_engaged: bool,
    pub cable_connected: bool,
    pub contact_resistance: Option<f64>,
    pub wear_level: Option<Percentage>,
}

impl Default for ConnectorPhysicalStatus {
    fn default() -> Self {
        Self {
            cable_temperature: None,
            lock_engaged: false,
            cable_connected: false,
            contact_resistance: None,
            wear_level: None,
        }
    }
}

impl Connector {
    pub fn new(id: ConnectorId, evse_id: EvseId, connector_type: ConnectorType, max_power: Power) -> Self {
        Self {
            id,
            connector_type,
            state: ConnectorState::Available,
            evse_id,
            max_power,
            current_power: None,
            active_session: None,
            meter: None,
            capabilities: Capabilities::default(),
            physical_status: ConnectorPhysicalStatus::default(),
            metadata: HashMap::new(),
        }
    }

    pub fn is_charging(&self) -> bool {
        matches!(self.state, ConnectorState::Charging)
    }

    pub fn is_available(&self) -> bool {
        matches!(self.state, ConnectorState::Available)
    }

    pub fn active_session(&self) -> Option<&ChargingSession> {
        self.active_session.as_ref()
    }

    pub fn active_session_mut(&mut self) -> Option<&mut ChargingSession> {
        self.active_session.as_mut()
    }

    pub fn start_session(&mut self, session: ChargingSession) -> Result<()> {
        if !self.is_available() {
            return Err(CoreError::InvalidState(
                format!("Connector {} is not available", self.id)
            ));
        }
        self.active_session = Some(session);
        self.state = ConnectorState::Charging;
        Ok(())
    }

    pub fn stop_session(&mut self) -> Result<Option<ChargingSession>> {
        if self.is_available() {
            return Err(CoreError::InvalidState(
                format!("Connector {} has no active session", self.id)
            ));
        }
        let session = self.active_session.take();
        self.state = ConnectorState::Available;
        self.current_power = None;
        Ok(session)
    }
}