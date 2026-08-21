//! Root cause analyzer

use super::*;
use std::collections::{HashMap, HashSet};

pub struct RootCauseAnalyzer {
    inference_engine: InferenceEngine,
    pattern_repository: PatternRepository,
}

impl RootCauseAnalyzer {
    pub fn new() -> Self {
        let mut analyzer = Self {
            inference_engine: InferenceEngine::new(),
            pattern_repository: PatternRepository::new(),
        };
        analyzer.load_patterns();
        analyzer
    }

    fn load_patterns(&mut self) {
        self.pattern_repository.register(RootCausePattern {
            id: "cert_validation_failure".to_string(),
            name: "Certificate Validation Failure".to_string(),
            description: "ISO 15118 certificate validation failed".to_string(),
            symptoms: vec![
                "ISO15118CertificateValidation".to_string(),
                "EVCommunicationError".to_string(),
                "ConnectionLost".to_string(),
            ],
            causes: vec![
                PossibleCause {
                    id: "expired_cert".to_string(),
                    description: "Certificate has expired".to_string(),
                    probability: 0.4,
                    evidence: vec!["Expired certificate detected".to_string()],
                    mitigation: "Renew certificate and update system time".to_string(),
                },
                PossibleCause {
                    id: "invalid_trust_chain".to_string(),
                    description: "Certificate trust chain is invalid".to_string(),
                    probability: 0.3,
                    evidence: vec!["Invalid trust chain".to_string()],
                    mitigation: "Verify and update trust chain configuration".to_string(),
                },
                PossibleCause {
                    id: "system_time".to_string(),
                    description: "System time is incorrect".to_string(),
                    probability: 0.2,
                    evidence: vec!["System time mismatch".to_string()],
                    mitigation: "Synchronize system time with NTP".to_string(),
                },
                PossibleCause {
                    id: "secc_mismatch".to_string(),
                    description: "SECC certificate mismatch".to_string(),
                    probability: 0.1,
                    evidence: vec!["SECC certificate mismatch".to_string()],
                    mitigation: "Update SECC certificate configuration".to_string(),
                },
            ],
            confidence_weight: 1.0,
        });

        self.pattern_repository.register(RootCausePattern {
            id: "network_connectivity".to_string(),
            name: "Network Connectivity Issue".to_string(),
            description: "Network connection problems detected".to_string(),
            symptoms: vec![
                "ConnectionLost".to_string(),
                "Timeout".to_string(),
                "Heartbeat".to_string(),
            ],
            causes: vec![
                PossibleCause {
                    id: "network_cable".to_string(),
                    description: "Network cable disconnected or damaged".to_string(),
                    probability: 0.35,
                    evidence: vec!["Physical connection issues".to_string()],
                    mitigation: "Check and reconnect network cable".to_string(),
                },
                PossibleCause {
                    id: "wifi_signal".to_string(),
                    description: "Weak WiFi signal".to_string(),
                    probability: 0.3,
                    evidence: vec!["Poor signal strength".to_string()],
                    mitigation: "Improve WiFi coverage or use wired connection".to_string(),
                },
                PossibleCause {
                    id: "network_config".to_string(),
                    description: "Network configuration error".to_string(),
                    probability: 0.2,
                    evidence: vec!["IP configuration issues".to_string()],
                    mitigation: "Verify network configuration".to_string(),
                },
                PossibleCause {
                    id: "firewall_block".to_string(),
                    description: "Firewall blocking connection".to_string(),
                    probability: 0.15,
                    evidence: vec!["Connection blocked".to_string()],
                    mitigation: "Check firewall rules".to_string(),
                },
            ],
            confidence_weight: 0.9,
        });

        self.pattern_repository.register(RootCausePattern {
            id: "metering_issue".to_string(),
            name: "Metering System Issue".to_string(),
            description: "Problems with energy metering".to_string(),
            symptoms: vec![
                "MeterValues".to_string(),
                "PowerMeterFailure".to_string(),
                "Transaction".to_string(),
            ],
            causes: vec![
                PossibleCause {
                    id: "meter_calibration".to_string(),
                    description: "Meter calibration error".to_string(),
                    probability: 0.4,
                    evidence: vec!["Calibration mismatch".to_string()],
                    mitigation: "Recalibrate meter".to_string(),
                },
                PossibleCause {
                    id: "meter_firmware".to_string(),
                    description: "Meter firmware bug".to_string(),
                    probability: 0.3,
                    evidence: vec!["Firmware inconsistency".to_string()],
                    mitigation: "Update meter firmware".to_string(),
                },
                PossibleCause {
                    id: "hardware_fault".to_string(),
                    description: "Hardware fault in metering circuit".to_string(),
                    probability: 0.2,
                    evidence: vec!["Hardware error detected".to_string()],
                    mitigation: "Replace meter hardware".to_string(),
                },
                PossibleCause {
                    id: "communication_error".to_string(),
                    description: "Meter communication error".to_string(),
                    probability: 0.1,
                    evidence: vec!["Communication issues".to_string()],
                    mitigation: "Check meter communication interface".to_string(),
                },
            ],
            confidence_weight: 0.85,
        });
    }

