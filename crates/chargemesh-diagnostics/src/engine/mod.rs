//! Core diagnostics engine

mod diagnostics_engine;
mod timeline;
mod analyzer;
mod root_cause;

pub use diagnostics_engine::*;
pub use timeline::*;
pub use analyzer::*;
pub use root_cause::*;

use super::*;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Main diagnostics engine
pub struct DiagnosticsEngine {
    timeline_collector: Arc<TimelineCollector>,
    analyzers: Vec<Box<dyn Analyzer>>,
    root_cause_analyzer: Arc<RootCauseAnalyzer>,
    report_generator: Arc<ReportGenerator>,
    config: EngineConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EngineConfig {
    pub max_timeline_entries: usize,
    pub analysis_interval: chrono::Duration,
    pub enable_ml: bool,
    pub enable_root_cause: bool,
    pub auto_report: bool,
    pub retention_days: u32,
}

impl Default for EngineConfig {
    fn default() -> Self {
        Self {
            max_timeline_entries: 10000,
            analysis_interval: chrono::Duration::seconds(60),
            enable_ml: true,
            enable_root_cause: true,
            auto_report: true,
            retention_days: 30,
        }
    }
}

impl DiagnosticsEngine {
    pub fn new(config: EngineConfig) -> Self {
        let mut engine = Self {
            timeline_collector: Arc::new(TimelineCollector::new()),
            analyzers: Vec::new(),
            root_cause_analyzer: Arc::new(RootCauseAnalyzer::new()),
            report_generator: Arc::new(ReportGenerator::new()),
            config,
        };

        engine.register_default_analyzers();
        engine
    }

    fn register_default_analyzers(&mut self) {
        self.analyzers.push(Box::new(analyzers::PatternAnalyzer::new()));
        self.analyzers.push(Box::new(analyzers::SequenceAnalyzer::new()));
        self.analyzers.push(Box::new(analyzers::PerformanceAnalyzer::new()));
        self.analyzers.push(Box::new(analyzers::SecurityAnalyzer::new()));
    }

    /// Add a timeline entry
    pub async fn add_entry(&self, entry: TimelineEntry) -> Result<()> {
        self.timeline_collector.add_entry(entry).await
    }

    /// Add multiple timeline entries
    pub async fn add_entries(&self, entries: Vec<TimelineEntry>) -> Result<()> {
        for entry in entries {
            self.add_entry(entry).await?;
        }
        Ok(())
    }

    /// Run diagnostics on the collected timeline
    pub async fn run_diagnostics(
        &self,
        context: &DiagnosticContext,
    ) -> Result<DiagnosticReport> {
        let timeline = self.timeline_collector.get_timeline().await;

        let mut findings = Vec::new();

        for analyzer in &self.analyzers {
            let result = analyzer.analyze(&timeline, context).await?;
            findings.extend(result.findings);
        }

        let root_causes = if self.config.enable_root_cause {
            self.root_cause_analyzer.analyze(&findings, context).await?
        } else {
            Vec::new()
        };

        let report = self.report_generator.generate(
            &timeline,
            &findings,
            &root_causes,
            context,
        ).await?;

        Ok(report)
    }

    pub async fn get_status(&self) -> DiagnosticStatus {
        self.timeline_collector.get_status().await
    }

    pub async fn clear(&self) -> Result<()> {
        self.timeline_collector.clear().await
    }
}

impl Default for DiagnosticsEngine {
    fn default() -> Self {
        Self::new(EngineConfig::default())
    }
}