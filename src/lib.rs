#![deny(clippy::all)]

mod commands;
mod executers;
mod model;
mod templates;
mod utils;

#[macro_use]
extern crate napi_derive;

use anyhow::{Ok as AnyOk, Result as AnyResult};
use clap::{Args, Parser, Subcommand};
use napi::{Error as NapiError, Result as NapiResult, Status as NapiStatus};

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
  Update(UpdateArgs),
}

#[derive(Args)]
struct AddArgs {
  repository: String,
  #[clap(short, long)]
  workspaces: Option<Vec<String>>,
  #[clap(short, long)]
  branch: Option<String>,
  #[clap(short, long)]
  excludes: Option<Vec<String>>,
}

#[derive(Args)]
struct UpdateArgs {
  alias: String,
}

fn run() -> AnyResult<()> {
  let cli = Cli::parse();
  match &cli.command {
    Commands::Init {} => {
      commands::init::run()?;
    }
    Commands::Add(args) => {
      commands::add::run(
        args.repository.clone(),
        args.workspaces.clone(),
        args.branch.clone(),
        args.excludes.clone(),
      )?;
    }
    Commands::Install {} => {
      commands::install::run()?;
    }
    Commands::Update(args) => {
      commands::update::run(args.alias.clone())?;
    }
  }
  AnyOk(())
}

#[napi]
pub fn run_napi() -> NapiResult<()> {
  run().map_err(|e| NapiError::new(NapiStatus::GenericFailure, format!("{:?}", e)))
}
