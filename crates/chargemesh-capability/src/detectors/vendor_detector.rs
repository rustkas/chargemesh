//! Vendor-based capability detector

use super::*;
use crate::profiles::*;

pub struct VendorDetector {
    profiles: Vec<ModelProfile>,
}

impl VendorDetector {
    pub fn new() -> Self {
        let mut detector = Self {
            profiles: Vec::new(),
        };

        detector.load_vendor_profiles();
        detector
    }

    fn load_vendor_profiles(&mut self) {
        // ABB Terra series
        let profile = ModelProfile::new(
            VendorInfo {
                name: "ABB".to_string(),
                id: Some("ABB".to_string()),
                known_models: vec![
                    "Terra 54".to_string(),
                    "Terra 184".to_string(),
                    "Terra HP".to_string(),
                ],
            },
            "Terra 54".to_string(),
        )
        .with_capability(
            CapabilityType::BasicCharging,
            CapabilityState::Supported { parameters: Default::default() }
        )
        .with_capability(
            CapabilityType::SmartCharging,
            CapabilityState::Supported { parameters: Default::default() }
        )
        .with_capability(
            CapabilityType::RFIDAuthorization,
            CapabilityState::Supported { parameters: Default::default() }
        );

        self.profiles.push(profile);

        // Siemens VersiCharge
        let profile = ModelProfile::new(
            VendorInfo {
                name: "Siemens".to_string(),
                id: Some("SIEMENS".to_string()),
                known_models: vec![
                    "VersiCharge".to_string(),
                    "VersiCharge Pro".to_string(),
                ],
            },
            "VersiCharge".to_string(),
        )
        .with_capability(
            CapabilityType::BasicCharging,
            CapabilityState::Supported { parameters: Default::default() }
        )
        .with_capability(
            CapabilityType::LoadBalancing,
            CapabilityState::Supported { parameters: Default::default() }
        );

        self.profiles.push(profile);
    }
}

#[async_trait]
impl CapabilityDetector for VendorDetector {
    async fn detect(&self, context: &CapabilityContext) -> Result<CapabilitySet> {
        let mut caps = CapabilitySet::new();

        for profile in &self.profiles {
            if profile.vendor.name == context.vendor.name 
                && profile.model == context.model {
                caps = profile.capabilities.clone();
                caps.source = CapabilitySource::VendorProfile;
                break;
            }
        }

        Ok(caps)
    }
}

impl Default for VendorDetector {
    fn default() -> Self {
        Self::new()
    }
}