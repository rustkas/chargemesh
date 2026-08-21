//! Smart Charging implementation

mod optimizer;
mod scheduler;
mod constraints;
mod algorithms;

pub use optimizer::*;
pub use scheduler::*;
pub use constraints::*;
pub use algorithms::*;

use super::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmartChargingConfig {
    pub enabled: bool,
    pub algorithm: SmartChargingAlgorithm,
    pub optimization_target: OptimizationTarget,
    pub constraints: Vec<ChargingConstraint>,
    pub update_interval: chrono::Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum SmartChargingAlgorithm {
    Greedy,
    LinearProgramming,
    DynamicProgramming,
    GeneticAlgorithm,
    RuleBased,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum OptimizationTarget {
    MinimizeCost,
    MinimizeCarbon,
    MaximizeRenewable,
    MinimizePeakLoad,
    Balanced,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChargingConstraint {
    pub constraint_type: ConstraintType,
    pub max_power: Option<f64>,
    pub min_power: Option<f64>,
    pub max_current: Option<f64>,
    pub time_window: Option<TimeWindow>,
    pub priority: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ConstraintType {
    GridCapacity,
    StationCapacity,
    EVCapability,
    TimeOfUse,
    CarbonIntensity,
    UserPreference,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeWindow {
    pub start: chrono::DateTime<chrono::Utc>,
    pub end: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChargingPlan {
    pub id: String,
    pub session_id: String,
    pub schedule: Vec<ChargingSlot>,
    pub total_energy: f64,
    pub total_cost: f64,
    pub carbon_emissions: f64,
    pub optimization_target: OptimizationTarget,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChargingSlot {
    pub start_time: chrono::DateTime<chrono::Utc>,
    pub end_time: chrono::DateTime<chrono::Utc>,
    pub power: f64,
    pub energy: f64,
    pub source: EnergySource,
}