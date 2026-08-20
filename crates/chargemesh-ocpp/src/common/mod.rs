//! Common types and utilities for OCPP

pub mod messages;
pub mod types;
pub mod errors;
pub mod websocket;

pub use messages::*;
pub use types::*;
pub use errors::*;
pub use websocket::*;

use serde::{Deserialize, Serialize};
use std::fmt;

/// OCPP Message wrapper
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OcppMessage {
    Call(Call),
    CallResult(CallResult),
    CallError(CallError),
}

impl fmt::Display for OcppMessage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OcppMessage::Call(call) => write!(f, "Call({})", call.action),
            OcppMessage::CallResult(result) => write!(f, "CallResult({})", result.message_id),
            OcppMessage::CallError(error) => write!(f, "CallError({})", error.error_code),
        }
    }
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

impl Call {
    pub fn new(message_id: String, action: String, payload: serde_json::Value) -> Self {
        Self {
            message_id,
            action,
            payload,
        }
    }
}

impl CallResult {
    pub fn new(message_id: String, payload: serde_json::Value) -> Self {
        Self { message_id, payload }
    }
}

impl CallError {
    pub fn new(
        message_id: String,
        error_code: String,
        error_description: String,
        details: serde_json::Value,
    ) -> Self {
        Self {
            message_id,
            error_code,
            error_description,
            details,
        }
    }
}