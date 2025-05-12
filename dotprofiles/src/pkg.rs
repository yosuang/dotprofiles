mod info;

use clap::{Parser, Subcommand};
use config::config::Config;

#[derive(Parser, Debug)]
pub struct PkgSubCommand {
    #[clap(subcommand)]
    command: PackageCommand,
}

#[derive(Subcommand, Debug)]
pub enum PackageCommand {
    Info(info::Info),
}

pub fn run_package(cmd: PkgSubCommand, config: &Config) -> anyhow::Result<()> {
    let pm = dotprofiles_pkgmgmt::build_package_manager(config)?;
    match cmd.command {
        PackageCommand::Info(cmd) => cmd.run(pm),
    }
}
