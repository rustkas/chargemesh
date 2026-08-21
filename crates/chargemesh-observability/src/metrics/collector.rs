//! Metrics collector

use super::*;
use std::collections::HashMap;

pub struct MetricsCollector {
    registry: Arc<MetricRegistry>,
}

impl MetricsCollector {
    pub fn new(registry: Arc<MetricRegistry>) -> Self {
        Self { registry }
    }

    pub async fn collect_station_metrics(&self, station_id: &str, status: &str) -> Result<()> {
        let mut labels = HashMap::new();
        labels.insert("station_id".to_string(), station_id.to_string());
        labels.insert("status".to_string(), status.to_string());

        self.registry.increment_counter("station_status_changes", labels).await?;
        Ok(())
    }

    pub async fn collect_session_metrics(
        &self,
        session_id: &str,
        duration_seconds: f64,
        energy_kwh: f64,
    ) -> Result<()> {
        let mut labels = HashMap::new();
        labels.insert("session_id".to_string(), session_id.to_string());

        self.registry.record_metric(
            "session_duration_seconds",
            duration_seconds,
            MetricType::Histogram,
            labels.clone(),
        ).await?;

        self.registry.record_metric(
            "session_energy_kwh",
            energy_kwh,
            MetricType::Counter,
            labels,
        ).await?;

        Ok(())
    }

    pub async fn collect_protocol_metrics(
        &self,
        protocol: &str,
        message_type: &str,
        success: bool,
    ) -> Result<()> {
        let mut labels = HashMap::new();
        labels.insert("protocol".to_string(), protocol.to_string());
        labels.insert("message_type".to_string(), message_type.to_string());
        labels.insert("success".to_string(), success.to_string());

        self.registry.increment_counter("protocol_messages", labels).await?;
        Ok(())
    }

    pub async fn collect_error_metrics(
        &self,
        error_type: &str,
        severity: &str,
        component: &str,
    ) -> Result<()> {
        let mut labels = HashMap::new();
        labels.insert("error_type".to_string(), error_type.to_string());
        labels.insert("severity".to_string(), severity.to_string());
        labels.insert("component".to_string(), component.to_string());

        self.registry.increment_counter("errors_total", labels).await?;
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMetrics {
    pub uptime_seconds: u64,
    pub memory_usage_bytes: u64,
    pub cpu_usage_percent: f64,
    pub active_sessions: u32,
    pub total_stations: u32,
    pub online_stations: u32,
    pub charging_stations: u32,
    pub error_count: u32,
    pub protocol_errors: u32,
    pub iso15118_errors: u32,
    pub network_errors: u32,
    pub hardware_errors: u32,
}