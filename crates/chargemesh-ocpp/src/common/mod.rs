//! OCPP common types and utilities

pub mod messages;
pub mod types;
pub mod errors;
pub mod websocket;

pub use messages::*;
pub use types::*;
pub use errors::*;
pub use websocket::*;

use serde::{Deserialize, Serialize};

/// OCPP Message wrapper
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OcppMessage {
    Call(Call),
    CallResult(CallResult),
    CallError(CallError),
}

/// OCPP Call (request)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Call {
    #[serde(rename = "messageId")]
    pub message_id: String,
    pub action: String,
    pub payload: serde_json::Value,
}

/// OCPP CallResult (response)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallResult {
    #[serde(rename = "messageId")]
    pub message_id: String,
    pub payload: serde_json::Value,
}

/// OCPP CallError
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallError {
    #[serde(rename = "messageId")]
    pub message_id: String,
    pub error_code: String,
    pub error_description: String,
    pub details: serde_json::Value,
}
