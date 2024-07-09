use crate::dialect::Dialect;
use std::{
    path::PathBuf,
    sync::{Arc, RwLock},
};

pub struct Package {
    pub path: PathBuf,
    pub state: PackageState,
    pub dialects: Vec<Arc<RwLock<dyn Dialect + Send + Sync>>>,
}

impl Package {
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
