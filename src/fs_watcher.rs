use notify::{Error, RecursiveMode, Watcher};
use notify_debouncer_full::{new_debouncer, DebouncedEvent};
use std::path::PathBuf;
use std::time::Duration;
use tokio::sync::mpsc::{self, Receiver};

pub fn watch_fs(cwd: &PathBuf) -> Result<(Receiver<DebouncedEvent>, impl FnOnce()), Error> {
    let (notify_tx, notify_rx) = std::sync::mpsc::channel();

    let mut debouncer = new_debouncer(Duration::from_millis(50), None, move |res| {
        notify_tx.send(res).unwrap();
    })?;

    debouncer
        .watcher()
        .watch(cwd.as_path(), RecursiveMode::Recursive)?;

    let stop = move || drop(debouncer);
    let (tx, rx) = mpsc::channel(32);

    tokio::spawn(async move {
        while let Ok(res) = notify_rx.recv() {
            match res {
                Ok(events) => {
                    for event in events {
                        tx.send(event).await.unwrap();
                    }
                }
                Err(errors) => {
                    for error in errors {
                        eprintln!("{error:?}");
                    }
                }
            }
        }
    });

    Ok((rx, stop))
}
