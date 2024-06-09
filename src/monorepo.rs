use std::path::PathBuf;

use notify_debouncer_full::DebouncedEvent;

use crate::js::npm::flavor::find_npm_flavor;

pub struct Monorepo {
    path: PathBuf,
    state: MonorepoState,
}

pub enum MonorepoState {
    Initial,
    Loading,
    Errored,
    Ready,
}

impl Monorepo {
    pub fn new(path: PathBuf) -> Self {
        Self {
            // npm_flavor: None,
            path,
            state: MonorepoState::Initial,
        }
    }

    pub async fn next(
        &self,
        event: Option<DebouncedEvent>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        match event {
            None => {
                println!("Initial event");
                let flavor = find_npm_flavor(&self.path).await;
            }

            _ => {
                println!("Received an event: {:?}", event);
            }
        }
        Ok(())
    }
}
