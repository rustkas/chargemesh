//! ChargeMesh WASM bindings for Web Inspector
//!
//! This crate provides WASM exports for use in the Emerge-based Web Inspector.

#![warn(missing_docs)]
#![warn(rust_2018_idioms)]

use wasm_bindgen::prelude::*;
use serde_wasm_bindgen::to_value;
use std::collections::HashMap;

// ============================================================================
// OCPP Parser
// ============================================================================

/// Parse an OCPP message from JSON string
#[wasm_bindgen]
pub fn parse_ocpp_message(raw: &str) -> JsValue {
    match chargemesh_ocpp::v16::parse_ocpp_message(raw) {
        Ok(parsed) => {
            let result = serde_json::json!({
                "success": true,
                "timestamp": parsed.timestamp.to_rfc3339(),
                "direction": format!("{:?}", parsed.direction),
                "message": format!("{:?}", parsed.message),
                "raw": parsed.raw,
            });
            to_value(&result).unwrap_or(JsValue::NULL)
        }
        Err(e) => {
            let result = serde_json::json!({
                "success": false,
                "error": e,
            });
            to_value(&result).unwrap_or(JsValue::NULL)
        }
    }
}

// ============================================================================
// Timeline Analysis
// ============================================================================

/// Analyze a timeline of OCPP messages
#[wasm_bindgen]
pub fn analyze_timeline(messages: JsValue) -> JsValue {
    let entries: Vec<serde_json::Value> = serde_wasm_bindgen::from_value(messages).unwrap_or_default();

    let mut timeline = Vec::new();
    let mut state_machine = chargemesh_ocpp::v16::Ocpp16Session::new();
    let mut errors = Vec::new();

    for entry in entries {
        if let Some(raw) = entry.get("raw").and_then(|v| v.as_str()) {
            if let Ok(parsed) = chargemesh_ocpp::v16::parse_ocpp_message(raw) {
                // Process message through state machine
                match &parsed.message {
                    chargemesh_ocpp::common::OcppMessage::Call(call) => {
                        match call.action.as_str() {
                            "BootNotification" => {
                                state_machine.boot("CP-001".to_string());
                            }
                            "StatusNotification" => {
                                if let Ok(req) = serde_json::from_value::<chargemesh_ocpp::v16::StatusNotificationRequest>(
                                    call.payload.clone()
                                ) {
                                    state_machine.update_connector_status(
                                        req.connector_id,
                                        req.status
                                    );
                                }
                            }
                            "StartTransaction" => {
                                if let Ok(req) = serde_json::from_value::<chargemesh_ocpp::v16::StartTransactionRequest>(
                                    call.payload.clone()
                                ) {
                                    state_machine.start_transaction(
                                        req.transaction_id.unwrap_or(12345),
                                        req.connector_id,
                                        req.id_tag,
                                        req.meter_start,
                                    );
                                }
                            }
                            "StopTransaction" => {
                                if let Ok(req) = serde_json::from_value::<chargemesh_ocpp::v16::StopTransactionRequest>(
                                    call.payload.clone()
                                ) {
                                    state_machine.stop_transaction(req.transaction_id);
                                }
                            }
                            _ => {}
                        }
                    }
                    chargemesh_ocpp::common::OcppMessage::CallError(error) => {
                        errors.push(serde_json::json!({
                            "timestamp": parsed.timestamp.to_rfc3339(),
                            "error_code": error.error_code,
                            "error_description": error.error_description,
                        }));
                    }
                    _ => {}
                }

                timeline.push(serde_json::json!({
                    "timestamp": parsed.timestamp.to_rfc3339(),
                    "direction": format!("{:?}", parsed.direction),
                    "message": format!("{:?}", parsed.message),
                }));
            }
        }
    }

    // Build capability detection
    let context = chargemesh_capability::CapabilityContext {
        station_id: "CP-001".to_string(),
        protocol: chargemesh_capability::ProtocolInfo {
            name: chargemesh_capability::ProtocolName::OCPP,
            version: "1.6".to_string(),
            transport: "WebSocket".to_string(),
            security_profile: "Basic".to_string(),
        },
        vendor: chargemesh_capability::VendorInfo {
            name: "ABB".to_string(),
            id: Some("ABB".to_string()),
            known_models: vec!["Terra 54".to_string()],
        },
        firmware: chargemesh_capability::FirmwareInfo {
            version: "1.2.3".to_string(),
            build_date: None,
            checksum: None,
            compatibility: vec![],
        },
        configuration: HashMap::new(),
        runtime: chargemesh_capability::RuntimeState {
            is_online: true,
            is_booted: true,
            active_sessions: 0,
            total_energy_delivered: 0,
            uptime_seconds: 0,
            temperature: None,
            load_percentage: None,
        },
        model: "Terra 54".to_string(),
        hardware_version: None,
        serial_number: None,
    };

    let engine = chargemesh_capability::CapabilityEngine::new();
    let capabilities = match tokio::runtime::Runtime::new() {
        Ok(rt) => rt.block_on(engine.determine_capabilities(&context)).ok(),
        Err(_) => None,
    };

    let result = serde_json::json!({
        "timeline": timeline,
        "state": {
            "state": format!("{:?}", state_machine.state),
            "connectors": state_machine.connector_status,
            "transactions": state_machine.active_transactions.len(),
        },
        "errors": errors,
        "capabilities": capabilities.map(|c| c.to_json()),
    });

    to_value(&result).unwrap_or(JsValue::NULL)
}

