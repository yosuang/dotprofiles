use clap::Parser;

#[derive(Parser, Debug)]
pub struct ListSubCommand {
    app: Option<String>,
}

pub fn run_list(cmd: ListSubCommand) -> anyhow::Result<()> {
    let work_dir = &*config::WorkspaceDir;
    std::fs::read_dir(work_dir)?
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
