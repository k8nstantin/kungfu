use clap::{Parser, Subcommand};
use anyhow::Result;

mod cli;
mod core;
mod mcp;
mod sync;
mod pkg;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new KungFu Dojo (workspace)
    Init,
    /// Start the local MCP Gateway for agents
    Mcp,
    /// Plant an immutable Seed for deployment
    Seed { 
        /// Version name (e.g. v1.0)
        name: String 
    },
    /// Transcribe DNA into physical files
    Transcribe { 
        /// Destination path (defaults to current dir)
        dest: Option<std::path::PathBuf> 
    },
    /// View the status of the current Dojo
    Status,
    /// Start continuous background sync
    Osmose { 
        /// Central Dojo URL
        url: String 
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Init => {
            cli::init::run()?;
        }
        Commands::Mcp => {
            mcp::server::run_server().await?;
        }
        Commands::Seed { name } => {
            cli::seed::run(name.clone())?;
        }
        Commands::Transcribe { dest } => {
            cli::transcribe::run(dest.clone())?;
        }
        Commands::Status => {
            println!("Dojo is quiet. No active flows.");
        }
        Commands::Osmose { url } => {
            sync::osmosis::begin_osmosis(&url).await?;
        }
    }

    Ok(())
}
