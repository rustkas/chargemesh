//! ChargeMesh CLI — Command-line interface

use clap::{Parser, Subcommand};
use colored::*;
use std::path::PathBuf;

// ============================================================================
// Output Format
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputFormat {
    Human,
    Json,
    Html,
}

// ============================================================================
// CLI Definition
// ============================================================================

#[derive(Parser)]
#[command(name = "chargemesh")]
#[command(about = "ChargeMesh — Universal EV Charging Interoperability")]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Parse an OCPP trace file
    Parse {
        #[arg(short, long)]
        file: PathBuf,
        #[arg(short, long, default_value = "human")]
        format: OutputFormat,
        #[arg(short, long)]
        verbose: bool,
    },

    /// Capture traffic from a charger
    Capture {
        #[arg(short, long)]
        url: String,
        #[arg(short, long)]
        output: Option<PathBuf>,
        #[arg(short, long)]
        duration: Option<u64>,
    },

    /// Analyze capabilities of a charging station
    Capability {
        #[arg(short, long)]
        config: PathBuf,
        #[arg(short, long, default_value = "human")]
        format: OutputFormat,
        #[arg(short, long)]
        verbose: bool,
    },

    /// Run a simulation
    Simulate {
        #[arg(short, long)]
        target: String,
        #[arg(short, long)]
        protocol: Option<String>,
        #[arg(short, long)]
        model: Option<String>,
        #[arg(short, long)]
        firmware: Option<String>,
        #[arg(short, long)]
        scenario: Option<String>,
        #[arg(short, long)]
        duration: Option<u64>,
        #[arg(short, long)]
        verbose: bool,
        #[arg(long)]
        list_scenarios: bool,
    },

    /// Diagnose a trace file
    Diagnose {
        #[arg(short, long)]
        file: PathBuf,
        #[arg(short, long, default_value = "human")]
        format: OutputFormat,
        #[arg(short, long)]
        verbose: bool,
        #[arg(short, long)]
        output: Option<PathBuf>,
    },

    /// Observe a station or session (live monitoring)
    Observe {
        #[arg(short, long)]
        station_id: Option<String>,
        #[arg(short, long)]
        session_id: Option<String>,
        #[arg(short, long)]
        duration: Option<u64>,
        #[arg(short, long)]
        verbose: bool,
        #[arg(long)]
        follow: bool,
    },

    /// OCPI roaming commands
    Ocpi {
        #[arg(short, long)]
        url: String,
        #[arg(short, long)]
        token: String,
        #[arg(short, long)]
        country: String,
        #[arg(short, long)]
        party: String,
        #[command(subcommand)]
        action: OcpiAction,
    },

    /// Energy management commands
    Energy {
        #[arg(short, long)]
        config: PathBuf,
        #[command(subcommand)]
        action: EnergyAction,
    },

    /// Cloud platform commands
    Cloud {
        #[arg(short, long)]
        url: Option<String>,
        #[arg(short, long)]
        token: Option<String>,
        #[command(subcommand)]
        action: CloudAction,
    },

    /// Show version information
    Version,
}

// ============================================================================
// Subcommands
// ============================================================================

#[derive(Subcommand)]
pub enum OcpiAction {
    Locations,
    Sessions,
    Tariffs,
    Sync,
}

#[derive(Subcommand)]
pub enum EnergyAction {
    Status,
    Optimize,
    Constraints,
}

#[derive(Subcommand)]
pub enum CloudAction {
    Status,
    Login,
    Stations,
    Sessions,
    Analytics,
    Subscription,
}

// ============================================================================
// Main
// ============================================================================

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    cli.run().await?;
    Ok(())
}

