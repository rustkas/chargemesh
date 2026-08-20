//! Common OCPP types

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use std::collections::HashMap;

/// OCPP Error codes (OCPP 1.6)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum OcppErrorCode {
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

/// OCPP 2.x Error codes
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum OcppErrorCode201 {
    NotImplemented,
    NotSupported,
    InternalError,
    ProtocolError,
    SecurityError,
    Rejected,
    Timeout,
    GenericError,
    // More specific errors
    CertificateExpired,
    CertificateRevoked,
    InvalidCertificate,
    EvseIdMismatch,
    ConnectorIdMismatch,
}

/// Charging profile status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ChargingProfileStatus {
    Accepted,
    Rejected,
    Unknown,
}

/// Registration status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum RegistrationStatus {
    Accepted,
    Pending,
    Rejected,
}

/// Message format for OCPP
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChargingSchedule {
    pub duration: Option<u64>,
    pub start_schedule: Option<DateTime<Utc>>,
    pub charging_rate_unit: ChargingRateUnit,
    pub charging_schedule_period: Vec<ChargingSchedulePeriod>,
    pub min_charging_rate: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ChargingRateUnit {
    Watts,
    Amperes,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChargingSchedulePeriod {
    pub start_period: u64,
    pub limit: f64,
    pub number_phases: Option<u8>,
}

/// Meter values
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MeterValue {
    pub timestamp: DateTime<Utc>,
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
pub enum RecurrencyKind {
    Daily,
    Weekly,
    Monthly,
    Yearly,
}