//! Authorization models

use super::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Authorization {
    pub id: Id,
    pub auth_type: AuthorizationType,
    pub token: String,
    pub status: AuthorizationStatus,
    pub authorized_at: Timestamp,
    pub expires_at: Option<Timestamp>,
    pub details: AuthorizationDetails,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AuthorizationType {
    RFID,
    OCPI,
    ISO15118,
    Vendor,
    Other(String),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AuthorizationStatus {
    Pending,
    Authorized,
    Rejected,
    Expired,
    Revoked,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AuthorizationDetails {
    pub user_id: Option<String>,
    pub emsp_id: Option<String>,
    pub cpo_id: Option<String>,
    pub contract_id: Option<String>,
    pub session_limit: Option<AuthorizationLimit>,
    pub allowed_connectors: Vec<ConnectorType>,
    pub allowed_stations: Vec<StationId>,
    pub restrictions: Vec<AuthorizationRestriction>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorizationLimit {
    pub max_energy: Option<Energy>,
    pub max_cost: Option<Money>,
    pub max_duration: Option<chrono::Duration>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorizationRestriction {
    pub restriction_type: RestrictionType,
    pub value: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum RestrictionType {
    TimeRestriction,
    EnergyRestriction,
    LocationRestriction,
    VehicleRestriction,
}

impl Authorization {
    pub fn new(auth_type: AuthorizationType, token: String) -> Self {
        Self {
            id: Id::new(),
            auth_type,
            token,
            status: AuthorizationStatus::Pending,
            authorized_at: now(),
            expires_at: None,
            details: AuthorizationDetails::default(),
        }
    }

    pub fn is_valid(&self) -> bool {
        match self.status {
            AuthorizationStatus::Authorized => {
                if let Some(expires) = self.expires_at {
                    now() < expires
                } else {
                    true
                }
            }
            _ => false,
        }
    }

    pub fn expire(&mut self) {
        self.status = AuthorizationStatus::Expired;
    }

    pub fn revoke(&mut self) {
        self.status = AuthorizationStatus::Revoked;
    }
}