//! OCPP Parser Integration Tests

use chargemesh_ocpp::v16::*;

#[test]
fn test_parse_boot_notification() {
    let json = r#"[2, "1", "BootNotification", {"chargePointVendor": "ABB", "chargePointModel": "Terra 54"}]"#;
    let parsed = parse_ocpp_message(json).unwrap();
    
    match parsed.message {
        OcppMessage::Call { action, payload, .. } => {
            assert_eq!(action, "BootNotification");
            let req: BootNotificationRequest = serde_json::from_value(payload).unwrap();
            assert_eq!(req.charge_point_vendor, "ABB");
            assert_eq!(req.charge_point_model, "Terra 54");
        }
        _ => panic!("Expected Call message"),
    }
}

#[test]
fn test_parse_boot_notification_response() {
    let json = r#"[3, "1", {"status": "Accepted", "currentTime": "2024-01-01T00:00:00Z", "interval": 60}]"#;
    let parsed = parse_ocpp_message(json).unwrap();
    
    match parsed.message {
        OcppMessage::CallResult { payload, .. } => {
            let resp: BootNotificationResponse = serde_json::from_value(payload).unwrap();
            assert_eq!(resp.status, RegistrationStatus::Accepted);
            assert_eq!(resp.interval, 60);
        }
        _ => panic!("Expected CallResult message"),
    }
}

#[test]
fn test_parse_status_notification_faulted() {
    let json = r#"[2, "2", "StatusNotification", {"connectorId": 1, "status": "Faulted", "errorCode": "HighTemperature"}]"#;
    let parsed = parse_ocpp_message(json).unwrap();
    
    match parsed.message {
        OcppMessage::Call { action, payload, .. } => {
            assert_eq!(action, "StatusNotification");
            let req: StatusNotificationRequest = serde_json::from_value(payload).unwrap();
            assert_eq!(req.connector_id, 1);
            assert_eq!(req.status, ChargePointStatus::Faulted);
            assert_eq!(req.error_code, ChargePointErrorCode::HighTemperature);
        }
        _ => panic!("Expected Call message"),
    }
}