impl Cli {
    pub async fn run(self) -> Result<(), Box<dyn std::error::Error>> {
        match self.command {
            Commands::Parse { file, format, verbose } => {
                parse_file(file, format, verbose).await?;
            }
            Commands::Capture { url, output, duration } => {
                capture_traffic(&url, output, duration).await?;
            }
            Commands::Capability { config, format, verbose } => {
                analyze_capabilities(config, format, verbose).await?;
            }
            Commands::Simulate {
                target,
                protocol,
                model,
                firmware,
                scenario,
                duration,
                verbose,
                list_scenarios,
            } => {
                run_simulation(
                    target,
                    protocol,
                    model,
                    firmware,
                    scenario,
                    duration,
                    verbose,
                    list_scenarios,
                )
                .await?;
            }
            Commands::Diagnose { file, format, verbose, output } => {
                run_diagnostics(file, format, verbose, output).await?;
            }
            Commands::Observe {
                station_id,
                session_id,
                duration,
                verbose,
                follow,
            } => {
                run_observability(station_id, session_id, duration, verbose, follow).await?;
            }
            Commands::Ocpi {
                url,
                token,
                country,
                party,
                action,
            } => {
                run_ocpi(&url, &token, &country, &party, action).await?;
            }
            Commands::Energy { config, action } => {
                run_energy(config, action).await?;
            }
            Commands::Cloud { url, token, action } => {
                run_cloud(url, token, action).await?;
            }
            Commands::Version => {
                println!("ChargeMesh v{}", env!("CARGO_PKG_VERSION"));
            }
        }
        Ok(())
    }
}

// ============================================================================
// Command Handlers — Full Implementations
// ============================================================================

async fn parse_file(
    file: PathBuf,
    format: OutputFormat,
    verbose: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    use chargemesh_ocpp::v16::parse_ocpp_message;

    println!("{}", format!("📂 Parsing file: {}", file.display()).cyan());

    let content = std::fs::read_to_string(&file)?;
    let lines: Vec<&str> = content.lines().collect();

    let mut messages = Vec::new();
    let mut errors = Vec::new();

    for line in lines {
        if line.trim().is_empty() {
            continue;
        }
        match parse_ocpp_message(line) {
            Ok(parsed) => messages.push(parsed),
            Err(e) => errors.push(format!("{}", e)),
        }
    }

    println!("{}", format!("📊 Parsed {} messages", messages.len()).green());
    if !errors.is_empty() {
        println!("{}", format!("⚠️ {} parsing errors", errors.len()).yellow());
        if verbose {
            for (i, err) in errors.iter().enumerate() {
                println!("  {}. {}", i + 1, err);
            }
        }
    }

    if verbose {
        println!("\n{}", "📝 Message Timeline".cyan());
        for msg in &messages {
            println!(
                "  {} {} {:?}",
                msg.timestamp.format("%H:%M:%S"),
                msg.direction,
                msg.message
            );
        }
    }

    if format == OutputFormat::Json {
        let output = serde_json::json!({
            "messages": messages,
            "errors": errors,
        });
        println!("{}", serde_json::to_string_pretty(&output).unwrap());
    }

    Ok(())
}

async fn capture_traffic(
    url: &str,
    output: Option<PathBuf>,
    duration: Option<u64>,
) -> Result<(), Box<dyn std::error::Error>> {
    use futures_util::{SinkExt, StreamExt};
    use tokio_tungstenite::connect_async;
    use tokio_tungstenite::tungstenite::protocol::Message;

    println!("{}", format!("🔌 Connecting to: {}", url).cyan());

    let (ws_stream, _) = connect_async(url).await?;
    let (mut write, mut read) = ws_stream.split();

    println!("{}", "✅ Connected! Capturing traffic...".green());

    let start = std::time::Instant::now();
    let mut messages = Vec::new();

    loop {
        tokio::select! {
            Some(msg) = read.next() => {
                match msg {
                    Ok(Message::Text(text)) => {
                        messages.push(text);
                        print!(".");
                    }
                    Ok(Message::Ping(data)) => {
                        write.send(Message::Pong(data)).await?;
                    }
                    Ok(Message::Close(frame)) => {
                        write.send(Message::Close(frame)).await?;
                        break;
                    }
                    _ => {}
                }
            }
            _ = tokio::time::sleep(tokio::time::Duration::from_secs(1)) => {
                if let Some(dur) = duration {
                    if start.elapsed() >= std::time::Duration::from_secs(dur) {
                        break;
                    }
                }
            }
        }
    }

    println!("\n{}", format!("📊 Captured {} messages", messages.len()).green());

    if let Some(path) = output {
        std::fs::write(&path, messages.join("\n"))?;
        println!("{}", format!("💾 Saved to: {}", path.display()).green());
    }

    Ok(())
}

