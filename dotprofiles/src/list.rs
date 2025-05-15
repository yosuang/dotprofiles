use clap::Parser;
use config::config::Config;

#[derive(Parser, Debug)]
pub struct ListSubCommand {
    app: Option<String>,
}

pub fn run_list(cmd: ListSubCommand, config: &Config) -> anyhow::Result<()> {
    let app_dir = config.get_app_dir()?;
    let entries: Vec<_> = std::fs::read_dir(app_dir)?
        .filter_map(Result::ok)
        .filter(|e| e.file_type().map_or(false, |ft| ft.is_dir()))
        .filter(|e| {
            cmd.app.as_ref().map_or(true, |app_name| {
                e.file_name()
                    .to_string_lossy()
                    .to_lowercase()
                    .contains(&app_name.to_lowercase())
            })
        })
        .collect();

    if entries.is_empty() {
        println!("No apps found");
    } else {
        entries
            .iter()
            .for_each(|e| println!("{}", e.file_name().to_string_lossy()));
    }
    Ok(())
}
