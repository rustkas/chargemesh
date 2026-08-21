//! Core capability engine

mod capability;
mod resolver;
mod registry;
mod evaluator;

pub use capability::*;
pub use resolver::*;
pub use registry::*;
pub use evaluator::*;

use super::*;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Main capability engine
pub struct CapabilityEngine {
    registry: Arc<RwLock<CapabilityRegistry>>,
    resolver: Arc<CapabilityResolver>,
    evaluator: Arc<CapabilityEvaluator>,
}

impl CapabilityEngine {
    pub fn new() -> Self {
        let registry = Arc::new(RwLock::new(CapabilityRegistry::new()));
        let resolver = Arc::new(CapabilityResolver::new());
        let evaluator = Arc::new(CapabilityEvaluator::new());

        Self {
            registry,
            resolver,
            evaluator,
        }
    }

    /// Register a capability definition
    pub async fn register_capability(&self, capability: CapabilityDefinition) -> Result<()> {
        let mut registry = self.registry.write().await;
        registry.register(capability)
    }

    /// Determine capabilities for a station
    pub async fn determine_capabilities(
        &self,
        context: &CapabilityContext,
    ) -> Result<CapabilitySet> {
        // Step 1: Resolve base capabilities from protocol and vendor
        let base_caps = self.resolver.resolve_base(context).await?;

        // Step 2: Evaluate rules to refine capabilities
        let evaluated_caps = self.evaluator.evaluate(context, base_caps).await?;

        // Step 3: Apply runtime constraints
        let final_caps = self.apply_runtime_constraints(context, evaluated_caps).await?;

        Ok(final_caps)
    }

    /// Apply runtime constraints to capabilities
    async fn apply_runtime_constraints(
        &self,
        context: &CapabilityContext,
        mut capabilities: CapabilitySet,
    ) -> Result<CapabilitySet> {
        // If station is offline, capabilities are limited
        if !context.runtime.is_online {
            capabilities.set_capability(
                CapabilityType::RemoteDiagnostics,
                CapabilityState::NotAvailable {
                    reason: "Station is offline".to_string(),
                }
            );
            capabilities.set_capability(
                CapabilityType::RemoteFirmwareUpdate,
                CapabilityState::NotAvailable {
                    reason: "Station is offline".to_string(),
                }
            );
        }

        // If station is not booted, some capabilities not available
        if !context.runtime.is_booted {
            capabilities.set_capability(
                CapabilityType::SmartCharging,
                CapabilityState::NotAvailable {
                    reason: "Station not booted".to_string(),
                }
            );
        }

        // Check temperature constraints
        if let Some(temp) = context.runtime.temperature {
            if temp > 50.0 {
                capabilities.set_capability(
                    CapabilityType::FastCharging,
                    CapabilityState::Limited {
                        reason: "High temperature".to_string(),
                        parameters: {
                            let mut params = std::collections::HashMap::new();
                            params.insert("temperature".to_string(), serde_json::json!(temp));
                            params
                        },
                    }
                );
            }
        }

        // Check load constraints
        if let Some(load) = context.runtime.load_percentage {
            if load > 90 {
                capabilities.set_capability(
                    CapabilityType::SmartCharging,
                    CapabilityState::Limited {
                        reason: "High load".to_string(),
                        parameters: {
                            let mut params = std::collections::HashMap::new();
                            params.insert("load".to_string(), serde_json::json!(load));
                            params
                        },
                    }
                );
            }
        }

        Ok(capabilities)
    }

    /// Get detailed capability information
    pub async fn get_capability_details(
        &self,
        capability_type: &CapabilityType,
    ) -> Option<CapabilityDefinition> {
        let registry = self.registry.read().await;
        registry.get(capability_type)
    }
}

impl Default for CapabilityEngine {
    fn default() -> Self {
        Self::new()
    }
}