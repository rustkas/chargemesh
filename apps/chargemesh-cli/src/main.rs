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
    /// Parse and analyze an OCPP trace file
    Parse {
        #[arg(short, long)]
        file: PathBuf,
        #[arg(short, long, default_value = "human")]
        format: OutputFormat,
        #[arg(short, long)]
        verbose: bool,
    },

    /// Connect to a charger and capture traffic
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
        /// Station configuration file (JSON/YAML)
        #[arg(short, long)]
        config: PathBuf,

        /// Output format (human, json)
        #[arg(short, long, default_value = "human")]
        format: OutputFormat,

        /// Verbose output
        #[arg(short, long)]
        verbose: bool,
    },

    /// Show version information
    Version,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OutputFormat {
    Human,
    Json,
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
        Commands::Version => {
            println!("ChargeMesh v{}", env!("CARGO_PKG_VERSION"));
        }
    }

    Ok(())
}

async fn analyze_capabilities(
    path: PathBuf,
    format: OutputFormat,
    verbose: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", format!("🔍 Analyzing capabilities from: {}", path.display()).cyan());

    let content = std::fs::read_to_string(&path)?;
    let context: chargemesh_capability::CapabilityContext = serde_json::from_str(&content)?;

    if verbose {
        println!("{}", "📋 Context:".cyan());
        println!("  Station: {}", context.station_id);
        println!("  Vendor: {}", context.vendor.name);
        println!("  Model: {}", context.model);
        println!("  Protocol: {:?} v{}", context.protocol.name, context.protocol.version);
        println!("  Firmware: {}", context.firmware.version);
        println!("  Online: {}", context.runtime.is_online);
    }

    let engine = chargemesh_capability::CapabilityEngine::new();
    let capabilities = engine.determine_capabilities(&context).await?;

    match format {
        OutputFormat::Human => {
            println!("\n{}", "═══════════════════════════════════════════════════════════".bold().yellow());
            println!("{}", "  🔧 CAPABILITY REPORT".bold().white());
            println!("{}", "═══════════════════════════════════════════════════════════".bold().yellow());

            // Group capabilities by category
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
        OutputFormat::Json => {
            let output = capabilities.to_json();
            println!("{}", serde_json::to_string_pretty(&output).unwrap());
        }
    }

    Ok(())
}