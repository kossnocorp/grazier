use crate::{
    dialect::{Dialect, DialectSourceUpdate},
    error::{Error, Result},
};
use flavor::{detect_npm_flavor, is_npm_flavor_source, NpmFlavor};
use notify_debouncer_full::DebouncedEvent;
use std::path::PathBuf;

pub mod flavor;

pub const JS_PACKAGE_JSON: &str = "package.json";

pub struct JS {
    path: PathBuf,
    state: JSState,
}

impl JS {
    pub fn new(path: PathBuf) -> Self {
        Self {
            path,
            state: JSState::Initial,
        }
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

            Err(Error) => {
                self.state = JSState::Errored(Error);
                Ok(DialectSourceUpdate::Errored)
            }
        }
    }
}

impl Dialect for JS {
    fn update_source(&mut self, event: Option<&DebouncedEvent>) -> Result<DialectSourceUpdate> {
        match &self.state {
            // Initialize the npm source.
            JSState::Initial => {
                if let Some(flavor) = detect_npm_flavor(&self.path) {
                    let packages = flavor.packages();
                    self.state = JSState::Loaded(flavor);
                    return self.update_packages(packages);
                } else {
                    // None of the npm flavors were detected, which means it's not a npm source.
                    self.state = JSState::NotFound;
                }
            }

            JSState::Loaded(flavor) => {
                // Load the packages if the flavor was updated.
                if let Some(event) = event {
                    if self.did_flavor_update(event) {
                        let packages = flavor.packages();
                        return self.update_packages(packages);
                    }
                }
            }

            // If the npm source was not found or errored, check if it was updated and
            // reinitialize the source if it was.
            JSState::Errored(_) | JSState::NotFound => {
                if let Some(event) = event {
                    // A flavor file was updated, reinitialize the source.
                    if self.did_flavor_update(event) {
                        self.state = JSState::Initial;
                        return self.update_source(Some(event));
                    }
                }
            }
        }

        Ok(DialectSourceUpdate::Ignored)
    }
}

pub enum JSState {
    Initial,
    NotFound,
    Loaded(Box<dyn NpmFlavor>),
    Errored(Error),
}
