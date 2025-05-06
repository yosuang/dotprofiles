use clap::{Parser, Subcommand};
use env_logger::Env;
use log::{debug, info, LevelFilter};

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

    let mut builder =
        env_logger::Builder::from_env(Env::default().default_filter_or(format!("{BIN_NAME}=info")));
    if cli.verbose {
        builder.filter(Some(BIN_NAME), LevelFilter::Debug);
    };
    builder.init();

    debug!("args:{cli:?}");

    match cli.command {
        Command::Activate(activate) => {
            info!("Activating profiles: {:?}", activate);
        }
        Command::Deactivate(_) => {
            info!("Deactivating profiles");
        }
    }
}
