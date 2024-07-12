use glob::glob;
use serde::{Deserialize, Serialize};
use tracing::debug;

use super::{NpmFlavor, NpmFlavorDetectable};
use crate::{error::Result, js::JS_PACKAGE_JSON};
use std::{
    collections::HashMap,
    fs::{self},
    path::PathBuf,
};

#[derive(Debug)]
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
    fn packages(&self, path: &PathBuf) -> Result<Vec<PathBuf>> {
        let contents = fs::read_to_string(path.join(JS_PACKAGE_JSON))?;
        let package_json: PackageJson = serde_json::from_str(&contents)?;
        match package_json.workspaces {
            Some(workspaces) => {
                let packages = workspaces
                    .into_iter()
                    .map(|x| glob(&path.join(x).display().to_string()))
                    .collect::<std::result::Result<Vec<_>, _>>()?
                    .into_iter()
                    .flatten()
                    .collect::<std::result::Result<Vec<_>, _>>()?;
                Ok(packages)
            }
            None => {
                debug!("No workspaces found in package.json");
                Ok(vec![])
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct PackageJson {
    workspaces: Option<Vec<String>>,
    #[serde(flatten)]
    pub unknowns: HashMap<String, serde_json::Value>,
}