    pub async fn analyze(
        &self,
        findings: &[Finding],
        context: &DiagnosticContext,
    ) -> Result<Vec<RootCause>> {
        let mut root_causes = Vec::new();

        let mut symptoms: HashSet<String> = HashSet::new();
        for finding in findings {
            symptoms.insert(finding.title.clone());
            for tag in &finding.tags {
                symptoms.insert(tag.clone());
            }
        }

        let patterns = self.pattern_repository.find_matching(&symptoms);

        for pattern in patterns {
            let confidence = self.calculate_confidence(&pattern, findings);

            if confidence > 0.3 {
                let root_cause = RootCause {
                    id: uuid::Uuid::new_v4().to_string(),
                    title: pattern.name.clone(),
                    description: pattern.description.clone(),
                    confidence,
                    severity: self.determine_severity(&pattern, findings),
                    causes: pattern.causes.clone(),
                    affected_components: self.determine_components(findings),
                    evidence: self.extract_evidence(findings),
                    recommendations: self.generate_recommendations(&pattern),
                    timestamp: chrono::Utc::now(),
                };
                root_causes.push(root_cause);
            }
        }

        root_causes.sort_by(|a, b| b.confidence.partial_cmp(&a.confidence).unwrap());

        Ok(root_causes)
    }

    fn calculate_confidence(&self, pattern: &RootCausePattern, findings: &[Finding]) -> f64 {
        let matching_symptoms: Vec<&String> = pattern.symptoms.iter()
            .filter(|s| {
                findings.iter().any(|f| {
                    f.title.contains(s) || f.tags.iter().any(|t| t.contains(s))
                })
            })
            .collect();

        let ratio = matching_symptoms.len() as f64 / pattern.symptoms.len() as f64;
        ratio * pattern.confidence_weight
    }

    fn determine_severity(&self, pattern: &RootCausePattern, findings: &[Finding]) -> DiagnosticSeverity {
        findings.iter()
            .filter(|f| {
                pattern.symptoms.iter().any(|s| {
                    f.title.contains(s) || f.tags.iter().any(|t| t.contains(s))
                })
            })
            .map(|f| f.severity.clone())
            .max()
            .unwrap_or(DiagnosticSeverity::Error)
    }

    fn determine_components(&self, findings: &[Finding]) -> Vec<Component> {
        let mut components = HashSet::new();
        for finding in findings {
            components.insert(finding.component.clone());
        }
        components.into_iter().collect()
    }

    fn extract_evidence(&self, findings: &[Finding]) -> Vec<Evidence> {
        findings.iter()
            .flat_map(|f| f.evidence.clone())
            .take(10)
            .collect()
    }

    fn generate_recommendations(&self, pattern: &RootCausePattern) -> Vec<Recommendation> {
        pattern.causes.iter().map(|cause| {
            Recommendation {
                action: cause.description.clone(),
                description: cause.mitigation.clone(),
                priority: RecommendationPriority::High,
                estimated_time: Some(chrono::Duration::minutes(10)),
                steps: vec![
                    format!("Investigate: {}", cause.description),
                    "Implement mitigation".to_string(),
                    "Verify resolution".to_string(),
                ],
            }
        }).collect()
    }
}

impl Default for RootCauseAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}