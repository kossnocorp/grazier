use notify_debouncer_full::DebouncedEvent;

pub struct Monorepo {
    state: MonorepoState,
}

pub enum MonorepoState {
    Initial,
    Loading,
    Errored,
    Ready,
}

impl Monorepo {
    pub fn new() -> Self {
        Self {
            // npm_flavor: None,
            state: MonorepoState::Initial,
        }
    }

    pub async fn next(
        &self,
        event: Option<DebouncedEvent>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        tokio::time::sleep(std::time::Duration::from_millis(500)).await;
        println!("Received an event: {:?}", event);
        Ok(())
    }
}
