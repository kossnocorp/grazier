use super::{NpmFlavor, NpmFlavorDetectable};
use crate::{error::Result, js::JS_PACKAGE_JSON};
use std::{fs, path::PathBuf};

pub struct PlainFlavor {}

impl PlainFlavor {
    pub fn new() -> Self {
        Self {}
    }
}

impl NpmFlavorDetectable for PlainFlavor {
    fn detect(path: &PathBuf) -> bool {
        fs::metadata(path.join(JS_PACKAGE_JSON)).is_ok()
    }

    fn is_source_file(name: &str) -> bool {
        name == JS_PACKAGE_JSON
    }
}

impl NpmFlavor for PlainFlavor {
    fn packages(&self) -> Result<Vec<PathBuf>> {
        Ok(vec![])
    }
}
