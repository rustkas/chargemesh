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
    /// Inspect a charging station or trace file
    Inspect {
        /// OCPP trace file to analyze
        #[arg(short, long)]
        file: Option<PathBuf>,
        
        /// Connect to live charger via WebSocket
        #[arg(short, long)]
        connect: Option<String>,
        
        /// Output format (human, json, yaml)
        #[arg(short, long, default_value = "human")]
        format: OutputFormat,
        
        /// Verbose output
        #[arg(short, long)]
        verbose: bool,
    },
    
    /// Run a simulation
    Simulate {
        /// Scenario to run
        #[arg(short, long)]
        scenario: SimulationScenario,
        
        /// Number of iterations
        #[arg(short, long, default_value = "1")]
        iterations: u32,
    },
    
    /// Show version information
    Version,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OutputFormat {
    Human,
    Json,
    Yaml,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SimulationScenario {
    Normal,
    ConnectorLock,
    NetworkFailure,
    AuthFailure,
    CertificateFailure,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter("chargemesh=info")
        .init();
    
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Inspect { file, connect, format, verbose } => {
            if let Some(path) = file {
                inspect_file(path, format, verbose).await?;
            } else if let Some(url) = connect {
                inspect_live(&url, format, verbose).await?;
            } else {
                println!("{}", "Please provide either --file or --connect".yellow());
                println!("{}", "Example: chargemesh inspect --file trace.ocpp".dimmed());
                println!("{}", "Example: chargemesh inspect --connect ws://charger:9000".dimmed());
            }
        }
        Commands::Simulate { scenario, iterations } => {
            run_simulation(scenario, iterations).await?;
        }
        Commands::Version => {
            println!("ChargeMesh v{}", env!("CARGO_PKG_VERSION"));
        }
    }
    
    Ok(())
}

async fn inspect_file(
    path: PathBuf,
    _format: OutputFormat,
    _verbose: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", format!("📂 Inspecting file: {}", path.display()).cyan());
    
    let content = std::fs::read_to_string(&path)?;
    let lines: Vec<&str> = content.lines().collect();
    
    let mut inspector = chargemesh_inspector::Inspector::new();
    let mut message_count = 0;
    
    for line in lines {
        if let Ok(parsed) = chargemesh_ocpp::v16::parse_ocpp_message(line) {
            inspector.add_message(parsed);
            message_count += 1;
        }
    }
    
    println!("{}", format!("📊 Processed {} OCPP messages", message_count).green());
    
    let report = inspector.generate_report();
    
    // Render based on format
    render_human_report(&report);
    
    Ok(())
}

async fn inspect_live(
    _url: &str,
    _format: OutputFormat,
    _verbose: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", "🔌 Connecting to live charger...".cyan());
    println!("{}", "WebSocket capture not yet implemented".yellow());
    Ok(())
}

async fn run_simulation(
    _scenario: SimulationScenario,
    _iterations: u32,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", "🎮 Running simulation...".cyan());
    println!("{}", "Simulation not yet implemented".yellow());
    Ok(())
}

fn render_human_report(report: &chargemesh_inspector::InspectionReport) {
    println!("\n{}", "═".repeat(60).bold().yellow());
    println!("{}", "  🔍 CHARGE MESH INSPECTOR - DIAGNOSIS REPORT".bold().white());
    println!("{}", "═".repeat(60).bold().yellow());
    
    println!("\n{}", "📋 STATION INFO".bold().cyan());
    println!("  Vendor:  {}", report.station.vendor);
    println!("  Model:   {}", report.station.model);
    println!("  Status:  {:?}", report.station.state);
    
    println!("\n{}", "📊 SUMMARY".bold().cyan());
    println!("  {}", report.summary);
    
    if !report.errors.is_empty() {
        println!("\n{}", "❌ ERRORS".bold().red());
        for (i, error) in report.errors.iter().enumerate() {
            println!("  {}. {}", i + 1, error.description.red());
            if let Some(root_cause) = &error.root_cause {
                println!("     💡 {}", root_cause.yellow());
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
    
    println!("\n{}", "═".repeat(60).bold().yellow());
}
