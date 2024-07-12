use glob::glob;
use serde::{Deserialize, Serialize};
use tracing::debug;
use tracing_subscriber::field::debug;

use crate::error::Result;

use super::{NpmFlavor, NpmFlavorDetectable};
use std::{fs, path::PathBuf};

const PNPM_LOCK: &str = "pnpm-lock.yaml";

const PNPM_WORKSPACE: &str = "pnpm-workspace.yaml";

#[derive(Debug)]
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
    fn packages(&self, path: &PathBuf) -> Result<Vec<PathBuf>> {
        debug!("Reading pnpm workspace file {:?}", path);
        let contents = fs::read_to_string(path.join(PNPM_WORKSPACE))?;
        let workspace: PnpmWorkspace = serde_yml::from_str(&contents)?;
        let packages = workspace
            .packages
            .into_iter()
            .map(|x| glob(&path.join(x).display().to_string()))
            .collect::<std::result::Result<Vec<_>, _>>()?
            .into_iter()
            .flatten()
            .collect::<std::result::Result<Vec<_>, _>>()?;
        Ok(packages)
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct PnpmWorkspace {
    packages: Vec<String>,
}
