use clap::Parser;

#[derive(Parser, Debug)]
pub struct ListSubCommand;

pub fn run_list(_: ListSubCommand) -> anyhow::Result<()> {
    let work_dir = &*config::WorkspaceDir;
    if let Ok(entries) = std::fs::read_dir(work_dir) {
        for entry in entries.flatten() {
            if let Ok(file_type) = entry.file_type() {
                if file_type.is_dir() {
                    println!("{}", entry.file_name().to_string_lossy());
                }
            }
        }
    }
    Ok(())
}
