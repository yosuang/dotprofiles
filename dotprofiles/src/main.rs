mod activate;
mod pkg;

use std::str::FromStr;
use std::{fmt::Debug, process};

use crate::activate::{Activate, handle_activate};
use crate::pkg::{Package, handle_package};
use clap::{Parser, Subcommand};
use dotprofiles_config::config::{Config, parse_config};
use log::{LevelFilter, debug, error};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(long)]
    verbose: bool,

    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    Activate(Activate),
    Package(Package),
}

const BIN_NAME: &str = env!("CARGO_BIN_NAME");

fn main() {
    let cli = Cli::parse();
    let configuration = parse_config();

    init_logger(&cli, &configuration);

    debug!("{configuration:?}");
    debug!("{cli:?}");

    if let Err(e) = run(cli) {
        error!("Run failed, err:{:?}", e);
        process::exit(1);
    }
}

fn init_logger(cli: &Cli, configuration: &Config) {
    let mut builder = env_logger::builder();
    if cli.verbose {
        builder.filter(Some(BIN_NAME), LevelFilter::Debug);
    } else if let Ok(level) = LevelFilter::from_str(configuration.log_level.as_str()) {
        builder.filter(Some(BIN_NAME), level);
    } else {
        builder.filter(Some(BIN_NAME), LevelFilter::Info);
    }
    builder.init();
}

fn run(cli: Cli) -> Result<(), &'static str> {
    match cli.command {
        Command::Activate(activate) => handle_activate(&activate),
        Command::Package(package) => handle_package(&package),
    }
}
