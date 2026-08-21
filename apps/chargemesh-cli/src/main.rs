//! ChargeMesh CLI — Command-line interface

use clap::{Parser, Subcommand};
use colored::*;
use std::path::PathBuf;

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
    Parse {
        #[arg(short, long)]
        file: PathBuf,
        #[arg(short, long, default_value = "human")]
        format: OutputFormat,
        #[arg(short, long)]
        verbose: bool,
    },

    Capture {
        #[arg(short, long)]
        url: String,
        #[arg(short, long)]
        output: Option<PathBuf>,
        #[arg(short, long)]
        duration: Option<u64>,
    },

    Capability {
        #[arg(short, long)]
        config: PathBuf,
        #[arg(short, long, default_value = "human")]
        format: OutputFormat,
        #[arg(short, long)]
        verbose: bool,
    },

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

    Version,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OutputFormat {
    Human,
    Json,
    Html,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter("chargemesh=info")
        .init();

    let cli = Cli::parse();

    match cli.command {
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
            run_simulation(target, protocol, model, firmware, scenario, duration, verbose, list_scenarios).await?;
        }
        Commands::Diagnose { file, format, verbose, output } => {
            run_diagnostics(file, format, verbose, output).await?;
        }
        Commands::Version => {
            println!("ChargeMesh v{}", env!("CARGO_PKG_VERSION"));
        }
    }

    Ok(())
}

async fn run_diagnostics(
    path: PathBuf,
    format: OutputFormat,
    verbose: bool,
    output: Option<PathBuf>,
) -> Result<(), Box<dyn std::error::Error>> {
    use chargemesh_diagnostics::*;

    println!("{}", format!("🔍 Running diagnostics on: {}", path.display()).cyan());

    let content = std::fs::read_to_string(&path)?;
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
        OutputFormat::Human => render_human_report(&report, verbose),
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

fn render_human_report(report: &chargemesh_diagnostics::report::DiagnosticReport, verbose: bool) {
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