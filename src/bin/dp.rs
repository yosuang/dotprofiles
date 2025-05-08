use std::str::FromStr;

use clap::{Parser, Subcommand};
use dotprofilers::{DefaultConfigFilePath, config::Config};
use log::{LevelFilter, debug, info};

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
    Deactivate(Deactivate),
}

#[derive(Parser, Debug)]
struct Activate {
    profiles: String,
}

#[derive(Parser, Debug)]
struct Deactivate {}

const BIN_NAME: &str = env!("CARGO_BIN_NAME");

fn main() {
    let cli = Cli::parse();

    let configuration: Config =
        Config::load(&DefaultConfigFilePath).unwrap_or(Config::default_config());

    let mut builder = env_logger::builder();
    if cli.verbose {
        builder.filter(Some(BIN_NAME), LevelFilter::Debug);
    } else if let Ok(level) = LevelFilter::from_str(&configuration.log_level) {
        builder.filter(Some(BIN_NAME), level);
    } else {
        builder.filter(Some(BIN_NAME), LevelFilter::Info);
    }
    builder.init();

    debug!("{cli:?}");

    match cli.command {
        Command::Activate(activate) => {
            info!("Activating profiles: {:?}", activate);
        }
        Command::Deactivate(_) => {
            info!("Deactivating profiles");
        }
    }
}
