use clap::{Parser, Subcommand};
use commands::watch::watch_command;
use std::path::PathBuf;

mod commands;
mod fs_watcher;
mod monorepo;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Starts the watch mode
    Watch {
        /// The directory to watch
        #[arg(short, long)]
        cwd: PathBuf,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Watch { cwd }) => watch_command(cwd).await,
        None => Ok(()),
    }
}
