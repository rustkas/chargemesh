//! Dashboard widgets

use super::*;

pub struct OverviewWidget {
    title: String,
}

impl OverviewWidget {
    pub fn new() -> Self {
        Self {
            title: "Overview".to_string(),
        }
    }
}

#[async_trait]
impl DashboardWidget for OverviewWidget {
    async fn render(&self, data: &DashboardData) -> Result<String> {
        let mut output = String::new();
        output.push_str("║\n");
        output.push_str("║  📊 OVERVIEW\n");
        output.push_str("║\n");

        let total_metrics = data.metrics.len();
        let total_logs = data.logs.len();
        let total_events = data.events.len();

        output.push_str(&format!("║    Metrics: {:>8}  Logs: {:>8}  Events: {:>8}  ║\n",
            total_metrics, total_logs, total_events));
        output.push_str(&format!("║    Traces:  {:>8}  Correlations: {:>6}              ║\n",
            data.traces.len(), data.correlations.total_correlations));
        output.push_str("║\n");

        Ok(output)
    }

    fn name(&self) -> &str {
        &self.title
    }
}

pub struct StationsWidget {
    title: String,
}

impl StationsWidget {
    pub fn new() -> Self {
        Self {
            title: "Stations".to_string(),
        }
    }
}

#[async_trait]
impl DashboardWidget for StationsWidget {
    async fn render(&self, data: &DashboardData) -> Result<String> {
        let total_stations = data.metrics.iter()
            .find(|m| m.name == "stations_total")
            .map(|m| m.value as u32)
            .unwrap_or(0);

        let online_stations = data.metrics.iter()
            .find(|m| m.name == "stations_online")
            .map(|m| m.value as u32)
            .unwrap_or(0);

        let charging_stations = data.metrics.iter()
            .find(|m| m.name == "stations_charging")
            .map(|m| m.value as u32)
            .unwrap_or(0);

        let errors = data.metrics.iter()
            .find(|m| m.name == "errors_total")
            .map(|m| m.value as u32)
            .unwrap_or(0);

        let mut output = String::new();
        output.push_str("║\n");
        output.push_str("║  🏭 STATIONS\n");
        output.push_str("║\n");
        output.push_str(&format!("║    Total: {:>8}  Online: {:>8}  Charging: {:>8}  ║\n",
            total_stations, online_stations, charging_stations));
        output.push_str(&format!("║    Errors: {:>7}                                          ║\n", errors));
        output.push_str("║\n");

        Ok(output)
    }

    fn name(&self) -> &str {
        &self.title
    }
}

pub struct ErrorsWidget {
    title: String,
}

impl ErrorsWidget {
    pub fn new() -> Self {
        Self {
            title: "Errors".to_string(),
        }
    }
}

#[async_trait]
impl DashboardWidget for ErrorsWidget {
    async fn render(&self, data: &DashboardData) -> Result<String> {
        let mut output = String::new();
        output.push_str("║\n");
        output.push_str("║  ❌ ERRORS\n");
        output.push_str("║\n");

        let protocol_errors = data.metrics.iter()
            .find(|m| m.name == "protocol_errors_total")
            .map(|m| m.value as u32)
            .unwrap_or(0);

        let iso15118_errors = data.metrics.iter()
            .find(|m| m.name == "iso15118_errors_total")
            .map(|m| m.value as u32)
            .unwrap_or(0);

        let network_errors = data.metrics.iter()
            .find(|m| m.name == "network_errors_total")
            .map(|m| m.value as u32)
            .unwrap_or(0);

        let hardware_errors = data.metrics.iter()
            .find(|m| m.name == "hardware_errors_total")
            .map(|m| m.value as u32)
            .unwrap_or(0);

        output.push_str(&format!("║    Protocol:   {:>5}  ISO 15118:  {:>5}                 ║\n",
            protocol_errors, iso15118_errors));
        output.push_str(&format!("║    Network:    {:>5}  Hardware:   {:>5}                 ║\n",
            network_errors, hardware_errors));
        output.push_str("║\n");

        Ok(output)
    }

    fn name(&self) -> &str {
        &self.title
    }
}

pub struct ProtocolWidget {
    title: String,
}

impl ProtocolWidget {
    pub fn new() -> Self {
        Self {
            title: "Protocols".to_string(),
        }
    }
}

#[async_trait]
impl DashboardWidget for ProtocolWidget {
    async fn render(&self, data: &DashboardData) -> Result<String> {
        let mut output = String::new();
        output.push_str("║\n");
        output.push_str("║  📡 PROTOCOLS\n");
        output.push_str("║\n");

        let mut ocpp16_count = 0;
        let mut ocpp201_count = 0;
        let mut ocpp21_count = 0;
        let mut iso15118_count = 0;

        for log in &data.logs {
            if let Some(protocol) = log.fields.get("protocol") {
                if let Some(p) = protocol.as_str() {
                    match p {
                        "ocpp1.6" => ocpp16_count += 1,
                        "ocpp2.0.1" => ocpp201_count += 1,
                        "ocpp2.1" => ocpp21_count += 1,
                        "iso15118" => iso15118_count += 1,
                        _ => {}
                    }
                }
            }
        }

        output.push_str(&format!("║    OCPP 1.6:  {:>5}  OCPP 2.0.1: {:>5}                  ║\n",
            ocpp16_count, ocpp201_count));
        output.push_str(&format!("║    OCPP 2.1:  {:>5}  ISO 15118:  {:>5}                  ║\n",
            ocpp21_count, iso15118_count));
        output.push_str("║\n");

        Ok(output)
    }

    fn name(&self) -> &str {
        &self.title
    }
}

pub struct CorrelationsWidget {
    title: String,
}

impl CorrelationsWidget {
    pub fn new() -> Self {
        Self {
            title: "Correlations".to_string(),
        }
    }
}

#[async_trait]
impl DashboardWidget for CorrelationsWidget {
    async fn render(&self, data: &DashboardData) -> Result<String> {
        let mut output = String::new();
        output.push_str("║\n");
        output.push_str("║  🔗 CORRELATIONS\n");
        output.push_str("║\n");

        output.push_str(&format!("║    Device → Session:   {:>3}                                ║\n",
            data.correlations.device_to_session.len()));
        output.push_str(&format!("║    Session → Error:    {:>3}                                ║\n",
            data.correlations.session_to_error.len()));
        output.push_str(&format!("║    Error → Root Cause: {:>3}                                ║\n",
            data.correlations.error_to_root_cause.len()));
        output.push_str(&format!("║    Total:             {:>3}                                ║\n",
            data.correlations.total_correlations));
        output.push_str("║\n");

        Ok(output)
    }

    fn name(&self) -> &str {
        &self.title
    }
}