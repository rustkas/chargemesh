//! Pattern-based analyzer

use super::*;
use regex::Regex;
use std::collections::HashMap;

pub struct PatternAnalyzer {
    patterns: Vec<DiagnosticPattern>,
}

#[derive(Debug, Clone)]
pub struct DiagnosticPattern {
    pub name: String,
    pub description: String,
    pub condition: PatternCondition,
    pub severity: DiagnosticSeverity,
    pub recommendations: Vec<Recommendation>,
}

#[derive(Debug, Clone)]
pub enum PatternCondition {
    Any { event_type: EventType, status: Option<EntryStatus> },
    Sequence { events: Vec<EventType> },
    Timeout { event_type: EventType, threshold: chrono::Duration },
    ErrorAfter { event_type: EventType, error_code: Option<String> },
    Custom { regex: Regex },
}

impl PatternAnalyzer {
    pub fn new() -> Self {
        let mut analyzer = Self {
            patterns: Vec::new(),
        };
        analyzer.load_default_patterns();
        analyzer
    }

    fn load_default_patterns(&mut self) {
        // Pattern 1: Certificate validation failure
        self.patterns.push(DiagnosticPattern {
            name: "Certificate Validation Failure".to_string(),
            description: "ISO 15118 certificate validation failed".to_string(),
            condition: PatternCondition::Sequence {
                events: vec![
                    EventType::ISO15118Handshake,
                    EventType::ISO15118CertificateValidation,
                ],
            },
            severity: DiagnosticSeverity::Error,
            recommendations: vec![
                Recommendation {
                    action: "Check certificate validity".to_string(),
                    description: "Verify certificate is not expired and trust chain is valid".to_string(),
                    priority: RecommendationPriority::High,
                    estimated_time: Some(chrono::Duration::minutes(5)),
                    steps: vec![
                        "Check certificate expiry date".to_string(),
                        "Verify trust chain".to_string(),
                        "Check system time synchronization".to_string(),
                    ],
                },
            ],
        });

        // Pattern 2: Network disconnection during charging
        self.patterns.push(DiagnosticPattern {
            name: "Network Disconnection".to_string(),
            description: "Network connection was lost during charging".to_string(),
            condition: PatternCondition::Sequence {
                events: vec![
                    EventType::ConnectionEstablished,
                    EventType::ConnectionLost,
                ],
            },
            severity: DiagnosticSeverity::Warning,
            recommendations: vec![
                Recommendation {
                    action: "Check network connectivity".to_string(),
                    description: "Verify network stability and signal strength".to_string(),
                    priority: RecommendationPriority::High,
                    estimated_time: Some(chrono::Duration::minutes(10)),
                    steps: vec![
                        "Check network cable / WiFi signal".to_string(),
                        "Verify router configuration".to_string(),
                        "Check for network interference".to_string(),
                    ],
                },
            ],
        });

        // Pattern 3: Meter value anomalies
        self.patterns.push(DiagnosticPattern {
            name: "Meter Value Anomaly".to_string(),
            description: "Unexpected meter value readings detected".to_string(),
            condition: PatternCondition::Timeout {
                event_type: EventType::MeterValues,
                threshold: chrono::Duration::seconds(30),
            },
            severity: DiagnosticSeverity::Warning,
            recommendations: vec![
                Recommendation {
                    action: "Check metering system".to_string(),
                    description: "Verify meter accuracy and calibration".to_string(),
                    priority: RecommendationPriority::Medium,
                    estimated_time: Some(chrono::Duration::minutes(15)),
                    steps: vec![
                        "Check meter calibration".to_string(),
                        "Verify meter readings against expected values".to_string(),
                        "Check meter firmware version".to_string(),
                    ],
                },
            ],
        });

        // Pattern 4: Smart charging profile rejection
        self.patterns.push(DiagnosticPattern {
            name: "Smart Charging Profile Rejected".to_string(),
            description: "Charging profile was rejected by the station".to_string(),
            condition: PatternCondition::ErrorAfter {
                event_type: EventType::SetChargingProfile,
                error_code: Some("Rejected".to_string()),
            },
            severity: DiagnosticSeverity::Warning,
            recommendations: vec![
                Recommendation {
                    action: "Review charging profile parameters".to_string(),
                    description: "Verify profile parameters are within supported ranges".to_string(),
                    priority: RecommendationPriority::Medium,
                    estimated_time: Some(chrono::Duration::minutes(5)),
                    steps: vec![
                        "Check max/min power limits".to_string(),
                        "Verify schedule overlaps".to_string(),
                        "Check profile version compatibility".to_string(),
                    ],
                },
            ],
        });
    }
}

