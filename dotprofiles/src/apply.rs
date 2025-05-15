use anyhow::anyhow;
use clap::Parser;
use config::config::Config;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Parser, Debug)]
pub struct ApplySubCommand {
    app: String,
}

pub fn run_apply(cmd: ApplySubCommand, config: &Config) -> anyhow::Result<()> {
    let matched_apps = find_app(&cmd.app, config)?;
    let app = match matched_apps.len() {
        0 => {
            println!("No apps matched");
            None
        }
        1 => matched_apps.into_iter().next(),
        _ => {
            println!(
                "Multiple apps matched: {}",
                matched_apps
                    .iter()
                    .map(|p| p.file_name().unwrap().to_string_lossy())
                    .collect::<Vec<_>>()
                    .join(" | ")
            );
            None
        }
    };
    match app {
        Some(app_path) => apply_profiles_of(app_path, config),
        None => Ok(()),
    }
}

fn apply_profiles_of(app_path: impl AsRef<Path>, _config: &Config) -> anyhow::Result<()> {
    if !app_path.as_ref().exists() {
        return Err(anyhow!("App does not exist"));
    }

    let current_dir = std::env::current_dir()?;
    copy_dir_all(app_path, &current_dir)?;
    Ok(())
}

fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> anyhow::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}

fn find_app(app_name: &str, config: &Config) -> anyhow::Result<Vec<PathBuf>> {
    let matched_apps: Vec<PathBuf> = list_apps(config)?
        .into_iter()
        .filter(|path| {
            path.file_name()
                .map_or(false, |filename| filename.to_string_lossy() == app_name)
        })
        .collect();
    Ok(matched_apps)
}

fn list_apps(config: &Config) -> anyhow::Result<Vec<PathBuf>> {
    let workspace_dir = config.get_app_dir()?;
    let mut apps = Vec::new();

    if let Ok(entries) = fs::read_dir(workspace_dir) {
        for entry in entries.flatten() {
            if let Ok(file_type) = entry.file_type() {
                if file_type.is_dir() {
                    apps.push(entry.path());
                }
            }
        }
    }

    anyhow::Ok(apps)
}
