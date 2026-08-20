//! OCPP 1.6 Messages

use serde::{Deserialize, Serialize};
use super::types::*;
use crate::common::*;

// ============================================================================
// BootNotification
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BootNotificationRequest {
    pub charge_point_vendor: String,
    pub charge_point_model: String,
    pub charge_point_serial_number: Option<String>,
    pub firmware_version: Option<String>,
    pub iccid: Option<String>,
    pub imsi: Option<String>,
    pub meter_type: Option<String>,
    pub meter_serial_number: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BootNotificationResponse {
    pub status: RegistrationStatus,
    pub current_time: String,
    pub interval: u64,
}

// ============================================================================
// Heartbeat
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeartbeatRequest;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HeartbeatResponse {
    pub current_time: String,
}

// ============================================================================
// StatusNotification
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatusNotificationRequest {
    pub connector_id: u64,
    pub status: ChargePointStatus,
    pub error_code: ChargePointErrorCode,
    pub info: Option<String>,
    pub timestamp: Option<String>,
    pub vendor_id: Option<String>,
    pub vendor_error_code: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusNotificationResponse;

// ============================================================================
// Authorize
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthorizeRequest {
    pub id_tag: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthorizeResponse {
    pub id_tag_info: IdTagInfo,
}

// ============================================================================
// StartTransaction
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StartTransactionRequest {
    pub connector_id: u64,
    pub id_tag: String,
    pub meter_start: u64,
    pub timestamp: String,
    pub reservation_id: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StartTransactionResponse {
    pub transaction_id: u64,
    pub id_tag_info: IdTagInfo,
}

// ============================================================================
// StopTransaction
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StopTransactionRequest {
    pub transaction_id: u64,
    pub meter_stop: u64,
    pub timestamp: String,
    pub id_tag: Option<String>,
    pub reason: Option<StopReason>,
    pub transaction_data: Option<Vec<TransactionData>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StopTransactionResponse {
    pub id_tag_info: Option<IdTagInfo>,
}

// ============================================================================
// MeterValues
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MeterValuesRequest {
    pub connector_id: u64,
    pub transaction_id: Option<u64>,
    pub meter_value: Vec<MeterValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeterValuesResponse;

// ============================================================================
// RemoteStartTransaction
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoteStartTransactionRequest {
    pub id_tag: String,
    pub connector_id: Option<u64>,
    pub charging_profile: Option<ChargingProfile>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoteStartTransactionResponse {
    pub status: RemoteStartStopStatus,
}

// ============================================================================
// RemoteStopTransaction
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoteStopTransactionRequest {
    pub transaction_id: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoteStopTransactionResponse {
    pub status: RemoteStartStopStatus,
}

// ============================================================================
// SetChargingProfile
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetChargingProfileRequest {
    pub connector_id: u64,
    pub cs_charging_profiles: ChargingProfile,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetChargingProfileResponse {
    pub status: ChargingProfileStatus,
}

// ============================================================================
// Reset
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResetRequest {
    pub reset_type: ResetType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResetResponse {
    pub status: ResetStatus,
}

// ============================================================================
// ChangeConfiguration
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeConfigurationRequest {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeConfigurationResponse {
    pub status: ConfigurationStatus,
}

// ============================================================================
// GetConfiguration
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetConfigurationRequest {
    pub key: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetConfigurationResponse {
    pub configuration_key: Vec<ConfigurationKey>,
    pub unknown_key: Vec<String>,
}