mod commands;
mod executers;
mod model;
mod templates;
mod utils;

use anyhow::{Ok, Result};
use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Init {},
    Add(AddArgs),
    Install {},
    Update {},
}

#[derive(Args)]
struct AddArgs {
    repository: String,
    #[clap(short, long)]
    workspace: Option<String>,
    #[clap(short, long)]
    branch: Option<String>,
    #[clap(short, long)]
    excludes: Option<String>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Init {} => {
            commands::init::run()?;
        }
        Commands::Add(args) => {
            commands::add::run(
                args.repository.clone(),
                args.workspace.clone(),
                args.branch.clone(),
            )?;
        }
        Commands::Install {} => todo!(),
        Commands::Update {} => todo!(),
    }
    Ok(())
}
