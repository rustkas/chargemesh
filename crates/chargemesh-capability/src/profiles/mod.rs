//! Vendor and model profiles

mod vendor_profiles;
mod model_profiles;
mod compatibility;

pub use vendor_profiles::*;
pub use model_profiles::*;
pub use compatibility::*;

use super::*;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelProfile {
    pub vendor: VendorInfo,
    pub model: String,
    pub hardware_versions: Vec<String>,
    pub capabilities: CapabilitySet,
    pub metadata: HashMap<String, serde_json::Value>,
}

impl ModelProfile {
    pub fn new(vendor: VendorInfo, model: String) -> Self {
        Self {
            vendor,
            model,
            hardware_versions: Vec::new(),
            capabilities: CapabilitySet::new(),
            metadata: HashMap::new(),
        }
    }

    pub fn with_capability(mut self, cap: CapabilityType, state: CapabilityState) -> Self {
        self.capabilities.add_capability(cap, state);
        self
    }

    pub fn with_hardware_version(mut self, version: String) -> Self {
        self.hardware_versions.push(version);
        self
    }

    pub fn with_metadata(mut self, key: String, value: serde_json::Value) -> Self {
        self.metadata.insert(key, value);
        self
    }
}

pub struct ModelProfileRegistry {
    profiles: Vec<ModelProfile>,
}

impl ModelProfileRegistry {
    pub fn new() -> Self {
        Self {
            profiles: Vec::new(),
        }
    }

    pub fn register(&mut self, profile: ModelProfile) {
        self.profiles.push(profile);
    }

    pub fn get_profile(&self, vendor: &str, model: &str) -> Option<&ModelProfile> {
        self.profiles.iter().find(|p| {
            p.vendor.name == vendor && p.model == model
        })
    }

    pub fn get_profiles_by_vendor(&self, vendor: &str) -> Vec<&ModelProfile> {
        self.profiles.iter()
            .filter(|p| p.vendor.name == vendor)
            .collect()
    }
}

impl Default for ModelProfileRegistry {
    fn default() -> Self {
        Self::new()
    }
}