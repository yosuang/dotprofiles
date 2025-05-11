use std::{fs, path::PathBuf};

use clap::Parser;

#[derive(Debug, Parser)]
pub struct Aider;

impl Aider {
    pub fn run(self) -> anyhow::Result<()> {
        let aider_dir: Option<PathBuf> = super::find_group_paths_like("aider")?;
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
            println!("Directory not exist!")
        }
        Ok(())
    }
}
