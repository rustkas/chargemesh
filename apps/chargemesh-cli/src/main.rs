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
        Commands::Observe {
            station_id,
            session_id,
            duration,
            verbose,
            follow,
        } => {
            run_observability(station_id, session_id, duration, verbose, follow).await?;
        }
        Commands::Version => {
            println!("ChargeMesh v{}", env!("CARGO_PKG_VERSION"));
        }
    }

    Ok(())
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
        // Collect station-specific metrics
    }

    if let Some(sid) = session_id {
        println!("📍 Monitoring session: {}", sid);
        // Collect session-specific metrics
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