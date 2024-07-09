use crate::error::Result;
use notify_debouncer_full::DebouncedEvent;
use std::path::PathBuf;

pub trait Dialect {
    // fn packages(&self) -> Vec<PathBuf>;

    fn update_source(&mut self, event: Option<&DebouncedEvent>) -> Result<DialectSourceUpdate>;
}

pub enum DialectSourceUpdate {
    Ignored,
    Updated(Vec<PathBuf>),
    Errored,
}
