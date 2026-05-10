use clap::{Parser, Subcommand};
use anyhow::Result;

mod cli;
mod core;

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
    /// View the status of the current Dojo
    Status,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Init => {
            cli::init::run()?;
        }
        Commands::Status => {
            println!("Dojo is quiet. No active flows.");
        }
    }

    Ok(())
}

mod pkg;
