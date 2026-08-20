//! OCPP 1.6 State Machine

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// OCPP 1.6 Session State
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Ocpp16State {
    /// Station is not connected
    Disconnected,

    /// Station is connecting
    Connecting,

    /// Station has booted but not authenticated
    Booted,

    /// Station is online and authenticated
    Online,

    /// Station is in a faulted state
    Faulted,
}

/// Active transaction in OCPP 1.6
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ocpp16Transaction {
    pub transaction_id: u64,
    pub connector_id: u64,
    pub id_tag: String,
    pub meter_start: u64,
    pub start_time: chrono::DateTime<chrono::Utc>,
    pub meter_readings: Vec<MeterValue>,
}

/// OCPP 1.6 Session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ocpp16Session {
    pub state: Ocpp16State,
    pub charge_point_id: Option<String>,
    pub connector_status: HashMap<u64, ChargePointStatus>,
    pub active_transactions: HashMap<u64, Ocpp16Transaction>,
    pub interval: u64,
    pub last_heartbeat: Option<chrono::DateTime<chrono::Utc>>,
}

impl Ocpp16Session {
    pub fn new() -> Self {
        Self {
            state: Ocpp16State::Disconnected,
            charge_point_id: None,
            connector_status: HashMap::new(),
            active_transactions: HashMap::new(),
            interval: 60,
            last_heartbeat: None,
        }
    }

    pub fn boot(&mut self, charge_point_id: String) {
        self.charge_point_id = Some(charge_point_id);
        self.state = Ocpp16State::Booted;
    }

    pub fn online(&mut self) {
        self.state = Ocpp16State::Online;
    }

    pub fn update_connector_status(&mut self, connector_id: u64, status: ChargePointStatus) {
        self.connector_status.insert(connector_id, status);
    }

    pub fn start_transaction(
        &mut self,
        transaction_id: u64,
        connector_id: u64,
        id_tag: String,
        meter_start: u64,
    ) {
        let transaction = Ocpp16Transaction {
            transaction_id,
            connector_id,
            id_tag,
            meter_start,
            start_time: chrono::Utc::now(),
            meter_readings: Vec::new(),
        };
        self.active_transactions.insert(transaction_id, transaction);
    }

    pub fn stop_transaction(&mut self, transaction_id: u64) {
        self.active_transactions.remove(&transaction_id);
    }

    pub fn add_meter_reading(&mut self, transaction_id: u64, meter_value: MeterValue) {
        if let Some(transaction) = self.active_transactions.get_mut(&transaction_id) {
            transaction.meter_readings.push(meter_value);
        }
    }

    pub fn heartbeat(&mut self) {
        self.last_heartbeat = Some(chrono::Utc::now());
    }

    pub fn is_online(&self) -> bool {
        matches!(self.state, Ocpp16State::Online)
    }

    pub fn get_transaction(&self, transaction_id: u64) -> Option<&Ocpp16Transaction> {
        self.active_transactions.get(&transaction_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_session_lifecycle() {
        let mut session = Ocpp16Session::new();
        assert_eq!(session.state, Ocpp16State::Disconnected);

        session.boot("CP-001".to_string());
        assert_eq!(session.state, Ocpp16State::Booted);
        assert_eq!(session.charge_point_id, Some("CP-001".to_string()));

        session.online();
        assert_eq!(session.state, Ocpp16State::Online);
        assert!(session.is_online());
    }

    #[test]
    fn test_transaction_management() {
        let mut session = Ocpp16Session::new();
        session.start_transaction(12345, 1, "RFID-123".to_string(), 1000);

        assert!(session.active_transactions.contains_key(&12345));

        let tx = session.get_transaction(12345).unwrap();
        assert_eq!(tx.connector_id, 1);
        assert_eq!(tx.id_tag, "RFID-123");

        session.stop_transaction(12345);
        assert!(!session.active_transactions.contains_key(&12345));
    }
}