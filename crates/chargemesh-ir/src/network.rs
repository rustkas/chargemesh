//! Charging Network model

use super::*;
use std::collections::HashMap;

/// A network of charging stations (e.g., a CPO's entire fleet)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChargingNetwork {
    /// Network identifier
    pub id: String,

    /// Network name
    pub name: String,

    /// Network operator
    pub operator: String,

    /// List of stations in the network
    pub stations: Vec<ChargingStation>,

    /// Network-wide capabilities
    pub capabilities: Capabilities,

    /// Network status
    pub status: NetworkStatus,

    /// Created at
    pub created_at: Timestamp,

    /// Updated at
    pub updated_at: Timestamp,

    /// Custom metadata
    pub metadata: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum NetworkStatus {
    Active,
    Maintenance,
    Degraded,
    Inactive,
}

impl ChargingNetwork {
    pub fn new(id: impl Into<String>, name: impl Into<String>, operator: impl Into<String>) -> Self {
        let now = now();
        Self {
            id: id.into(),
            name: name.into(),
            operator: operator.into(),
            stations: Vec::new(),
            capabilities: Capabilities::default(),
            status: NetworkStatus::Active,
            created_at: now,
            updated_at: now,
            metadata: HashMap::new(),
        }
    }

    pub fn add_station(&mut self, station: ChargingStation) {
        self.stations.push(station);
        self.updated_at = now();
    }

    pub fn get_station(&self, id: &StationId) -> Option<&ChargingStation> {
        self.stations.iter().find(|s| &s.id == id)
    }

    pub fn get_station_mut(&mut self, id: &StationId) -> Option<&mut ChargingStation> {
        self.stations.iter_mut().find(|s| &s.id == id)
    }

    pub fn remove_station(&mut self, id: &StationId) -> Option<ChargingStation> {
        if let Some(pos) = self.stations.iter().position(|s| &s.id == id) {
            self.updated_at = now();
            Some(self.stations.remove(pos))
        } else {
            None
        }
    }

    pub fn total_connectors(&self) -> usize {
        self.stations.iter().map(|s| s.total_connectors()).sum()
    }

    pub fn total_evses(&self) -> usize {
        self.stations.iter().map(|s| s.evses.len()).sum()
    }
}