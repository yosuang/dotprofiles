pub trait PackageManager {
    fn info(&self) -> PackageManagerInfo;
}

#[derive(Debug)]
pub struct PackageManagerInfo {
    pub name: String,
    pub version: String,
}
