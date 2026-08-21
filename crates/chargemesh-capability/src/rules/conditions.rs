//! Rule conditions

use super::*;

#[derive(Debug, Clone)]
pub enum Condition {
    /// Check if a capability exists
    CapabilityExists {
        capability: CapabilityType,
    },

    /// Check if a capability has a specific state
    CapabilityState {
        capability: CapabilityType,
        state: CapabilityState,
    },

    /// Check protocol version
    ProtocolVersion {
        protocol: ProtocolName,
        min_version: Option<String>,
        max_version: Option<String>,
    },

    /// Check firmware version
    FirmwareVersion {
        min_version: Option<String>,
        max_version: Option<String>,
    },

    /// Check vendor/model
    HardwareModel {
        vendor: String,
        model: String,
    },

    /// Check runtime condition
    RuntimeCondition {
        condition_type: RuntimeConditionType,
        value: serde_json::Value,
    },

    /// Logical NOT
    Not {
        inner: Box<Condition>,
    },

    /// Logical AND
    And {
        left: Box<Condition>,
        right: Box<Condition>,
    },

    /// Logical OR
    Or {
        left: Box<Condition>,
        right: Box<Condition>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RuntimeConditionType {
    Online,
    Booted,
    TemperatureBelow,
    TemperatureAbove,
    LoadBelow,
    LoadAbove,
    ActiveSessionsBelow,
}

impl Condition {
    pub fn evaluate(&self, context: &CapabilityContext, capabilities: &CapabilitySet) -> bool {
        match self {
            Condition::CapabilityExists { capability } => {
                capabilities.has_capability(capability)
            }
            Condition::CapabilityState { capability, state } => {
                capabilities
                    .get_capability(capability)
                    .map(|s| std::mem::discriminant(s) == std::mem::discriminant(state))
                    .unwrap_or(false)
            }
            Condition::ProtocolVersion { protocol, min_version, max_version } => {
                if context.protocol.name != *protocol {
                    return false;
                }

                if let Some(min) = min_version {
                    if context.protocol.version < *min {
                        return false;
                    }
                }

                if let Some(max) = max_version {
                    if context.protocol.version > *max {
                        return false;
                    }
                }

                true
            }
            Condition::FirmwareVersion { min_version, max_version } => {
                if let Some(min) = min_version {
                    if context.firmware.version < *min {
                        return false;
                    }
                }

                if let Some(max) = max_version {
                    if context.firmware.version > *max {
                        return false;
                    }
                }

                true
            }
            Condition::HardwareModel { vendor, model } => {
                context.vendor.name == *vendor && context.model == *model
            }
            Condition::RuntimeCondition { condition_type, value } => {
                match condition_type {
                    RuntimeConditionType::Online => {
                        context.runtime.is_online == value.as_bool().unwrap_or(false)
                    }
                    RuntimeConditionType::Booted => {
                        context.runtime.is_booted == value.as_bool().unwrap_or(false)
                    }
                    RuntimeConditionType::TemperatureBelow => {
                        let threshold = value.as_f64().unwrap_or(0.0);
                        context.runtime.temperature
                            .map(|t| t as f64 < threshold)
                            .unwrap_or(false)
                    }
                    RuntimeConditionType::TemperatureAbove => {
                        let threshold = value.as_f64().unwrap_or(0.0);
                        context.runtime.temperature
                            .map(|t| t as f64 > threshold)
                            .unwrap_or(false)
                    }
                    RuntimeConditionType::LoadBelow => {
                        let threshold = value.as_u64().unwrap_or(0) as u8;
                        context.runtime.load_percentage
                            .map(|l| l < threshold)
                            .unwrap_or(false)
                    }
                    RuntimeConditionType::LoadAbove => {
                        let threshold = value.as_u64().unwrap_or(0) as u8;
                        context.runtime.load_percentage
                            .map(|l| l > threshold)
                            .unwrap_or(false)
                    }
                    RuntimeConditionType::ActiveSessionsBelow => {
                        let limit = value.as_u64().unwrap_or(0);
                        context.runtime.active_sessions < limit as u32
                    }
                }
            }
            Condition::Not { inner } => {
                !inner.evaluate(context, capabilities)
            }
            Condition::And { left, right } => {
                left.evaluate(context, capabilities) && right.evaluate(context, capabilities)
            }
            Condition::Or { left, right } => {
                left.evaluate(context, capabilities) || right.evaluate(context, capabilities)
            }
        }
    }
}