use serde::de;

use crate::dialect::Dialect;
use std::{
    path::PathBuf,
    sync::{Arc, RwLock},
};

#[derive(Debug)]
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

#[derive(Debug)]
pub enum PackageState {
    Initial,
    Loaded,
    Errored,
}
