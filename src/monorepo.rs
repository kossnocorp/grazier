use crate::{
    dialect::{Dialect, DialectSourceUpdate},
    js::JS,
    package::Package,
};
use notify_debouncer_full::DebouncedEvent;
use std::{collections::HashMap, path::PathBuf};

pub struct Monorepo<'a> {
    path: PathBuf,
    state: MonorepoState,
    dialects: Vec<Box<dyn Dialect>>,
    packages: HashMap<PathBuf, Package<'a>>,
}

pub enum MonorepoState {
    Initial,
    Loading,
    Errored,
    Ready,
}

impl Monorepo<'_> {
    pub fn new(path: PathBuf) -> Self {
        let js = JS::new(path.clone());
        Self {
            path,
            state: MonorepoState::Initial,
            dialects: vec![Box::new(js)],
            packages: HashMap::new(),
        }
    }

    pub async fn update(
        &mut self,
        event: Option<DebouncedEvent>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        for dialect in &mut self.dialects {
            match dialect.update_source(event.as_ref()) {
                Ok(DialectSourceUpdate::Updated(packages)) => {
                    // let mut removed = Vec::new();
                    // let mut added = Vec::new();

                    self.packages.retain(|path, package| {
                        let is_dialect_package = package.dialects.iter().any(|d| {
                            std::ptr::eq(d as *const _, dialect.as_ref() as *const dyn Dialect)
                        });
                        // if value.dialects
                        true
                    });

                    // for path in self.packages.keys() {
                    //     // let
                    //     if !packages.contains(path) {
                    //         // removed.push(path.clone());
                    //     }
                    // }

                    // for path in packages {
                    //     if !self.packages.contains_key(&path) {
                    //         let package = Package::new(path.clone());
                    //         self.packages.insert(path.clone(), package);
                    //         added.push(path);
                    //     }
                    // }

                    // self.packages.
                    // [TODO] Handle removed packages
                    // [TODO] Handle added packages
                }

                Ok(DialectSourceUpdate::Errored) => {
                    // [TODO] Handle error
                }

                Err(e) => {
                    // [TODO] Handle error
                }

                Ok(DialectSourceUpdate::Ignored) => {}
            };

            // if let SourceUpdate::Updated(packages) = source_update {
            //     sources_updated = true;
            // }
        }

        // if sources_updated {
        //     // [TODO] Calculate the new packages list:
        //     // - Initialize new
        //     // - Handle removed
        // }

        Ok(())
    }
}
