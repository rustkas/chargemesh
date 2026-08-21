//! EV Simulator

use super::*;
use crate::core::*;
use crate::ev::battery::*;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct EvSimulator {
    config: EvSimConfig,
    battery: Arc<RwLock<Battery>>,
    state: Arc<RwLock<EvState>>,
    iso15118: Option<Iso15118Handler>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvState {
    pub connected: bool,
    pub authorized: bool,
    pub charging: bool,
    pub v2g_active: bool,
    pub current_power: u64,
    pub total_energy_delivered: u64,
    pub soc: u8,
    pub error: Option<String>,
    pub last_update: chrono::DateTime<chrono::Utc>,
}

pub struct Iso15118Handler {
    pub handshake_complete: bool,
    pub certificate_valid: bool,
    pub contract_id: Option<String>,
}

impl Iso15118Handler {
    pub fn new() -> Self {
        Self {
            handshake_complete: false,
            certificate_valid: true,
            contract_id: None,
        }
    }

    pub async fn handshake(&self) -> Result<(), String> {
        Ok(())
    }

    pub async fn plug_and_charge(&self, _certificate: &str) -> Result<bool, String> {
        Ok(true)
    }
}

impl EvSimulator {
    pub fn new(config: EvSimConfig) -> Result<Self> {
        let battery = Battery::new(config.battery_capacity, config.initial_soc);
        let iso15118 = if config.supports_plug_and_charge {
            Some(Iso15118Handler::new())
        } else {
            None
        };

        Ok(Self {
            config,
            battery: Arc::new(RwLock::new(battery)),
            state: Arc::new(RwLock::new(EvState {
                connected: false,
                authorized: false,
                charging: false,
                v2g_active: false,
                current_power: 0,
                total_energy_delivered: 0,
                soc: config.initial_soc,
                error: None,
                last_update: chrono::Utc::now(),
            })),
            iso15118,
        })
    }

    pub async fn connect(&self) -> Result<()> {
        let mut state = self.state.write().await;
        state.connected = true;
        state.last_update = chrono::Utc::now();

        if let Some(iso) = &self.iso15118 {
            iso.handshake().await
                .map_err(|e| SimulatorError::Simulation(e))?;
        }

        Ok(())
    }

    pub async fn disconnect(&self) -> Result<()> {
        let mut state = self.state.write().await;
        state.connected = false;
        state.authorized = false;
        state.charging = false;
        state.v2g_active = false;
        state.current_power = 0;
        state.last_update = chrono::Utc::now();
        Ok(())
    }

    pub async fn authorize(&self, token: &str) -> Result<bool> {
        let mut state = self.state.write().await;
        let authorized = token.starts_with("RFID-") || token.starts_with("V2G-");
        state.authorized = authorized;
        state.last_update = chrono::Utc::now();
        Ok(authorized)
    }

    pub async fn start_charging(&self) -> Result<()> {
        let mut state = self.state.write().await;
        if !state.connected {
            return Err(SimulatorError::Simulation("EV not connected".to_string()));
        }
        if !state.authorized {
            return Err(SimulatorError::Simulation("EV not authorized".to_string()));
        }
        if state.charging {
            return Err(SimulatorError::Simulation("EV already charging".to_string()));
        }
        state.charging = true;
        state.last_update = chrono::Utc::now();
        Ok(())
    }

    pub async fn stop_charging(&self) -> Result<()> {
        let mut state = self.state.write().await;
        state.charging = false;
        state.last_update = chrono::Utc::now();
        Ok(())
    }

    pub async fn update(&self) -> Result<()> {
        let mut state = self.state.write().await;
        let mut battery = self.battery.write().await;

        if state.charging {
            let power = self.config.max_power.min(22000);
            let duration = 1;
            let energy_wh = (power * duration) / 3600;

            battery.charge(energy_wh)
                .map_err(|e| SimulatorError::Simulation(e))?;

            state.soc = battery.soc();
            state.total_energy_delivered += energy_wh;
            state.current_power = power;

            if battery.is_full() {
                state.charging = false;
            }
        }

        state.last_update = chrono::Utc::now();
        Ok(())
    }

    pub async fn state(&self) -> EvState {
        self.state.read().await.clone()
    }

    pub async fn get_battery(&self) -> Battery {
        self.battery.read().await.clone()
    }
}