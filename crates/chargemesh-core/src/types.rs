//! Core types

use serde::{Deserialize, Serialize};
use std::fmt;

/// Unique identifier (UUID v4)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Id(uuid::Uuid);

impl Id {
    pub fn new() -> Self {
        Self(uuid::Uuid::new_v4())
    }
    
    pub fn as_uuid(&self) -> &uuid::Uuid {
        &self.0
    }
    
    pub fn to_string(&self) -> String {
        self.0.to_string()
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

/// Power in watts
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Power(pub u64);

impl Power {
    pub fn new(watts: u64) -> Self {
        Self(watts)
    }
    
    pub fn as_watts(&self) -> u64 {
        self.0
    }
    
    pub fn as_kw(&self) -> f64 {
        self.0 as f64 / 1000.0
    }
}

/// Energy in watt-hours
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Energy(pub u64);

impl Energy {
    pub fn new(wh: u64) -> Self {
        Self(wh)
    }
    
    pub fn as_wh(&self) -> u64 {
        self.0
    }
    
    pub fn as_kwh(&self) -> f64 {
        self.0 as f64 / 1000.0
    }
}

/// Timestamp (ISO 8601 with UTC)
pub type Timestamp = chrono::DateTime<chrono::Utc>;

/// Duration in seconds
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Duration(pub u64);

impl Duration {
    pub fn new(seconds: u64) -> Self {
        Self(seconds)
    }
    
    pub fn as_seconds(&self) -> u64 {
        self.0
    }
    
    pub fn as_minutes(&self) -> f64 {
        self.0 as f64 / 60.0
    }
}
