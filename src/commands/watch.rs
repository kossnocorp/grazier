use crate::{fs_watcher::watch_fs, monorepo::Monorepo};
use std::path::PathBuf;

pub fn watch_command(cwd: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let mut monorepo = Monorepo::new(cwd.clone());

    // Call initial update to make the monorepo initialize
    monorepo.update(None)?;

    // Start watching the filesystem
    let (events_tx, stop) = watch_fs(cwd)?;
    for event in events_tx {
        println!("{event:?}");
        monorepo.update(Some(event))?;
    }
    stop();

    Ok(())
}
