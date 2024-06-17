use super::{NpmFlavor, NpmFlavorDetectable};
use crate::js::npm::NPM_PACKAGE_JSON;
use std::{fs, path::PathBuf};

pub struct PlainFlavor {}

impl PlainFlavor {
    pub fn new() -> Self {
        Self {}
    }
}

impl NpmFlavorDetectable for PlainFlavor {
    fn detect(path: &PathBuf) -> bool {
        fs::metadata(path.join(NPM_PACKAGE_JSON)).is_ok()
    }

    fn is_source_file(name: &str) -> bool {
        name == NPM_PACKAGE_JSON
    }
}

impl NpmFlavor for PlainFlavor {
    fn workspaces(&self) -> Vec<PathBuf> {
        vec![]
    }
}
