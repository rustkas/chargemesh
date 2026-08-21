//! API Server

mod rest;
mod auth;
mod websocket;

pub use rest::*;
pub use auth::*;
pub use websocket::*;

use super::*;
use axum::{
    extract::Request,
    middleware,
    response::Json,
    routing::{get, post, put, delete},
    Router,
};
use std::sync::Arc;

pub struct ApiServer {
    port: u16,
    app: Option<Router>,
    server: Option<tokio::task::JoinHandle<()>>,
}

impl ApiServer {
    pub fn new(port: u16) -> Self {
        Self {
            port,
            app: None,
            server: None,
        }
    }

    pub async fn start(&self) -> Result<()> {
        let app = self.build_router().await?;

        let port = self.port;
        let server = tokio::spawn(async move {
            let addr = format!("0.0.0.0:{}", port).parse().unwrap();
            tracing::info!("API server listening on {}", addr);

            axum::Server::bind(&addr)
                .serve(app.into_make_service())
                .await
                .unwrap();
        });

        Ok(())
    }

    pub async fn stop(&self) -> Result<()> {
        if let Some(handle) = &self.server {
            handle.abort();
        }
        Ok(())
    }

    async fn build_router(&self) -> Result<Router> {
        let router = Router::new()
            .route("/health", get(health_check))
            .route("/api/v1/subscriptions", post(create_subscription))
            .route("/api/v1/subscriptions/:id", get(get_subscription))
            .route("/api/v1/subscriptions/:id", put(update_subscription))
            .route("/api/v1/stations", get(list_stations))
            .route("/api/v1/stations/:id", get(get_station))
            .route("/api/v1/sessions", get(list_sessions))
            .route("/api/v1/sessions/:id", get(get_session))
            .route("/api/v1/analytics/usage", get(get_usage_analytics))
            .route("/api/v1/analytics/performance", get(get_performance_analytics));

        Ok(router)
    }
}

// ============================================================================
// Handlers
// ============================================================================

async fn health_check() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "ok",
        "version": env!("CARGO_PKG_VERSION"),
    }))
}

async fn create_subscription(
    Json(subscription): Json<crate::billing::Subscription>,
) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "success",
        "data": subscription,
    }))
}

async fn get_subscription(
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "success",
        "data": {
            "id": id,
            "tier": "Pro",
            "status": "Active",
        }
    }))
}

async fn update_subscription(
    axum::extract::Path(id): axum::extract::Path<String>,
    Json(subscription): Json<crate::billing::Subscription>,
) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "success",
        "data": subscription,
    }))
}

async fn list_stations() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "success",
        "data": [],
        "total": 0,
    }))
}

async fn get_station(
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "success",
        "data": {
            "id": id,
            "vendor": "ABB",
            "model": "Terra 54",
            "status": "online",
        }
    }))
}

async fn list_sessions() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "success",
        "data": [],
        "total": 0,
    }))
}

async fn get_session(
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "success",
        "data": {
            "id": id,
            "station_id": "CP-001",
            "status": "charging",
            "energy": 12.5,
        }
    }))
}

async fn get_usage_analytics() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "success",
        "data": {
            "total_sessions": 1234,
            "total_energy_kwh": 45678.5,
            "avg_session_duration_minutes": 45,
            "success_rate": 97.8,
        }
    }))
}

async fn get_performance_analytics() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "success",
        "data": {
            "station_utilization": 72.5,
            "avg_power_kw": 11.2,
            "peak_power_kw": 50.0,
            "error_rate": 2.2,
        }
    }))
}