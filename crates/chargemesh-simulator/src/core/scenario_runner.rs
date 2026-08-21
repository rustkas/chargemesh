//! Scenario runner

use super::*;
use crate::faults::FaultInjector;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, warn};

pub struct ScenarioRunner {
    fault_injector: Arc<FaultInjector>,
    status: Arc<RwLock<SimulationStatus>>,
}

impl ScenarioRunner {
    pub fn new() -> Self {
        Self {
            fault_injector: Arc::new(FaultInjector::new()),
            status: Arc::new(RwLock::new(SimulationStatus::Pending)),
        }
    }

    pub async fn run(&self, scenario: &Scenario) -> Result<()> {
        *self.status.write().await = SimulationStatus::Running;

        info!("Starting scenario: {}", scenario.name);
        info!("Description: {}", scenario.description);

        let mut step_count = 0;
        for step in &scenario.steps {
            step_count += 1;
            info!("Step {}: {:?}", step_count, step.action);

            self.execute_step(step).await?;

            if let Some(delay) = step.delay {
                tokio::time::sleep(tokio::time::Duration::from_secs(
                    delay.num_seconds() as u64
                )).await;
            }
        }

        // Inject faults if any
        for fault in &scenario.faults {
            if rand::random::<f64>() < fault.probability {
                info!("Injecting fault: {:?}", fault.fault_type);
                self.fault_injector.inject_fault(fault.clone()).await?;
            }
        }

        *self.status.write().await = SimulationStatus::Completed;
        info!("Scenario completed successfully");

        Ok(())
    }

    async fn execute_step(&self, step: &ScenarioStep) -> Result<()> {
        match &step.action {
            ScenarioAction::Log { message } => {
                println!("📝 {}", message);
            }
            ScenarioAction::ConnectEV => {
                println!("🔌 Connecting EV...");
            }
            ScenarioAction::StartCharging => {
                println!("⚡ Starting charging...");
            }
            ScenarioAction::StopCharging => {
                println!("⏹️ Stopping charging...");
            }
            ScenarioAction::DisconnectEV => {
                println!("🔌 Disconnecting EV...");
            }
            ScenarioAction::Authorize { token } => {
                println!("🔑 Authorizing with token: {}", token);
            }
            ScenarioAction::SetProfile { max_power, duration } => {
                println!("📊 Setting charging profile: {}W for {}s", max_power, duration.num_seconds());
            }
            ScenarioAction::InjectFault { fault } => {
                println!("💥 Injecting fault: {:?}", fault);
                self.fault_injector.inject_fault(crate::faults::FaultInjection {
                    fault_type: fault.clone(),
                    probability: 1.0,
                    duration: chrono::Duration::seconds(5),
                    condition: None,
                    parameters: serde_json::json!({}),
                }).await?;
            }
            ScenarioAction::WaitFor { condition } => {
                println!("⏳ Waiting for: {:?}", condition);
                // Simulate wait
                tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
            }
            ScenarioAction::Custom { name, data } => {
                println!("🔧 Custom action: {} ({})", name, data);
            }
        }
        Ok(())
    }

    pub async fn status(&self) -> SimulationStatus {
        *self.status.read().await
    }
}

impl Default for ScenarioRunner {
    fn default() -> Self {
        Self::new()
    }
}