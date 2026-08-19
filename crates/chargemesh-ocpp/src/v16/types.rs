//! OCPP 1.6 Types

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum RegistrationStatus {
    Accepted,
    Pending,
    Rejected,
}

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
pub struct MeterValue {
    pub timestamp: String,
    pub sampled_value: Vec<SampledValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SampledValue {
    pub value: String,
    pub context: Option<ReadingContext>,
    pub format: Option<ValueFormat>,
    pub measurand: Option<Measurand>,
    pub unit: Option<UnitOfMeasure>,
    pub location: Option<ValueLocation>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ReadingContext {
    SamplePeriodic,
    SampleClock,
    SampleTrigger,
    SampleStart,
    SampleStop,
    #[serde(rename = "Transaction.Begin")]
    TransactionBegin,
    #[serde(rename = "Transaction.End")]
    TransactionEnd,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ValueFormat {
    Raw,
    SignedData,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Measurand {
    EnergyActiveImportRegister,
    EnergyActiveExportRegister,
    PowerActiveImport,
    PowerActiveExport,
    CurrentImport,
    CurrentExport,
    Voltage,
    Frequency,
    Temperature,
    #[serde(rename = "SoC")]
    SoC,
    #[serde(rename = "RPM")]
    RPM,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum UnitOfMeasure {
    Wh,
    kWh,
    W,
    kW,
    A,
    V,
    Hz,
    Celsius,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ValueLocation {
    Outlet,
    Inlet,
    Body,
    #[serde(rename = "Cable")]
    Cable,
    #[serde(rename = "EV")]
    EV,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionData {
    pub timestamp: String,
    pub sampled_value: Vec<SampledValue>,
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
pub struct ChargingSchedule {
    pub duration: Option<u64>,
    pub start_schedule: Option<String>,
    pub charging_rate_unit: ChargingRateUnit,
    pub charging_schedule_period: Vec<ChargingSchedulePeriod>,
    pub min_charging_rate: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChargingSchedulePeriod {
    pub start_period: u64,
    pub limit: f64,
    pub number_phases: Option<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ChargingProfileKind {
    Recurring,
    OneTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ChargingProfilePurpose {
    ChargePointMaxProfile,
    TxDefaultProfile,
    TxProfile,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ChargingRateUnit {
    Watts,
    Amperes,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum RecurrencyKind {
    Daily,
    Weekly,
    Monthly,
    Yearly,
}
