use notify::{Error, RecursiveMode, Watcher};
use notify_debouncer_full::{new_debouncer, DebouncedEvent};
use std::path::PathBuf;
use std::sync::mpsc::{self, Receiver};
use std::thread;
use std::time::Duration;

pub fn watch_fs(cwd: &PathBuf) -> Result<(Receiver<DebouncedEvent>, impl FnOnce()), Error> {
    let (notify_tx, notify_rx) = mpsc::channel();

    let mut debouncer = new_debouncer(Duration::from_millis(50), None, move |res| {
        notify_tx.send(res).ok();
    })?;

    debouncer
        .watcher()
        .watch(cwd.as_path(), RecursiveMode::Recursive)?;

    let stop = move || drop(debouncer);
    let (events_tx, events_rx) = mpsc::channel();

    thread::spawn(move || {
        for res in notify_rx {
            match res {
                Ok(events) => {
                    for event in events {
                        if events_tx.send(event).is_err() {
                            break;
                        }
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

    Ok((events_rx, stop))
}
