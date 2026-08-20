//! OCPP 1.6 Server (CSMS side)

use super::*;
use crate::common::*;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{info, error, warn};

/// Message handler type
pub type MessageHandler = Box<dyn Fn(&str, serde_json::Value) -> OcppResult<serde_json::Value> + Send + Sync>;

/// OCPP 1.6 Server
pub struct Ocpp16Server {
    handlers: Arc<Mutex<HashMap<String, MessageHandler>>>,
}

impl Ocpp16Server {
    pub fn new() -> Self {
        Self {
            handlers: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Register a handler for a specific action
    pub fn register_handler<F>(&self, action: &str, handler: F)
    where
        F: Fn(&str, serde_json::Value) -> OcppResult<serde_json::Value> + Send + Sync + 'static,
    {
        let mut handlers = self.handlers.blocking_lock();
        handlers.insert(action.to_string(), Box::new(handler));
    }

    /// Handle an incoming message
    pub async fn handle_message(&self, message: &str) -> OcppResult<Option<String>> {
        let msg: OcppMessage = serde_json::from_str(message)
            .map_err(|e| OcppError::Serialization(e.to_string()))?;

        match msg {
            OcppMessage::Call(call) => {
                let handlers = self.handlers.lock().await;

                if let Some(handler) = handlers.get(&call.action) {
                    match handler(&call.message_id, call.payload) {
                        Ok(response) => {
                            let result = OcppMessage::CallResult(CallResult::new(call.message_id, response));
                            let json = serde_json::to_string(&result)
                                .map_err(|e| OcppError::Serialization(e.to_string()))?;
                            Ok(Some(json))
                        }
                        Err(e) => {
                            let error = OcppMessage::CallError(CallError::new(
                                call.message_id,
                                "InternalError".to_string(),
                                e.to_string(),
                                serde_json::json!({}),
                            ));
                            let json = serde_json::to_string(&error)
                                .map_err(|e| OcppError::Serialization(e.to_string()))?;
                            Ok(Some(json))
                        }
                    }
                } else {
                    let error = OcppMessage::CallError(CallError::new(
                        call.message_id,
                        "NotSupported".to_string(),
                        format!("Action {} not supported", call.action),
                        serde_json::json!({}),
                    ));
                    let json = serde_json::to_string(&error)
                        .map_err(|e| OcppError::Serialization(e.to_string()))?;
                    Ok(Some(json))
                }
            }
            OcppMessage::CallResult(_) | OcppMessage::CallError(_) => {
                // These are responses, not requests
                Ok(None)
            }
        }
    }
}

impl Default for Ocpp16Server {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Default Handlers
// ============================================================================

pub struct DefaultHandlers;

impl DefaultHandlers {
    pub fn boot_notification(&self, _message_id: &str, payload: serde_json::Value) -> OcppResult<serde_json::Value> {
        let request: BootNotificationRequest = serde_json::from_value(payload)
            .map_err(|e| OcppError::Serialization(e.to_string()))?;

        info!("BootNotification from {} ({})",
            request.charge_point_vendor, request.charge_point_model);

        let response = BootNotificationResponse {
            status: RegistrationStatus::Accepted,
            current_time: chrono::Utc::now().to_rfc3339(),
            interval: 60,
        };

        serde_json::to_value(response)
            .map_err(|e| OcppError::Serialization(e.to_string()))
    }

    pub fn heartbeat(&self, _message_id: &str, _payload: serde_json::Value) -> OcppResult<serde_json::Value> {
        let response = HeartbeatResponse {
            current_time: chrono::Utc::now().to_rfc3339(),
        };

        serde_json::to_value(response)
            .map_err(|e| OcppError::Serialization(e.to_string()))
    }

    pub fn status_notification(&self, _message_id: &str, payload: serde_json::Value) -> OcppResult<serde_json::Value> {
        let request: StatusNotificationRequest = serde_json::from_value(payload)
            .map_err(|e| OcppError::Serialization(e.to_string()))?;

        info!("StatusNotification connector {} status {:?}",
            request.connector_id, request.status);

        let response = StatusNotificationResponse;
        serde_json::to_value(response)
            .map_err(|e| OcppError::Serialization(e.to_string()))
    }

    pub fn authorize(&self, _message_id: &str, payload: serde_json::Value) -> OcppResult<serde_json::Value> {
        let request: AuthorizeRequest = serde_json::from_value(payload)
            .map_err(|e| OcppError::Serialization(e.to_string()))?;

        info!("Authorize request for ID tag: {}", request.id_tag);

        let response = AuthorizeResponse {
            id_tag_info: IdTagInfo {
                status: AuthorizationStatus::Accepted,
                expiry_date: None,
                parent_id_tag: None,
            },
        };

        serde_json::to_value(response)
            .map_err(|e| OcppError::Serialization(e.to_string()))
    }

    pub fn start_transaction(&self, _message_id: &str, payload: serde_json::Value) -> OcppResult<serde_json::Value> {
        let request: StartTransactionRequest = serde_json::from_value(payload)
            .map_err(|e| OcppError::Serialization(e.to_string()))?;

        info!("StartTransaction connector {} tag {}",
            request.connector_id, request.id_tag);

        let response = StartTransactionResponse {
            transaction_id: rand::random::<u64>(),
            id_tag_info: IdTagInfo {
                status: AuthorizationStatus::Accepted,
                expiry_date: None,
                parent_id_tag: None,
            },
        };

        serde_json::to_value(response)
            .map_err(|e| OcppError::Serialization(e.to_string()))
    }

    pub fn stop_transaction(&self, _message_id: &str, payload: serde_json::Value) -> OcppResult<serde_json::Value> {
        let request: StopTransactionRequest = serde_json::from_value(payload)
            .map_err(|e| OcppError::Serialization(e.to_string()))?;

        info!("StopTransaction {} meter: {}",
            request.transaction_id, request.meter_stop);

        let response = StopTransactionResponse { id_tag_info: None };

        serde_json::to_value(response)
            .map_err(|e| OcppError::Serialization(e.to_string()))
    }

    pub fn meter_values(&self, _message_id: &str, payload: serde_json::Value) -> OcppResult<serde_json::Value> {
        let request: MeterValuesRequest = serde_json::from_value(payload)
            .map_err(|e| OcppError::Serialization(e.to_string()))?;

        debug!("MeterValues connector {} {} readings",
            request.connector_id, request.meter_value.len());

        let response = MeterValuesResponse;
        serde_json::to_value(response)
            .map_err(|e| OcppError::Serialization(e.to_string()))
    }
}