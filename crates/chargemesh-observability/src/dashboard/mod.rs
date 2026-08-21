//! Dashboard renderer

mod renderer;
mod widgets;

pub use renderer::*;
pub use widgets::*;

use super::*;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardData {
    pub metrics: Vec<Metric>,
    pub logs: Vec<LogEntry>,
    pub traces: Vec<Trace>,
    pub events: Vec<Event>,
    pub correlations: CorrelationSummary,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

pub struct DashboardRenderer {
    widgets: Vec<Box<dyn DashboardWidget>>,
}

#[async_trait]
pub trait DashboardWidget: Send + Sync {
    async fn render(&self, data: &DashboardData) -> Result<String>;
    fn name(&self) -> &str;
}

impl DashboardRenderer {
    pub fn new() -> Self {
        let mut renderer = Self {
            widgets: Vec::new(),
        };

        renderer.register_widget(Box::new(OverviewWidget::new()));
        renderer.register_widget(Box::new(StationsWidget::new()));
        renderer.register_widget(Box::new(ErrorsWidget::new()));
        renderer.register_widget(Box::new(ProtocolWidget::new()));
        renderer.register_widget(Box::new(CorrelationsWidget::new()));

        renderer
    }

    pub async fn start(&self) -> Result<()> {
        Ok(())
    }

    pub async fn stop(&self) -> Result<()> {
        Ok(())
    }

    pub fn register_widget(&mut self, widget: Box<dyn DashboardWidget>) {
        self.widgets.push(widget);
    }

    pub async fn render(&self, data: &DashboardData) -> Result<String> {
        let mut output = String::new();

        output.push_str("╔══════════════════════════════════════════════════════════════════╗\n");
        output.push_str("║                          ChargeMesh                             ║\n");
        output.push_str("║                    Observability Platform                       ║\n");
        output.push_str("╠══════════════════════════════════════════════════════════════════╣\n");
        output.push_str(&format!("║  Updated: {:30}                       ║\n", data.timestamp.format("%Y-%m-%d %H:%M:%S")));
        output.push_str("╠══════════════════════════════════════════════════════════════════╣\n");

        for widget in &self.widgets {
            if let Ok(rendered) = widget.render(data).await {
                output.push_str(&rendered);
            }
        }

        output.push_str("╚══════════════════════════════════════════════════════════════════╝\n");

        Ok(output)
    }
}

impl Default for DashboardRenderer {
    fn default() -> Self {
        Self::new()
    }
}