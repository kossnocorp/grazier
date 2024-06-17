use crate::source::{Source, SourceUpdate};
use flavor::{detect_npm_flavor, is_npm_flavor_source, NpmFlavor};
use notify_debouncer_full::DebouncedEvent;
use std::path::PathBuf;

pub mod flavor;

pub const NPM_PACKAGE_JSON: &str = "package.json";

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
}

impl Source for Npm {
    fn workspaces(&self) -> Vec<PathBuf> {
        vec![]
    }

    fn update(
        &mut self,
        event: Option<&DebouncedEvent>,
    ) -> Result<SourceUpdate, Box<dyn std::error::Error>> {
        match self.state {
            // Initialize the npm source.
            NpmState::Initial => {
                let flavor = detect_npm_flavor(&self.path);
                match flavor {
                    // A npm flavor was detected, assign it and continue...
                    Some(flavor) => {
                        self.state = NpmState::Detected(flavor);
                        return self.update(event);
                    }

                    // None of the npm flavors were detected, which means
                    // it's not a npm source.
                    None => {
                        self.state = NpmState::NotFound;
                        return Ok(SourceUpdate::Ignored);
                    }
                }
            }

            // If the npm source was not found or errored, check if it was updated and
            // reinitialize the source if it was.
            NpmState::Errored | NpmState::NotFound => match event {
                Some(event) => {
                    let flavor_updated = event.paths.iter().any(|path| {
                        if let Some(file_name) = path.file_name() {
                            if let Some(file_name_str) = file_name.to_str() {
                                return is_npm_flavor_source(file_name_str);
                            }
                        }
                        false
                    });

                    if flavor_updated {
                        self.state = NpmState::Initial;
                        return self.update(Some(event));
                    }
                }

                None => {
                    return Ok(SourceUpdate::Ignored);
                }
            },

            // If the npm source flavor was detected, load the workspaces.
            NpmState::Detected(flavor) => {
                // [TODO] Load the workspaces.
                return Ok(SourceUpdate::Updated);
            }

            NpmState::Loaded(flavor) => {
                // [TODO] Check if the flavor could be updated.
                // [TODO] Load the workspaces if the flavor was updated.
                return Ok(SourceUpdate::Updated);
            }
        }

        if let NpmState::Detected(flavor) = self.state {
            // [TODO] Load the workspaces.
            return Ok(SourceUpdate::Updated);
        }

        match self.state {
            NpmState::Initial => {
                let flavor = flavor::detect_npm_flavor(&self.path)?;
                self.state = NpmState::Loaded(flavor);
            }

            _ => {}
        }

        Ok(SourceUpdate::Ignored)
    }
}

pub enum NpmUpdate {
    Ignored,
    Errored,
    Updated(Vec<PathBuf>),
}

pub enum NpmState {
    Initial,
    NotFound,
    Detected(Box<dyn NpmFlavor + 'static>),
    Loaded(Box<dyn NpmFlavor + 'static>),
    Errored,
}
