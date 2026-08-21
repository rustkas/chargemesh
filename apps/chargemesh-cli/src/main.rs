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

    Energy {
        #[arg(short, long)]
        config: PathBuf,
        #[command(subcommand)]
        action: EnergyAction,
    },

    Version,
}

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

impl Cli {
    pub async fn run(self) -> Result<(), Box<dyn std::error::Error>> {
        match self.command {
            Commands::Ocpi { url, token, country, party, action } => {
                use chargemesh_integration::ocpi::*;
                let client = OcpiClient::new(&url, &token, OcpiVersion::V2_2, &country, &party);
                match action {
                    OcpiAction::Locations => {
                        let locations = client.get_locations().await?;
                        println!("{}", serde_json::to_string_pretty(&locations)?);
                    }
                    OcpiAction::Sessions => {
                        let sessions = client.get_sessions().await?;
                        println!("{}", serde_json::to_string_pretty(&sessions)?);
                    }
                    _ => println!("OCPI action not fully implemented yet"),
                }
            }
            Commands::Energy { config, action } => {
                use chargemesh_integration::energy::*;
                match action {
                    EnergyAction::Status => {
                        println!("📊 Energy Management Status");
                        // Implementation would connect to EMS
                    }
                    EnergyAction::Optimize => {
                        println!("⚡ Running smart charging optimization...");
                        // Would use SmartChargingOptimizer
                    }
                    EnergyAction::Constraints => {
                        println!("📋 Current energy constraints");
                    }
                }
            }
            _ => {}
        }
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    cli.run().await?;
    Ok(())
}