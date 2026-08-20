//! Identifiers for all entities in the system

use super::*;
use serde::{Deserialize, Serialize};
use std::fmt;

// ============================================================================
// Core Identifiers
// ============================================================================

/// Unique identifier (UUID v4)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Id(uuid::Uuid);

impl Id {
    /// Create a new random UUID
    pub fn new() -> Self {
        Self(uuid::Uuid::new_v4())
    }

    /// Create from a UUID
    pub fn from_uuid(uuid: uuid::Uuid) -> Self {
        Self(uuid)
    }

    /// Get the inner UUID
    pub fn as_uuid(&self) -> &uuid::Uuid {
        &self.0
    }

    /// Convert to string
    pub fn to_string(&self) -> String {
        self.0.to_string()
    }

    /// Parse from string
    pub fn parse(s: &str) -> CoreResult<Self> {
        let uuid = uuid::Uuid::parse_str(s)
            .map_err(|e| CoreError::InvalidId(e.to_string()))?;
        Ok(Self(uuid))
    }
}

impl Default for Id {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Id {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// ============================================================================
// String-based Identifiers
// ============================================================================

/// Station identifier (OCPP charge point ID)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct StationId(pub String);

impl StationId {
    /// Create a new station ID
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    /// Get the inner string
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Check if the ID is valid (non-empty)
    pub fn is_valid(&self) -> bool {
        !self.0.is_empty()
    }
}

impl fmt::Display for StationId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<&str> for StationId {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

impl From<String> for StationId {
    fn from(s: String) -> Self {
        Self(s)
    }
}

/// EVSE identifier
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct EvseId(pub String);

impl EvseId {
    /// Create a new EVSE ID
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    /// Get the inner string
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Check if the ID is valid (non-empty)
    pub fn is_valid(&self) -> bool {
        !self.0.is_empty()
    }
}

impl fmt::Display for EvseId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<&str> for EvseId {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

/// Connector identifier
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ConnectorId(pub String);

impl ConnectorId {
    /// Create a new connector ID
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    /// Get the inner string
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ConnectorId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<&str> for ConnectorId {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

/// Session identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SessionId(pub Id);

impl SessionId {
    /// Create a new session ID
    pub fn new() -> Self {
        Self(Id::new())
    }

    /// Create from a UUID
    pub fn from_uuid(uuid: uuid::Uuid) -> Self {
        Self(Id::from_uuid(uuid))
    }

    /// Get the inner ID
    pub fn as_id(&self) -> &Id {
        &self.0
    }

    /// Convert to string
    pub fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl Default for SessionId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for SessionId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Transaction identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TransactionId(pub Id);

impl TransactionId {
    /// Create a new transaction ID
    pub fn new() -> Self {
        Self(Id::new())
    }

    /// Create from a UUID
    pub fn from_uuid(uuid: uuid::Uuid) -> Self {
        Self(Id::from_uuid(uuid))
    }

    /// Get the inner ID
    pub fn as_id(&self) -> &Id {
        &self.0
    }
}

impl Default for TransactionId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for TransactionId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}