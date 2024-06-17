use std::path::PathBuf;

use notify_debouncer_full::DebouncedEvent;

pub trait Source {
    fn workspaces(&self) -> Vec<PathBuf>;

    fn update(
        &mut self,
        event: Option<&DebouncedEvent>,
    ) -> Result<SourceUpdate, Box<dyn std::error::Error>>;
}

pub enum SourceUpdate {
    Ignored,
    Updated,
    Errored,
}
