//! Fault injection system

mod fault_injector;
mod scenarios;

pub use fault_injector::*;
pub use scenarios::*;

use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum FaultType {
    // Network faults
    NetworkDisconnect,
    NetworkTimeout,
    NetworkLatency,
    NetworkPacketLoss,

    // Protocol faults
    InvalidMessage,
    CorruptedMessage,
    OutOfOrderMessage,
    UnexpectedMessage,

    // EVSE faults
    ConnectorLockFailure,
    PowerMeterFailure,
    PowerSwitchFailure,
    OverCurrent,
    OverVoltage,
    HighTemperature,

    // EV faults
    EVCommunicationError,
    BatteryFailure,
    CertificateFailure,

    // Grid faults
    GridOutage,
    GridVoltageDip,
    GridFrequencyDeviation,
    GridHighDemand,

    // Authorization faults
    AuthorizationFailure,
    TokenExpired,
    TokenRevoked,
    PaymentFailure,

    // System faults
    FirmwareBug,
    MemoryLeak,
    Timeout,
    Crash,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FaultInjection {
    pub fault_type: FaultType,
    pub probability: f64,
    pub duration: chrono::Duration,
    pub condition: Option<core::Condition>,
    pub parameters: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActiveFault {
    pub fault_type: FaultType,
    pub injected_at: chrono::DateTime<chrono::Utc>,
    pub duration: chrono::Duration,
    pub active: bool,
}

pub struct FaultInjector {
    active_faults: tokio::sync::RwLock<Vec<ActiveFault>>,
}

impl FaultInjector {
    pub fn new() -> Self {
        Self {
            active_faults: tokio::sync::RwLock::new(Vec::new()),
        }
    }

    pub async fn inject_fault(&self, fault: FaultInjection) -> Result<()> {
        let active = ActiveFault {
            fault_type: fault.fault_type,
            injected_at: chrono::Utc::now(),
            duration: fault.duration,
            active: true,
        };
        self.active_faults.write().await.push(active);
        Ok(())
    }

    pub async fn clear_fault(&self, fault_type: &FaultType) {
        self.active_faults
            .write()
            .await
            .retain(|f| &f.fault_type != fault_type);
    }

    pub async fn clear_all_faults(&self) {
        self.active_faults.write().await.clear();
    }

    pub async fn is_fault_active(&self, fault_type: &FaultType) -> bool {
        self.active_faults
            .read()
            .await
            .iter()
            .any(|f| &f.fault_type == fault_type && f.active)
    }

    pub async fn get_active_faults(&self) -> Vec<FaultType> {
        self.active_faults
            .read()
            .await
            .iter()
            .filter(|f| f.active)
            .map(|f| f.fault_type.clone())
            .collect()
    }

    pub async fn update(&self) {
        let now = chrono::Utc::now();
        for fault in self.active_faults.write().await.iter_mut() {
            if now - fault.injected_at > fault.duration {
                fault.active = false;
            }
        }
    }
}

impl Default for FaultInjector {
    fn default() -> Self {
        Self::new()
    }
}