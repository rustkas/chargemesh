//! V2G (Vehicle-to-Grid) integration

use super::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct V2GConfig {
    pub enabled: bool,
    pub max_discharge_power: f64,
    pub min_soc: f64,
    pub max_soc: f64,
    pub scheduled_discharge: Option<Vec<V2GSchedule>>,
    pub grid_services: Vec<GridService>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct V2GSchedule {
    pub start_time: chrono::DateTime<chrono::Utc>,
    pub end_time: chrono::DateTime<chrono::Utc>,
    pub power: f64,
    pub priority: V2GPriority,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum V2GPriority {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum GridService {
    FrequencyRegulation,
    VoltageSupport,
    PeakShaving,
    LoadBalancing,
    EmergencyPower,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct V2GSession {
    pub id: String,
    pub station_id: String,
    pub session_id: String,
    pub start_time: chrono::DateTime<chrono::Utc>,
    pub end_time: Option<chrono::DateTime<chrono::Utc>>,
    pub direction: V2GDirection,
    pub power: f64,
    pub energy: f64,
    pub status: V2GSessionStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum V2GDirection {
    Charge,
    Discharge,
    Bidirectional,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum V2GSessionStatus {
    Pending,
    Active,
    Completed,
    Cancelled,
    Faulted,
}

pub struct V2GIntegration {
    config: Arc<tokio::sync::RwLock<V2GConfig>>,
    sessions: Arc<tokio::sync::RwLock<Vec<V2GSession>>>,
}

impl V2GIntegration {
    pub fn new() -> Self {
        Self {
            config: Arc::new(tokio::sync::RwLock::new(V2GConfig {
                enabled: false,
                max_discharge_power: 11000.0,
                min_soc: 20.0,
                max_soc: 80.0,
                scheduled_discharge: None,
                grid_services: Vec::new(),
            })),
            sessions: Arc::new(tokio::sync::RwLock::new(Vec::new())),
        }
    }

    pub async fn enable(&self, config: V2GConfig) -> Result<()> {
        let mut current = self.config.write().await;
        *current = config;
        Ok(())
    }

    pub async fn start_discharge(&self, station_id: &str, session_id: &str, power: f64) -> Result<String> {
        let config = self.config.read().await;
        if !config.enabled {
            return Err(IntegrationError::V2g("V2G not enabled".to_string()));
        }
        if power > config.max_discharge_power {
            return Err(IntegrationError::V2g("Power exceeds maximum".to_string()));
        }

        let session = V2GSession {
            id: uuid::Uuid::new_v4().to_string(),
            station_id: station_id.to_string(),
            session_id: session_id.to_string(),
            start_time: chrono::Utc::now(),
            end_time: None,
            direction: V2GDirection::Discharge,
            power,
            energy: 0.0,
            status: V2GSessionStatus::Active,
        };

        let id = session.id.clone();
        let mut sessions = self.sessions.write().await;
        sessions.push(session);
        Ok(id)
    }

    pub async fn stop_discharge(&self, session_id: &str) -> Result<()> {
        let mut sessions = self.sessions.write().await;
        if let Some(session) = sessions.iter_mut().find(|s| s.id == session_id) {
            session.end_time = Some(chrono::Utc::now());
            session.status = V2GSessionStatus::Completed;
        }
        Ok(())
    }

    pub async fn get_active_sessions(&self) -> Vec<V2GSession> {
        let sessions = self.sessions.read().await;
        sessions.iter().filter(|s| s.status == V2GSessionStatus::Active).cloned().collect()
    }
}

impl Default for V2GIntegration {
    fn default() -> Self {
        Self::new()
    }
}