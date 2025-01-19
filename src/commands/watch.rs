use crate::{fs_watcher::watch_fs, monorepo::Monorepo};
use std::path::PathBuf;

pub fn watch_command(cwd: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let mut monorepo = Monorepo::new(cwd.clone());

    // Call initial update to make the monorepo initialize
    monorepo.update(None)?;

    // Start watching the filesystem
    let (rx, stop) = watch_fs(cwd)?;
    for event in rx {
        println!("{event:?}");
        monorepo.update(Some(event))?;
    }
    stop();

    Ok(())
}
