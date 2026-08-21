//! Energy management integration

mod ems;
mod der;
mod bess;
mod grid;
mod v2g;

pub use ems::*;
pub use der::*;
pub use bess::*;
pub use grid::*;
pub use v2g::*;

use super::*;

/// Energy source type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum EnergySource {
    Grid,
    Solar,
    Wind,
    Hydro,
    Battery,
    Diesel,
    Other,
}

/// Energy price signal
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnergyPrice {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub price: f64,
    pub currency: String,
    pub source: EnergySource,
}

/// Energy constraint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnergyConstraint {
    pub max_power: f64,
    pub min_power: f64,
    pub max_energy: f64,
    pub carbon_intensity: Option<f64>,
    pub source: EnergySource,
    pub time_range: Option<TimeRange>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeRange {
    pub start: chrono::DateTime<chrono::Utc>,
    pub end: chrono::DateTime<chrono::Utc>,
}

/// Energy management status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum EnergyManagementStatus {
    Normal,
    PeakShaving,
    LoadShedding,
    Emergency,
    Faulted,
}