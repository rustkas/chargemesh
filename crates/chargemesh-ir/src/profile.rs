//! Charging Profile model

use super::*;
use std::collections::HashMap;

/// Charging profile (smart charging)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChargingProfile {
    pub id: Id,
    pub name: Option<String>,
    pub profile_type: ProfileType,
    pub schedule: Vec<ChargingPeriod>,
    pub max_power: Option<Power>,
    pub min_power: Option<Power>,
    pub target_soc: Option<Percentage>,
    pub departure_time: Option<Timestamp>,
    pub energy_constraint: Option<EnergyConstraint>,
    pub start_time: Option<Timestamp>,
    pub end_time: Option<Timestamp>,
    pub priority: Option<u8>,
    pub parameters: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ProfileType {
    Slow,
    Normal,
    Fast,
    Eco,
    BatterySaver,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChargingPeriod {
    pub id: Id,
    pub start_time: Option<Timestamp>,
    pub duration: chrono::Duration,
    pub power_limit: Power,
    pub current_limit: Option<f64>,
    pub voltage: Option<f64>,
    pub energy_limit: Option<Energy>,
    pub time_of_day: Option<TimeOfDaySchedule>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeOfDaySchedule {
    pub start: String,        // "HH:MM"
    pub end: String,          // "HH:MM"
    pub days: Vec<u8>,        // 0=Monday, 6=Sunday
}

impl Default for ChargingProfile {
    fn default() -> Self {
        Self {
            id: Id::new(),
            name: None,
            profile_type: ProfileType::Normal,
            schedule: Vec::new(),
            max_power: None,
            min_power: None,
            target_soc: None,
            departure_time: None,
            energy_constraint: None,
            start_time: None,
            end_time: None,
            priority: None,
            parameters: HashMap::new(),
        }
    }
}