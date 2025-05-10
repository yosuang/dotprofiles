use clap::Parser;
use log::info;

#[derive(Parser, Debug)]
pub struct Activate {
    profiles: String,
}

pub fn handle_activate(activate: &Activate) -> Result<(), &'static str> {
    info!("Activating profiles: {:?}", activate);
    Ok(())
}
