//! Capability detectors

mod protocol_detector;
mod vendor_detector;
mod firmware_detector;
mod runtime_detector;

pub use protocol_detector::*;
pub use vendor_detector::*;
pub use firmware_detector::*;
pub use runtime_detector::*;

use super::*;
use async_trait::async_trait;

#[async_trait]
pub trait CapabilityDetector: Send + Sync {
    async fn detect(&self, context: &CapabilityContext) -> Result<CapabilitySet>;
}