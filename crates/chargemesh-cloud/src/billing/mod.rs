//! Billing and subscription management

mod subscription;
mod pricing;
mod usage;

pub use subscription::*;
pub use pricing::*;
pub use usage::*;

use super::*;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subscription {
    pub id: String,
    pub tenant_id: String,
    pub tier: CloudTier,
    pub start_date: chrono::DateTime<chrono::Utc>,
    pub end_date: Option<chrono::DateTime<chrono::Utc>>,
    pub status: SubscriptionStatus,
    pub price: f64,
    pub currency: String,
    pub billing_cycle: BillingCycle,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum SubscriptionStatus {
    Active,
    Pending,
    Expired,
    Cancelled,
    Suspended,
    Trial,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum BillingCycle {
    Monthly,
    Quarterly,
    Annually,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageRecord {
    pub id: String,
    pub tenant_id: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub metric: UsageMetric,
    pub value: f64,
    pub unit: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum UsageMetric {
    Stations,
    Sessions,
    Energy,
    Errors,
    ApiCalls,
    Storage,
    Bandwidth,
    Custom,
}

pub struct BillingManager {
    subscriptions: Arc<tokio::sync::RwLock<HashMap<String, Subscription>>>,
    usage: Arc<tokio::sync::RwLock<Vec<UsageRecord>>>,
}

impl BillingManager {
    pub fn new() -> Self {
        Self {
            subscriptions: Arc::new(tokio::sync::RwLock::new(HashMap::new())),
            usage: Arc::new(tokio::sync::RwLock::new(Vec::new())),
        }
    }

    pub async fn start(&self) -> Result<()> {
        Ok(())
    }

    pub async fn stop(&self) -> Result<()> {
        Ok(())
    }

    pub async fn create_subscription(&self, subscription: Subscription) -> Result<()> {
        let mut subs = self.subscriptions.write().await;
        subs.insert(subscription.id.clone(), subscription);
        Ok(())
    }

    pub async fn get_subscription(&self, id: &str) -> Option<Subscription> {
        let subs = self.subscriptions.read().await;
        subs.get(id).cloned()
    }

    pub async fn record_usage(&self, usage: UsageRecord) -> Result<()> {
        let mut usage_records = self.usage.write().await;
        usage_records.push(usage);
        Ok(())
    }
}

impl Default for BillingManager {
    fn default() -> Self {
        Self::new()
    }
}