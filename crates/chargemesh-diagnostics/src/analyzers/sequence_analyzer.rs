//! Sequence analyzer for detecting issues in event order

use super::*;
use std::collections::VecDeque;

pub struct SequenceAnalyzer {
    expected_sequences: Vec<ExpectedSequence>,
}

#[derive(Debug, Clone)]
pub struct ExpectedSequence {
    pub name: String,
    pub description: String,
    pub steps: Vec<EventType>,
    pub severity_on_missing: DiagnosticSeverity,
    pub severity_on_incorrect_order: DiagnosticSeverity,
}

impl SequenceAnalyzer {
    pub fn new() -> Self {
        let mut analyzer = Self {
            expected_sequences: Vec::new(),
        };
        analyzer.load_default_sequences();
        analyzer
    }

    fn load_default_sequences(&mut self) {
        self.expected_sequences.push(ExpectedSequence {
            name: "Normal Charging Sequence".to_string(),
            description: "Expected sequence for a normal charging session".to_string(),
            steps: vec![
                EventType::BootNotification,
                EventType::Heartbeat,
                EventType::StatusNotification,
                EventType::Authorize,
                EventType::StartTransaction,
                EventType::MeterValues,
                EventType::StopTransaction,
            ],
            severity_on_missing: DiagnosticSeverity::Warning,
            severity_on_incorrect_order: DiagnosticSeverity::Error,
        });

        self.expected_sequences.push(ExpectedSequence {
            name: "ISO 15118 Plug & Charge".to_string(),
            description: "Expected sequence for ISO 15118 Plug & Charge".to_string(),
            steps: vec![
                EventType::ISO15118Handshake,
                EventType::ISO15118CertificateValidation,
                EventType::Authorize,
                EventType::StartTransaction,
            ],
            severity_on_missing: DiagnosticSeverity::Error,
            severity_on_incorrect_order: DiagnosticSeverity::Critical,
        });
    }
}

#[async_trait]
impl Analyzer for SequenceAnalyzer {
    async fn analyze(
        &self,
        timeline: &[TimelineEntry],
        context: &DiagnosticContext,
    ) -> Result<AnalysisResult> {
        let mut findings = Vec::new();
        let events: Vec<&EventType> = timeline.iter().map(|e| &e.event_type).collect();

        for sequence in &self.expected_sequences {
            let result = self.check_sequence(&events, sequence);

            if let Some(issue) = result {
                let finding = Finding {
                    id: uuid::Uuid::new_v4().to_string(),
                    title: format!("Sequence Issue: {}", sequence.name),
                    description: format!(
                        "Expected sequence '{}' has issues: {}",
                        sequence.name,
                        issue
                    ),
                    severity: if result.contains("order") {
                        sequence.severity_on_incorrect_order.clone()
                    } else {
                        sequence.severity_on_missing.clone()
                    },
                    component: Component::Protocol,
                    timestamp: chrono::Utc::now(),
                    evidence: vec![],
                    recommendations: vec![
                        Recommendation {
                            action: "Check protocol implementation".to_string(),
                            description: "Verify correct message ordering".to_string(),
                            priority: RecommendationPriority::High,
                            estimated_time: Some(chrono::Duration::minutes(10)),
                            steps: vec![
                                "Verify protocol version compatibility".to_string(),
                                "Check message sequence numbers".to_string(),
                                "Validate state machine transitions".to_string(),
                            ],
                        },
                    ],
                    tags: vec!["sequence".to_string(), "protocol".to_string()],
                    confidence: 0.9,
                };
                findings.push(finding);
            }
        }

        let severity = if findings.iter().any(|f| f.severity == DiagnosticSeverity::Critical) {
            DiagnosticSeverity::Critical
        } else if findings.iter().any(|f| f.severity == DiagnosticSeverity::Error) {
            DiagnosticSeverity::Error
        } else {
            DiagnosticSeverity::Info
        };

        Ok(AnalysisResult {
            findings,
            summary: format!("Sequence analysis complete: {} issues found", findings.len()),
            severity,
            metadata: serde_json::json!({
                "analyzer": "SequenceAnalyzer",
                "sequences_checked": self.expected_sequences.len(),
            }),
        })
    }

    fn name(&self) -> &str {
        "SequenceAnalyzer"
    }

    fn description(&self) -> &str {
        "Analyzes event sequences for missing or out-of-order events"
    }
}

impl SequenceAnalyzer {
    fn check_sequence(
        &self,
        timeline: &[&EventType],
        expected: &ExpectedSequence,
    ) -> Option<String> {
        let mut missing = Vec::new();
        for expected_event in &expected.steps {
            if !timeline.contains(&expected_event) {
                missing.push(expected_event);
            }
        }

        if !missing.is_empty() {
            let missing_names: Vec<String> = missing.iter().map(|e| format!("{:?}", e)).collect();
            return Some(format!("Missing events: {}", missing_names.join(", ")));
        }

        let mut pos = 0;
        for step in &expected.steps {
            if let Some(found_pos) = timeline.iter().position(|&e| e == step) {
                if found_pos < pos {
                    return Some(format!(
                        "Out of order: {:?} appears after later events",
                        step
                    ));
                }
                pos = found_pos;
            }
        }

        None
    }
}

impl Default for SequenceAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}