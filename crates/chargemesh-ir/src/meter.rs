//! Meter models

use super::*;

/// Physical energy meter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Meter {
    pub id: String,
    pub manufacturer: String,
    pub model: String,
    pub serial_number: String,
    pub current_reading: MeterValue,
    pub last_update: Timestamp,
    pub calibration_date: Option<Timestamp>,
    pub accuracy_class: Option<String>,
}

/// Meter value (reading)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeterValue {
    pub timestamp: Timestamp,
    pub energy_import: Energy,     // Energy consumed from grid
    pub energy_export: Energy,     // Energy exported to grid (V2G)
    pub power: Option<Power>,
    pub voltage: Option<f64>,
    pub current: Option<f64>,
    pub power_factor: Option<f64>,
    pub frequency: Option<f64>,
    pub signed: Option<SignedMeterValue>,
}

/// Signed meter value (ISO 15118)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignedMeterValue {
    pub signature: String,         // Base64 encoded signature
    pub algorithm: String,         // Signing algorithm
    pub certificate_chain: Vec<String>, // PEM encoded certificates
}

impl MeterValue {
    pub fn new(energy_import: Energy) -> Self {
        Self {
            timestamp: now(),
            energy_import,
            energy_export: Energy::new(0),
            power: None,
            voltage: None,
            current: None,
            power_factor: None,
            frequency: None,
            signed: None,
        }
    }

    pub fn with_power(mut self, power: Power, voltage: f64, current: f64) -> Self {
        self.power = Some(power);
        self.voltage = Some(voltage);
        self.current = Some(current);
        self
    }

    pub fn signed(mut self, signature: String, algorithm: String, certificates: Vec<String>) -> Self {
        self.signed = Some(SignedMeterValue {
            signature,
            algorithm,
            certificate_chain: certificates,
        });
        self
    }

    pub fn is_signed(&self) -> bool {
        self.signed.is_some()
    }
}