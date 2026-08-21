//! EVSE Simulator

use super::*;
use crate::core::*;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct EvseSimulator {
    config: EvseSimConfig,
    stations: Vec<Station>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvseSimConfig {
    pub station_id: String,
    pub vendor: String,
    pub model: String,
    pub firmware_version: String,
    pub connectors: Vec<ConnectorConfig>,
    pub max_power: u64,
    pub has_iso15118: bool,
    pub has_v2g: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectorConfig {
    pub id: u8,
    pub connector_type: String,
    pub max_power: u64,
}

impl EvseSimulator {
    pub fn new(config: EvseSimConfig) -> Result<Self> {
        let station = Station::new(
            config.station_id.clone(),
            config.vendor.clone(),
            config.model.clone(),
        )?;

        Ok(Self {
            config,
            stations: vec![station],
        })
    }

    pub async fn get_station(&self, id: &str) -> Option<&Station> {
        self.stations.iter().find(|s| s.id == id)
    }

    pub async fn update(&self) -> Result<()> {
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Station {
    pub id: String,
    pub vendor: String,
    pub model: String,
    pub firmware_version: String,
    pub status: StationStatus,
    pub connectors: Vec<Connector>,
    pub max_power: u64,
    pub has_iso15118: bool,
    pub has_v2g: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum StationStatus {
    Offline,
    Booted,
    Enabled,
    Disabled,
    Faulted,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Connector {
    pub id: u8,
    pub connector_type: String,
    pub max_power: u64,
    pub status: ConnectorStatus,
    pub current_power: u64,
    pub transaction_id: Option<String>,
    pub meter_start: u64,
    pub meter_current: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ConnectorStatus {
    Available,
    Preparing,
    Charging,
    SuspendedEVSE,
    SuspendedEV,
    Finishing,
    Reserved,
    Unavailable,
    Faulted,
}

impl Station {
    pub fn new(id: String, vendor: String, model: String) -> Result<Self> {
        Ok(Self {
            id,
            vendor,
            model,
            firmware_version: "1.0.0".to_string(),
            status: StationStatus::Booted,
            connectors: vec![
                Connector {
                    id: 1,
                    connector_type: "Type2".to_string(),
                    max_power: 22000,
                    status: ConnectorStatus::Available,
                    current_power: 0,
                    transaction_id: None,
                    meter_start: 0,
                    meter_current: 0,
                },
                Connector {
                    id: 2,
                    connector_type: "CCS".to_string(),
                    max_power: 50000,
                    status: ConnectorStatus::Available,
                    current_power: 0,
                    transaction_id: None,
                    meter_start: 0,
                    meter_current: 0,
                },
            ],
            max_power: 50000,
            has_iso15118: true,
            has_v2g: false,
        })
    }

    pub fn start_transaction(&mut self, connector_id: u8, meter_start: u64) -> Option<String> {
        if let Some(connector) = self.connectors.iter_mut().find(|c| c.id == connector_id) {
            if connector.status == ConnectorStatus::Available {
                let transaction_id = uuid::Uuid::new_v4().to_string();
                connector.status = ConnectorStatus::Charging;
                connector.transaction_id = Some(transaction_id.clone());
                connector.meter_start = meter_start;
                connector.meter_current = meter_start;
                return Some(transaction_id);
            }
        }
        None
    }

    pub fn stop_transaction(&mut self, connector_id: u8) -> Option<u64> {
        if let Some(connector) = self.connectors.iter_mut().find(|c| c.id == connector_id) {
            if connector.transaction_id.is_some() {
                let meter_stop = connector.meter_current;
                connector.status = ConnectorStatus::Available;
                connector.transaction_id = None;
                connector.current_power = 0;
                return Some(meter_stop);
            }
        }
        None
    }

    pub fn update_meter(&mut self, connector_id: u8, energy: u64) {
        if let Some(connector) = self.connectors.iter_mut().find(|c| c.id == connector_id) {
            connector.meter_current += energy;
        }
    }
}