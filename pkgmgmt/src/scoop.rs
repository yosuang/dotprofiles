use crate::package_manager::{PackageManager, PackageManagerInfo};

pub struct Scoop;

impl Scoop {
    pub fn new() -> Self {
        Scoop
    }
}

impl PackageManager for Scoop {
    fn info(&self) -> PackageManagerInfo {
        PackageManagerInfo {
            name: String::from("Scoop"),
            version: String::from("0.1.0"),
        }
    }
}
