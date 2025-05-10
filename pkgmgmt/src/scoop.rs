use crate::{PackageManage, PackageManagerInfo};
use std::{
    path::PathBuf,
    process::{Command, Stdio},
};

pub struct Scoop {
    executable_path: PathBuf,
}

impl Scoop {
    pub fn new(executable_path: &str) -> Self {
        Scoop {
            executable_path: PathBuf::from(executable_path),
        }
    }
}

impl PackageManage for Scoop {
    fn info(&self) -> PackageManagerInfo {
        PackageManagerInfo {
            name: String::from("Scoop"),
        }
    }

    fn show_version(&self) {
        Command::new(&self.executable_path)
            .arg("--version")
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()
            .expect("Run failed");
    }
}
