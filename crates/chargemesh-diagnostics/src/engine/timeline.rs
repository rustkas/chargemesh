//! Timeline management for diagnostics

use super::*;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct TimelineCollector {
    entries: Arc<RwLock<Vec<TimelineEntry>>>,
    status: Arc<RwLock<DiagnosticStatus>>,
    index: Arc<RwLock<HashMap<String, Vec<usize>>>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimelineEntry {
    pub id: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub event_type: EventType,
    pub component: Component,
    pub status: EntryStatus,
    pub details: serde_json::Value,
    pub session_id: Option<String>,
    pub station_id: Option<String>,
    pub connector_id: Option<u8>,
    pub transaction_id: Option<String>,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum EventType {
    // Protocol events
    BootNotification,
    Heartbeat,
    StatusNotification,
    Authorize,
    StartTransaction,
    StopTransaction,
    MeterValues,
    RemoteStart,
    RemoteStop,
    SetChargingProfile,
    Reset,
    ChangeConfiguration,
    GetConfiguration,

    // ISO 15118 events
    ISO15118Handshake,
    ISO15118CertificateValidation,
    ISO15118PlugAndCharge,
    ISO15118V2G,

    // System events
    ConnectionEstablished,
    ConnectionLost,
    Timeout,
    Error,
    Warning,
    Info,

    // Diagnostic events
    DiagnosticStart,
    DiagnosticComplete,
    FaultDetected,
    FaultCleared,
    RootCauseIdentified,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Component {
    EVSE,
    EV,
    CSMS,
    OCPI,
    Grid,
    Certificate,
    Network,
    Protocol,
    Security,
    Metering,
    Power,
    System,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum EntryStatus {
    Success,
    Failure,
    Pending,
    Timeout,
    Warning,
    Information,
}

impl TimelineCollector {
    pub fn new() -> Self {
        Self {
            entries: Arc::new(RwLock::new(Vec::new())),
            status: Arc::new(RwLock::new(DiagnosticStatus::Pending)),
            index: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn add_entry(&self, entry: TimelineEntry) -> Result<()> {
        let mut entries = self.entries.write().await;
        let idx = entries.len();
        entries.push(entry.clone());

        let mut index = self.index.write().await;
        if let Some(session_id) = &entry.session_id {
            index.entry(session_id.clone())
                .or_insert_with(Vec::new)
                .push(idx);
        }
        if let Some(station_id) = &entry.station_id {
            index.entry(station_id.clone())
                .or_insert_with(Vec::new)
                .push(idx);
        }

        if entry.status == EntryStatus::Failure || entry.status == EntryStatus::Timeout {
            *self.status.write().await = DiagnosticStatus::Analyzing;
        }

        Ok(())
    }

    pub async fn get_timeline(&self) -> Vec<TimelineEntry> {
        self.entries.read().await.clone()
    }

    pub async fn get_entries_by_session(&self, session_id: &str) -> Vec<TimelineEntry> {
        let entries = self.entries.read().await;
        let index = self.index.read().await;

        if let Some(indices) = index.get(session_id) {
            indices.iter().filter_map(|&i| entries.get(i).cloned()).collect()
        } else {
            Vec::new()
        }
    }

    pub async fn get_entries_by_station(&self, station_id: &str) -> Vec<TimelineEntry> {
        let entries = self.entries.read().await;
        let index = self.index.read().await;

        if let Some(indices) = index.get(station_id) {
            indices.iter().filter_map(|&i| entries.get(i).cloned()).collect()
        } else {
            Vec::new()
        }
    }

    pub async fn get_entries_by_time_range(
        &self,
        start: chrono::DateTime<chrono::Utc>,
        end: chrono::DateTime<chrono::Utc>,
    ) -> Vec<TimelineEntry> {
        let entries = self.entries.read().await;
        entries.iter()
            .filter(|e| e.timestamp >= start && e.timestamp <= end)
            .cloned()
            .collect()
    }

    pub async fn get_failed_entries(&self) -> Vec<TimelineEntry> {
        let entries = self.entries.read().await;
        entries.iter()
            .filter(|e| e.status == EntryStatus::Failure || e.status == EntryStatus::Timeout)
            .cloned()
            .collect()
    }

    pub async fn get_status(&self) -> DiagnosticStatus {
        *self.status.read().await
    }

    pub async fn clear(&self) -> Result<()> {
        let mut entries = self.entries.write().await;
        entries.clear();

        let mut index = self.index.write().await;
        index.clear();

        *self.status.write().await = DiagnosticStatus::Pending;
        Ok(())
    }
}

impl Default for TimelineCollector {
    fn default() -> Self {
        Self::new()
    }
}