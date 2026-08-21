//! Core simulator components

mod simulator;
mod scenario;
mod event;
mod time;

pub use simulator::*;
pub use scenario::*;
pub use event::*;
pub use time::*;

use super::*;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Main simulator instance
pub struct Simulator {
    config: SimulatorConfig,
    status: Arc<RwLock<SimulationStatus>>,
    ev_sim: Option<Arc<ev::EvSimulator>>,
    evse_sim: Option<Arc<evse::EvseSimulator>>,
    csms_sim: Option<Arc<csms::CsmsSimulator>>,
    ocpi_sim: Option<Arc<ocpi::OcpiSimulator>>,
    grid_sim: Option<Arc<grid::GridSimulator>>,
    fault_injector: Arc<faults::FaultInjector>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulatorConfig {
    pub mode: SimulationMode,
    pub speed: f32,
    pub seed: Option<u64>,
    pub max_duration: Option<chrono::Duration>,
    pub verbose: bool,
    pub protocol: String,
    pub station_config: StationSimConfig,
    pub ev_config: EvSimConfig,
    pub grid_config: GridSimConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StationSimConfig {
    pub vendor: String,
    pub model: String,
    pub firmware_version: String,
    pub connector_count: u8,
    pub max_power: u64,
    pub has_iso15118: bool,
    pub has_v2g: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvSimConfig {
    pub battery_capacity: u64,
    pub initial_soc: u8,
    pub target_soc: u8,
    pub supports_plug_and_charge: bool,
    pub supports_v2g: bool,
    pub max_power: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GridSimConfig {
    pub available_capacity: u64,
    pub max_power: u64,
    pub grid_load_percentage: u8,
    pub has_solar: bool,
    pub has_battery_storage: bool,
}

impl Simulator {
    pub fn new(config: SimulatorConfig) -> Self {
        Self {
            config,
            status: Arc::new(RwLock::new(SimulationStatus::Pending)),
            ev_sim: None,
            evse_sim: None,
            csms_sim: None,
            ocpi_sim: None,
            grid_sim: None,
            fault_injector: Arc::new(faults::FaultInjector::new()),
        }
    }

    pub async fn start(&mut self) -> Result<()> {
        *self.status.write().await = SimulationStatus::Running;
        self.initialize_simulators().await?;
        self.run_simulation().await?;
        Ok(())
    }

    pub async fn stop(&self) -> Result<()> {
        *self.status.write().await = SimulationStatus::Completed;
        Ok(())
    }

    pub async fn pause(&self) -> Result<()> {
        *self.status.write().await = SimulationStatus::Paused;
        Ok(())
    }

    pub async fn resume(&self) -> Result<()> {
        *self.status.write().await = SimulationStatus::Running;
        Ok(())
    }

    async fn initialize_simulators(&mut self) -> Result<()> {
        let evse_config = evse::EvseSimConfig::from(&self.config.station_config);
        self.evse_sim = Some(Arc::new(evse::EvseSimulator::new(evse_config)?));

        self.ev_sim = Some(Arc::new(ev::EvSimulator::new(self.config.ev_config.clone())?));

        self.csms_sim = Some(Arc::new(csms::CsmsSimulator::new()?));

        self.grid_sim = Some(Arc::new(grid::GridSimulator::new(self.config.grid_config.clone())?));

        if self.config.protocol.contains("ocpi") {
            self.ocpi_sim = Some(Arc::new(ocpi::OcpiSimulator::new()?));
        }

        Ok(())
    }

    async fn run_simulation(&mut self) -> Result<()> {
        let start_time = chrono::Utc::now();

        while *self.status.read().await == SimulationStatus::Running {
            if let Some(max_duration) = self.config.max_duration {
                if chrono::Utc::now() - start_time > max_duration {
                    self.stop().await?;
                    break;
                }
            }

            self.simulation_step().await?;

            let delay = (100 / self.config.speed as u64).max(1);
            tokio::time::sleep(tokio::time::Duration::from_millis(delay)).await;
        }

        Ok(())
    }

    async fn simulation_step(&mut self) -> Result<()> {
        if let Some(evse) = &self.evse_sim {
            evse.update().await?;
        }

        if let Some(ev) = &self.ev_sim {
            ev.update().await?;
        }

        if let Some(grid) = &self.grid_sim {
            grid.update().await?;
        }

        self.process_events().await?;
        Ok(())
    }

    async fn process_events(&self) -> Result<()> {
        Ok(())
    }
}

impl Default for Simulator {
    fn default() -> Self {
        Self::new(SimulatorConfig {
            mode: SimulationMode::Normal,
            speed: 1.0,
            seed: None,
            max_duration: None,
            verbose: false,
            protocol: "ocpp-1.6".to_string(),
            station_config: StationSimConfig {
                vendor: "ABB".to_string(),
                model: "Terra 54".to_string(),
                firmware_version: "1.2.3".to_string(),
                connector_count: 2,
                max_power: 50000,
                has_iso15118: true,
                has_v2g: false,
            },
            ev_config: EvSimConfig {
                battery_capacity: 75000,
                initial_soc: 20,
                target_soc: 80,
                supports_plug_and_charge: true,
                supports_v2g: false,
                max_power: 22000,
            },
            grid_config: GridSimConfig {
                available_capacity: 100000,
                max_power: 100000,
                grid_load_percentage: 50,
                has_solar: true,
                has_battery_storage: false,
            },
        })
    }
}