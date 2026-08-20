//! OCPP 1.6 Client (Charge Point side)

use super::*;
use crate::common::*;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{info, error, debug};

/// OCPP 1.6 Client
pub struct Ocpp16Client {
    ws: Arc<WebSocketConnection>,
    message_id_counter: Arc<Mutex<u64>>,
    charge_point_id: Option<String>,
}

impl Ocpp16Client {
    /// Create a new OCPP 1.6 client and connect to CSMS
    pub async fn connect(url: &str) -> OcppResult<Self> {
        info!("Connecting to OCPP 1.6 CSMS at {}", url);
        let ws = Arc::new(WebSocketConnection::connect(url).await?);

        Ok(Self {
            ws,
            message_id_counter: Arc::new(Mutex::new(0)),
            charge_point_id: None,
        })
    }

    fn next_message_id(&self) -> String {
        let mut counter = self.message_id_counter.blocking_lock();
        *counter += 1;
        format!("{}", *counter)
    }

    async fn send_request(&self, action: &str, payload: serde_json::Value) -> OcppResult<serde_json::Value> {
        let message_id = self.next_message_id();
        let call = OcppMessage::Call(Call::new(message_id.clone(), action.to_string(), payload));

        let json = serde_json::to_string(&call)
            .map_err(|e| OcppError::Serialization(e.to_string()))?;

        debug!("Sending {}: {}", action, json);
        self.ws.send(Message::text(json)).await?;

        // Wait for response
        let response = self.ws.receive().await?;
        let text = response.to_text()
            .map_err(|e| OcppError::Protocol(e.to_string()))?;

        debug!("Received response: {}", text);

        let message: OcppMessage = serde_json::from_str(text)
            .map_err(|e| OcppError::Serialization(e.to_string()))?;

        match message {
            OcppMessage::CallResult(result) => {
                if result.message_id == message_id {
                    Ok(result.payload)
                } else {
                    Err(OcppError::Protocol("Message ID mismatch".to_string()))
                }
            }
            OcppMessage::CallError(error) => {
                Err(OcppError::Protocol(format!(
                    "OCPP Error: {} - {}",
                    error.error_code, error.error_description
                )))
            }
            _ => Err(OcppError::Protocol("Unexpected message type".to_string())),
        }
    }

    // ============================================================================
    // Core OCPP 1.6 Messages
    // ============================================================================

    /// Send BootNotification
    pub async fn boot_notification(
        &self,
        vendor: &str,
        model: &str,
    ) -> OcppResult<BootNotificationResponse> {
        let request = BootNotificationRequest {
            charge_point_vendor: vendor.to_string(),
            charge_point_model: model.to_string(),
            charge_point_serial_number: None,
            firmware_version: None,
            iccid: None,
            imsi: None,
            meter_type: None,
            meter_serial_number: None,
        };

        let payload = serde_json::to_value(request)
            .map_err(|e| OcppError::Serialization(e.to_string()))?;

        let response = self.send_request("BootNotification", payload).await?;

        let response: BootNotificationResponse = serde_json::from_value(response)
            .map_err(|e| OcppError::Serialization(e.to_string()))?;

        Ok(response)
    }

    /// Send Heartbeat
    pub async fn heartbeat(&self) -> OcppResult<HeartbeatResponse> {
        let payload = serde_json::json!({});
        let response = self.send_request("Heartbeat", payload).await?;

        let response: HeartbeatResponse = serde_json::from_value(response)
            .map_err(|e| OcppError::Serialization(e.to_string()))?;

        Ok(response)
    }

    /// Send StatusNotification
    pub async fn status_notification(
        &self,
        connector_id: u64,
        status: ChargePointStatus,
        error_code: ChargePointErrorCode,
    ) -> OcppResult<StatusNotificationResponse> {
        let request = StatusNotificationRequest {
            connector_id,
            status,
            error_code,
            info: None,
            timestamp: None,
            vendor_id: None,
            vendor_error_code: None,
        };

        let payload = serde_json::to_value(request)
            .map_err(|e| OcppError::Serialization(e.to_string()))?;

        let response = self.send_request("StatusNotification", payload).await?;

        let response: StatusNotificationResponse = serde_json::from_value(response)
            .map_err(|e| OcppError::Serialization(e.to_string()))?;

        Ok(response)
    }

    /// Send Authorize
    pub async fn authorize(&self, id_tag: &str) -> OcppResult<AuthorizeResponse> {
        let request = AuthorizeRequest {
            id_tag: id_tag.to_string(),
        };

        let payload = serde_json::to_value(request)
            .map_err(|e| OcppError::Serialization(e.to_string()))?;

        let response = self.send_request("Authorize", payload).await?;

        let response: AuthorizeResponse = serde_json::from_value(response)
            .map_err(|e| OcppError::Serialization(e.to_string()))?;

        Ok(response)
    }

