//! Tenant management

use super::*;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tenant {
    pub id: String,
    pub name: String,
    pub tier: CloudTier,
    pub subscription_id: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub status: TenantStatus,
    pub quota: Quota,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TenantStatus {
    Active,
    Suspended,
    Pending,
    Deleted,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quota {
    pub max_stations: u32,
    pub max_evses: u32,
    pub max_sessions: u32,
    pub storage_gb: u32,
    pub api_calls_per_month: u32,
    pub max_users: u32,
    pub retention_days: u32,
}

pub struct TenantManager {
    tenants: Arc<tokio::sync::RwLock<HashMap<String, Tenant>>>,
}

impl TenantManager {
    pub fn new() -> Self {
        Self {
            tenants: Arc::new(tokio::sync::RwLock::new(HashMap::new())),
        }
    }

    pub async fn create_tenant(&self, tenant: Tenant) -> Result<()> {
        let mut tenants = self.tenants.write().await;
        tenants.insert(tenant.id.clone(), tenant);
        Ok(())
    }

    pub async fn get_tenant(&self, id: &str) -> Option<Tenant> {
        let tenants = self.tenants.read().await;
        tenants.get(id).cloned()
    }

    pub async fn update_tenant(&self, tenant: Tenant) -> Result<()> {
        let mut tenants = self.tenants.write().await;
        tenants.insert(tenant.id.clone(), tenant);
        Ok(())
    }

    pub async fn check_quota(&self, tenant_id: &str) -> Result<Quota> {
        let tenants = self.tenants.read().await;
        if let Some(tenant) = tenants.get(tenant_id) {
            Ok(tenant.quota.clone())
        } else {
            Err(CloudError::NotFound("Tenant not found".to_string()))
        }
    }
}

impl Default for TenantManager {
    fn default() -> Self {
        Self::new()
    }
}