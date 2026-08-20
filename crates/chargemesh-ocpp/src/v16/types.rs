//! OCPP 1.6 Types

use serde::{Deserialize, Serialize};
use crate::common::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum ChargePointStatus {
    Available,
    Preparing,
    Charging,
    SuspendedEVSE,
    SuspendedEV,
    Finishing,
    Reserved,
    Unavailable,
    Faulted,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum ChargePointErrorCode {
    NoError,
    ConnectorLockFailure,
    HighTemperature,
    GroundFailure,
    EVCommunicationError,
    PowerMeterFailure,
    PowerSwitchFailure,
    OverCurrentFailure,
    OverVoltage,
    UnderVoltage,
    InternalError,
    OtherError,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum StopReason {
    EMERGENCY_STOP,
    EV_DISCONNECTED,
    HARD_RESET,
    LOCAL,
    OTHER,
    POWER_LOSS,
    REBOOT,
    REMOTE,
    SOFT_RESET,
    UNLOCK_COMMAND,
    DE_AUTHORIZED,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum RemoteStartStopStatus {
    Accepted,
    Rejected,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ConfigurationStatus {
    Accepted,
    Rejected,
    NotSupported,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ResetType {
    Hard,
    Soft,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ResetStatus {
    Accepted,
    Rejected,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IdTagInfo {
    pub status: AuthorizationStatus,
    pub expiry_date: Option<String>,
    pub parent_id_tag: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AuthorizationStatus {
    Accepted,
    Blocked,
    Expired,
    Invalid,
    ConcurrentTx,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfigurationKey {
    pub key: String,
    pub readonly: bool,
    pub value: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChargingProfile {
    pub charging_profile_id: u64,
    pub stack_level: u64,
    pub charging_profile_kind: ChargingProfileKind,
    pub charging_profile_purpose: ChargingProfilePurpose,
    pub charging_schedule: ChargingSchedule,
    pub transaction_id: Option<String>,
    pub recurrency_kind: Option<RecurrencyKind>,
    pub valid_from: Option<String>,
    pub valid_to: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionData {
    pub timestamp: String,
    pub sampled_value: Vec<SampledValue>,
}