//! Capability registry

use super::*;
use std::collections::HashMap;

pub struct CapabilityRegistry {
    capabilities: HashMap<CapabilityType, CapabilityDefinition>,
    by_category: HashMap<CapabilityCategory, Vec<CapabilityType>>,
}

impl CapabilityRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            capabilities: HashMap::new(),
            by_category: HashMap::new(),
        };

        // Register default capabilities
        registry.register_defaults();

        registry
    }

    fn register_defaults(&mut self) {
        // Core capabilities
        self.register(CapabilityDefinition {
            id: CapabilityType::BasicCharging,
            name: "Basic Charging".to_string(),
            description: "Standard charging capability".to_string(),
            category: CapabilityCategory::Core,
            dependencies: vec![],
            required_protocols: vec![
                ProtocolRequirement {
                    protocol: ProtocolName::OCPP,
                    min_version: Some("1.6".to_string()),
                    max_version: None,
                }
            ],
            required_hardware: vec![],
            min_firmware_version: None,
            config_params: vec![],
            default_state: CapabilityState::Supported {
                parameters: Default::default(),
            },
        });

        self.register(CapabilityDefinition {
            id: CapabilityType::SmartCharging,
            name: "Smart Charging".to_string(),
            description: "Dynamic power management based on external signals".to_string(),
            category: CapabilityCategory::Smart,
            dependencies: vec![CapabilityType::BasicCharging],
            required_protocols: vec![
                ProtocolRequirement {
                    protocol: ProtocolName::OCPP,
                    min_version: Some("1.6".to_string()),
                    max_version: None,
                }
            ],
            required_hardware: vec![],
            min_firmware_version: Some("1.5.0".to_string()),
            config_params: vec![
                ConfigParam {
                    key: "max_power".to_string(),
                    description: "Maximum power in watts".to_string(),
                    required: true,
                    default_value: Some(serde_json::json!(22000)),
                },
                ConfigParam {
                    key: "min_power".to_string(),
                    description: "Minimum power in watts".to_string(),
                    required: false,
                    default_value: Some(serde_json::json!(1000)),
                },
            ],
            default_state: CapabilityState::NotSupported {
                reason: Some("Requires OCPP 1.6+ and firmware support".to_string()),
            },
        });

        self.register(CapabilityDefinition {
            id: CapabilityType::PlugAndCharge,
            name: "Plug & Charge".to_string(),
            description: "ISO 15118 automatic authentication".to_string(),
            category: CapabilityCategory::Protocol,
            dependencies: vec![CapabilityType::ISO15118],
            required_protocols: vec![
                ProtocolRequirement {
                    protocol: ProtocolName::ISO15118,
                    min_version: Some("2.0".to_string()),
                    max_version: None,
                }
            ],
            required_hardware: vec![],
            min_firmware_version: Some("2.0.0".to_string()),
            config_params: vec![],
            default_state: CapabilityState::NotSupported {
                reason: Some("Requires ISO 15118 support".to_string()),
            },
        });

        self.register(CapabilityDefinition {
            id: CapabilityType::V2G,
            name: "Vehicle-to-Grid".to_string(),
            description: "Bidirectional power flow from EV to grid".to_string(),
            category: CapabilityCategory::Core,
            dependencies: vec![CapabilityType::Bidirectional],
            required_protocols: vec![
                ProtocolRequirement {
                    protocol: ProtocolName::ISO15118,
                    min_version: Some("2.0".to_string()),
                    max_version: None,
                },
                ProtocolRequirement {
                    protocol: ProtocolName::OCPP,
                    min_version: Some("2.1".to_string()),
                    max_version: None,
                }
            ],
            required_hardware: vec![],
            min_firmware_version: Some("3.0.0".to_string()),
            config_params: vec![
                ConfigParam {
                    key: "max_discharge_power".to_string(),
                    description: "Maximum discharge power in watts".to_string(),
                    required: true,
                    default_value: Some(serde_json::json!(11000)),
                },
            ],
            default_state: CapabilityState::NotSupported {
                reason: Some("Requires ISO 15118-20 and hardware support".to_string()),
            },
        });

        self.register(CapabilityDefinition {
            id: CapabilityType::RemoteDiagnostics,
            name: "Remote Diagnostics".to_string(),
            description: "Ability to run diagnostics remotely".to_string(),
            category: CapabilityCategory::Management,
            dependencies: vec![CapabilityType::BasicCharging],
            required_protocols: vec![
                ProtocolRequirement {
                    protocol: ProtocolName::OCPP,
                    min_version: Some("1.6".to_string()),
                    max_version: None,
                }
            ],
            required_hardware: vec![],
            min_firmware_version: None,
            config_params: vec![],
            default_state: CapabilityState::Supported {
                parameters: Default::default(),
            },
        });
    }

    pub fn register(&mut self, definition: CapabilityDefinition) {
        let cap_type = definition.id.clone();
        let category = definition.category.clone();

        self.capabilities.insert(cap_type.clone(), definition);
        self.by_category
            .entry(category)
            .or_insert_with(Vec::new)
            .push(cap_type);
    }

    pub fn get(&self, cap_type: &CapabilityType) -> Option<CapabilityDefinition> {
        self.capabilities.get(cap_type).cloned()
    }

    pub fn get_by_category(&self, category: &CapabilityCategory) -> Vec<CapabilityDefinition> {
        self.by_category
            .get(category)
            .map(|types| {
                types
                    .iter()
                    .filter_map(|t| self.capabilities.get(t).cloned())
                    .collect()
            })
            .unwrap_or_default()
    }

    pub fn list_all(&self) -> Vec<CapabilityDefinition> {
        self.capabilities.values().cloned().collect()
    }
}

impl Default for CapabilityRegistry {
    fn default() -> Self {
        Self::new()
    }
}