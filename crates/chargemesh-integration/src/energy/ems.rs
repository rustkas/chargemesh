//! Energy Management System (EMS) integration

use super::*;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnergyManagementSystem {
    pub id: String,
    pub name: String,
    pub status: EnergyManagementStatus,
    pub available_power: f64,
    pub current_load: f64,
    pub peak_load: f64,
    pub grid_import: f64,
    pub solar_generation: Option<f64>,
    pub battery_state: Option<BatteryState>,
    pub price_signal: Option<EnergyPrice>,
    pub constraints: Vec<EnergyConstraint>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatteryState {
    pub soc: f64,
    pub capacity: f64,
    pub charging: bool,
    pub power: f64,
}

pub struct EmsIntegration {
    ems: Arc<tokio::sync::RwLock<Option<EnergyManagementSystem>>>,
}

impl EmsIntegration {
    pub fn new() -> Self {
        Self {
            ems: Arc::new(tokio::sync::RwLock::new(None)),
        }
    }

    pub async fn connect(&self, ems: EnergyManagementSystem) -> Result<()> {
        let mut current = self.ems.write().await;
        *current = Some(ems);
        Ok(())
    }

    pub async fn get_status(&self) -> Option<EnergyManagementSystem> {
        let ems = self.ems.read().await;
        ems.clone()
    }

    pub async fn get_available_power(&self) -> f64 {
        let ems = self.ems.read().await;
        ems.as_ref().map(|e| e.available_power).unwrap_or(0.0)
    }

    pub async fn get_current_price(&self) -> Option<f64> {
        let ems = self.ems.read().await;
        ems.as_ref().and_then(|e| e.price_signal.as_ref()).map(|p| p.price)
    }

    pub async fn get_constraints(&self) -> Vec<EnergyConstraint> {
        let ems = self.ems.read().await;
        ems.as_ref().map(|e| e.constraints.clone()).unwrap_or_default()
    }
}

impl Default for EmsIntegration {
    fn default() -> Self {
        Self::new()
    }
}