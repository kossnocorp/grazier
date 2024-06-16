use std::path::PathBuf;

use flavor::{find_npm_flavor, NpmFlavor, PlainFlavor};
use notify_debouncer_full::DebouncedEvent;

pub mod flavor;

pub struct Npm {
    path: PathBuf,
    state: NpmState,
}

impl Npm {
    pub fn new(path: PathBuf) -> Self {
        Self {
            path,
            state: NpmState::Initial,
        }
    }

    pub async fn next(
        &mut self,
        event: Option<&DebouncedEvent>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if let NpmState::Initial = self.state {
            let flavor = find_npm_flavor(&self.path).await;
            match flavor {
                Ok(flavor) => self.state = NpmState::Loaded(flavor),
                Err(_) => {
                    self.state = NpmState::Errored;
                    return Ok(());
                }
            }
        }

        if let NpmState::Errored = self.state {
            // TODO: Check
            return Ok(());
        }

        match self.state {
            NpmState::Initial => {
                let flavor = flavor::find_npm_flavor(&self.path).await?;
                self.state = NpmState::Loaded(flavor);
            }

            _ => {}
        }
        Ok(())
    }
}

pub enum NpmState {
    Initial,
    Loaded(NpmFlavor),
    Errored,
}
