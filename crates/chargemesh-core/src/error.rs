//! Core error types

use serde::{Deserialize, Serialize};
use std::fmt;
use thiserror::Error;

/// Core error type for the ChargeMesh platform
#[derive(Debug, Error, Clone, Serialize, Deserialize)]
pub enum CoreError {
    /// Invalid identifier
    #[error("Invalid identifier: {0}")]
    InvalidId(String),

    /// Invalid timestamp
    #[error("Invalid timestamp: {0}")]
    InvalidTimestamp(String),

    /// Invalid configuration
    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),

    /// Crypto error
    #[error("Crypto error: {0}")]
    Crypto(String),

    /// Value out of range
    #[error("Value out of range: {0}")]
    OutOfRange(String),

    /// Serialization error
    #[error("Serialization error: {0}")]
    Serialization(String),

    /// Deserialization error
    #[error("Deserialization error: {0}")]
    Deserialization(String),

    /// Not found
    #[error("Not found: {0}")]
    NotFound(String),

    /// Internal error
    #[error("Internal error: {0}")]
    Internal(String),
}

/// Core result type alias
pub type CoreResult<T> = std::result::Result<T, CoreError>;

impl From<serde_json::Error> for CoreError {
    fn from(err: serde_json::Error) -> Self {
        CoreError::Serialization(err.to_string())
    }
}

impl From<chrono::ParseError> for CoreError {
    fn from(err: chrono::ParseError) -> Self {
        CoreError::InvalidTimestamp(err.to_string())
    }
}

impl From<std::num::ParseIntError> for CoreError {
    fn from(err: std::num::ParseIntError) -> Self {
        CoreError::InvalidId(err.to_string())
    }
}