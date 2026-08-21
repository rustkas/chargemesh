//! Smart charging optimizer

use super::*;
use crate::energy::EmsIntegration;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct ChargingSession {
    pub id: String,
    pub station_id: String,
    pub connector_id: u8,
    pub required_energy: f64,
    pub max_power: f64,
    pub min_power: f64,
    pub start_time: chrono::DateTime<chrono::Utc>,
    pub deadline: Option<chrono::DateTime<chrono::Utc>>,
    pub priority: u8,
    pub current_soc: f64,
    pub target_soc: f64,
    pub battery_capacity: f64,
}

pub struct SmartChargingOptimizer {
    config: Arc<tokio::sync::RwLock<SmartChargingConfig>>,
    sessions: Arc<tokio::sync::RwLock<HashMap<String, ChargingSession>>>,
    ems: Arc<EmsIntegration>,
}

impl SmartChargingOptimizer {
    pub fn new(config: SmartChargingConfig, ems: Arc<EmsIntegration>) -> Self {
        Self {
            config: Arc::new(tokio::sync::RwLock::new(config)),
            sessions: Arc::new(tokio::sync::RwLock::new(HashMap::new())),
            ems,
        }
    }

    pub async fn register_session(&self, session: ChargingSession) -> Result<()> {
        let mut sessions = self.sessions.write().await;
        sessions.insert(session.id.clone(), session);
        Ok(())
    }

    pub async fn optimize(&self) -> Result<Vec<ChargingPlan>> {
        let config = self.config.read().await;
        let sessions = self.sessions.read().await;

        let mut plans = Vec::new();
        match config.algorithm {
            SmartChargingAlgorithm::Greedy => {
                plans = self.greedy_optimize(&sessions).await?;
            }
            SmartChargingAlgorithm::LinearProgramming => {
                plans = self.lp_optimize(&sessions).await?;
            }
            _ => {
                plans = self.greedy_optimize(&sessions).await?;
            }
        }
        Ok(plans)
    }

    async fn greedy_optimize(
        &self,
        sessions: &HashMap<String, ChargingSession>,
    ) -> Result<Vec<ChargingPlan>> {
        let mut plans = Vec::new();
        let now = chrono::Utc::now();
        let available_power = self.ems.get_available_power().await;

        let mut sorted_sessions: Vec<_> = sessions.values().collect();
        sorted_sessions.sort_by_key(|s| s.priority);

        for session in sorted_sessions {
            let power = session.max_power.min(available_power);
            let duration = chrono::Duration::seconds(
                (session.required_energy / power * 3600.0) as i64
            );

            let plan = ChargingPlan {
                id: uuid::Uuid::new_v4().to_string(),
                session_id: session.id.clone(),
                schedule: vec![ChargingSlot {
                    start_time: now,
                    end_time: now + duration,
                    power,
                    energy: session.required_energy,
                    source: EnergySource::Grid,
                }],
                total_energy: session.required_energy,
                total_cost: session.required_energy * 0.15,
                carbon_emissions: session.required_energy * 0.4,
                optimization_target: OptimizationTarget::MinimizeCost,
            };
            plans.push(plan);
        }
        Ok(plans)
    }

    async fn lp_optimize(
        &self,
        sessions: &HashMap<String, ChargingSession>,
    ) -> Result<Vec<ChargingPlan>> {
        self.greedy_optimize(sessions).await
    }
}