//! Charging Session model

use super::*;
use std::collections::HashMap;

/// Charging session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    /// Session identifier
    pub id: Id,
    
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum SessionState {
    Initializing,
    Authorizing,
    Authorized,
    Charging,
    Suspended,
    Finishing,
    Completed,
    Faulted,
}

impl Session {
    pub fn new(station_id: StationId, evse_id: EvseId, connector_id: ConnectorId) -> Self {
        let now = chrono::Utc::now();
        Self {
            id: Id::new(),
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
    
    pub fn is_active(&self) -> bool {
        matches!(
            self.state,
            SessionState::Authorized | SessionState::Charging | SessionState::Suspended
        )
    }
    
    pub fn total_energy(&self) -> Energy {
        if let (Some(start), Some(last)) = (
            self.meter_start.as_ref(),
            self.meter_readings.last()
        ) {
            Energy::new(
                last.energy_import.0.saturating_sub(start.energy_import.0)
            )
        } else {
            self.energy_consumed
        }
    }
}
