//! Tariff model

use super::*;
use std::collections::HashMap;

/// Tariff (pricing model)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tariff {
    pub id: String,
    pub name: String,
    pub currency: String,
    pub energy_price: f64,         // Price per kWh (major units)
    pub time_price: Option<f64>,   // Price per minute
    pub parking_price: Option<f64>, // Price per minute for parking
    pub tariff_type: TariffType,
    pub time_of_day: Option<Vec<TimeOfDayPrice>>,
    pub applicable_days: Option<Vec<u8>>, // 0=Monday, 6=Sunday
    pub start_time: Option<Timestamp>,
    pub end_time: Option<Timestamp>,
    pub conditions: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TariffType {
    Flat,
    TimeOfDay,
    Tiered,
    Dynamic,
    UserDefined,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeOfDayPrice {
    pub start: String,            // "HH:MM"
    pub end: String,              // "HH:MM"
    pub energy_price: f64,
    pub time_price: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TieredPrice {
    pub name: String,
    pub start: f64,               // kWh
    pub end: f64,                 // kWh
    pub price: f64,               // Price per kWh
}

impl Default for Tariff {
    fn default() -> Self {
        Self {
            id: "default".to_string(),
            name: "Default Tariff".to_string(),
            currency: "USD".to_string(),
            energy_price: 0.15,
            time_price: None,
            parking_price: None,
            tariff_type: TariffType::Flat,
            time_of_day: None,
            applicable_days: None,
            start_time: None,
            end_time: None,
            conditions: None,
        }
    }
}