//! ChargeMesh Diagnostics Engine
//!
//! Intelligent diagnostics system for EV charging infrastructure.
//! Analyzes protocol/event timelines, identifies root causes,
//! and generates actionable recommendations.

#![warn(missing_docs)]
#![warn(rust_2018_idioms)]

pub mod engine;
pub mod timeline;
pub mod analyzers;
pub mod root_cause;
pub mod report;
pub mod ml;

pub use engine::*;
pub use timeline::*;
pub use analyzers::*;
pub use root_cause::*;
pub use report::*;
pub use ml::*;

use serde::{Deserialize, Serialize};

/// Diagnostic result type
pub type Result<T> = std::result::Result<T, DiagnosticError>;

#[derive(Debug, thiserror::Error)]
pub enum DiagnosticError {
    #[error("No timeline data available")]
    NoData,

    #[error("Invalid timeline entry: {0}")]
    InvalidEntry(String),

    #[error("Analysis failed: {0}")]
    AnalysisFailed(String),

    #[error("Root cause not found: {0}")]
    RootCauseNotFound(String),

    #[error("Pattern not recognized: {0}")]
    PatternNotRecognized(String),

    #[error("ML inference error: {0}")]
    MLInference(String),

    #[error("Storage error: {0}")]
    Storage(String),

    #[error("Internal error: {0}")]
    Internal(String),
}

/// Diagnostic severity
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum DiagnosticSeverity {
    Info,
    Warning,
    Error,
    Critical,
}

/// Diagnostic status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum DiagnosticStatus {
    Pending,
    Analyzing,
    Complete,
    Failed,
}

impl std::fmt::Display for DiagnosticSeverity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DiagnosticSeverity::Info => write!(f, "ℹ️ INFO"),
            DiagnosticSeverity::Warning => write!(f, "⚠️ WARNING"),
            DiagnosticSeverity::Error => write!(f, "❌ ERROR"),
            DiagnosticSeverity::Critical => write!(f, "🚨 CRITICAL"),
        }
    }
}