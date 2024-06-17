use crate::{fs_watcher::watch_fs, monorepo::Monorepo};
use std::path::PathBuf;

pub async fn watch_command(cwd: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let mut monorepo = Monorepo::new(cwd.clone());

    // Call initial update to make the monorepo initialize
    monorepo.update(None).await?;

    // Start watching the filesystem
    let (mut rx, stop) = watch_fs(cwd)?;
    while let Some(event) = rx.recv().await {
        println!("{event:?}");
        monorepo.update(Some(event)).await?;
    }
    stop();

    Ok(())
}
