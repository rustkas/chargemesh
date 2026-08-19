//! Diagnostic Engine Tests

use chargemesh_inspector::*;
use chargemesh_ocpp::v16::*;

#[test]
fn test_diagnose_faulty_charger() {
    let mut inspector = Inspector::new();
    
    // Simulate a faulty charger session
    let messages = vec![
        r#"[2, "1", "BootNotification", {"chargePointVendor": "ABB", "chargePointModel": "Terra 54"}]"#,
        r#"[2, "2", "StatusNotification", {"connectorId": 1, "status": "Available", "errorCode": "NoError"}]"#,
        r#"[2, "3", "Authorize", {"idTag": "RFID-123"}]"#,
        r#"[3, "3", {"idTagInfo": {"status": "Accepted"}}]"#,
        r#"[2, "4", "StartTransaction", {"connectorId": 1, "idTag": "RFID-123", "meterStart": 0, "timestamp": "2024-01-01T00:00:00Z"}]"#,
        r#"[3, "4", {"transactionId": 12345, "idTagInfo": {"status": "Accepted"}}]"#,
        r#"[2, "5", "StatusNotification", {"connectorId": 1, "status": "Faulted", "errorCode": "EVCommunicationError"}]"#,
    ];
    
    for msg in messages {
        let parsed = parse_ocpp_message(msg).unwrap();
        inspector.add_message(parsed);
    }
    
    let report = inspector.generate_report();
    
    assert!(!report.errors.is_empty());
    assert!(!report.violations.is_empty());
    assert!(!report.recommendations.is_empty());
    
    // Check that we detected the communication error
    let has_comm_error = report.errors.iter().any(|e| 
        e.error_code == "EVCommunicationError"
    );
    assert!(has_comm_error);
}
