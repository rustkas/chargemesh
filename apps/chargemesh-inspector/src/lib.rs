//! ChargeMesh Inspector — Diagnostics engine

mod analyzer;
mod report;
mod capture;

pub use analyzer::*;
pub use report::*;
pub use capture::*;

use chargemesh_core::*;
use chargemesh_ir::*;
use chargemesh_ocpp::v16::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InspectionReport {
    pub station: Station,
    pub timeline: Vec<TimelineEntry>,
    pub violations: Vec<StateViolation>,
    pub errors: Vec<DiagnosedError>,
    pub recommendations: Vec<Recommendation>,
    pub summary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimelineEntry {
    pub timestamp: Timestamp,
    pub direction: String,
    pub message_type: String,
    pub details: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateViolation {
    pub timestamp: Timestamp,
    pub expected_state: String,
    pub actual_state: String,
    pub message: String,
    pub severity: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosedError {
    pub error_code: String,
    pub description: String,
    pub severity: String,
    pub root_cause: Option<String>,
    pub evidence: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recommendation {
    pub action: String,
    pub description: String,
    pub priority: String,
}

pub struct Inspector {
    messages: Vec<ParsedMessage>,
    station: Station,
    errors: Vec<DiagnosedError>,
    violations: Vec<StateViolation>,
    recommendations: Vec<Recommendation>,
}

impl Inspector {
    pub fn new() -> Self {
        Self {
            messages: Vec::new(),
            station: Station::new(
                StationId::new("unknown"),
                "unknown".to_string(),
                "unknown".to_string(),
            ),
            errors: Vec::new(),
            violations: Vec::new(),
            recommendations: Vec::new(),
        }
    }
    
    pub fn add_message(&mut self, message: ParsedMessage) {
        self.messages.push(message.clone());
        self.process_message(message);
    }
    
    fn process_message(&mut self, message: ParsedMessage) {
        match message.message {
            OcppMessage::Call { action, payload, .. } => {
                self.process_call(&action, &payload);
            }
            OcppMessage::CallError { error_code, error_description, .. } => {
                self.process_error(&error_code, &error_description);
            }
            _ => {}
        }
    }
    
    fn process_call(&mut self, action: &str, payload: &serde_json::Value) {
        match action {
            "BootNotification" => {
                if let Ok(req) = serde_json::from_value::<BootNotificationRequest>(payload.clone()) {
                    self.station.vendor = req.charge_point_vendor;
                    self.station.model = req.charge_point_model;
                    self.station.firmware_version = req.firmware_version;
                    self.station.state = StationState::Booted;
                }
            }
            "StatusNotification" => {
                if let Ok(req) = serde_json::from_value::<StatusNotificationRequest>(payload.clone()) {
                    if req.status == ChargePointStatus::Faulted {
                        self.detect_fault(req.error_code, req.info);
                    }
                }
            }
            _ => {}
        }
    }
    
    fn process_error(&mut self, error_code: &str, description: &str) {
        let error = DiagnosedError {
            error_code: error_code.to_string(),
            description: description.to_string(),
            severity: "Error".to_string(),
            root_cause: self.infer_root_cause(error_code, description),
            evidence: Vec::new(),
        };
        self.errors.push(error);
    }
    
    fn detect_fault(&mut self, error_code: ChargePointErrorCode, info: Option<String>) {
        let violation = StateViolation {
            timestamp: chrono::Utc::now(),
            expected_state: "Charging".to_string(),
            actual_state: "Faulted".to_string(),
            message: format!("Fault detected: {:?} - {}", error_code, info.unwrap_or_default()),
            severity: "Critical".to_string(),
        };
        self.violations.push(violation);
        
        let error = DiagnosedError {
            error_code: format!("{:?}", error_code),
            description: info.unwrap_or_else(|| "Unknown fault".to_string()),
            severity: "Critical".to_string(),
            root_cause: self.map_error_to_root_cause(&error_code),
            evidence: Vec::new(),
        };
        self.errors.push(error);
        
        self.recommendations.push(Recommendation {
            action: "Investigate fault".to_string(),
            description: self.map_error_to_root_cause(&error_code)
                .unwrap_or_else(|| "Check station logs and status".to_string()),
            priority: "High".to_string(),
        });
    }
    
    fn map_error_to_root_cause(&self, error_code: &ChargePointErrorCode) -> Option<String> {
        match error_code {
            ChargePointErrorCode::ConnectorLockFailure => {
                Some("Connector lock mechanism failed. Check physical lock and actuator.".to_string())
            }
            ChargePointErrorCode::HighTemperature => {
                Some("Station temperature exceeded safe threshold. Check cooling system.".to_string())
            }
            ChargePointErrorCode::EVCommunicationError => {
                Some("Communication with EV failed. Check cable connection and ISO 15118 handshake.".to_string())
            }
            ChargePointErrorCode::PowerMeterFailure => {
                Some("Power meter reading error. Meter may need calibration or replacement.".to_string())
            }
            ChargePointErrorCode::OverCurrentFailure => {
                Some("Current exceeded maximum limit. Possible short circuit or EV issue.".to_string())
            }
            ChargePointErrorCode::InternalError => {
                Some("Internal station error. Check firmware and system logs.".to_string())
            }
            _ => Some("Unknown error. Check station status and logs.".to_string()),
        }
    }
    
    fn infer_root_cause(&self, error_code: &str, description: &str) -> Option<String> {
        let combined = format!("{} {}", error_code, description).to_lowercase();
        
        if combined.contains("certificate") {
            Some("ISO 15118 certificate validation failed. Check certificate validity and trust chain.".to_string())
        } else if combined.contains("timeout") {
            Some("Network timeout. Check connectivity and firewall settings.".to_string())
        } else if combined.contains("authorization") || combined.contains("auth") {
            Some("Authorization failed. Check RFID token validity and backend connectivity.".to_string())
        } else {
            None
        }
    }
    
    pub fn generate_report(&self) -> InspectionReport {
        let summary = if self.errors.is_empty() && self.violations.is_empty() {
            "✅ No issues detected. Charging session appears normal.".to_string()
        } else {
            format!(
                "❌ Found {} errors and {} violations. Review details below.",
                self.errors.len(),
                self.violations.len()
            )
        };
        
        InspectionReport {
            station: self.station.clone(),
            timeline: Vec::new(),
            violations: self.violations.clone(),
            errors: self.errors.clone(),
            recommendations: self.recommendations.clone(),
            summary,
        }
    }
}

impl Default for Inspector {
    fn default() -> Self {
        Self::new()
    }
}
