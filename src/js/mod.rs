use crate::{
    dialect::{Dialect, DialectSourceUpdate},
    error::Result,
};
use notify_debouncer_full::DebouncedEvent;
use npm::Npm;
use std::path::PathBuf;

pub mod npm;

pub const JS_PACKAGE_JSON: &str = "package.json";

pub struct JS {
    // path: PathBuf,
    npm: Npm,
}

impl JS {
    pub fn new(path: PathBuf) -> Self {
        Self {
            npm: Npm::new(path),
            // path,
        }
    }
}

impl Dialect for JS {
    fn update_source(&mut self, event: Option<&DebouncedEvent>) -> Result<DialectSourceUpdate> {
        self.npm.update(event)
    }
}
