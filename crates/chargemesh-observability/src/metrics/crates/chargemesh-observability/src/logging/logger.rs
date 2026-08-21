//! Structured logging

use super::*;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    pub id: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub level: ObservabilityLevel,
    pub message: String,
    pub component: String,
    pub fields: HashMap<String, serde_json::Value>,
    pub session_id: Option<String>,
    pub station_id: Option<String>,
    pub transaction_id: Option<String>,
    pub trace_id: Option<String>,
    pub span_id: Option<String>,
}

pub struct StructuredLogger {
    entries: Arc<tokio::sync::RwLock<Vec<LogEntry>>>,
    max_entries: usize,
}

impl StructuredLogger {
    pub fn new() -> Self {
        Self {
            entries: Arc::new(tokio::sync::RwLock::new(Vec::new())),
            max_entries: 100000,
        }
    }

    pub async fn start(&self) -> Result<()> {
        Ok(())
    }

    pub async fn stop(&self) -> Result<()> {
        Ok(())
    }

    pub async fn log(
        &self,
        level: ObservabilityLevel,
        message: &str,
        component: &str,
        fields: HashMap<String, serde_json::Value>,
    ) -> Result<()> {
        let entry = LogEntry {
            id: uuid::Uuid::new_v4().to_string(),
            timestamp: chrono::Utc::now(),
            level,
            message: message.to_string(),
            component: component.to_string(),
            fields,
            session_id: None,
            station_id: None,
            transaction_id: None,
            trace_id: None,
            span_id: None,
        };

        let mut entries = self.entries.write().await;
        entries.push(entry);

        if entries.len() > self.max_entries {
            let _ = entries.drain(0..(entries.len() - self.max_entries));
        }

        Ok(())
    }

    pub async fn collect(&self) -> Vec<LogEntry> {
        self.entries.read().await.clone()
    }

    pub async fn get_errors(&self) -> Vec<LogEntry> {
        let entries = self.entries.read().await;
        entries.iter()
            .filter(|e| e.level >= ObservabilityLevel::Error)
            .cloned()
            .collect()
    }
}

impl Default for StructuredLogger {
    fn default() -> Self {
        Self::new()
    }
}