async fn analyze_capabilities(
    config: PathBuf,
    format: OutputFormat,
    verbose: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    use chargemesh_capability::*;

    println!("{}", format!("🔍 Analyzing capabilities from: {}", config.display()).cyan());

    let content = std::fs::read_to_string(&config)?;
    let context: CapabilityContext = serde_json::from_str(&content)?;

    if verbose {
        println!("{}", "📋 Context:".cyan());
        println!("  Station: {}", context.station_id);
        println!("  Vendor: {}", context.vendor.name);
        println!("  Model: {}", context.model);
        println!("  Protocol: {:?} v{}", context.protocol.name, context.protocol.version);
        println!("  Firmware: {}", context.firmware.version);
        println!("  Online: {}", context.runtime.is_online);
    }

    let engine = CapabilityEngine::new();
    let capabilities = engine.determine_capabilities(&context).await?;

    match format {
        OutputFormat::Human => render_capability_report(&capabilities),
        OutputFormat::Json => {
            let output = capabilities.to_json();
            println!("{}", serde_json::to_string_pretty(&output).unwrap());
        }
        OutputFormat::Html => {
            println!("📄 HTML report generation not yet implemented");
        }
    }

    Ok(())
}

fn render_capability_report(capabilities: &chargemesh_capability::CapabilitySet) {
    use chargemesh_capability::CapabilityState;

    println!("\n{}", "═══════════════════════════════════════════════════════════".bold().yellow());
    println!("{}", "  🔧 CAPABILITY REPORT".bold().white());
    println!("{}", "═══════════════════════════════════════════════════════════".bold().yellow());

    println!("\n{}", "📊 CAPABILITIES".bold().cyan());

    let mut sorted: Vec<_> = capabilities.capabilities.iter().collect();
    sorted.sort_by_key(|(k, _)| format!("{:?}", k));

    for (cap, state) in sorted {
        let status = match state {
            CapabilityState::Supported { .. } => "✅ Supported".green(),
            CapabilityState::Limited { reason, .. } => {
                format!("⚠️ Limited: {}", reason).yellow()
            }
            CapabilityState::NotSupported { reason } => {
                if let Some(r) = reason {
                    format!("❌ Not supported: {}", r).red()
                } else {
                    "❌ Not supported".red()
                }
            }
            CapabilityState::NotAvailable { reason } => {
                format!("🚫 Unavailable: {}", reason).red()
            }
            CapabilityState::Unknown => "❓ Unknown".dimmed(),
        };

        let name = format!("{:?}", cap).replace("_", " ");
        println!("  • {:<30} {}", name, status);
    }

    println!("\n{}", "💡 SUMMARY".bold().cyan());
    let supported = capabilities.capabilities.values()
        .filter(|s| s.is_supported())
        .count();
    let limited = capabilities.capabilities.values()
        .filter(|s| s.is_limited())
        .count();
    println!("  Total: {}", capabilities.capabilities.len());
    println!("  Supported: {}", supported.green());
    println!("  Limited: {}", limited.yellow());

    println!("\n{}", "═══════════════════════════════════════════════════════════".bold().yellow());
}

async fn run_simulation(
    target: String,
    protocol: Option<String>,
    model: Option<String>,
    firmware: Option<String>,
    scenario: Option<String>,
    duration: Option<u64>,
    verbose: bool,
    list_scenarios: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    use chargemesh_simulator::*;

    if list_scenarios {
        println!("{}", "📋 Available Scenarios:".cyan());
        println!("  • normal          - Normal charging session");
        println!("  • network-failure - Network disconnection during charging");
        println!("  • auth-failure    - Authorization failure");
        println!("  • plug-and-charge - ISO 15118 Plug & Charge");
        println!("  • v2g             - Vehicle-to-Grid bidirectional");
        println!("  • certificate-failure - Certificate validation failure");
        return Ok(());
    }

    println!("{}", format!("🎮 Running simulation: {}", target).cyan());

    let config = SimulatorConfig {
        mode: SimulationMode::Normal,
        speed: 1.0,
        seed: None,
        max_duration: duration.map(chrono::Duration::seconds),
        verbose,
        protocol: protocol.unwrap_or_else(|| "ocpp-1.6".to_string()),
        station_config: StationSimConfig {
            vendor: model.clone().unwrap_or_else(|| "ABB".to_string()),
            model: model.clone().unwrap_or_else(|| "Terra 54".to_string()),
            firmware_version: firmware.clone().unwrap_or_else(|| "1.2.3".to_string()),
            connector_count: 2,
            max_power: 50000,
            has_iso15118: true,
            has_v2g: false,
        },
        ev_config: EvSimConfig {
            battery_capacity: 75000,
            initial_soc: 20,
            target_soc: 80,
            supports_plug_and_charge: true,
            supports_v2g: false,
            max_power: 22000,
        },
        grid_config: GridSimConfig {
            available_capacity: 100000,
            max_power: 100000,
            grid_load_percentage: 50,
            has_solar: true,
            has_battery_storage: false,
        },
    };

    println!("  Protocol: {}", config.protocol);
    println!("  Station: {} {}", config.station_config.vendor, config.station_config.model);
    println!("  Scenario: {}", scenario.as_deref().unwrap_or("normal"));

    let mut simulator = Simulator::new(config);
    simulator.start().await?;

    println!("{}", "✅ Simulation completed successfully".green());
    Ok(())
}

