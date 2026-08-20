//! Charging Session model

use super::*;
use std::collections::HashMap;

/// Charging session — the core unit of a charging transaction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChargingSession {
    /// Session identifier
    pub id: SessionId,

    /// Station ID
    pub station_id: StationId,

    /// EVSE ID
    pub evse_id: EvseId,

    /// Connector ID
    pub connector_id: ConnectorId,

    /// Vehicle (if known)
    pub vehicle: Option<Vehicle>,

    /// Authorization
    pub authorization: Option<Authorization>,

    /// Current state
    pub state: SessionState,

    /// Start timestamp
    pub start_time: Timestamp,

    /// End timestamp (if finished)
    pub end_time: Option<Timestamp>,

    /// Last update timestamp
    pub last_update: Timestamp,

    /// Meter reading at start
    pub meter_start: Option<Meter>,

    /// Meter readings during session
    pub meter_readings: Vec<MeterValue>,

    /// Tariff applied
    pub tariff: Option<Tariff>,

    /// Energy consumed (watt-hours)
    pub energy_consumed: Energy,

    /// Charging profile (if smart charging)
    pub charging_profile: Option<ChargingProfile>,

    /// Session events
    pub events: Vec<SessionEvent>,

    /// Error (if any)
    pub error: Option<ChargingError>,

    /// Custom metadata
    pub metadata: HashMap<String, serde_json::Value>,
}

/// Session state lifecycle
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum SessionState {
    /// Session initialized, waiting for authorization
    Initializing,
    /// Authorizing user/vehicle
    Authorizing,
    /// Authorization successful, ready to charge
    Authorized,
    /// Actively charging
    Charging,
    /// Temporarily suspended (e.g., smart charging pause)
    Suspended,
    /// Finishing (wrapping up)
    Finishing,
    /// Session completed successfully
    Completed,
    /// Session failed with error
    Faulted,
}

impl SessionState {
    pub fn is_terminal(&self) -> bool {
        matches!(self, SessionState::Completed | SessionState::Faulted)
    }

    pub fn is_active(&self) -> bool {
        matches!(
            self,
            SessionState::Authorized | SessionState::Charging | SessionState::Suspended
        )
    }

    pub fn is_charging(&self) -> bool {
        matches!(self, SessionState::Charging)
    }
}

/// Session event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionEvent {
    pub timestamp: Timestamp,
    pub event_type: SessionEventType,
    pub description: String,
    pub data: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum SessionEventType {
    Started,
    Authorized,
    ChargingStarted,
    ChargingPaused,
    ChargingResumed,
    SmartChargingAdjusted,
    ChargingStopped,
    Completed,
    Faulted,
    MeterUpdate,
    ProfileApplied,
}

impl ChargingSession {
    pub fn new(station_id: StationId, evse_id: EvseId, connector_id: ConnectorId) -> Self {
        let now = now();
        Self {
            id: SessionId::new(),
            station_id,
            evse_id,
            connector_id,
            vehicle: None,
            authorization: None,
            state: SessionState::Initializing,
            start_time: now,
            end_time: None,
            last_update: now,
            meter_start: None,
            meter_readings: Vec::new(),
            tariff: None,
            energy_consumed: Energy::new(0),
            charging_profile: None,
            events: Vec::new(),
            error: None,
            metadata: HashMap::new(),
        }
    }

    pub fn add_meter_reading(&mut self, meter: MeterValue) {
        self.meter_readings.push(meter);
        self.last_update = now();
    }

    pub fn latest_meter_reading(&self) -> Option<&MeterValue> {
        self.meter_readings.last()
    }

    pub fn total_energy(&self) -> Energy {
        if let (Some(start), Some(last)) = (
            self.meter_start.as_ref(),
            self.latest_meter_reading()
        ) {
            Energy::new(
                last.energy_import.0.saturating_sub(start.energy_import.0)
            )
        } else {
            self.energy_consumed
        }
    }

    pub fn add_event(&mut self, event_type: SessionEventType, description: String) {
        self.events.push(SessionEvent {
            timestamp: now(),
            event_type,
            description,
            data: None,
        });
        self.last_update = now();
    }

    pub fn is_active(&self) -> bool {
        self.state.is_active()
    }

    pub fn transition_to(&mut self, new_state: SessionState) -> Result<()> {
        if new_state.is_terminal() && self.state.is_terminal() {
            return Err(CoreError::InvalidState(
                format!("Cannot transition from {:?} to {:?}", self.state, new_state)
            ));
        }
        self.state = new_state;
        self.last_update = now();
        Ok(())
    }
}