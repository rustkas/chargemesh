//! OCPP Integration Tests

use chargemesh_ocpp::v16::*;
use chargemesh_ocpp::common::*;

#[test]
fn test_parse_boot_notification() {
    let json = r#"[2, "1", "BootNotification", {"chargePointVendor": "ABB", "chargePointModel": "Terra 54"}]"#;
    let parsed = parse_ocpp_message(json).unwrap();

    match parsed.message {
        OcppMessage::Call(call) => {
            assert_eq!(call.action, "BootNotification");
            let req: BootNotificationRequest = serde_json::from_value(call.payload).unwrap();
            assert_eq!(req.charge_point_vendor, "ABB");
            assert_eq!(req.charge_point_model, "Terra 54");
        }
        _ => panic!("Expected Call message"),
    }
}

#[test]
fn test_parse_status_notification_faulted() {
    let json = r#"[2, "2", "StatusNotification", {"connectorId": 1, "status": "Faulted", "errorCode": "HighTemperature"}]"#;
    let parsed = parse_ocpp_message(json).unwrap();

    match parsed.message {
        OcppMessage::Call(call) => {
            assert_eq!(call.action, "StatusNotification");
            let req: StatusNotificationRequest = serde_json::from_value(call.payload).unwrap();
            assert_eq!(req.connector_id, 1);
            assert_eq!(req.status, ChargePointStatus::Faulted);
            assert_eq!(req.error_code, ChargePointErrorCode::HighTemperature);
        }
        _ => panic!("Expected Call message"),
    }
}

#[test]
fn test_serialize_authorize() {
    let request = AuthorizeRequest {
        id_tag: "RFID-123".to_string(),
    };
    let json = serde_json::to_string(&request).unwrap();
    assert!(json.contains("RFID-123"));
}

#[test]
fn test_serialize_boot_notification_response() {
    let response = BootNotificationResponse {
        status: RegistrationStatus::Accepted,
        current_time: "2024-01-01T00:00:00Z".to_string(),
        interval: 60,
    };
    let json = serde_json::to_string(&response).unwrap();
    assert!(json.contains("Accepted"));
    assert!(json.contains("60"));
}

#[test]
fn test_session_state() {
    let mut session = Ocpp16Session::new();

    session.boot("CP-TEST".to_string());
    assert_eq!(session.state, Ocpp16State::Booted);

    session.update_connector_status(1, ChargePointStatus::Available);
    assert_eq!(
        session.connector_status.get(&1),
        Some(&ChargePointStatus::Available)
    );

    session.start_transaction(12345, 1, "RFID-123".to_string(), 1000);
    assert!(session.active_transactions.contains_key(&12345));

    session.stop_transaction(12345);
    assert!(!session.active_transactions.contains_key(&12345));
}

#[test]
fn test_transaction_meter_readings() {
    let mut session = Ocpp16Session::new();
    session.start_transaction(12345, 1, "RFID-123".to_string(), 1000);

    let meter = MeterValue {
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

    session.add_meter_reading(12345, meter);
    let tx = session.get_transaction(12345).unwrap();
    assert_eq!(tx.meter_readings.len(), 1);
}