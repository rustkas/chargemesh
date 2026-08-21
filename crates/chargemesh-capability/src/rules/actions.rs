//! Rule actions

use super::*;

#[derive(Debug, Clone)]
pub enum Action {
    /// Set a capability to a specific state
    SetCapability {
        capability: CapabilityType,
        state: CapabilityState,
    },

    /// Remove a capability
    RemoveCapability {
        capability: CapabilityType,
    },

    /// Add a parameter to a capability
    AddParameter {
        capability: CapabilityType,
        key: String,
        value: serde_json::Value,
    },
}

impl Action {
    pub fn apply(&self, capabilities: &mut CapabilitySet) {
        match self {
            Action::SetCapability { capability, state } => {
                capabilities.set_capability(capability.clone(), state.clone());
            }
            Action::RemoveCapability { capability } => {
                capabilities.capabilities.remove(capability);
            }
            Action::AddParameter { capability, key, value } => {
                if let Some(state) = capabilities.capabilities.get_mut(capability) {
                    match state {
                        CapabilityState::Supported { parameters } => {
                            parameters.insert(key.clone(), value.clone());
                        }
                        CapabilityState::Limited { parameters, .. } => {
                            parameters.insert(key.clone(), value.clone());
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}