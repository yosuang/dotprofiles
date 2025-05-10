use std::process;
use std::str::FromStr;

use clap::{Parser, Subcommand};
use dotprofiles::{DefaultConfigFilePath, config::Config};
use dotprofiles_pkgmgmt::{PackageManager, Scoop};
use log::{LevelFilter, debug, error, info};

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
    let configuration = parse_config();

    init_logger(&cli, &configuration);

    let scoop_info = Scoop::new().info();
    info!("{:?}", scoop_info);

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

fn parse_config() -> Config {
    Config::load(&DefaultConfigFilePath).unwrap_or(Config::default_config())
}

fn run(cli: Cli) -> Result<(), &'static str> {
    match cli.command {
        Command::Activate(activate) => {
            info!("Activating profiles: {:?}", activate);
            Ok(())
        }
        Command::Deactivate(_) => {
            info!("Deactivating profiles");
            Err("unimplemented!")
        }
    }
}
