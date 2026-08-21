//! Protocol-based capability detector

use super::*;
use crate::engine::*;

pub struct ProtocolDetector;

impl ProtocolDetector {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl CapabilityDetector for ProtocolDetector {
    async fn detect(&self, context: &CapabilityContext) -> Result<CapabilitySet> {
        let mut caps = CapabilitySet::new();

        match context.protocol.name {
            ProtocolName::OCPP => {
                self.detect_ocpp(&context.protocol.version, &mut caps);
            }
            ProtocolName::ISO15118 => {
                self.detect_iso15118(&context.protocol.version, &mut caps);
            }
            ProtocolName::OCPI => {
                caps.add_capability(
                    CapabilityType::OCPI,
                    CapabilityState::Supported { parameters: Default::default() }
                );
                caps.add_capability(
                    CapabilityType::OCPIRoaming,
                    CapabilityState::Supported { parameters: Default::default() }
                );
            }
            _ => {}
        }

        // Always add basic charging
        caps.add_capability(
            CapabilityType::BasicCharging,
            CapabilityState::Supported { parameters: Default::default() }
        );

        Ok(caps)
    }
}

impl ProtocolDetector {
    fn detect_ocpp(&self, version: &str, caps: &mut CapabilitySet) {
        // Check for specific versions
        if version.starts_with("1.6") {
            caps.add_capability(
                CapabilityType::OCPP1_6,
                CapabilityState::Supported { parameters: Default::default() }
            );
        }

        if version.starts_with("2.0") || version.starts_with("2.0.1") {
            caps.add_capability(
                CapabilityType::OCPP2_0_1,
                CapabilityState::Supported { parameters: Default::default() }
            );
            caps.add_capability(
                CapabilityType::SmartCharging,
                CapabilityState::Supported { parameters: Default::default() }
            );
            caps.add_capability(
                CapabilityType::Reservation,
                CapabilityState::Supported { parameters: Default::default() }
            );
            caps.add_capability(
                CapabilityType::ConfigurationManagement,
                CapabilityState::Supported { parameters: Default::default() }
            );
        }

        if version.starts_with("2.1") {
            caps.add_capability(
                CapabilityType::OCPP2_1,
                CapabilityState::Supported { parameters: Default::default() }
            );
            caps.add_capability(
                CapabilityType::V2G,
                CapabilityState::Supported { parameters: Default::default() }
            );
            caps.add_capability(
                CapabilityType::ExternalConstraints,
                CapabilityState::Supported { parameters: Default::default() }
            );
            caps.add_capability(
                CapabilityType::LocalGeneration,
                CapabilityState::Supported { parameters: Default::default() }
            );
        }

        // Remote capabilities (available in all OCPP versions)
        caps.add_capability(
            CapabilityType::RemoteDiagnostics,
            CapabilityState::Supported { parameters: Default::default() }
        );
        caps.add_capability(
            CapabilityType::RemoteReset,
            CapabilityState::Supported { parameters: Default::default() }
        );
    }

    fn detect_iso15118(&self, version: &str, caps: &mut CapabilitySet) {
        caps.add_capability(
            CapabilityType::ISO15118,
            CapabilityState::Supported { parameters: Default::default() }
        );

        if version.starts_with("2.0") {
            caps.add_capability(
                CapabilityType::PlugAndCharge,
                CapabilityState::Supported { parameters: Default::default() }
            );
            caps.add_capability(
                CapabilityType::SignedMetering,
                CapabilityState::Supported { parameters: Default::default() }
            );
        }

        if version.starts_with("2.0") || version.starts_with("2.0.20") {
            // ISO 15118-20 adds V2G
            caps.add_capability(
                CapabilityType::ISO15118_20,
                CapabilityState::Supported { parameters: Default::default() }
            );
        }
    }
}

impl Default for ProtocolDetector {
    fn default() -> Self {
        Self::new()
    }
}