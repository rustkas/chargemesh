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
        /// Path to the OCPP trace file
        #[arg(short, long)]
        file: PathBuf,

        /// Output format (human, json)
        #[arg(short, long, default_value = "human")]
        format: OutputFormat,

        /// Verbose output
        #[arg(short, long)]
        verbose: bool,
    },

    /// Connect to a charger and capture traffic
    Capture {
        /// WebSocket URL of the charger
        #[arg(short, long)]
        url: String,

        /// Output file for captured traffic
        #[arg(short, long)]
        output: Option<PathBuf>,

        /// Duration to capture (seconds)
        #[arg(short, long)]
        duration: Option<u64>,
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
        Commands::Version => {
            println!("ChargeMesh v{}", env!("CARGO_PKG_VERSION"));
        }
    }

    Ok(())
}

async fn parse_file(
    path: PathBuf,
    format: OutputFormat,
    verbose: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", format!("📂 Parsing file: {}", path.display()).cyan());

    let content = std::fs::read_to_string(&path)?;
    let lines: Vec<&str> = content.lines().collect();

    let mut messages = Vec::new();
    let mut errors = Vec::new();

    for line in lines {
        if line.trim().is_empty() {
            continue;
        }
        match chargemesh_ocpp::v16::parse_ocpp_message(line) {
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
    println!("{}", format!("🔌 Connecting to: {}", url).cyan());

    use tokio_tungstenite::connect_async;
    use tokio_tungstenite::tungstenite::protocol::Message;
    use futures_util::{SinkExt, StreamExt};

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