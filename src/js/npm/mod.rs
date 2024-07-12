use crate::{
    dialect::DialectSourceUpdate,
    error::{Error, Result},
};
use notify_debouncer_full::DebouncedEvent;
use plain::PlainFlavor;
use pnpm::PnpmFlavor;
use std::{fmt::Debug, path::PathBuf};
use tracing::debug;

mod plain;
mod pnpm;

#[derive(Debug)]
pub struct Npm {
    path: PathBuf,
    state: NpmState,
}

impl Npm {
    pub fn new(path: PathBuf) -> Self {
        Self {
            path,
            state: NpmState::Initial,
        }
    }

    pub fn update(&mut self, event: Option<&DebouncedEvent>) -> Result<DialectSourceUpdate> {
        match &self.state {
            // Initialize the npm source.
            NpmState::Initial => {
                debug!("Initializing npm source");
                if let Some(flavor) = detect_npm_flavor(&self.path) {
                    debug!("Detected npm flavor: {:?}", flavor);
                    let packages = flavor.packages(&self.path);
                    self.state = NpmState::Loaded(flavor);
                    return self.update_packages(packages);
                } else {
                    // None of the npm flavors were detected, which means it's not a npm source.
                    debug!("Can't detect npm flavor");
                    self.state = NpmState::NotFound;
                }
            }

            NpmState::Loaded(flavor) => {
                // Load the packages if the flavor was updated.
                if let Some(event) = event {
                    if self.did_flavor_update(event) {
                        let packages = flavor.packages(&self.path);
                        return self.update_packages(packages);
                    }
                }
            }

            // If the npm source was not found or errored, check if it was updated and
            // reinitialize the source if it was.
            NpmState::Errored(_) | NpmState::NotFound => {
                if let Some(event) = event {
                    // A flavor file was updated, reinitialize the source.
                    if self.did_flavor_update(event) {
                        self.state = NpmState::Initial;
                        return self.update(Some(event));
                    }
                }
            }
        }

        Ok(DialectSourceUpdate::Ignored)
    }

    fn did_flavor_update(&self, event: &DebouncedEvent) -> bool {
        event.paths.iter().any(|path| {
            path.file_name()
                .and_then(|file_name| file_name.to_str())
                .map_or(false, is_npm_flavor_source)
        })
    }

    fn update_packages(&mut self, packages: Result<Vec<PathBuf>>) -> Result<DialectSourceUpdate> {
        match packages {
            Ok(packages) => Ok(DialectSourceUpdate::Updated(packages)),

            Err(e) => {
                self.state = NpmState::Errored(e);
                Ok(DialectSourceUpdate::Errored)
            }
        }
    }
}

#[derive(Debug)]
pub enum NpmState {
    Initial,
    NotFound,
    Loaded(Box<dyn NpmFlavor>),
    Errored(Error),
}

pub fn detect_npm_flavor(path: &PathBuf) -> Option<Box<dyn NpmFlavor>> {
    if PnpmFlavor::detect(path) {
        return Some(Box::new(PnpmFlavor::new()));
    } else if PlainFlavor::detect(path) {
        return Some(Box::new(PlainFlavor::new()));
    }
    None
}

pub fn is_npm_flavor_source(filename: &str) -> bool {
    PnpmFlavor::is_source_file(filename) || PlainFlavor::is_source_file(filename)
}

pub trait NpmFlavor: Send + Sync + Debug {
    fn packages(&self, path: &PathBuf) -> Result<Vec<PathBuf>>;
}

pub trait NpmFlavorDetectable {
    fn detect(path: &PathBuf) -> bool;

    fn is_source_file(name: &str) -> bool;
}
