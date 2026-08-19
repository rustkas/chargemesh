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
        
