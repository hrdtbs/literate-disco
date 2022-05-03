mod commands;
mod model;

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
    Add {},
    Install {},
    Update {},
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Init {} => {
            commands::init::run();
        }
        Commands::Add {} => todo!(),
        Commands::Install {} => todo!(),
        Commands::Update {} => todo!(),
    }
}
