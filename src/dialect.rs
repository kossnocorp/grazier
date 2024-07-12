use crate::error::Result;
use notify_debouncer_full::DebouncedEvent;
use std::{fmt::Debug, path::PathBuf};

pub trait Dialect: Debug {
    // fn packages(&self) -> Vec<PathBuf>;

    fn update_source(&mut self, event: Option<&DebouncedEvent>) -> Result<DialectSourceUpdate>;
}

pub enum DialectSourceUpdate {
    Ignored,
    Updated(Vec<PathBuf>),
    Errored,
}
