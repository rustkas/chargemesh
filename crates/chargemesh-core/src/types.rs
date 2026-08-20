//! Core types for the ChargeMesh platform

use serde::{Deserialize, Serialize};
use std::fmt;

// ============================================================================
// Power
// ============================================================================

/// Power in watts
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Power(pub u64);

impl Power {
    /// Create a new power value in watts
    pub fn new(watts: u64) -> Self {
        Self(watts)
    }

    /// Get power in watts
    pub fn as_watts(&self) -> u64 {
        self.0
    }

    /// Get power in kilowatts (as f64)
    pub fn as_kw(&self) -> f64 {
        self.0 as f64 / 1000.0
    }

    /// Get power in megawatts (as f64)
    pub fn as_mw(&self) -> f64 {
        self.0 as f64 / 1_000_000.0
    }
}

impl fmt::Display for Power {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.0 >= 1_000_000 {
            write!(f, "{:.2} MW", self.as_mw())
        } else if self.0 >= 1000 {
            write!(f, "{:.2} kW", self.as_kw())
        } else {
            write!(f, "{} W", self.0)
        }
    }
}

impl Default for Power {
    fn default() -> Self {
        Self(0)
    }
}

// ============================================================================
// Energy
// ============================================================================

/// Energy in watt-hours
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Energy(pub u64);

impl Energy {
    /// Create a new energy value in watt-hours
    pub fn new(wh: u64) -> Self {
        Self(wh)
    }

    /// Get energy in watt-hours
    pub fn as_wh(&self) -> u64 {
        self.0
    }

    /// Get energy in kilowatt-hours (as f64)
    pub fn as_kwh(&self) -> f64 {
        self.0 as f64 / 1000.0
    }

    /// Get energy in megawatt-hours (as f64)
    pub fn as_mwh(&self) -> f64 {
        self.0 as f64 / 1_000_000.0
    }
}

impl fmt::Display for Energy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.0 >= 1_000_000 {
            write!(f, "{:.2} MWh", self.as_mwh())
        } else if self.0 >= 1000 {
            write!(f, "{:.2} kWh", self.as_kwh())
        } else {
            write!(f, "{} Wh", self.0)
        }
    }
}

impl Default for Energy {
    fn default() -> Self {
        Self(0)
    }
}

// ============================================================================
// Duration
// ============================================================================

/// Duration in seconds
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Duration(pub u64);

impl Duration {
    /// Create a new duration in seconds
    pub fn new(seconds: u64) -> Self {
        Self(seconds)
    }

    /// Get duration in seconds
    pub fn as_seconds(&self) -> u64 {
        self.0
    }

    /// Get duration in minutes (as f64)
    pub fn as_minutes(&self) -> f64 {
        self.0 as f64 / 60.0
    }

    /// Get duration in hours (as f64)
    pub fn as_hours(&self) -> f64 {
        self.0 as f64 / 3600.0
    }
}

impl fmt::Display for Duration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.0 >= 3600 {
            let hours = self.0 / 3600;
            let minutes = (self.0 % 3600) / 60;
            write!(f, "{}h {}m", hours, minutes)
        } else if self.0 >= 60 {
            let minutes = self.0 / 60;
            let seconds = self.0 % 60;
            write!(f, "{}m {}s", minutes, seconds)
        } else {
            write!(f, "{}s", self.0)
        }
    }
}

// ============================================================================
// Temperature
// ============================================================================

/// Temperature in degrees Celsius
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Temperature(pub f32);

impl Temperature {
    /// Create a new temperature in Celsius
    pub fn new(celsius: f32) -> Self {
        Self(celsius)
    }

    /// Get temperature in Celsius
    pub fn as_celsius(&self) -> f32 {
        self.0
    }

    /// Get temperature in Fahrenheit
    pub fn as_fahrenheit(&self) -> f32 {
        self.0 * 1.8 + 32.0
    }

    /// Get temperature in Kelvin
    pub fn as_kelvin(&self) -> f32 {
        self.0 + 273.15
    }
}

impl fmt::Display for Temperature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.1}°C", self.0)
    }
}

// ============================================================================
// Percentage
// ============================================================================

/// Percentage (0-100)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Percentage(pub u8);

impl Percentage {
    /// Create a new percentage (clamped to 0-100)
    pub fn new(value: u8) -> Self {
        Self(value.min(100))
    }

    /// Get percentage value
    pub fn as_u8(&self) -> u8 {
        self.0
    }

    /// Get percentage as f64 (0.0 - 1.0)
    pub fn as_ratio(&self) -> f64 {
        self.0 as f64 / 100.0
    }

    /// Get percentage as f64 (0.0 - 100.0)
    pub fn as_f64(&self) -> f64 {
        self.0 as f64
    }
}

impl fmt::Display for Percentage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}%", self.0)
    }
}

// ============================================================================
// Money
// ============================================================================

/// Money in minor units (e.g., cents)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Money(pub i64);

impl Money {
    /// Create a new money value in minor units
    pub fn new(minor_units: i64) -> Self {
        Self(minor_units)
    }

    /// Get money in minor units
    pub fn as_minor_units(&self) -> i64 {
        self.0
    }

    /// Get money as f64 (major units)
    pub fn as_major_units(&self) -> f64 {
        self.0 as f64 / 100.0
    }
}

impl fmt::Display for Money {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "${:.2}", self.as_major_units())
    }
}