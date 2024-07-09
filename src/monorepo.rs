use crate::{
    dialect::{Dialect, DialectSourceUpdate},
    js::JS,
    package::Package,
};
use notify_debouncer_full::DebouncedEvent;
use std::{
    collections::HashMap,
    path::PathBuf,
    sync::{Arc, RwLock},
};

pub struct Monorepo {
    path: PathBuf,
    state: MonorepoState,
    dialects: Vec<Arc<RwLock<dyn Dialect + Send + Sync>>>,
    packages: HashMap<PathBuf, Package>,
}

pub enum MonorepoState {
    Initial,
    Loading,
    Errored,
    Ready,
}

impl Monorepo {
    pub fn new(path: PathBuf) -> Self {
        let js = Arc::new(RwLock::new(JS::new(path.clone())));
        Self {
            path,
            state: MonorepoState::Initial,
            dialects: vec![js],
            packages: HashMap::new(),
        }
    }

    pub async fn update(
        &mut self,
        event: Option<DebouncedEvent>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        for dialect in &self.dialects {
            let update_result = {
                let mut dialect_guard = dialect.write().map_err(|_| "Dialect lock poisoned")?;
                dialect_guard.update_source(event.as_ref())
            };

            match update_result {
                Ok(DialectSourceUpdate::Updated(packages)) => {
                    // let dialect_ptr = dialect.as_ref() as *const dyn Dialect;

                    // Here we do several things:
                    // - Add the dialect to the packages in the updated list that don't have it.
                    // - Remove the dialect from removed packages.
                    // - Remove packages that have no dialects left.
                    self.packages.retain(|path, package| {
                        let is_dialect_package =
                            package.dialects.iter().any(|d| Arc::ptr_eq(d, dialect));

                        // If the package is in the updated list, retain it.
                        if packages.contains(path) {
                            // If the package has no dialect assigned, add the dialect
                            if !is_dialect_package {
                                package.dialects.push(Arc::clone(dialect));
                            }

                            return true;
                        }

                        // Otherwise, the package is not in the updated list.

                        // If the package isn't a dialect package, retain it.
                        if !is_dialect_package {
                            return true;
                        }

                        // Otherwise, remove the dialect from the package.
                        package.dialects.retain(|d| !Arc::ptr_eq(d, dialect));

                        // If the package has no dialects left, remove it.
                        !package.dialects.is_empty()
                    });

                    // Now add new packages to the list.
                    for path in packages {
                        if !self.packages.contains_key(&path) {
                            let mut package = Package::new(path.clone());
                            package.dialects.push(Arc::clone(dialect));
                            self.packages.insert(path.clone(), package);
                        }
                    }
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
