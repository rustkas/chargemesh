//! Time utilities

use serde::{Deserialize, Serialize};
use std::fmt;

/// Timestamp (ISO 8601 with UTC)
pub type Timestamp = chrono::DateTime<chrono::Utc>;

/// Current timestamp helper
pub fn now() -> Timestamp {
    chrono::Utc::now()
}

/// Parse timestamp from RFC3339 string
pub fn parse_timestamp(s: &str) -> CoreResult<Timestamp> {
    chrono::DateTime::parse_from_rfc3339(s)
        .map(|dt| dt.with_timezone(&chrono::Utc))
        .map_err(|e| CoreError::InvalidTimestamp(e.to_string()))
}

/// Format timestamp as RFC3339 string
pub fn format_timestamp(ts: &Timestamp) -> String {
    ts.to_rfc3339()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeRange {
    pub start: Timestamp,
    pub end: Timestamp,
}

impl TimeRange {
    pub fn new(start: Timestamp, end: Timestamp) -> Self {
        Self { start, end }
    }

    pub fn contains(&self, ts: &Timestamp) -> bool {
        ts >= &self.start && ts <= &self.end
    }

    pub fn duration(&self) -> chrono::Duration {
        self.end - self.start
    }
}

impl fmt::Display for TimeRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} → {}",
            self.start.to_rfc3339(),
            self.end.to_rfc3339()
        )
    }
}