//! Report generator

use super::*;
use handlebars::Handlebars;
use serde_json::json;

pub struct ReportGenerator {
    handlebars: Handlebars<'static>,
}

impl ReportGenerator {
    pub fn new() -> Self {
        let mut handlebars = Handlebars::new();
        handlebars.register_template_string("diagnostic_report", include_str!("templates/diagnostic_report.html"))
            .unwrap();
        Self { handlebars }
    }

    pub async fn generate(
        &self,
        timeline: &[TimelineEntry],
        findings: &[Finding],
        root_causes: &[RootCause],
        context: &DiagnosticContext,
    ) -> Result<DiagnosticReport> {
        let statistics = self.calculate_statistics(timeline, findings);

        let mut recommendations = Vec::new();
        for finding in findings {
            recommendations.extend(finding.recommendations.clone());
        }
        for root_cause in root_causes {
            recommendations.extend(root_cause.recommendations.clone());
        }

        let severity = self.determine_overall_severity(findings, root_causes);

        let report = DiagnosticReport {
            id: uuid::Uuid::new_v4().to_string(),
            title: "ChargeMesh Diagnostic Report".to_string(),
            generated_at: chrono::Utc::now(),
            summary: self.generate_summary(findings, root_causes),
            severity,
            timeline: timeline.to_vec(),
            findings: findings.to_vec(),
            root_causes: root_causes.to_vec(),
            recommendations,
            statistics,
            context: context.clone(),
            format: ReportFormat::JSON,
        };

        Ok(report)
    }

    fn calculate_statistics(
        &self,
        timeline: &[TimelineEntry],
        findings: &[Finding],
    ) -> ReportStatistics {
        let success_count = timeline.iter()
            .filter(|e| e.status == EntryStatus::Success)
            .count();
        let failure_count = timeline.iter()
            .filter(|e| e.status == EntryStatus::Failure)
            .count();
        let timeout_count = timeline.iter()
            .filter(|e| e.status == EntryStatus::Timeout)
            .count();
        let error_count = findings.iter()
            .filter(|f| f.severity == DiagnosticSeverity::Error)
            .count();
        let warnings_count = findings.iter()
            .filter(|f| f.severity == DiagnosticSeverity::Warning)
            .count();

        let session_duration = if let (Some(start), Some(stop)) = (
            timeline.iter().find(|e| e.event_type == EventType::StartTransaction),
            timeline.iter().find(|e| e.event_type == EventType::StopTransaction),
        ) {
            Some(stop.timestamp - start.timestamp)
        } else {
            None
        };

        ReportStatistics {
            total_entries: timeline.len(),
            success_count,
            failure_count,
            timeout_count,
            error_count,
            warnings_count,
            session_duration,
            total_energy: None,
        }
    }

    fn determine_overall_severity(
        &self,
        findings: &[Finding],
        root_causes: &[RootCause],
    ) -> DiagnosticSeverity {
        if root_causes.iter().any(|rc| rc.severity == DiagnosticSeverity::Critical) {
            return DiagnosticSeverity::Critical;
        }
        if findings.iter().any(|f| f.severity == DiagnosticSeverity::Critical) {
            return DiagnosticSeverity::Critical;
        }
        if root_causes.iter().any(|rc| rc.severity == DiagnosticSeverity::Error) {
            return DiagnosticSeverity::Error;
        }
        if findings.iter().any(|f| f.severity == DiagnosticSeverity::Error) {
            return DiagnosticSeverity::Error;
        }
        if findings.iter().any(|f| f.severity == DiagnosticSeverity::Warning) {
            return DiagnosticSeverity::Warning;
        }
        DiagnosticSeverity::Info
    }

    fn generate_summary(&self, findings: &[Finding], root_causes: &[RootCause]) -> String {
        let mut summary = String::new();

        if root_causes.is_empty() {
            summary.push_str("No critical issues detected. System appears to be operating normally.");
        } else {
            summary.push_str(&format!(
                "Found {} root cause(s) with {} total findings. ",
                root_causes.len(),
                findings.len()
            ));

            let critical = root_causes.iter()
                .filter(|rc| rc.severity == DiagnosticSeverity::Critical)
                .count();
            let errors = root_causes.iter()
                .filter(|rc| rc.severity == DiagnosticSeverity::Error)
                .count();

            if critical > 0 {
                summary.push_str(&format!("{} critical issue(s) detected. ", critical));
            }
            if errors > 0 {
                summary.push_str(&format!("{} error(s) detected. ", errors));
            }

            summary.push_str("\n\nTop root causes:\n");
            for (i, rc) in root_causes.iter().take(3).enumerate() {
                summary.push_str(&format!(
                    "{}. {} (confidence: {:.0}%)\n",
                    i + 1,
                    rc.title,
                    rc.confidence * 100.0
                ));
            }
        }

        summary
    }

    pub fn render_html(&self, report: &DiagnosticReport) -> Result<String> {
        let data = json!({
            "report": report,
            "severity_class": match report.severity {
                DiagnosticSeverity::Info => "info",
                DiagnosticSeverity::Warning => "warning",
                DiagnosticSeverity::Error => "error",
                DiagnosticSeverity::Critical => "critical",
            },
        });

        self.handlebars.render("diagnostic_report", &data)
            .map_err(|e| DiagnosticError::Internal(e.to_string()))
    }
}

impl Default for ReportGenerator {
    fn default() -> Self {
        Self::new()
    }
}