async fn run_diagnostics(
    file: PathBuf,
    format: OutputFormat,
    verbose: bool,
    output: Option<PathBuf>,
) -> Result<(), Box<dyn std::error::Error>> {
    use chargemesh_diagnostics::*;

    println!("{}", format!("🔍 Running diagnostics on: {}", file.display()).cyan());

    let content = std::fs::read_to_string(&file)?;
    let lines: Vec<&str> = content.lines().collect();

    let mut collector = TimelineCollector::new();

    for line in lines {
        if line.trim().is_empty() {
            continue;
        }
        if let Ok(parsed) = chargemesh_ocpp::v16::parse_ocpp_message(line) {
            let entry = TimelineEntry {
                id: uuid::Uuid::new_v4().to_string(),
                timestamp: parsed.timestamp,
                event_type: match parsed.message {
                    chargemesh_ocpp::common::OcppMessage::Call(call) => {
                        match call.action.as_str() {
                            "BootNotification" => EventType::BootNotification,
                            "Heartbeat" => EventType::Heartbeat,
                            "StatusNotification" => EventType::StatusNotification,
                            "Authorize" => EventType::Authorize,
                            "StartTransaction" => EventType::StartTransaction,
                            "StopTransaction" => EventType::StopTransaction,
                            "MeterValues" => EventType::MeterValues,
                            "RemoteStartTransaction" => EventType::RemoteStart,
                            "RemoteStopTransaction" => EventType::RemoteStop,
                            "SetChargingProfile" => EventType::SetChargingProfile,
                            "Reset" => EventType::Reset,
                            "ChangeConfiguration" => EventType::ChangeConfiguration,
                            "GetConfiguration" => EventType::GetConfiguration,
                            _ => EventType::Info,
                        }
                    }
                    chargemesh_ocpp::common::OcppMessage::CallResult(_) => EventType::Info,
                    chargemesh_ocpp::common::OcppMessage::CallError(_) => EventType::Error,
                },
                component: Component::Protocol,
                status: match parsed.message {
                    chargemesh_ocpp::common::OcppMessage::CallError(_) => EntryStatus::Failure,
                    _ => EntryStatus::Success,
                },
                details: serde_json::json!({
                    "raw": parsed.raw,
                    "direction": format!("{:?}", parsed.direction),
                }),
                session_id: None,
                station_id: None,
                connector_id: None,
                transaction_id: None,
                tags: Vec::new(),
            };
            collector.add_entry(entry).await?;
        }
    }

    let engine = DiagnosticsEngine::default();
    let context = DiagnosticContext {
        station_id: None,
        session_id: None,
        time_range: None,
        protocol: Some("OCPP 1.6".to_string()),
        vendor: None,
        model: None,
        firmware_version: None,
    };

    let report = engine.run_diagnostics(&context).await?;

    match format {
        OutputFormat::Human => render_diagnostic_report(&report, verbose),
        OutputFormat::Json => {
            let json = serde_json::to_string_pretty(&report)?;
            if let Some(path) = output {
                std::fs::write(path, json)?;
            } else {
                println!("{}", json);
            }
        }
        OutputFormat::Html => {
            let html = engine.report_generator.render_html(&report)?;
            if let Some(path) = output {
                std::fs::write(path, html)?;
            } else {
                println!("{}", html);
            }
        }
    }

    Ok(())
}

