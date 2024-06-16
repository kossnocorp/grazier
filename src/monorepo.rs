use crate::js::npm::{flavor::find_npm_flavor, Npm};
use notify_debouncer_full::DebouncedEvent;
use std::path::PathBuf;

pub struct Monorepo {
    path: PathBuf,
    state: MonorepoState,
    npm: Npm,
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
            // npm_flavor: None,
            path,
            state: MonorepoState::Initial,
            npm,
        }
    }

    pub async fn next(
        &mut self,
        event: Option<DebouncedEvent>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.npm.next(event.as_ref()).await?;

        // match event {
        //     None => {
        //         println!("Initial event");
        //         let flavor = find_npm_flavor(&self.path).await;
        //     }

        //     _ => {
        //         println!("Received an event: {:?}", event);
        //     }
        // }
        Ok(())
    }
}
