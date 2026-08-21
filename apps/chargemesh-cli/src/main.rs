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
        #[arg(short, long)]
        config: PathBuf,
        #[arg(short, long, default_value = "human")]
        format: OutputFormat,
        #[arg(short, long)]
        verbose: bool,
    },

    /// Run a simulation
    Simulate {
        /// Simulation target (charger, ev, grid)
        #[arg(short, long)]
        target: String,

        /// Protocol to use
        #[arg(short, long)]
        protocol: Option<String>,

        /// Model of the device
        #[arg(short, long)]
        model: Option<String>,

        /// Firmware version
        #[arg(short, long)]
        firmware: Option<String>,

        /// Scenario to run (normal, network-failure, auth-failure, etc.)
        #[arg(short, long)]
        scenario: Option<String>,

        /// Duration in seconds
        #[arg(short, long)]
        duration: Option<u64>,

        /// Verbose output
        #[arg(short, long)]
        verbose: bool,

        /// List available scenarios
        #[arg(long)]
        list_scenarios: bool,
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
        Commands::Version => {
            println!("ChargeMesh v{}", env!("CARGO_PKG_VERSION"));
        }
    }

    Ok(())
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

    let config = chargemesh_simulator::SimulatorConfig {
        mode: chargemesh_simulator::SimulationMode::Normal,
        speed: 1.0,
        seed: None,
        max_duration: duration.map(chrono::Duration::seconds),
        verbose,
        protocol: protocol.unwrap_or_else(|| "ocpp-1.6".to_string()),
        station_config: chargemesh_simulator::StationSimConfig {
            vendor: model.clone().unwrap_or_else(|| "ABB".to_string()),
            model: model.clone().unwrap_or_else(|| "Terra 54".to_string()),
            firmware_version: firmware.clone().unwrap_or_else(|| "1.2.3".to_string()),
            connector_count: 2,
            max_power: 50000,
            has_iso15118: true,
            has_v2g: false,
        },
        ev_config: chargemesh_simulator::EvSimConfig {
            battery_capacity: 75000,
            initial_soc: 20,
            target_soc: 80,
            supports_plug_and_charge: true,
            supports_v2g: false,
            max_power: 22000,
        },
        grid_config: chargemesh_simulator::GridSimConfig {
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

    // Run the scenario
    let scenario_runner = chargemesh_simulator::core::ScenarioRunner::new();
    let scenario = match scenario.as_deref() {
        Some("normal") => chargemesh_simulator::core::Scenarios::normal_session(),
        Some("network-failure") => chargemesh_simulator::core::Scenarios::network_failure(),
        Some("auth-failure") => chargemesh_simulator::core::Scenarios::auth_failure(),
        Some("plug-and-charge") => chargemesh_simulator::core::Scenarios::plug_and_charge(),
        Some("v2g") => chargemesh_simulator::core::Scenarios::v2g(),
        Some("certificate-failure") => chargemesh_simulator::core::Scenarios::certificate_failure(),
        _ => chargemesh_simulator::core::Scenarios::normal_session(),
    };

    println!("{}", "🔄 Running scenario...".yellow());
    scenario_runner.run(&scenario).await?;

    println!("{}", "✅ Simulation completed successfully".green());
    Ok(())
}