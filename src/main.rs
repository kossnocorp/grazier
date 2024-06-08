use clap::{Parser, Subcommand};
use notify::{RecursiveMode, Watcher};
use notify_debouncer_full::new_debouncer;
use std::path::PathBuf;
use std::time::Duration;

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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Watch { cwd }) => {
            let (tx, rx) = std::sync::mpsc::channel();

            let mut debouncer = new_debouncer(Duration::from_secs(2), None, tx)?;

            debouncer
                .watcher()
                .watch(cwd.as_path(), RecursiveMode::Recursive)?;

            for result in rx {
                match result {
                    Ok(events) => events.iter().for_each(|event| println!("{event:?}")),
                    Err(errors) => errors.iter().for_each(|error| println!("{error:?}")),
                }
                println!();
            }
        }
        None => {}
    }

    Ok(())
}
