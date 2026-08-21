//! Runtime-based capability detector

use super::*;

pub struct RuntimeDetector;

impl RuntimeDetector {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl CapabilityDetector for RuntimeDetector {
    async fn detect(&self, context: &CapabilityContext) -> Result<CapabilitySet> {
        let mut caps = CapabilitySet::new();

        // Check online status
        if !context.runtime.is_online {
            caps.add_capability(
                CapabilityType::RemoteDiagnostics,
                CapabilityState::NotAvailable {
                    reason: "Station is offline".to_string(),
                }
            );
            caps.add_capability(
                CapabilityType::RemoteFirmwareUpdate,
                CapabilityState::NotAvailable {
                    reason: "Station is offline".to_string(),
                }
            );
        }

        // Check load
        if let Some(load) = context.runtime.load_percentage {
            if load > 80 {
                caps.add_capability(
                    CapabilityType::FastCharging,
                    CapabilityState::Limited {
                        reason: "High load - fast charging limited".to_string(),
                        parameters: {
                            let mut params = std::collections::HashMap::new();
                            params.insert("current_load".to_string(), serde_json::json!(load));
                            params
                        },
                    }
                );
            }
        }

        // Check temperature
        if let Some(temp) = context.runtime.temperature {
            if temp > 45.0 {
                caps.add_capability(
                    CapabilityType::SmartCharging,
                    CapabilityState::Limited {
                        reason: format!("High temperature: {:.1}°C - smart charging limited", temp),
                        parameters: {
                            let mut params = std::collections::HashMap::new();
                            params.insert("temperature".to_string(), serde_json::json!(temp));
                            params
                        },
                    }
                );
            }
        }

        // Check if active sessions limit other capabilities
        if context.runtime.active_sessions > 0 {
            caps.add_capability(
                CapabilityType::Reservation,
                CapabilityState::NotAvailable {
                    reason: "Active session in progress".to_string(),
                }
            );
        }

        caps.source = CapabilitySource::ProtocolDiscovery;
        Ok(caps)
    }
}

impl Default for RuntimeDetector {
    fn default() -> Self {
        Self::new()
    }
}