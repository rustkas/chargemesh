//! End-to-end capability engine tests

use chargemesh_capability::*;
use std::collections::HashMap;

fn create_test_context() -> CapabilityContext {
    let mut config = HashMap::new();
    config.insert("certificate_enabled".to_string(), serde_json::json!(true));

    CapabilityContext {
        station_id: "CP-TEST-001".to_string(),
        protocol: ProtocolInfo {
            name: ProtocolName::OCPP,
            version: "2.0.1".to_string(),
            transport: "WebSocket".to_string(),
            security_profile: "TLS".to_string(),
        },
        vendor: VendorInfo {
            name: "ABB".to_string(),
            id: Some("ABB".to_string()),
            known_models: vec!["Terra 54".to_string()],
        },
        firmware: FirmwareInfo {
            version: "2.1.0".to_string(),
            build_date: Some("2024-01-15".to_string()),
            checksum: None,
            compatibility: vec![],
        },
        configuration: config,
        runtime: RuntimeState {
            is_online: true,
            is_booted: true,
            active_sessions: 0,
            total_energy_delivered: 1000,
            uptime_seconds: 3600,
            temperature: Some(35.0),
            load_percentage: Some(50),
        },
        model: "Terra 54".to_string(),
        hardware_version: Some("2.0".to_string()),
        serial_number: Some("SN12345".to_string()),
    }
}

#[tokio::test]
async fn test_full_capability_workflow() {
    let engine = CapabilityEngine::new();
    let context = create_test_context();

    let capabilities = engine.determine_capabilities(&context).await.unwrap();

    // Print capabilities as JSON
    let json = capabilities.to_json();
    println!("{}", serde_json::to_string_pretty(&json).unwrap());

    // Check specific capabilities
    assert!(capabilities.is_available_or_limited(&CapabilityType::SmartCharging));
    assert!(capabilities.is_supported(&CapabilityType::BasicCharging));
    assert!(capabilities.is_supported(&CapabilityType::RemoteDiagnostics));

    // Check that OCPP 2.0.1 features are detected
    assert!(capabilities.is_supported(&CapabilityType::OCPP2_0_1));
    assert!(capabilities.is_supported(&CapabilityType::ConfigurationManagement));
}

#[tokio::test]
async fn test_rule_evaluation() {
    let engine = CapabilityEngine::new();
    let mut context = create_test_context();

    // Add ISO 15118 support
    context.protocol = ProtocolInfo {
        name: ProtocolName::ISO15118,
        version: "2.0".to_string(),
        transport: "TCP".to_string(),
        security_profile: "TLS-PKI".to_string(),
    };
    context.firmware.version = "2.0.0".to_string();

    let capabilities = engine.determine_capabilities(&context).await.unwrap();

    // Plug & Charge should be supported via rule
    assert!(capabilities.is_supported(&CapabilityType::PlugAndCharge));
}