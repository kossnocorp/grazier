use crate::error::Result;

use super::{NpmFlavor, NpmFlavorDetectable};
use std::{fs, path::PathBuf};

const PNPM_LOCK: &str = "pnpm-lock.yaml";

const PNPM_WORKSPACE: &str = "pnpm-workspace.yaml";

pub struct PnpmFlavor {}

impl PnpmFlavor {
    pub fn new() -> Self {
        Self {}
    }
}

impl NpmFlavorDetectable for PnpmFlavor {
    fn detect(path: &PathBuf) -> bool {
        fs::metadata(path.join(PNPM_LOCK)).is_ok()
            || fs::metadata(path.join(PNPM_WORKSPACE)).is_ok()
    }

    fn is_source_file(name: &str) -> bool {
        name == PNPM_WORKSPACE
    }
}

impl NpmFlavor for PnpmFlavor {
    fn packages(&self) -> Result<Vec<PathBuf>> {
        Ok(vec![])
    }
}
