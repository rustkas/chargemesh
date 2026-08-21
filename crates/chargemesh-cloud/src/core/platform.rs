//! Cloud platform core

use super::*;
use std::sync::Arc;
use tokio::sync::RwLock;
use crate::api::ApiServer;
use crate::billing::BillingManager;
use crate::tenant::TenantManager;
use crate::analytics::AnalyticsEngine;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlatformConfig {
    pub name: String,
    pub version: String,
    pub environment: Environment,
    pub api_port: u16,
    pub database_url: String,
    pub redis_url: String,
    pub jwt_secret: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Environment {
    Development,
    Staging,
    Production,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum PlatformStatus {
    Stopped,
    Starting,
    Running,
    Degraded,
    Stopping,
    Failed,
}

pub struct CloudPlatform {
    config: PlatformConfig,
    tenant_manager: Arc<TenantManager>,
    billing_manager: Arc<BillingManager>,
    analytics_engine: Arc<AnalyticsEngine>,
    api_server: Arc<ApiServer>,
    status: Arc<RwLock<PlatformStatus>>,
}

impl CloudPlatform {
    pub async fn new(config: PlatformConfig) -> Result<Self> {
        let platform = Self {
            tenant_manager: Arc::new(TenantManager::new()),
            billing_manager: Arc::new(BillingManager::new()),
            analytics_engine: Arc::new(AnalyticsEngine::new()),
            api_server: Arc::new(ApiServer::new(config.api_port)),
            config,
            status: Arc::new(RwLock::new(PlatformStatus::Stopped)),
        };

        Ok(platform)
    }

    pub async fn start(&self) -> Result<()> {
        *self.status.write().await = PlatformStatus::Starting;

        self.api_server.start().await?;
        self.billing_manager.start().await?;
        self.analytics_engine.start().await?;

        *self.status.write().await = PlatformStatus::Running;
        tracing::info!("Cloud platform started successfully");
        Ok(())
    }

    pub async fn stop(&self) -> Result<()> {
        *self.status.write().await = PlatformStatus::Stopping;

        self.api_server.stop().await?;
        self.billing_manager.stop().await?;
        self.analytics_engine.stop().await?;

        *self.status.write().await = PlatformStatus::Stopped;
        Ok(())
    }

    pub async fn get_status(&self) -> PlatformStatus {
        *self.status.read().await
    }
}