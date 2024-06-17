use crate::{
    js::npm::{Npm, NpmUpdate},
    source::{Source, SourceUpdate},
};
use notify_debouncer_full::DebouncedEvent;
use std::path::PathBuf;

pub struct Monorepo {
    path: PathBuf,
    state: MonorepoState,
    sources: Vec<Box<dyn Source>>,
    // npm: Npm,
}

pub enum MonorepoState {
    Initial,
    Loading,
    Errored,
    Ready,
}

impl Monorepo {
    pub fn new(path: PathBuf) -> Self {
        let npm = Npm::new(path.clone());
        Self {
            path,
            state: MonorepoState::Initial,
            sources: vec![Box::new(npm)],
        }
    }

    pub async fn update(
        &mut self,
        event: Option<DebouncedEvent>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut sources_updated = false;

        // [TODO] Run updates in parallel
        for source in &mut self.sources {
            let source_update = source.update(event.as_ref())?;
            if let SourceUpdate::Updated = source_update {
                sources_updated = true;
            }
        }

        if sources_updated {
            // [TODO] Calculate the new workspaces list:
            // - Initialize new
            // - Handle removed
        }

        Ok(())
    }
}
