use clap::builder::OsStr;
use notify_debouncer_full::DebouncedEvent;
use plain::PlainFlavor;
use pnpm::PnpmFlavor;
use std::{
    fs::{self, read},
    path::PathBuf,
};

use crate::error::Result;

mod plain;
mod pnpm;

// pub enum NpmFlavor {
//     Pnpm(PnpmFlavor),
//     Npm(PlainFlavor),
// }

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

pub trait NpmFlavor {
    fn packages(&self) -> Result<Vec<PathBuf>>;
}

pub trait NpmFlavorDetectable {
    fn detect(path: &PathBuf) -> bool;

    fn is_source_file(name: &str) -> bool;
}
