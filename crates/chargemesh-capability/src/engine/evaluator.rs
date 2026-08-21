//! Capability evaluator — evaluates rules to refine capabilities

use super::*;
use crate::rules::*;

pub struct CapabilityEvaluator {
    rule_engine: RuleEngine,
}

impl CapabilityEvaluator {
    pub fn new() -> Self {
        let mut evaluator = Self {
            rule_engine: RuleEngine::new(),
        };

        evaluator.load_default_rules();
        evaluator
    }

    fn load_default_rules(&mut self) {
        // Rule: If ISO 15118 is supported, Plug & Charge might be available
        let rule = Rule::new(
            "plug_and_charge_availability".to_string(),
            "ISO 15118 support implies Plug & Charge".to_string(),
        )
        .with_condition(
            Condition::CapabilityExists {
                capability: CapabilityType::ISO15118,
            }
        )
        .with_condition(
            Condition::And {
                left: Box::new(Condition::ProtocolVersion {
                    protocol: ProtocolName::ISO15118,
                    min_version: Some("2.0".to_string()),
                    max_version: None,
                }),
                right: Box::new(Condition::FirmwareVersion {
                    min_version: Some("2.0.0".to_string()),
                    max_version: None,
                }),
            }
        )
        .with_action(
            Action::SetCapability {
                capability: CapabilityType::PlugAndCharge,
                state: CapabilityState::Supported {
                    parameters: Default::default(),
                },
            }
        );

        self.rule_engine.add_rule(rule);

        // Rule: OCPP 2.0.1 enables Smart Charging enhancements
        let rule = Rule::new(
            "smart_charging_enhanced".to_string(),
            "OCPP 2.0.1+ provides enhanced smart charging".to_string(),
        )
        .with_condition(
            Condition::And {
                left: Box::new(Condition::CapabilityExists {
                    capability: CapabilityType::OCPP2_0_1,
                }),
                right: Box::new(Condition::CapabilityExists {
                    capability: CapabilityType::SmartCharging,
                }),
            }
        )
        .with_action(
            Action::SetCapability {
                capability: CapabilityType::SmartCharging,
                state: CapabilityState::Supported {
                    parameters: {
                        let mut params = std::collections::HashMap::new();
                        params.insert(
                            "enhanced".to_string(),
                            serde_json::json!(true)
                        );
                        params.insert(
                            "features".to_string(),
                            serde_json::json!(["external_constraints", "local_generation"])
                        );
                        params
                    },
                },
            }
        );

        self.rule_engine.add_rule(rule);

        // Rule: V2G requires hardware support
        let rule = Rule::new(
            "v2g_hardware_requirement".to_string(),
            "V2G requires hardware support".to_string(),
        )
        .with_condition(
            Condition::And {
                left: Box::new(Condition::CapabilityExists {
                    capability: CapabilityType::V2G,
                }),
                right: Box::new(Condition::Not {
                    inner: Box::new(Condition::HardwareModel {
                        vendor: "ABB".to_string(),
                        model: "Terra 54".to_string(),
                    }),
                }),
            }
        )
        .with_action(
            Action::SetCapability {
                capability: CapabilityType::V2G,
                state: CapabilityState::NotSupported {
                    reason: Some("Hardware does not support bidirectional flow".to_string()),
                },
            }
        );

        self.rule_engine.add_rule(rule);

        // Rule: Reservation requires OCPP 2.0.1+
        let rule = Rule::new(
            "reservation_requires_ocpp201".to_string(),
            "Reservation requires OCPP 2.0.1+".to_string(),
        )
        .with_condition(
            Condition::Not {
                inner: Box::new(Condition::ProtocolVersion {
                    protocol: ProtocolName::OCPP,
                    min_version: Some("2.0.1".to_string()),
                    max_version: None,
                }),
            }
        )
        .with_action(
            Action::SetCapability {
                capability: CapabilityType::Reservation,
                state: CapabilityState::NotSupported {
                    reason: Some("Reservation requires OCPP 2.0.1 or higher".to_string()),
                },
            }
        );

        self.rule_engine.add_rule(rule);
    }

    /// Evaluate rules on a capability set
    pub async fn evaluate(
        &self,
        context: &CapabilityContext,
        mut capabilities: CapabilitySet,
    ) -> Result<CapabilitySet> {
        let rules = self.rule_engine.get_rules();
        let mut modified = false;

        for rule in rules {
            if self.rule_engine.evaluate_rule(rule, context, &capabilities) {
                self.rule_engine.apply_action(rule, &mut capabilities);
                modified = true;
            }
        }

        if modified {
            capabilities.source = CapabilitySource::RuleEvaluation;
        }

        Ok(capabilities)
    }
}

impl Default for CapabilityEvaluator {
    fn default() -> Self {
        Self::new()
    }
}