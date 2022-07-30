mod commands;
mod executers;
mod model;
mod templates;
mod utils;

use anyhow::{Ok, Result};
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Init {},
    Add {
        repository: String,
        workspace: Option<String>,
    },
    Install {},
    Update {},
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Init {} => {
            commands::init::run();
        }
        Commands::Add {
            repository,
            workspace,
        } => {
            commands::add::run(repository.clone(), workspace.clone())?;
        }
        Commands::Install {} => todo!(),
        Commands::Update {} => todo!(),
    }
    Ok(())
}
