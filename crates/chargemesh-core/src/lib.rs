//! ChargeMesh Core — Foundation types and utilities

pub mod types;
pub mod error;
pub mod config;
pub mod crypto;
pub mod time;
pub mod ident;

pub use types::*;
pub use error::*;
pub use config::*;
pub use crypto::*;
pub use time::*;
pub use ident::*;

use serde::{Deserialize, Serialize};

/// Core result type
pub type Result<T> = std::result::Result<T, CoreError>;

/// Core error types
#[derive(Debug, thiserror::Error)]
pub enum CoreError {
    #[error("Invalid identifier: {0}")]
    InvalidId(String),
    
    #[error("Invalid timestamp: {0}")]
    InvalidTimestamp(String),
    
    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),
    
    #[error("Crypto error: {0}")]
    Crypto(String),
    
    #[error("Internal error: {0}")]
    Internal(String),
}

/// Version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
