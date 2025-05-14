use clap::Parser;
use config::config::Config;

#[derive(Parser, Debug)]
pub struct ListSubCommand {
    app: Option<String>,
}

pub fn run_list(cmd: ListSubCommand, config: &Config) -> anyhow::Result<()> {
    let app_dir = config.get_app_dir()?;
    std::fs::read_dir(app_dir)?
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false))
        .filter(|entry| {
            let dir_name = entry.file_name().to_string_lossy().to_lowercase();
            cmd.app
                .as_ref()
                .map_or(true, |app| dir_name.contains(&app.to_lowercase()))
        })
        .for_each(|entry| println!("{}", entry.file_name().to_string_lossy()));
    Ok(())
}
