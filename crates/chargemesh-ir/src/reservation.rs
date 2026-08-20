//! Reservation model

use super::*;
use std::collections::HashMap;

/// Connector reservation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reservation {
    pub id: Id,
    pub station_id: StationId,
    pub evse_id: EvseId,
    pub connector_id: ConnectorId,
    pub user_id: String,
    pub authorization_token: String,
    pub start_time: Timestamp,
    pub end_time: Timestamp,
    pub status: ReservationStatus,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
    pub expiry_time: Option<Timestamp>,
    pub metadata: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ReservationStatus {
    Pending,
    Confirmed,
    Active,
    Expired,
    Cancelled,
    Completed,
}

impl Reservation {
    pub fn new(
        station_id: StationId,
        evse_id: EvseId,
        connector_id: ConnectorId,
        user_id: String,
        authorization_token: String,
        end_time: Timestamp,
    ) -> Self {
        let now = now();
        Self {
            id: Id::new(),
            station_id,
            evse_id,
            connector_id,
            user_id,
            authorization_token,
            start_time: now,
            end_time,
            status: ReservationStatus::Pending,
            created_at: now,
            updated_at: now,
            expiry_time: None,
            metadata: HashMap::new(),
        }
    }

    pub fn is_active(&self) -> bool {
        matches!(
            self.status,
            ReservationStatus::Confirmed | ReservationStatus::Active
        ) && now() < self.end_time
    }

    pub fn confirm(&mut self) {
        self.status = ReservationStatus::Confirmed;
        self.updated_at = now();
    }

    pub fn cancel(&mut self) {
        self.status = ReservationStatus::Cancelled;
        self.updated_at = now();
    }

    pub fn expire(&mut self) {
        self.status = ReservationStatus::Expired;
        self.updated_at = now();
    }
}