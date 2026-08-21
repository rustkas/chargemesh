//! Metric registry

use super::*;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metric {
    pub name: String,
    pub value: f64,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub labels: HashMap<String, String>,
    pub metric_type: MetricType,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum MetricType {
    Counter,
    Gauge,
    Histogram,
    Summary,
}

pub struct MetricRegistry {
    metrics: Arc<RwLock<HashMap<String, Vec<Metric>>>>,
    counters: Arc<RwLock<HashMap<String, f64>>>,
    gauges: Arc<RwLock<HashMap<String, f64>>>,
}

impl MetricRegistry {
    pub fn new() -> Self {
        Self {
            metrics: Arc::new(RwLock::new(HashMap::new())),
            counters: Arc::new(RwLock::new(HashMap::new())),
            gauges: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn start(&self) -> Result<()> {
        Ok(())
    }

    pub async fn stop(&self) -> Result<()> {
        Ok(())
    }

    pub async fn increment_counter(&self, name: &str, labels: HashMap<String, String>) -> Result<()> {
        let mut counters = self.counters.write().await;
        let key = format!("{}:{:?}", name, labels);
        let value = counters.entry(key).or_insert(0.0);
        *value += 1.0;

        self.record_metric(name, *value, MetricType::Counter, labels).await?;
        Ok(())
    }

    pub async fn set_gauge(&self, name: &str, value: f64, labels: HashMap<String, String>) -> Result<()> {
        let mut gauges = self.gauges.write().await;
        let key = format!("{}:{:?}", name, labels);
        gauges.insert(key, value);

        self.record_metric(name, value, MetricType::Gauge, labels).await?;
        Ok(())
    }

    pub async fn record_metric(
        &self,
        name: &str,
        value: f64,
        metric_type: MetricType,
        labels: HashMap<String, String>,
    ) -> Result<()> {
        let mut metrics = self.metrics.write().await;
        let entry = metrics.entry(name.to_string()).or_insert_with(Vec::new);

        let metric = Metric {
            name: name.to_string(),
            value,
            timestamp: chrono::Utc::now(),
            labels,
            metric_type,
        };

        entry.push(metric);

        if entry.len() > 10000 {
            let _ = entry.drain(0..1000);
        }

        Ok(())
    }

    pub async fn collect(&self) -> Vec<Metric> {
        let metrics = self.metrics.read().await;
        let mut all = Vec::new();
        for values in metrics.values() {
            all.extend(values.clone());
        }
        all
    }

    pub async fn get_counter(&self, name: &str, labels: &HashMap<String, String>) -> f64 {
        let counters = self.counters.read().await;
        let key = format!("{}:{:?}", name, labels);
        *counters.get(&key).unwrap_or(&0.0)
    }

    pub async fn get_gauge(&self, name: &str, labels: &HashMap<String, String>) -> Option<f64> {
        let gauges = self.gauges.read().await;
        let key = format!("{}:{:?}", name, labels);
        gauges.get(&key).copied()
    }
}

impl Default for MetricRegistry {
    fn default() -> Self {
        Self::new()
    }
}