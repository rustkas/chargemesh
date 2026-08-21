//! ChargeMesh Observability Platform
//!
//! Complete observability solution for EV charging infrastructure:
//! metrics, logs, events, traces, and correlations.

#![warn(missing_docs)]
#![warn(rust_2018_idioms)]

pub mod platform;
pub mod metrics;
pub mod logging;
pub mod tracing;
pub mod events;
pub mod correlations;
pub mod dashboard;

pub use platform::*;
pub use metrics::*;
pub use logging::*;
pub use tracing::*;
pub use events::*;
pub use correlations::*;
pub use dashboard::*;

use serde::{Deserialize, Serialize};

/// Observability result type
pub type Result<T> = std::result::Result<T, ObservabilityError>;

#[derive(Debug, thiserror::Error)]
pub enum ObservabilityError {
    #[error("Metric error: {0}")]
    Metric(String),

    #[error("Logging error: {0}")]
    Logging(String),

    #[error("Tracing error: {0}")]
    Tracing(String),

    #[error("Event bus error: {0}")]
    EventBus(String),

    #[error("Correlation error: {0}")]
    Correlation(String),

    #[error("Dashboard error: {0}")]
    Dashboard(String),

    #[error("Internal error: {0}")]
    Internal(String),
}

/// Observability level
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum ObservabilityLevel {
    Debug,
    Info,
    Warning,
    Error,
    Critical,
}

impl std::fmt::Display for ObservabilityLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ObservabilityLevel::Debug => write!(f, "🔍 DEBUG"),
            ObservabilityLevel::Info => write!(f, "ℹ️ INFO"),
            ObservabilityLevel::Warning => write!(f, "⚠️ WARNING"),
            ObservabilityLevel::Error => write!(f, "❌ ERROR"),
            ObservabilityLevel::Critical => write!(f, "🚨 CRITICAL"),
        }
    }
}