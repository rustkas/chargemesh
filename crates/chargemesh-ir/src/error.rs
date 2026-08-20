//! Charging Error model (ChargeX taxonomy)

use super::*;

/// Charging error (normalized using ChargeX MREC taxonomy)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChargingError {
    pub code: String,              // CX### format
    pub error_type: ErrorType,
    pub description: String,
    pub source: ErrorSource,
    pub severity: ErrorSeverity,
    pub timestamp: Timestamp,
    pub responsibility: ErrorResponsibility,
    pub recommended_action: Option<String>,
    pub raw: Option<serde_json::Value>,
    pub context: ErrorContext,
    pub resolved: bool,
    pub resolved_at: Option<Timestamp>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ErrorType {
    Hardware,
    Communication,
    Authorization,
    Configuration,
    External,
    Roaming,
    Security,
    Internal,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ErrorSource {
    EVSE,
    EV,
    CSMS,
    Roaming,
    Grid,
    User,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ErrorSeverity {
    Info,
    Warning,
    Critical,
    Fatal,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ErrorResponsibility {
    OEM,
    CPO,
    EMSP,
    User,
    Grid,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ErrorContext {
    pub station_id: Option<StationId>,
    pub evse_id: Option<EvseId>,
    pub connector_id: Option<ConnectorId>,
    pub session_id: Option<SessionId>,
    pub transaction_id: Option<TransactionId>,
    pub user_id: Option<String>,
}

impl ChargingError {
    pub fn new(
        code: String,
        error_type: ErrorType,
        description: String,
        source: ErrorSource,
        severity: ErrorSeverity,
    ) -> Self {
        Self {
            code,
            error_type,
            description,
            source,
            severity,
            timestamp: now(),
            responsibility: ErrorResponsibility::Unknown,
            recommended_action: None,
            raw: None,
            context: ErrorContext::default(),
            resolved: false,
            resolved_at: None,
        }
    }

    pub fn with_context(mut self, context: ErrorContext) -> Self {
        self.context = context;
        self
    }

    pub fn resolve(&mut self) {
        self.resolved = true;
        self.resolved_at = Some(now());
    }
}