mod scoop;

pub use scoop::*;

use config::config::Config;

use crate::Scoop;

pub trait PackageManage {
    fn info(&self) -> PackageManagerInfo;
    fn show_version(&self);
}

pub fn build_package_manager(config: &Config) -> anyhow::Result<impl PackageManage, anyhow::Error> {
    if let Some(pm_name) = config.package_manager.as_ref() {
        if pm_name.eq_ignore_ascii_case("scoop") {
            return Ok(Scoop::new(
                config.scoop.as_ref().unwrap().executable_path.as_str(),
            ));
        }
        return Err(anyhow::anyhow!(
            "Invalid value '{}' of configure option 'package_manager'.",
            pm_name
        ));
    }
    Err(anyhow::anyhow!(
        "Configure option 'package_manager' does not exist."
    ))
}

#[derive(Debug)]
pub struct PackageManagerInfo {
    pub name: String,
}
