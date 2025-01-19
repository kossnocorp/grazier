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
use tracing::debug;

#[derive(Debug)]
pub struct Monorepo {
    path: PathBuf,
    state: MonorepoState,
    dialects: Vec<Arc<RwLock<dyn Dialect + Send + Sync>>>,
    packages: HashMap<PathBuf, Package>,
}

#[derive(Debug)]
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

    pub fn update(
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
                    debug!("Dialect source got updated {:?}", dialect);
                    debug!("Got the packages list {:?}", packages);

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
                        let no_dialects = package.dialects.is_empty();
                        if no_dialects {
                            debug!("Untracking package {:?}", package);
                        }
                        !no_dialects
                    });

                    // Now add new packages to the list.
                    for path in packages {
                        if !self.packages.contains_key(&path) {
                            let mut package = Package::new(path.clone());
                            package.dialects.push(Arc::clone(dialect));
                            debug!("Tracking package {:?}", package);
                            self.packages.insert(path.clone(), package);
                        }
                    }
                }

                Ok(DialectSourceUpdate::Errored) => {
                    debug!("Dialect source errored {:?}", dialect);
                    // [TODO] Handle error
                }

                Err(e) => {
                    debug!("Dialect source errored {:?}", dialect);
                    // [TODO] Handle error
                }

                Ok(DialectSourceUpdate::Ignored) => {
                    debug!("Dialect source ignored {:?}", dialect);
                }
            };
        }

        Ok(())
    }
}
