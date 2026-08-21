//! Observability platform core

mod observability;
mod config;
mod context;

pub use observability::*;
pub use config::*;
pub use context::*;

use super::*;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Main observability platform
pub struct ObservabilityPlatform {
    config: PlatformConfig,
    metrics: Arc<MetricRegistry>,
    logger: Arc<StructuredLogger>,
    tracer: Arc<Tracer>,
    event_bus: Arc<EventBus>,
    correlations: Arc<CorrelationTracer>,
    dashboard: Arc<DashboardRenderer>,
    status: Arc<RwLock<PlatformStatus>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlatformConfig {
    pub metrics_enabled: bool,
    pub logging_enabled: bool,
    pub tracing_enabled: bool,
    pub events_enabled: bool,
    pub dashboard_enabled: bool,
    pub correlations_enabled: bool,
    pub retention_days: u32,
    pub max_metrics: usize,
    pub max_logs: usize,
    pub max_traces: usize,
}

impl Default for PlatformConfig {
    fn default() -> Self {
        Self {
            metrics_enabled: true,
            logging_enabled: true,
            tracing_enabled: true,
            events_enabled: true,
            dashboard_enabled: true,
            correlations_enabled: true,
            retention_days: 30,
            max_metrics: 10000,
            max_logs: 100000,
            max_traces: 10000,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum PlatformStatus {
    Stopped,
    Starting,
    Running,
    Stopping,
    Failed,
}

impl ObservabilityPlatform {
    pub fn new(config: PlatformConfig) -> Self {
        Self {
            config: config.clone(),
            metrics: Arc::new(MetricRegistry::new()),
            logger: Arc::new(StructuredLogger::new()),
            tracer: Arc::new(Tracer::new()),
            event_bus: Arc::new(EventBus::new()),
            correlations: Arc::new(CorrelationTracer::new()),
            dashboard: Arc::new(DashboardRenderer::new()),
            status: Arc::new(RwLock::new(PlatformStatus::Stopped)),
        }
    }

    pub async fn start(&self) -> Result<()> {
        *self.status.write().await = PlatformStatus::Starting;

        if self.config.metrics_enabled {
            self.metrics.start().await?;
        }
        if self.config.logging_enabled {
            self.logger.start().await?;
        }
        if self.config.tracing_enabled {
            self.tracer.start().await?;
        }
        if self.config.events_enabled {
            self.event_bus.start().await?;
        }
        if self.config.dashboard_enabled {
            self.dashboard.start().await?;
        }
        if self.config.correlations_enabled {
            self.correlations.start().await?;
        }

        *self.status.write().await = PlatformStatus::Running;
        Ok(())
    }

    pub async fn stop(&self) -> Result<()> {
        *self.status.write().await = PlatformStatus::Stopping;

        self.metrics.stop().await?;
        self.logger.stop().await?;
        self.tracer.stop().await?;
        self.event_bus.stop().await?;
        self.dashboard.stop().await?;
        self.correlations.stop().await?;

        *self.status.write().await = PlatformStatus::Stopped;
        Ok(())
    }

    pub async fn get_status(&self) -> PlatformStatus {
        *self.status.read().await
    }

    pub async fn get_dashboard_data(&self) -> DashboardData {
        let metrics = self.metrics.collect().await;
        let logs = self.logger.collect().await;
        let traces = self.tracer.collect().await;
        let events = self.event_bus.collect().await;

        DashboardData {
            metrics,
            logs,
            traces,
            events,
            correlations: self.correlations.get_summary().await,
            timestamp: chrono::Utc::now(),
        }
    }
}

impl Default for ObservabilityPlatform {
    fn default() -> Self {
        Self::new(PlatformConfig::default())
    }
}