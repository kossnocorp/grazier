use crate::dialect::Dialect;
use std::path::PathBuf;

// use crate::npm_package::NpmPackage;

pub struct Package<'a> {
    pub path: PathBuf,
    pub state: PackageState,
    pub dialects: Vec<Box<&'a dyn Dialect>>,
}

impl Package<'_> {
    pub fn new(path: PathBuf) -> Self {
        Self {
            path,
            state: PackageState::Initial,
            dialects: vec![],
        }
    }
}

pub enum PackageState {
    Initial,
    Loaded,
    Errored,
}
