mod apply;
mod list;
mod pkg;

use apply::ApplySubCommand;
use clap::{Parser, Subcommand};
use config::config::Config;
use list::ListSubCommand;
use log::{LevelFilter, debug};
use pkg::PkgSubCommand;
use std::str::FromStr;
use std::{fmt::Debug, process};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: SubCommand,
}

#[derive(Subcommand, Debug)]
enum SubCommand {
    Pkg(PkgSubCommand),
    Apply(ApplySubCommand),
    List(ListSubCommand),
}

fn main() {
    let cli = Cli::parse();
    let config = config::parse_config().unwrap_or_else(|e| {
        eprintln!("{e}");
        process::exit(1);
    });

    init_logger(&config);

    debug!("{config:?}");
    debug!("{cli:?}");

    if let Err(e) = run(cli, &config) {
        eprintln!("{e}");
        process::exit(1);
    }
}

fn init_logger(configuration: &Config) {
    const BIN_NAME: &str = env!("CARGO_BIN_NAME");
    let mut builder = env_logger::builder();
    if let Some(log_level) = &configuration.log_level {
        if let Ok(log_level) = LevelFilter::from_str(&log_level) {
            builder.filter(Some(BIN_NAME), log_level);
        }
    } else {
        builder.filter(Some(BIN_NAME), LevelFilter::Info);
    }
    builder.init();
}

fn run(cli: Cli, config: &Config) -> anyhow::Result<()> {
    match cli.command {
        SubCommand::Pkg(pkg) => pkg::run_package(pkg, config),
        SubCommand::Apply(cmd) => apply::run_apply(cmd, config),
        SubCommand::List(cmd) => list::run_list(cmd, config),
    }
}
