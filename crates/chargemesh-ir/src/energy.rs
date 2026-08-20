//! Energy and constraints model

use super::*;

/// Energy constraint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnergyConstraint {
    pub max_power: Power,
    pub min_power: Option<Power>,
    pub max_energy: Option<Energy>,
    pub carbon_intensity: Option<f64>,  // g CO2/kWh
    pub source: EnergySource,
    pub constraint_type: ConstraintType,
    pub time_range: Option<TimeRange>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum EnergySource {
    Grid,
    Solar,
    Wind,
    Hydro,
    Battery,
    Mixed,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ConstraintType {
    Absolute,
    Dynamic,
    Periodic,
    Triggered,
}

/// Grid status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GridStatus {
    pub load: Percentage,
    pub available_capacity: Power,
    pub peak_load: Power,
    pub current_price: Money,
    pub frequency: f64,
    pub timestamp: Timestamp,
    pub alerts: Vec<GridAlert>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GridAlert {
    pub alert_type: GridAlertType,
    pub severity: ErrorSeverity,
    pub description: String,
    pub timestamp: Timestamp,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum GridAlertType {
    Overload,
    UnderVoltage,
    OverVoltage,
    FrequencyDeviation,
    DemandResponse,
    ScheduledMaintenance,
}