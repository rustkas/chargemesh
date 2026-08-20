//! Configuration utilities

use serde::{Deserialize, Serialize};

/// Configuration trait for components
pub trait Configurable: Sized {
    /// Load configuration from a source
    fn load() -> CoreResult<Self>;

    /// Load configuration from a specific source
    fn load_from(source: &str) -> CoreResult<Self>;

    /// Validate configuration
    fn validate(&self) -> CoreResult<()>;
}

/// Configuration source
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConfigSource {
    File(String),
    Env,
    String(String),
    Default,
}

impl ConfigSource {
    pub fn read(&self) -> CoreResult<String> {
        match self {
            ConfigSource::File(path) => {
                std::fs::read_to_string(path)
                    .map_err(|e| CoreError::InvalidConfig(format!("Failed to read {}: {}", path, e)))
            }
            ConfigSource::Env => {
                // Read from environment variable CONFIG
                std::env::var("CHARGEMESH_CONFIG")
                    .map_err(|_| CoreError::InvalidConfig("CHARGEMESH_CONFIG not set".to_string()))
            }
            ConfigSource::String(s) => Ok(s.clone()),
            ConfigSource::Default => {
                // Return default configuration as JSON
                Ok("{}".to_string())
            }
        }
    }
}