use clap::{Parser, Subcommand};
use log::info;

#[derive(Parser, Debug)]
pub struct Package {
    #[clap(subcommand)]
    command: PackageCommand,
}

#[derive(Subcommand, Debug)]
pub enum PackageCommand {
    Info,
}

pub fn handle_package(package: &Package) -> Result<(), &'static str> {
    info!("Processing package: {:?}", package);
    Ok(())
}
