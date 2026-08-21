//! Analyzer trait and base types

use super::*;
use async_trait::async_trait;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisResult {
    pub findings: Vec<Finding>,
    pub summary: String,
    pub severity: DiagnosticSeverity,
    pub metadata: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Finding {
    pub id: String,
    pub title: String,
    pub description: String,
    pub severity: DiagnosticSeverity,
    pub component: Component,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub evidence: Vec<Evidence>,
    pub recommendations: Vec<Recommendation>,
    pub tags: Vec<String>,
    pub confidence: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Evidence {
    pub entry_id: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub description: String,
    pub data: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recommendation {
    pub action: String,
    pub description: String,
    pub priority: RecommendationPriority,
    pub estimated_time: Option<chrono::Duration>,
    pub steps: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum RecommendationPriority {
    Immediate,
    High,
    Medium,
    Low,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosticContext {
    pub station_id: Option<String>,
    pub session_id: Option<String>,
    pub time_range: Option<(chrono::DateTime<chrono::Utc>, chrono::DateTime<chrono::Utc>)>,
    pub protocol: Option<String>,
    pub vendor: Option<String>,
    pub model: Option<String>,
    pub firmware_version: Option<String>,
}

#[async_trait]
pub trait Analyzer: Send + Sync {
    async fn analyze(
        &self,
        timeline: &[TimelineEntry],
        context: &DiagnosticContext,
    ) -> Result<AnalysisResult>;

    fn name(&self) -> &str;
    fn description(&self) -> &str;
}