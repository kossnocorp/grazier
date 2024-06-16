use crate::{fs_watcher::watch_fs, monorepo::Monorepo};
use std::path::PathBuf;

pub async fn watch_command(cwd: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let mut monorepo = Monorepo::new(cwd.clone());
    let (mut rx, stop) = watch_fs(cwd)?;

    monorepo.next(None).await?;

    while let Some(event) = rx.recv().await {
        println!("{event:?}");
        monorepo.next(Some(event)).await?;
    }

    stop();

    Ok(())
}