    /// Send StartTransaction
    pub async fn start_transaction(
        &self,
        connector_id: u64,
        id_tag: &str,
        meter_start: u64,
    ) -> OcppResult<StartTransactionResponse> {
        let request = StartTransactionRequest {
            connector_id,
            id_tag: id_tag.to_string(),
            meter_start,
            timestamp: chrono::Utc::now().to_rfc3339(),
            reservation_id: None,
        };

        let payload = serde_json::to_value(request)
            .map_err(|e| OcppError::Serialization(e.to_string()))?;

        let response = self.send_request("StartTransaction", payload).await?;

        let response: StartTransactionResponse = serde_json::from_value(response)
            .map_err(|e| OcppError::Serialization(e.to_string()))?;

        Ok(response)
    }

    /// Send StopTransaction
    pub async fn stop_transaction(
        &self,
        transaction_id: u64,
        meter_stop: u64,
        id_tag: Option<String>,
        reason: Option<StopReason>,
    ) -> OcppResult<StopTransactionResponse> {
        let request = StopTransactionRequest {
            transaction_id,
            meter_stop,
            timestamp: chrono::Utc::now().to_rfc3339(),
            id_tag,
            reason,
            transaction_data: None,
        };

        let payload = serde_json::to_value(request)
            .map_err(|e| OcppError::Serialization(e.to_string()))?;

        let response = self.send_request("StopTransaction", payload).await?;

        let response: StopTransactionResponse = serde_json::from_value(response)
            .map_err(|e| OcppError::Serialization(e.to_string()))?;

        Ok(response)
    }

    /// Send MeterValues
    pub async fn meter_values(
        &self,
        connector_id: u64,
        transaction_id: Option<u64>,
        values: Vec<MeterValue>,
    ) -> OcppResult<MeterValuesResponse> {
        let request = MeterValuesRequest {
            connector_id,
            transaction_id,
            meter_value: values,
        };

        let payload = serde_json::to_value(request)
            .map_err(|e| OcppError::Serialization(e.to_string()))?;

        let response = self.send_request("MeterValues", payload).await?;

        let response: MeterValuesResponse = serde_json::from_value(response)
            .map_err(|e| OcppError::Serialization(e.to_string()))?;

        Ok(response)
    }

    // ============================================================================
    // Remote Commands (CSMS initiated)
    // ============================================================================

    /// Handle RemoteStartTransaction (CSMS -> CP)
    pub async fn handle_remote_start(
        &self,
        request: RemoteStartTransactionRequest,
    ) -> OcppResult<RemoteStartTransactionResponse> {
        // In a real implementation, this would start the charging process
        // For now, just accept
        Ok(RemoteStartTransactionResponse {
            status: RemoteStartStopStatus::Accepted,
        })
    }

    /// Handle RemoteStopTransaction (CSMS -> CP)
    pub async fn handle_remote_stop(
        &self,
        request: RemoteStopTransactionRequest,
    ) -> OcppResult<RemoteStopTransactionResponse> {
        // In a real implementation, this would stop the charging process
        // For now, just accept
        Ok(RemoteStopTransactionResponse {
            status: RemoteStartStopStatus::Accepted,
        })
    }

    // ============================================================================
    // Configuration
    // ============================================================================

    /// Change configuration
    pub async fn change_configuration(
        &self,
        key: &str,
        value: &str,
    ) -> OcppResult<ChangeConfigurationResponse> {
        let request = ChangeConfigurationRequest {
            key: key.to_string(),
            value: value.to_string(),
        };

        let payload = serde_json::to_value(request)
            .map_err(|e| OcppError::Serialization(e.to_string()))?;

        let response = self.send_request("ChangeConfiguration", payload).await?;

        let response: ChangeConfigurationResponse = serde_json::from_value(response)
            .map_err(|e| OcppError::Serialization(e.to_string()))?;

        Ok(response)
    }

    /// Get configuration
    pub async fn get_configuration(
        &self,
        keys: Option<Vec<String>>,
    ) -> OcppResult<GetConfigurationResponse> {
        let request = GetConfigurationRequest { key: keys };

        let payload = serde_json::to_value(request)
            .map_err(|e| OcppError::Serialization(e.to_string()))?;

        let response = self.send_request("GetConfiguration", payload).await?;

        let response: GetConfigurationResponse = serde_json::from_value(response)
            .map_err(|e| OcppError::Serialization(e.to_string()))?;

        Ok(response)
    }

    // ============================================================================
    // Reset
    // ============================================================================

    /// Reset the charge point
    pub async fn reset(&self, reset_type: ResetType) -> OcppResult<ResetResponse> {
        let request = ResetRequest { reset_type };

        let payload = serde_json::to_value(request)
            .map_err(|e| OcppError::Serialization(e.to_string()))?;

        let response = self.send_request("Reset", payload).await?;

        let response: ResetResponse = serde_json::from_value(response)
            .map_err(|e| OcppError::Serialization(e.to_string()))?;

        Ok(response)
    }

    /// Close the connection
    pub async fn close(&self) -> OcppResult<()> {
        self.ws.close().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_client_serialization() {
        // This is a compile-time test for the client methods
        // Actual WebSocket tests would require a mock server
        let url = "ws://localhost:9000";
        let client = Ocpp16Client::connect(url).await;
        // Should fail because no server is running
        assert!(client.is_err());
    }
}