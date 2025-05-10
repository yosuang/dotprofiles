pub trait PackageManager {
    fn info(&self) -> PackageManagerInfo;
    fn show_version(&self);
}

#[derive(Debug)]
pub struct PackageManagerInfo {
    pub name: String,
}
