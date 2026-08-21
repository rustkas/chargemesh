//! Correlation tracing — связывание device → protocol → session → error → root cause

mod tracer;
mod graph;

pub use tracer::*;
pub use graph::*;

use super::*;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Correlation {
    pub id: String,
    pub source_type: CorrelationSource,
    pub target_type: CorrelationTarget,
    pub source_id: String,
    pub target_id: String,
    pub relationship: RelationshipType,
    pub confidence: f64,
    pub evidence: Vec<String>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum CorrelationSource {
    Device,
    Session,
    Transaction,
    Error,
    RootCause,
    Protocol,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum CorrelationTarget {
    Device,
    Session,
    Transaction,
    Error,
    RootCause,
    Protocol,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum RelationshipType {
    BelongsTo,
    CausedBy,
    RelatedTo,
    PrecededBy,
    FollowedBy,
    Contains,
    PartOf,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorrelationSummary {
    pub device_to_session: Vec<String>,
    pub session_to_error: Vec<String>,
    pub error_to_root_cause: Vec<String>,
    pub total_correlations: usize,
}