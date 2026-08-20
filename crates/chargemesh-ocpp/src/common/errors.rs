//! OCPP Error handling

use thiserror::Error;

#[derive(Debug, Error)]
pub enum OcppError {
    #[error("WebSocket error: {0}")]
    WebSocket(String),

    #[error("Serialization error: {0}")]
    Serialization(String),

    #[error("Protocol error: {0}")]
    Protocol(String),

    #[error("Timeout: {0}")]
    Timeout(String),

    #[error("Invalid message: {0}")]
    InvalidMessage(String),

    #[error("Station not found: {0}")]
    StationNotFound(String),

    #[error("Connector not available: {0}")]
    ConnectorNotAvailable(String),

    #[error("Transaction not found: {0}")]
    TransactionNotFound(String),

    #[error("Authorization failed: {0}")]
    AuthorizationFailed(String),

    #[error("Internal error: {0}")]
    Internal(String),
}

pub type OcppResult<T> = std::result::Result<T, OcppError>;