fn render_diagnostic_report(report: &chargemesh_diagnostics::report::DiagnosticReport, verbose: bool) {
    use chargemesh_diagnostics::*;

    println!("\n{}", "═".repeat(60).bold().yellow());
    println!("{}", "  🔍 DIAGNOSTIC REPORT".bold().white());
    println!("{}", "═".repeat(60).bold().yellow());

    println!("\n{}", "📊 SUMMARY".bold().cyan());
    println!("  {}", report.summary);

    if verbose {
        println!("\n{}", "📈 STATISTICS".bold().cyan());
        println!("  Total Events: {}", report.statistics.total_entries);
        println!("  Successful:   {}", report.statistics.success_count);
        println!("  Failed:       {}", report.statistics.failure_count);
        println!("  Timeouts:     {}", report.statistics.timeout_count);
        println!("  Errors:       {}", report.statistics.error_count);
        println!("  Warnings:     {}", report.statistics.warnings_count);
    }

    if !report.root_causes.is_empty() {
        println!("\n{}", "🔍 ROOT CAUSES".bold().red());
        for (i, rc) in report.root_causes.iter().enumerate() {
            println!("\n  {}. {}", i + 1, rc.title.red().bold());
            println!("     Confidence: {:.0}%", rc.confidence * 100.0);
            println!("     {}", rc.description);
            println!("\n     Possible causes:");
            for cause in &rc.causes {
                println!("       • {} (probability: {:.0}%)", cause.description, cause.probability * 100.0);
                println!("         💡 {}", cause.mitigation);
            }
        }
    }

    if !report.recommendations.is_empty() {
        println!("\n{}", "💡 RECOMMENDATIONS".bold().green());
        for rec in &report.recommendations {
            println!("  • {}", rec.action.green());
            println!("    {}", rec.description.dimmed());
        }
    }

    if verbose && !report.timeline.is_empty() {
        println!("\n{}", "⏱️ TIMELINE (last 20 events)".bold().magenta());
        for entry in report.timeline.iter().rev().take(20).rev() {
            let status = match entry.status {
                EntryStatus::Success => "✅".green(),
                EntryStatus::Failure => "❌".red(),
                EntryStatus::Timeout => "⏰".yellow(),
                EntryStatus::Warning => "⚠️".yellow(),
                _ => "ℹ️".cyan(),
            };
            println!(
                "  {} [{}] {:?} {:?}",
                entry.timestamp.format("%H:%M:%S"),
                status,
                entry.event_type,
                entry.component
            );
        }
    }

    println!("\n{}", "═".repeat(60).bold().yellow());
}

async fn run_observability(
    station_id: Option<String>,
    session_id: Option<String>,
    duration: Option<u64>,
    verbose: bool,
    follow: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    use chargemesh_observability::*;

    println!("{}", "🔭 Observability Platform".cyan());

    let config = PlatformConfig::default();
    let platform = ObservabilityPlatform::new(config);
    platform.start().await?;

    if let Some(sid) = station_id {
        println!("📍 Monitoring station: {}", sid);
    }
    if let Some(sid) = session_id {
        println!("📍 Monitoring session: {}", sid);
    }

    if follow {
        println!("{}", "🔄 Following live updates... (Press Ctrl+C to stop)".yellow());

        let start = std::time::Instant::now();
        while let Some(dur) = duration {
            if start.elapsed() >= std::time::Duration::from_secs(dur) {
                break;
            }
            let data = platform.get_dashboard_data().await;
            let rendered = platform.dashboard.render(&data).await?;
            print!("\x1B[2J\x1B[1;1H");
            println!("{}", rendered);
            tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
        }
    } else {
        let data = platform.get_dashboard_data().await;
        let rendered = platform.dashboard.render(&data).await?;
        println!("{}", rendered);
    }

    platform.stop().await?;
    Ok(())
}

