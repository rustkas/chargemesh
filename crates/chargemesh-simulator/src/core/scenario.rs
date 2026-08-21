//! Scenario definitions for simulation

use super::*;
use crate::faults::FaultType;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Scenario {
    pub name: String,
    pub description: String,
    pub steps: Vec<ScenarioStep>,
    pub faults: Vec<FaultInjection>,
    pub conditions: ScenarioConditions,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScenarioStep {
    pub action: ScenarioAction,
    pub delay: Option<chrono::Duration>,
    pub repeat: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ScenarioAction {
    ConnectEV,
    StartCharging,
    StopCharging,
    DisconnectEV,
    Authorize { token: String },
    SetProfile { max_power: u64, duration: chrono::Duration },
    InjectFault { fault: FaultType },
    WaitFor { condition: Condition },
    Log { message: String },
    Custom { name: String, data: serde_json::Value },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScenarioConditions {
    pub min_soc: Option<u8>,
    pub max_soc: Option<u8>,
    pub min_duration: Option<chrono::Duration>,
    pub max_duration: Option<chrono::Duration>,
    pub min_energy: Option<u64>,
    pub max_energy: Option<u64>,
    pub error_rate: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Condition {
    ChargingActive,
    ChargingStopped,
    TargetSocReached,
    BatteryFull,
    NetworkReconnected,
    Iso15118HandshakeComplete,
    V2GActive,
    AuthorizationComplete,
    FaultCleared,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FaultInjection {
    pub fault_type: FaultType,
    pub probability: f64,
    pub duration: chrono::Duration,
    pub condition: Option<Condition>,
}

/// Pre-defined scenarios
pub struct Scenarios;

impl Scenarios {
    pub fn normal_session() -> Scenario {
        Scenario {
            name: "Normal Charging Session".to_string(),
            description: "Standard EV charging session with no faults".to_string(),
            steps: vec![
                ScenarioStep {
                    action: ScenarioAction::ConnectEV,
                    delay: None,
                    repeat: None,
                },
                ScenarioStep {
                    action: ScenarioAction::Authorize { token: "RFID-123".to_string() },
                    delay: Some(chrono::Duration::seconds(1)),
                    repeat: None,
                },
                ScenarioStep {
                    action: ScenarioAction::StartCharging,
                    delay: Some(chrono::Duration::seconds(2)),
                    repeat: None,
                },
                ScenarioStep {
                    action: ScenarioAction::WaitFor {
                        condition: Condition::TargetSocReached,
                    },
                    delay: None,
                    repeat: None,
                },
                ScenarioStep {
                    action: ScenarioAction::StopCharging,
                    delay: None,
                    repeat: None,
                },
                ScenarioStep {
                    action: ScenarioAction::DisconnectEV,
                    delay: Some(chrono::Duration::seconds(1)),
                    repeat: None,
                },
            ],
            faults: vec![],
            conditions: ScenarioConditions {
                min_soc: Some(20),
                max_soc: Some(80),
                min_duration: Some(chrono::Duration::minutes(10)),
                max_duration: Some(chrono::Duration::minutes(60)),
                min_energy: Some(5000),
                max_energy: Some(50000),
                error_rate: Some(0.0),
            },
        }
    }

    pub fn network_failure() -> Scenario {
        Scenario {
            name: "Network Failure".to_string(),
            description: "Simulate network disconnection during charging".to_string(),
            steps: vec![
                ScenarioStep {
                    action: ScenarioAction::ConnectEV,
                    delay: None,
                    repeat: None,
                },
                ScenarioStep {
                    action: ScenarioAction::Authorize { token: "RFID-123".to_string() },
                    delay: Some(chrono::Duration::seconds(1)),
                    repeat: None,
                },
                ScenarioStep {
                    action: ScenarioAction::StartCharging,
                    delay: Some(chrono::Duration::seconds(2)),
                    repeat: None,
                },
                ScenarioStep {
                    action: ScenarioAction::InjectFault { fault: FaultType::NetworkDisconnect },
                    delay: Some(chrono::Duration::seconds(5)),
                    repeat: None,
                },
                ScenarioStep {
                    action: ScenarioAction::WaitFor {
                        condition: Condition::NetworkReconnected,
                    },
                    delay: Some(chrono::Duration::seconds(10)),
                    repeat: None,
                },
                ScenarioStep {
                    action: ScenarioAction::StopCharging,
                    delay: Some(chrono::Duration::seconds(2)),
                    repeat: None,
                },
            ],
            faults: vec![FaultInjection {
                fault_type: FaultType::NetworkDisconnect,
                probability: 1.0,
                duration: chrono::Duration::seconds(10),
                condition: Some(Condition::ChargingActive),
            }],
            conditions: ScenarioConditions {
                min_soc: Some(20),
                max_soc: Some(80),
                min_duration: None,
                max_duration: Some(chrono::Duration::minutes(30)),
                min_energy: None,
                max_energy: None,
                error_rate: Some(0.0),
            },
        }
    }

    pub fn auth_failure() -> Scenario {
        Scenario {
            name: "Authorization Failure".to_string(),
            description: "Simulate authorization failure".to_string(),
            steps: vec![
                ScenarioStep {
                    action: ScenarioAction::ConnectEV,
                    delay: None,
                    repeat: None,
                },
                ScenarioStep {
                    action: ScenarioAction::Authorize { token: "INVALID-TOKEN".to_string() },
                    delay: Some(chrono::Duration::seconds(1)),
                    repeat: None,
                },
                ScenarioStep {
                    action: ScenarioAction::Log {
                        message: "Authorization failed - expected".to_string(),
                    },
                    delay: Some(chrono::Duration::seconds(1)),
                    repeat: None,
                },
                ScenarioStep {
                    action: ScenarioAction::Authorize { token: "RFID-123".to_string() },
                    delay: Some(chrono::Duration::seconds(2)),
                    repeat: None,
                },
                ScenarioStep {
                    action: ScenarioAction::StartCharging,
                    delay: Some(chrono::Duration::seconds(1)),
                    repeat: None,
                },
            ],
            faults: vec![],
            conditions: ScenarioConditions {
                min_soc: Some(20),
                max_soc: Some(80),
                min_duration: None,
                max_duration: None,
                min_energy: None,
                max_energy: None,
                error_rate: Some(0.0),
            },
        }
    }

    pub fn plug_and_charge() -> Scenario {
        Scenario {
            name: "Plug & Charge".to_string(),
            description: "ISO 15118 Plug & Charge authentication".to_string(),
            steps: vec![
                ScenarioStep {
                    action: ScenarioAction::ConnectEV,
                    delay: None,
                    repeat: None,
                },
                ScenarioStep {
                    action: ScenarioAction::WaitFor {
                        condition: Condition::Iso15118HandshakeComplete,
                    },
                    delay: Some(chrono::Duration::seconds(3)),
                    repeat: None,
                },
                ScenarioStep {
                    action: ScenarioAction::StartCharging,
                    delay: Some(chrono::Duration::seconds(1)),
                    repeat: None,
                },
                ScenarioStep {
                    action: ScenarioAction::WaitFor {
                        condition: Condition::TargetSocReached,
                    },
                    delay: None,
                    repeat: None,
                },
                ScenarioStep {
                    action: ScenarioAction::StopCharging,
                    delay: None,
                    repeat: None,
                },
            ],
            faults: vec![],
            conditions: ScenarioConditions {
                min_soc: Some(20),
                max_soc: Some(80),
                min_duration: Some(chrono::Duration::minutes(10)),
                max_duration: Some(chrono::Duration::minutes(60)),
                min_energy: Some(5000),
                max_energy: Some(50000),
                error_rate: Some(0.0),
            },
        }
    }

    pub fn v2g() -> Scenario {
        Scenario {
            name: "V2G (Vehicle-to-Grid)".to_string(),
            description: "Bidirectional charging and grid feed-in".to_string(),
            steps: vec![
                ScenarioStep {
                    action: ScenarioAction::ConnectEV,
                    delay: None,
                    repeat: None,
                },
                ScenarioStep {
                    action: ScenarioAction::Authorize { token: "V2G-TOKEN".to_string() },
                    delay: Some(chrono::Duration::seconds(1)),
                    repeat: None,
                },
                ScenarioStep {
                    action: ScenarioAction::StartCharging,
                    delay: Some(chrono::Duration::seconds(2)),
                    repeat: None,
                },
                ScenarioStep {
                    action: ScenarioAction::WaitFor {
                        condition: Condition::BatteryFull,
                    },
                    delay: None,
                    repeat: None,
                },
                ScenarioStep {
                    action: ScenarioAction::InjectFault { fault: FaultType::GridHighDemand },
                    delay: Some(chrono::Duration::seconds(5)),
                    repeat: None,
                },
                ScenarioStep {
                    action: ScenarioAction::WaitFor {
                        condition: Condition::V2GActive,
                    },
                    delay: Some(chrono::Duration::seconds(5)),
                    repeat: None,
                },
                ScenarioStep {
                    action: ScenarioAction::StopCharging,
                    delay: Some(chrono::Duration::seconds(2)),
                    repeat: None,
                },
            ],
            faults: vec![FaultInjection {
                fault_type: FaultType::GridHighDemand,
                probability: 1.0,
                duration: chrono::Duration::seconds(30),
                condition: Some(Condition::BatteryFull),
            }],
            conditions: ScenarioConditions {
                min_soc: Some(80),
                max_soc: Some(100),
                min_duration: None,
                max_duration: Some(chrono::Duration::hours(2)),
                min_energy: None,
                max_energy: None,
                error_rate: Some(0.0),
            },
        }
    }

    pub fn certificate_failure() -> Scenario {
        Scenario {
            name: "Certificate Failure".to_string(),
            description: "ISO 15118 certificate validation failure".to_string(),
            steps: vec![
                ScenarioStep {
                    action: ScenarioAction::ConnectEV,
                    delay: None,
                    repeat: None,
                },
                ScenarioStep {
                    action: ScenarioAction::WaitFor {
                        condition: Condition::Iso15118HandshakeComplete,
                    },
                    delay: Some(chrono::Duration::seconds(3)),
                    repeat: None,
                },
                ScenarioStep {
                    action: ScenarioAction::InjectFault { fault: FaultType::CertificateFailure },
                    delay: Some(chrono::Duration::seconds(1)),
                    repeat: None,
                },
                ScenarioStep {
                    action: ScenarioAction::Log {
                        message: "Certificate validation failed - as expected".to_string(),
                    },
                    delay: Some(chrono::Duration::seconds(1)),
                    repeat: None,
                },
            ],
            faults: vec![FaultInjection {
                fault_type: FaultType::CertificateFailure,
                probability: 1.0,
                duration: chrono::Duration::seconds(5),
                condition: Some(Condition::Iso15118HandshakeComplete),
            }],
            conditions: ScenarioConditions {
                min_soc: None,
                max_soc: None,
                min_duration: None,
                max_duration: None,
                min_energy: None,
                max_energy: None,
                error_rate: Some(1.0),
            },
        }
    }
}