//! Capability resolver — determines base capabilities from protocol and vendor

use super::*;
use crate::profiles::*;
use crate::detectors::*;

pub struct CapabilityResolver {
    protocol_detector: ProtocolDetector,
    vendor_detector: VendorDetector,
    firmware_detector: FirmwareDetector,
    profile_registry: ModelProfileRegistry,
}

impl CapabilityResolver {
    pub fn new() -> Self {
        let mut resolver = Self {
            protocol_detector: ProtocolDetector::new(),
            vendor_detector: VendorDetector::new(),
            firmware_detector: FirmwareDetector::new(),
            profile_registry: ModelProfileRegistry::new(),
        };

        // Load default profiles
        resolver.load_default_profiles();

        resolver
    }

    fn load_default_profiles(&mut self) {
        // ABB Terra 54
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
            CapabilityType::OCPP1_6,
            CapabilityState::Supported { parameters: Default::default() }
        )
        .with_capability(
            CapabilityType::OCPP2_0_1,
            CapabilityState::Supported { parameters: Default::default() }
        )
        .with_capability(
            CapabilityType::SmartCharging,
            CapabilityState::Supported { parameters: Default::default() }
        )
        .with_capability(
            CapabilityType::Reservation,
            CapabilityState::Supported { parameters: Default::default() }
        )
        .with_capability(
            CapabilityType::RemoteDiagnostics,
            CapabilityState::Supported { parameters: Default::default() }
        );

        self.profile_registry.register(profile);

        // ABB Terra HP (high power)
        let profile = ModelProfile::new(
            VendorInfo {
                name: "ABB".to_string(),
                id: Some("ABB".to_string()),
                known_models: vec![
                    "Terra HP".to_string(),
                    "Terra 184".to_string(),
                ],
            },
            "Terra HP".to_string(),
        )
        .with_capability(
            CapabilityType::BasicCharging,
            CapabilityState::Supported { parameters: Default::default() }
        )
        .with_capability(
            CapabilityType::FastCharging,
            CapabilityState::Supported { parameters: Default::default() }
        )
        .with_capability(
            CapabilityType::OCPP2_1,
            CapabilityState::Supported { parameters: Default::default() }
        )
        .with_capability(
            CapabilityType::ISO15118,
            CapabilityState::Supported { parameters: Default::default() }
        )
        .with_capability(
            CapabilityType::PlugAndCharge,
            CapabilityState::Supported { parameters: Default::default() }
        )
        .with_capability(
            CapabilityType::V2G,
            CapabilityState::Supported { parameters: Default::default() }
        );

        self.profile_registry.register(profile);

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
        )
        .with_capability(
            CapabilityType::SmartCharging,
            CapabilityState::Supported { parameters: Default::default() }
        );

        self.profile_registry.register(profile);
    }

    /// Resolve base capabilities from context
    pub async fn resolve_base(
        &self,
        context: &CapabilityContext,
    ) -> Result<CapabilitySet> {
        let mut capabilities = CapabilitySet::new();

        // Step 1: Protocol-based capabilities
        let protocol_caps = self.protocol_detector.detect(context).await?;
        self.merge_capabilities(&mut capabilities, protocol_caps);

        // Step 2: Vendor/model profile
        if let Some(profile) = self.profile_registry.get_profile(
            &context.vendor.name,
            &context.model,
        ) {
            self.merge_capabilities(&mut capabilities, profile.capabilities.clone());
        }

        // Step 3: Firmware version capabilities
        let firmware_caps = self.firmware_detector.detect(context).await?;
        self.merge_capabilities(&mut capabilities, firmware_caps);

        // Step 4: Apply configuration overrides
        self.apply_configuration(&mut capabilities, context);

        // Step 5: Set detection source
        capabilities.source = CapabilitySource::ProtocolDiscovery;
        capabilities.detected_at = chrono::Utc::now();

        Ok(capabilities)
    }

    fn merge_capabilities(
        &self,
        target: &mut CapabilitySet,
        source: CapabilitySet,
    ) {
        for (cap, state) in source.capabilities {
            // Don't override existing capabilities unless source is more specific
            if !target.has_capability(&cap) {
                target.add_capability(cap, state);
            }
        }
    }

    fn apply_configuration(
        &self,
        capabilities: &mut CapabilitySet,
        context: &CapabilityContext,
    ) {
        // Check configuration for capability overrides
        for (key, value) in &context.configuration {
            if let Ok(cap_type) = self.parse_capability_key(key) {
                if let Some(enabled) = value.as_bool() {
                    if !enabled {
                        capabilities.set_capability(
                            cap_type,
                            CapabilityState::NotSupported {
                                reason: Some("Disabled in configuration".to_string()),
                            }
                        );
                    }
                }
            }
        }
    }

    fn parse_capability_key(&self, key: &str) -> Result<CapabilityType> {
        match key {
            "smartCharging" => Ok(CapabilityType::SmartCharging),
            "v2g" => Ok(CapabilityType::V2G),
            "iso15118" => Ok(CapabilityType::ISO15118),
            "plugAndCharge" => Ok(CapabilityType::PlugAndCharge),
            "reservation" => Ok(CapabilityType::Reservation),
            "remoteDiagnostics" => Ok(CapabilityType::RemoteDiagnostics),
            "remoteFirmwareUpdate" => Ok(CapabilityType::RemoteFirmwareUpdate),
            "loadBalancing" => Ok(CapabilityType::LoadBalancing),
            "bidirectional" => Ok(CapabilityType::Bidirectional),
            "signedMetering" => Ok(CapabilityType::SignedMetering),
            _ => Err(CapabilityError::CapabilityNotSupported(key.to_string())),
        }
    }
}

impl Default for CapabilityResolver {
    fn default() -> Self {
        Self::new()
    }
}