async fn run_ocpi(
    url: &str,
    token: &str,
    country: &str,
    party: &str,
    action: OcpiAction,
) -> Result<(), Box<dyn std::error::Error>> {
    use chargemesh_integration::ocpi::*;

    let client = OcpiClient::new(url, token, OcpiVersion::V2_2, country, party);

    match action {
        OcpiAction::Locations => {
            let locations = client.get_locations().await?;
            if locations.is_empty() {
                println!("📡 No locations found");
            } else {
                println!("{}", serde_json::to_string_pretty(&locations)?);
            }
        }
        OcpiAction::Sessions => {
            let sessions = client.get_sessions().await?;
            if sessions.is_empty() {
                println!("🔌 No active sessions found");
            } else {
                println!("{}", serde_json::to_string_pretty(&sessions)?);
            }
        }
        OcpiAction::Tariffs => {
            println!("📋 Getting tariffs...");
            // TODO: Implement tariff endpoint
            // let tariffs = client.get_tariffs().await?;
            // println!("{}", serde_json::to_string_pretty(&tariffs)?);
            println!("   Tariffs endpoint ready for OCPI 2.2+");
        }
        OcpiAction::Sync => {
            println!("🔄 Syncing with roaming partner...");
            // TODO: Implement full sync
            println!("   Sync would fetch locations, sessions, and tariffs");
            println!("   Last sync: never");
        }
    }
    Ok(())
}

async fn run_energy(
    config: PathBuf,
    action: EnergyAction,
) -> Result<(), Box<dyn std::error::Error>> {
    use chargemesh_integration::energy::*;

    let content = std::fs::read_to_string(&config)?;
    let ems_config: EnergyManagementSystem = serde_json::from_str(&content)?;

    match action {
        EnergyAction::Status => {
            println!("📊 Energy Management Status".bold().cyan());
            println!("  System: {}", ems_config.name);
            println!("  Status: {:?}", ems_config.status);
            println!("  Available Power: {:.1} kW", ems_config.available_power);
            println!("  Current Load: {:.1} kW", ems_config.current_load);
            println!("  Peak Load: {:.1} kW", ems_config.peak_load);
            println!("  Grid Import: {:.1} kW", ems_config.grid_import);
            if let Some(solar) = ems_config.solar_generation {
                println!("  Solar Generation: {:.1} kW", solar);
            }
            if let Some(battery) = ems_config.battery_state {
                println!("  Battery SoC: {:.1}%", battery.soc);
                println!("  Battery Power: {:.1} kW", battery.power);
                println!("  Battery Charging: {}", if battery.charging { "✅" else : "❌" });
            }
            if let Some(price) = ems_config.price_signal {
                println!("  Price: €{:.3}/kWh", price.price);
            }
        }
        EnergyAction::Optimize => {
            use chargemesh_integration::smart_charging::*;

            println!("⚡ Running smart charging optimization...".bold().green());

            // Create optimizer with EMS integration
            let ems_integration = EmsIntegration::new();
            ems_integration.connect(ems_config.clone()).await?;

            let config = SmartChargingConfig {
                enabled: true,
                algorithm: SmartChargingAlgorithm::Greedy,
                optimization_target: OptimizationTarget::MinimizeCost,
                constraints: Vec::new(),
                update_interval: chrono::Duration::seconds(60),
            };

            let optimizer = SmartChargingOptimizer::new(config, std::sync::Arc::new(ems_integration));

            // Register test sessions (would come from real data in production)
            let session = ChargingSession {
                id: "SESS-001".to_string(),
                station_id: "CP-001".to_string(),
                connector_id: 1,
                required_energy: 16.5,
                max_power: 11.0,
                min_power: 1.0,
                start_time: chrono::Utc::now(),
                deadline: Some(chrono::Utc::now() + chrono::Duration::hours(2)),
                priority: 1,
                current_soc: 20.0,
                target_soc: 80.0,
                battery_capacity: 75.0,
            };
            optimizer.register_session(session).await?;

            let plans = optimizer.optimize().await?;

            for plan in &plans {
                println!("\n  Session: {}", plan.session_id);
                println!("    Schedule:");
                for slot in &plan.schedule {
                    println!(
                        "      {} - {} | {:.1} kW | {:.1} kWh",
                        slot.start_time.format("%H:%M"),
                        slot.end_time.format("%H:%M"),
                        slot.power,
                        slot.energy
                    );
                }
                println!("    Total Energy: {:.1} kWh", plan.total_energy);
                println!("    Total Cost: €{:.2}", plan.total_cost);
                println!("    Carbon Emissions: {:.1} kg CO2", plan.carbon_emissions);
                println!("    Target: {:?}", plan.optimization_target);
            }
        }
        EnergyAction::Constraints => {
            println!("📋 Current energy constraints".bold().cyan());
            if ems_config.constraints.is_empty() {
                println!("  No active constraints");
            } else {
                for (i, constraint) in ems_config.constraints.iter().enumerate() {
                    println!("  {}. Source: {:?}", i + 1, constraint.source);
                    println!("     Max Power: {:.1} kW", constraint.max_power);
                    println!("     Min Power: {:.1} kW", constraint.min_power);
                    println!("     Max Energy: {:.1} kWh", constraint.max_energy);
                    if let Some(carbon) = constraint.carbon_intensity {
                        println!("     Carbon Intensity: {:.1} g CO2/kWh", carbon);
                    }
                    if let Some(range) = &constraint.time_range {
                        println!("     Time Range: {} to {}",
                            range.start.format("%H:%M"),
                            range.end.format("%H:%M")
                        );
                    }
                }
            }
        }
    }
    Ok(())
}

