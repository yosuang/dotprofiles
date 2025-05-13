use clap::Parser;
use std::fs;
use std::path::PathBuf;

#[derive(Parser, Debug)]
pub struct ApplySubCommand {
    app: String,
}

pub fn run_apply(cmd: ApplySubCommand) -> anyhow::Result<()> {
    apply_profiles_of(cmd.app.as_str())
}

fn apply_profiles_of(app: &str) -> anyhow::Result<()> {
    let aider_dir: Option<PathBuf> = find_app_profiles_like(app)?;
    if let Some(src_dir) = aider_dir {
        let current_dir = std::env::current_dir()?;

        for entry in fs::read_dir(src_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                let dest_path = current_dir.join(entry.file_name());
                fs::copy(path, dest_path)?;
            }
        }
    } else {
        println!("App profiles of '{}' does not exist.", app)
    }
    Ok(())
}

fn find_app_profiles_like(name: &str) -> anyhow::Result<Option<PathBuf>> {
    let group_paths: Vec<PathBuf> = list_app_profiles()
        .into_iter()
        .filter(|path| {
            path.file_name().map_or(false, |filename| {
                filename.to_string_lossy().to_lowercase() == name.to_lowercase()
            })
        })
        .collect();

    match group_paths.len() {
        0 => Ok(None),
        1 => Ok(group_paths.into_iter().next()),
        _ => Err(anyhow::anyhow!(
            "匹配到了多个应用配置: {}, 请使用更明确的名称",
            group_paths
                .iter()
                .map(|p| p.file_name().unwrap().to_string_lossy())
                .collect::<Vec<_>>()
                .join(", ")
        )),
    }
}

fn list_app_profiles() -> Vec<PathBuf> {
    let workspace_dir = config::WorkspaceDir.as_path();
    let mut groups = Vec::new();

    if let Ok(entries) = fs::read_dir(workspace_dir) {
        for entry in entries.flatten() {
            if let Ok(file_type) = entry.file_type() {
                if file_type.is_dir() {
                    groups.push(entry.path());
                }
            }
        }
    }

    groups
}
