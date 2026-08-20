//! Transaction model (billing record)

use super::*;
use std::collections::HashMap;

/// Charging transaction — the billing record for a session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    /// Transaction identifier
    pub id: TransactionId,

    /// Session ID
    pub session_id: SessionId,

    /// Transaction reference (e.g., from CSMS)
    pub transaction_ref: Option<String>,

    /// Authorization used
    pub authorization: Authorization,

    /// Start time
    pub start_time: Timestamp,

    /// End time
    pub end_time: Option<Timestamp>,

    /// Meter start reading
    pub meter_start: MeterValue,

    /// Meter end reading
    pub meter_end: Option<MeterValue>,

    /// Energy delivered (watt-hours)
    pub energy_delivered: Energy,

    /// Tariff applied
    pub tariff: Tariff,

    /// Cost (in minor units)
    pub cost: Money,

    /// Currency
    pub currency: String,

    /// Charging detail record (CDR) — for OCPI
    pub cdr: Option<ChargingDetailRecord>,

    /// Status
    pub status: TransactionStatus,

    /// Custom metadata
    pub metadata: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TransactionStatus {
    Pending,
    Completed,
    Failed,
    Cancelled,
}

/// Charging Detail Record (CDR) — for OCPI roaming
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChargingDetailRecord {
    pub id: String,
    pub session_id: String,
    pub cpo_id: String,
    pub emsp_id: String,
    pub auth_reference: String,
    pub start_time: Timestamp,
    pub end_time: Timestamp,
    pub total_energy: f64,
    pub total_cost: Money,
    pub currency: String,
    pub tariff: Tariff,
    pub meter_values: Vec<MeterValue>,
}

impl Transaction {
    pub fn new(
        session_id: SessionId,
        authorization: Authorization,
        meter_start: MeterValue,
        tariff: Tariff,
    ) -> Self {
        Self {
            id: TransactionId::new(),
            session_id,
            transaction_ref: None,
            authorization,
            start_time: now(),
            end_time: None,
            meter_start,
            meter_end: None,
            energy_delivered: Energy::new(0),
            tariff,
            cost: Money::new(0),
            currency: "USD".to_string(),
            cdr: None,
            status: TransactionStatus::Pending,
            metadata: HashMap::new(),
        }
    }

    pub fn complete(&mut self, meter_end: MeterValue) {
        self.meter_end = Some(meter_end.clone());
        self.energy_delivered = Energy::new(
            meter_end.energy_import.0.saturating_sub(
                self.meter_start.energy_import.0
            )
        );
        self.end_time = Some(now());
        self.status = TransactionStatus::Completed;
        self.cost = Money::new(
            (self.energy_delivered.0 as f64 * self.tariff.energy_price) as i64
        );
    }

    pub fn duration(&self) -> Option<chrono::Duration> {
        self.end_time.map(|end| end - self.start_time)
    }

    pub fn is_completed(&self) -> bool {
        matches!(self.status, TransactionStatus::Completed)
    }
}