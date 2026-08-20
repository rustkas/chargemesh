//! End-to-end OCPP tests

use chargemesh_ocpp::v16::*;
use chargemesh_ocpp::common::*;

#[test]
fn test_complete_session_flow() {
    let mut session = Ocpp16Session::new();

    // 1. Boot
    session.boot("CP-001".to_string());
    assert_eq!(session.state, Ocpp16State::Booted);

    // 2. Online
    session.online();
    assert!(session.is_online());

    // 3. Status update
    session.update_connector_status(1, ChargePointStatus::Available);
    assert_eq!(
        session.connector_status.get(&1),
        Some(&ChargePointStatus::Available)
    );

    // 4. Start transaction
    session.start_transaction(12345, 1, "RFID-123".to_string(), 1000);
    assert!(session.active_transactions.contains_key(&12345));

    // 5. Meter values
    let meter1 = MeterValue {
        timestamp: chrono::Utc::now(),
        sampled_value: vec![SampledValue {
            value: "10.5".to_string(),
            context: Some(ReadingContext::SamplePeriodic),
            format: None,
            measurand: Some(Measurand::EnergyActiveImportRegister),
            unit: Some(UnitOfMeasure::kWh),
            location: None,
        }],
    };
    session.add_meter_reading(12345, meter1);

    let meter2 = MeterValue {
        timestamp: chrono::Utc::now(),
        sampled_value: vec![SampledValue {
            value: "15.2".to_string(),
            context: Some(ReadingContext::SamplePeriodic),
            format: None,
            measurand: Some(Measurand::EnergyActiveImportRegister),
            unit: Some(UnitOfMeasure::kWh),
            location: None,
        }],
    };
    session.add_meter_reading(12345, meter2);

    // 6. Stop transaction
    let tx = session.get_transaction(12345).unwrap();
    assert_eq!(tx.meter_readings.len(), 2);

    session.stop_transaction(12345);
    assert!(!session.active_transactions.contains_key(&12345));

    // 7. Status update
    session.update_connector_status(1, ChargePointStatus::Available);
}

#[test]
fn test_fault_handling() {
    let mut session = Ocpp16Session::new();

    session.boot("CP-001".to_string());
    session.online();

    session.update_connector_status(1, ChargePointStatus::Faulted);
    assert_eq!(
        session.connector_status.get(&1),
        Some(&ChargePointStatus::Faulted)
    );

    // Simulate recovery
    session.update_connector_status(1, ChargePointStatus::Available);
    assert_eq!(
        session.connector_status.get(&1),
        Some(&ChargePointStatus::Available)
    );
}