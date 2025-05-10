use clap::Parser;
use log::info;

#[derive(Parser, Debug)]
pub struct ActivateSubCommand {
    profiles: String,
}

pub fn run_activate(activate: ActivateSubCommand) -> anyhow::Result<()> {
    info!("Activating profiles: {:?}", activate);
    Ok(())
}
