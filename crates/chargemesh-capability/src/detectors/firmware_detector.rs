//! Firmware-based capability detector

use super::*;
use regex::Regex;
use std::collections::HashMap;

pub struct FirmwareDetector {
    compatibility_map: HashMap<String, Vec<FirmwareCapability>>,
}

#[derive(Debug, Clone)]
pub struct FirmwareCapability {
    pub capability: CapabilityType,
    pub min_version: String,
    pub max_version: Option<String>,
    pub conditions: Vec<String>,
}

impl FirmwareDetector {
    pub fn new() -> Self {
        let mut detector = Self {
            compatibility_map: HashMap::new(),
        };

        detector.load_firmware_map();
        detector
    }

    fn load_firmware_map(&mut self) {
        // ABB firmware compatibility
        let abbcaps = vec![
            FirmwareCapability {
                capability: CapabilityType::SmartCharging,
                min_version: "1.5.0".to_string(),
                max_version: None,
                conditions: vec![],
            },
            FirmwareCapability {
                capability: CapabilityType::ISO15118,
                min_version: "2.0.0".to_string(),
                max_version: None,
                conditions: vec!["hardware:ccs".to_string()],
            },
            FirmwareCapability {
                capability: CapabilityType::PlugAndCharge,
                min_version: "2.0.0".to_string(),
                max_version: None,
                conditions: vec!["hardware:ccs".to_string(), "certificate:enabled".to_string()],
            },
            FirmwareCapability {
                capability: CapabilityType::V2G,
                min_version: "3.0.0".to_string(),
                max_version: None,
                conditions: vec!["hardware:bidirectional".to_string()],
            },
        ];

        self.compatibility_map.insert("ABB".to_string(), abbcaps);
    }

    fn compare_versions(&self, version: &str, min_version: &str) -> bool {
        // Simple version comparison (could use semver)
        let v_parts: Vec<u32> = version.split('.').map(|s| s.parse().unwrap_or(0)).collect();
        let min_parts: Vec<u32> = min_version.split('.').map(|s| s.parse().unwrap_or(0)).collect();

        for (v, m) in v_parts.iter().zip(min_parts.iter()) {
            if v < m {
                return false;
            }
            if v > m {
                return true;
            }
        }
        true
    }
}

#[async_trait]
impl CapabilityDetector for FirmwareDetector {
    async fn detect(&self, context: &CapabilityContext) -> Result<CapabilitySet> {
        let mut caps = CapabilitySet::new();

        if let Some(firmware_caps) = self.compatibility_map.get(&context.vendor.name) {
            for fw_cap in firmware_caps {
                if self.compare_versions(&context.firmware.version, &fw_cap.min_version) {
                    // Check max version if specified
                    if let Some(max_version) = &fw_cap.max_version {
                        if !self.compare_versions(&context.firmware.version, max_version) {
                            continue;
                        }
                    }

                    // Check conditions (simplified)
                    let all_conditions_met = fw_cap.conditions.iter().all(|cond| {
                        // Check hardware conditions
                        if cond.starts_with("hardware:") {
                            let required = cond.trim_start_matches("hardware:");
                            context.model.to_lowercase().contains(required)
                                || context
                                    .hardware_version
                                    .as_ref()
                                    .map(|v| v.to_lowercase().contains(required))
                                    .unwrap_or(false)
                        } else if cond.starts_with("certificate:") {
                            // Check if certificate is available
                            context.configuration.get("certificate_enabled")
                                .and_then(|v| v.as_bool())
                                .unwrap_or(false)
                        } else {
                            true
                        }
                    });

                    if all_conditions_met {
                        caps.add_capability(
                            fw_cap.capability.clone(),
                            CapabilityState::Supported {
                                parameters: Default::default(),
                            }
                        );
                    }
                }
            }
        }

        caps.source = CapabilitySource::ProtocolDiscovery;
        Ok(caps)
    }
}

impl Default for FirmwareDetector {
    fn default() -> Self {
        Self::new()
    }
}