//! Vehicle model

use super::*;
use std::collections::HashMap;

/// Electric Vehicle
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vehicle {
    pub id: String,
    pub make: Option<String>,
    pub model: Option<String>,
    pub model_year: Option<u16>,
    pub battery_capacity: Option<Energy>,
    pub state_of_charge: Option<Percentage>,
    pub target_state_of_charge: Option<Percentage>,
    pub vin: Option<String>,
    pub license_plate: Option<String>,
    pub supported_connectors: Vec<ConnectorType>,
    pub iso_certificate: Option<IsoCertificate>,
    pub metadata: HashMap<String, serde_json::Value>,
}

/// ISO 15118 certificate
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IsoCertificate {
    pub contract_id: String,
    pub issuer: String,
    pub valid_from: Timestamp,
    pub valid_to: Timestamp,
    pub certificate_chain: Vec<String>,  // PEM encoded
}

impl Vehicle {
    pub fn new(id: String) -> Self {
        Self {
            id,
            make: None,
            model: None,
            model_year: None,
            battery_capacity: None,
            state_of_charge: None,
            target_state_of_charge: None,
            vin: None,
            license_plate: None,
            supported_connectors: Vec::new(),
            iso_certificate: None,
            metadata: HashMap::new(),
        }
    }

    pub fn supports_plug_and_charge(&self) -> bool {
        self.iso_certificate.is_some()
    }

    pub fn required_energy(&self) -> Option<Energy> {
        if let (Some(capacity), Some(current_soc), Some(target_soc)) = (
            self.battery_capacity,
            self.state_of_charge,
            self.target_state_of_charge,
        ) {
            if target_soc > current_soc {
                let diff = (target_soc.0 - current_soc.0) as f64 / 100.0;
                Some(Energy::new((capacity.0 as f64 * diff) as u64))
            } else {
                Some(Energy::new(0))
            }
        } else {
            None
        }
    }
}