mod activate;
mod pkg;

use std::str::FromStr;
use std::{fmt::Debug, process};

use crate::activate::ActivateSubCommand;
use crate::pkg::PkgSubCommand;
use clap::{Parser, Subcommand};
use dotprofiles_config::config;
use dotprofiles_config::config::Config;
use log::{LevelFilter, debug, error};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(long)]
    verbose: bool,

    #[clap(subcommand)]
    sub: SubCommand,
}

#[derive(Subcommand, Debug)]
enum SubCommand {
    Activate(ActivateSubCommand),
    Pkg(PkgSubCommand),
}

const BIN_NAME: &str = env!("CARGO_BIN_NAME");

fn main() {
    let cli = Cli::parse();
    let config = config::parse_config();

    init_logger(&cli, &config);

    debug!("{config:?}");
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
    } else if let Some(log_level) = &configuration.log_level {
        if let Ok(log_level) = LevelFilter::from_str(&log_level) {
            builder.filter(Some(BIN_NAME), log_level);
        }
    } else {
        builder.filter(Some(BIN_NAME), LevelFilter::Info);
    }
    builder.init();
}

fn run(cli: Cli) -> anyhow::Result<()> {
    match cli.sub {
        SubCommand::Activate(activate) => activate::run_activate(activate),
        SubCommand::Pkg(pkg) => pkg::run_package(pkg, &config::parse_config()),
    }
}
