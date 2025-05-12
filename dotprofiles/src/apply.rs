mod aider;

use clap::{Parser, Subcommand};
use std::fs;
use std::path::PathBuf;

#[derive(Parser, Debug)]
pub struct ApplySubCommand {
    #[clap(subcommand)]
    item: ApplyCommand,
}

#[derive(Subcommand, Debug)]
pub enum ApplyCommand {
    Aider(aider::Aider),
}

pub fn run_apply(cmd: ApplySubCommand) -> anyhow::Result<()> {
    match cmd.item {
        ApplyCommand::Aider(cmd) => cmd.run(),
    }
}

fn list_group_paths() -> Vec<PathBuf> {
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

fn find_group_paths_like(name: &str) -> anyhow::Result<Option<PathBuf>> {
    let group_paths: Vec<PathBuf> = list_group_paths()
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
            "找到多个匹配组: {}, 请使用更明确的名称",
            group_paths
                .iter()
                .map(|p| p.file_name().unwrap().to_string_lossy())
                .collect::<Vec<_>>()
                .join(", ")
        )),
    }
}
