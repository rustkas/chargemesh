//! ChargeMesh Core — Foundation types and utilities
//!
//! This crate provides the fundamental building blocks for the entire
//! ChargeMesh platform: identifiers, types, errors, and utilities.

#![warn(missing_docs)]
#![warn(rust_2018_idioms)]

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

/// Core result type for all ChargeMesh operations
pub type Result<T> = std::result::Result<T, CoreError>;

/// Version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");