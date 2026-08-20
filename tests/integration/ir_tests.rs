//! Integration tests for EV-IR model

use chargemesh_ir::prelude::*;
use chargemesh_core::*;

#[test]
fn test_full_station_creation() {
    let mut station = ChargingStation::new(
        StationId::new("CP-001"),
        "ABB".to_string(),
        "Terra 54".to_string(),
    );

    let evse = EVSE::new(
        EvseId::new("EVSE-1"),
        StationId::new("CP-001"),
        Power::new(50000),
    );

    station.add_evse(evse);
    assert_eq!(station.evses.len(), 1);
}

#[test]
fn test_session_lifecycle() {
    let mut sm = SessionStateMachine::new();
    sm.start_authorization().unwrap();
    sm.authorize().unwrap();
    sm.start_charging().unwrap();

    assert_eq!(*sm.state(), SessionState::Charging);
}

#[test]
fn test_capabilities() {
    let mut caps = Capabilities::default();
    caps.add(CapabilityType::SmartCharging);
    caps.add(CapabilityType::V2G);

    assert!(caps.has(CapabilityType::SmartCharging));
    assert!(caps.has(CapabilityType::V2G));
    assert!(!caps.has(CapabilityType::LoadBalancing));
}

#[test]
fn test_network() {
    let mut network = ChargingNetwork::new("net-1", "Test Network", "Test Operator");
    let station = ChargingStation::new(
        StationId::new("CP-001"),
        "ABB".to_string(),
        "Terra 54".to_string(),
    );
    network.add_station(station);
    assert_eq!(network.stations.len(), 1);
}

#[test]
fn test_reservation() {
    let reservation = Reservation::new(
        StationId::new("CP-001"),
        EvseId::new("EVSE-1"),
        ConnectorId::new("CONN-1"),
        "user-1".to_string(),
        "token-123".to_string(),
        now() + chrono::Duration::hours(1),
    );
    assert_eq!(reservation.status, ReservationStatus::Pending);
    assert!(reservation.is_active());
}

#[test]
fn test_transaction() {
    let auth = Authorization::new(AuthorizationType::RFID, "RFID-123".to_string());
    let meter_start = MeterValue::new(Energy::new(1000));
    let tariff = Tariff::default();

    let mut tx = Transaction::new(
        SessionId::new(),
        auth,
        meter_start,
        tariff,
    );

    let meter_end = MeterValue::new(Energy::new(1045));
    tx.complete(meter_end);

    assert!(tx.is_completed());
    assert_eq!(tx.energy_delivered.0, 45);
}