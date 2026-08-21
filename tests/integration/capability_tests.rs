//! Capability engine tests

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
async fn test_capability_determination() {
    let engine = CapabilityEngine::new();
    let context = create_test_context();

    let capabilities = engine.determine_capabilities(&context).await.unwrap();

    // Check basic capabilities
    assert!(capabilities.is_supported(&CapabilityType::BasicCharging));
    assert!(capabilities.is_supported(&CapabilityType::OCPP2_0_1));
    assert!(capabilities.is_supported(&CapabilityType::OCPP1_6));
    assert!(capabilities.is_supported(&CapabilityType::SmartCharging));
    assert!(capabilities.is_supported(&CapabilityType::RemoteDiagnostics));
}

#[tokio::test]
async fn test_capability_limitations() {
    let engine = CapabilityEngine::new();
    let mut context = create_test_context();

    // Set high temperature
    context.runtime.temperature = Some(55.0);

    let capabilities = engine.determine_capabilities(&context).await.unwrap();

    // Fast charging should be limited
    if let Some(state) = capabilities.get_capability(&CapabilityType::FastCharging) {
        match state {
            CapabilityState::Limited { reason, .. } => {
                assert!(reason.contains("High temperature"));
            }
            _ => {}
        }
    }
}

#[tokio::test]
async fn test_offline_capabilities() {
    let engine = CapabilityEngine::new();
    let mut context = create_test_context();

    // Set offline
    context.runtime.is_online = false;

    let capabilities = engine.determine_capabilities(&context).await.unwrap();

    // Remote capabilities should be unavailable
    if let Some(state) = capabilities.get_capability(&CapabilityType::RemoteDiagnostics) {
        match state {
            CapabilityState::NotAvailable { reason } => {
                assert!(reason.contains("offline"));
            }
            _ => panic!("Expected NotAvailable state"),
        }
    }
}

#[tokio::test]
async fn test_capability_set_json() {
    let mut caps = CapabilitySet::new();
    caps.add_capability(
        CapabilityType::SmartCharging,
        CapabilityState::Supported {
            parameters: {
                let mut params = HashMap::new();
                params.insert("max_power".to_string(), serde_json::json!(22000));
                params
            },
        }
    );
    caps.add_capability(
        CapabilityType::V2G,
        CapabilityState::Limited {
            reason: "Hardware limitation".to_string(),
            parameters: {
                let mut params = HashMap::new();
                params.insert("max_power".to_string(), serde_json::json!(11000));
                params
            },
        }
    );

    let json = caps.to_json();
    assert!(json.is_object());

    let obj = json.as_object().unwrap();
    assert!(obj.contains_key("smartcharging"));
    assert!(obj.contains_key("v2g"));
}