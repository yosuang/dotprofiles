use clap::Parser;
use dotprofiles_pkgmgmt::PackageManage;

#[derive(Debug, Parser)]
pub struct Info;

impl Info {
    pub fn run(self, pm: impl PackageManage) -> anyhow::Result<()> {
        let pm_info = pm.info();
        println!("{}", pm_info.name);
        Ok(())
    }
}