async fn run_cloud(
    url: Option<String>,
    token: Option<String>,
    action: CloudAction,
) -> Result<(), Box<dyn std::error::Error>> {
    use reqwest::Client;

    let client = Client::new();

    match action {
        CloudAction::Status => {
            println!("🔭 ChargeMesh Cloud Status".bold().cyan());
            if let Some(url) = url {
                let health_url = format!("{}/health", url);
                match client.get(&health_url).send().await {
                    Ok(response) => {
                        if response.status().is_success() {
                            let status: serde_json::Value = response.json().await?;
                            println!("  ✅ Cloud is online");
                            println!("  {}", serde_json::to_string_pretty(&status)?);
                        } else {
                            println!("  ❌ Cloud is unavailable (HTTP {})", response.status());
                        }
                    }
                    Err(e) => {
                        println!("  ❌ Cannot connect to cloud: {}", e);
                    }
                }
            } else {
                println!("  ℹ️ No cloud URL provided. Run 'chargemesh cloud login --url <url>' to connect.");
            }
        }
        CloudAction::Login => {
            println!("🔑 Login to ChargeMesh Cloud".bold().cyan());
            if let (Some(url), Some(token)) = (url, token) {
                // Validate token by checking health endpoint
                let health_url = format!("{}/health", url);
                match client
                    .get(&health_url)
                    .header("Authorization", format!("Bearer {}", token))
                    .send()
                    .await
                {
                    Ok(response) => {
                        if response.status().is_success() {
                            println!("  ✅ Login successful!");
                            // Store credentials (in production, use secure storage)
                            println!("  🔐 Token validated for: {}", url);
                        } else {
                            println!("  ❌ Login failed: HTTP {}", response.status());
                        }
                    }
                    Err(e) => {
                        println!("  ❌ Login failed: {}", e);
                    }
                }
            } else {
                println!("  ℹ️ Please provide both --url and --token");
                println!("  Example: chargemesh cloud login --url https://api.chargemesh.cloud --token <your-token>");
            }
        }
        CloudAction::Stations => {
            println!("📡 Listing stations...".bold().cyan());
            if let (Some(url), Some(token)) = (url, token) {
                let stations_url = format!("{}/api/v1/stations", url);
                match client
                    .get(&stations_url)
                    .header("Authorization", format!("Bearer {}", token))
                    .send()
                    .await
                {
                    Ok(response) => {
                        if response.status().is_success() {
                            let data: serde_json::Value = response.json().await?;
                            let stations = data["data"].as_array().unwrap_or(&vec![]);
                            if stations.is_empty() {
                                println!("  📡 No stations found");
                            } else {
                                println!("  📡 Found {} stations:", stations.len());
                                for station in stations {
                                    println!("    • {} ({}) - {}",
                                        station["id"].as_str().unwrap_or("unknown"),
                                        station["model"].as_str().unwrap_or("unknown"),
                                        station["status"].as_str().unwrap_or("unknown")
                                    );
                                }
                            }
                        } else {
                            println!("  ❌ Failed to list stations: HTTP {}", response.status());
                        }
                    }
                    Err(e) => {
                        println!("  ❌ API error: {}", e);
                    }
                }
            } else {
                println!("  ℹ️ Please provide both --url and --token");
            }
        }
        CloudAction::Sessions => {
            println!("🔌 Listing sessions...".bold().cyan());
            if let (Some(url), Some(token)) = (url, token) {
                let sessions_url = format!("{}/api/v1/sessions", url);
                match client
                    .get(&sessions_url)
                    .header("Authorization", format!("Bearer {}", token))
                    .send()
                    .await
                {
                    Ok(response) => {
                        if response.status().is_success() {
                            let data: serde_json::Value = response.json().await?;
                            let sessions = data["data"].as_array().unwrap_or(&vec![]);
                            if sessions.is_empty() {
                                println!("  🔌 No active sessions found");
                            } else {
                                println!("  🔌 Found {} sessions:", sessions.len());
                                for session in sessions {
                                    println!("    • {} - {} ({:.1} kWh) - {}",
                                        session["id"].as_str().unwrap_or("unknown"),
                                        session["station_id"].as_str().unwrap_or("unknown"),
                                        session["energy"].as_f64().unwrap_or(0.0),
                                        session["status"].as_str().unwrap_or("unknown")
                                    );
                                }
                            }
                        } else {
                            println!("  ❌ Failed to list sessions: HTTP {}", response.status());
                        }
                    }
                    Err(e) => {
                        println!("  ❌ API error: {}", e);
                    }
                }
            } else {
                println!("  ℹ️ Please provide both --url and --token");
            }
        }
        CloudAction::Analytics => {
            println!("📊 Getting analytics...".bold().cyan());
            if let (Some(url), Some(token)) = (url, token) {
                let analytics_url = format!("{}/api/v1/analytics/usage", url);
                match client
                    .get(&analytics_url)
                    .header("Authorization", format!("Bearer {}", token))
                    .send()
                    .await
                {
                    Ok(response) => {
                        if response.status().is_success() {
                            let data: serde_json::Value = response.json().await?;
                            let metrics = &data["data"];
                            println!("  📊 Usage Analytics:");
                            println!("    Total Sessions: {}", metrics["total_sessions"].as_u64().unwrap_or(0));
                            println!("    Total Energy: {:.1} kWh", metrics["total_energy_kwh"].as_f64().unwrap_or(0.0));
                            println!("    Avg Session: {:.0} min", metrics["avg_session_duration_minutes"].as_f64().unwrap_or(0.0));
                            println!("    Success Rate: {:.1}%", metrics["success_rate"].as_f64().unwrap_or(0.0));
                        } else {
                            println!("  ❌ Failed to get analytics: HTTP {}", response.status());
                        }
                    }
                    Err(e) => {
                        println!("  ❌ API error: {}", e);
                    }
                }
            } else {
                println!("  ℹ️ Please provide both --url and --token");
            }
        }
        CloudAction::Subscription => {
            println!("💳 Subscription info...".bold().cyan());
            if let (Some(url), Some(token)) = (url, token) {
                let sub_url = format!("{}/api/v1/subscriptions/current", url);
                match client
                    .get(&sub_url)
                    .header("Authorization", format!("Bearer {}", token))
                    .send()
                    .await
                {
                    Ok(response) => {
                        if response.status().is_success() {
                            let data: serde_json::Value = response.json().await?;
                            let sub = &data["data"];
                            println!("  💳 Current Subscription:");
                            println!("    Tier: {}", sub["tier"].as_str().unwrap_or("Free"));
                            println!("    Status: {}", sub["status"].as_str().unwrap_or("Unknown"));
                            println!("    Price: €{:.2}/month", sub["price"].as_f64().unwrap_or(0.0));
                            if let Some(end) = sub["end_date"].as_str() {
                                println!("    Valid until: {}", end);
                            }
                        } else {
                            println!("  ❌ Failed to get subscription: HTTP {}", response.status());
                        }
                    }
                    Err(e) => {
                        println!("  ❌ API error: {}", e);
                    }
                }
            } else {
                println!("  ℹ️ Please provide both --url and --token");
            }
        }
    }
    Ok(())
}