// ============================================================================
// Diagnostic Analysis
// ============================================================================

/// Run diagnostics on a timeline
#[wasm_bindgen]
pub fn run_diagnostics(timeline: JsValue) -> JsValue {
    let entries: Vec<serde_json::Value> = serde_wasm_bindgen::from_value(timeline).unwrap_or_default();

    let mut collector = chargemesh_diagnostics::TimelineCollector::new();

    for entry in entries {
        if let Some(raw) = entry.get("raw").and_then(|v| v.as_str()) {
            if let Ok(parsed) = chargemesh_ocpp::v16::parse_ocpp_message(raw) {
                use chargemesh_diagnostics::*;

                let timeline_entry = TimelineEntry {
                    id: uuid::Uuid::new_v4().to_string(),
                    timestamp: parsed.timestamp,
                    event_type: match parsed.message {
                        chargemesh_ocpp::common::OcppMessage::Call(call) => {
                            match call.action.as_str() {
                                "BootNotification" => EventType::BootNotification,
                                "Heartbeat" => EventType::Heartbeat,
                                "StatusNotification" => EventType::StatusNotification,
                                "Authorize" => EventType::Authorize,
                                "StartTransaction" => EventType::StartTransaction,
                                "StopTransaction" => EventType::StopTransaction,
                                "MeterValues" => EventType::MeterValues,
                                _ => EventType::Info,
                            }
                        }
                        chargemesh_ocpp::common::OcppMessage::CallResult(_) => EventType::Info,
                        chargemesh_ocpp::common::OcppMessage::CallError(_) => EventType::Error,
                    },
                    component: match parsed.message {
                        chargemesh_ocpp::common::OcppMessage::CallError(_) => Component::Protocol,
                        _ => Component::Protocol,
                    },
                    status: match parsed.message {
                        chargemesh_ocpp::common::OcppMessage::CallError(_) => EntryStatus::Failure,
                        _ => EntryStatus::Success,
                    },
                    details: serde_json::json!({
                        "raw": parsed.raw,
                    }),
                    session_id: None,
                    station_id: None,
                    connector_id: None,
                    transaction_id: None,
                    tags: vec![],
                };

                let _ = collector.add_entry(timeline_entry);
            }
        }
    }

    let engine = chargemesh_diagnostics::DiagnosticsEngine::default();
    let context = chargemesh_diagnostics::DiagnosticContext {
        station_id: None,
        session_id: None,
        time_range: None,
        protocol: Some("OCPP 1.6".to_string()),
        vendor: None,
        model: None,
        firmware_version: None,
    };

    let report = match tokio::runtime::Runtime::new() {
        Ok(rt) => rt.block_on(engine.run_diagnostics(&context)).ok(),
        Err(_) => None,
    };

    let result = serde_json::json!({
        "report": report,
        "has_issues": report.as_ref().map(|r| !r.root_causes.is_empty()).unwrap_or(false),
    });

    to_value(&result).unwrap_or(JsValue::NULL)
}

// ============================================================================
// Capability Analysis
// ============================================================================

/// Analyze capabilities from station context
#[wasm_bindgen]
pub fn analyze_capabilities(context: JsValue) -> JsValue {
    let ctx: chargemesh_capability::CapabilityContext = serde_wasm_bindgen::from_value(context).unwrap_or_else(|_| {
        chargemesh_capability::CapabilityContext {
            station_id: "CP-001".to_string(),
            protocol: chargemesh_capability::ProtocolInfo {
                name: chargemesh_capability::ProtocolName::OCPP,
                version: "1.6".to_string(),
                transport: "WebSocket".to_string(),
                security_profile: "Basic".to_string(),
            },
            vendor: chargemesh_capability::VendorInfo {
                name: "ABB".to_string(),
                id: Some("ABB".to_string()),
                known_models: vec!["Terra 54".to_string()],
            },
            firmware: chargemesh_capability::FirmwareInfo {
                version: "1.2.3".to_string(),
                build_date: None,
                checksum: None,
                compatibility: vec![],
            },
            configuration: HashMap::new(),
            runtime: chargemesh_capability::RuntimeState {
                is_online: true,
                is_booted: true,
                active_sessions: 0,
                total_energy_delivered: 0,
                uptime_seconds: 0,
                temperature: None,
                load_percentage: None,
            },
            model: "Terra 54".to_string(),
            hardware_version: None,
            serial_number: None,
        }
    });

    let engine = chargemesh_capability::CapabilityEngine::new();

    let capabilities = match tokio::runtime::Runtime::new() {
        Ok(rt) => rt.block_on(engine.determine_capabilities(&ctx)).ok(),
        Err(_) => None,
    };

    let result = serde_json::json!({
        "capabilities": capabilities.map(|c| c.to_json()),
        "supported_count": capabilities.as_ref().map(|c| {
            c.capabilities.values().filter(|s| s.is_supported()).count()
        }).unwrap_or(0),
        "total_count": capabilities.as_ref().map(|c| c.capabilities.len()).unwrap_or(0),
    });

    to_value(&result).unwrap_or(JsValue::NULL)
}

// ============================================================================
// Utilities
// ============================================================================

/// Get version information
#[wasm_bindgen]
pub fn version() -> String {
    format!(
        "ChargeMesh WASM v{} (IR v{})",
        env!("CARGO_PKG_VERSION"),
        chargemesh_ir::IR_VERSION
    )
}