#[async_trait]
impl Analyzer for PatternAnalyzer {
    async fn analyze(
        &self,
        timeline: &[TimelineEntry],
        context: &DiagnosticContext,
    ) -> Result<AnalysisResult> {
        let mut findings = Vec::new();

        for pattern in &self.patterns {
            if self.matches_pattern(timeline, pattern) {
                let finding = self.create_finding(pattern, timeline);
                findings.push(finding);
            }
        }

        let severity = if findings.iter().any(|f| f.severity == DiagnosticSeverity::Critical) {
            DiagnosticSeverity::Critical
        } else if findings.iter().any(|f| f.severity == DiagnosticSeverity::Error) {
            DiagnosticSeverity::Error
        } else if findings.iter().any(|f| f.severity == DiagnosticSeverity::Warning) {
            DiagnosticSeverity::Warning
        } else {
            DiagnosticSeverity::Info
        };

        Ok(AnalysisResult {
            findings,
            summary: format!("Pattern analysis complete: {} findings", findings.len()),
            severity,
            metadata: serde_json::json!({
                "analyzer": "PatternAnalyzer",
                "patterns_checked": self.patterns.len(),
            }),
        })
    }

    fn name(&self) -> &str {
        "PatternAnalyzer"
    }

    fn description(&self) -> &str {
        "Identifies known failure patterns in the timeline"
    }
}

impl PatternAnalyzer {
    fn matches_pattern(&self, timeline: &[TimelineEntry], pattern: &DiagnosticPattern) -> bool {
        match &pattern.condition {
            PatternCondition::Any { event_type, status } => {
                timeline.iter().any(|entry| {
                    entry.event_type == *event_type &&
                    status.map(|s| entry.status == s).unwrap_or(true)
                })
            }
            PatternCondition::Sequence { events } => {
                let mut pos = 0;
                for entry in timeline {
                    if pos < events.len() && entry.event_type == events[pos] {
                        pos += 1;
                        if pos == events.len() {
                            return true;
                        }
                    }
                }
                false
            }
            PatternCondition::Timeout { event_type, threshold } => {
                let mut last_time = None;
                for entry in timeline {
                    if entry.event_type == *event_type {
                        last_time = Some(entry.timestamp);
                    }
                }
                if let Some(last) = last_time {
                    let now = chrono::Utc::now();
                    now - last > *threshold
                } else {
                    false
                }
            }
            PatternCondition::ErrorAfter { event_type, error_code } => {
                let mut found_event = false;
                for entry in timeline {
                    if !found_event && entry.event_type == *event_type {
                        found_event = true;
                        continue;
                    }
                    if found_event && entry.event_type == EventType::Error {
                        if let Some(code) = error_code {
                            if let Some(details) = entry.details.get("error_code") {
                                if details.as_str() == Some(code) {
                                    return true;
                                }
                            }
                        } else {
                            return true;
                        }
                    }
                }
                false
            }
            PatternCondition::Custom { regex } => {
                timeline.iter().any(|entry| {
                    regex.is_match(&serde_json::to_string(&entry).unwrap_or_default())
                })
            }
        }
    }

    fn create_finding(
        &self,
        pattern: &DiagnosticPattern,
        timeline: &[TimelineEntry],
    ) -> Finding {
        let evidence = timeline.iter()
            .filter(|e| e.status == EntryStatus::Failure || e.status == EntryStatus::Timeout)
            .take(5)
            .map(|e| Evidence {
                entry_id: e.id.clone(),
                timestamp: e.timestamp,
                description: format!("{:?} - {:?}", e.event_type, e.status),
                data: e.details.clone(),
            })
            .collect();

        Finding {
            id: uuid::Uuid::new_v4().to_string(),
            title: pattern.name.clone(),
            description: pattern.description.clone(),
            severity: pattern.severity.clone(),
            component: Component::System,
            timestamp: chrono::Utc::now(),
            evidence,
            recommendations: pattern.recommendations.clone(),
            tags: vec!["pattern".to_string()],
            confidence: 0.85,
        }
    }
}

impl Default for PatternAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}