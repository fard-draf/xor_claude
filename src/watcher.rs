use notify::{Config, Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::{Path, PathBuf};
use std::sync::mpsc::channel;
use tokio::sync::mpsc;

pub async fn watch_directory(
    dir_path: &Path,
    tx: mpsc::Sender<PathBuf>,
) -> Result<(), Box<dyn std::error::Error>> {
    let (sync_tx, sync_rx) = channel();

    let mut watcher = RecommendedWatcher::new(
        move |res: Result<Event, notify::Error>| {
            if let Ok(event) = res {
                match event.kind {
                    EventKind::Create(_) | EventKind::Modify(_) => {
                        for path in event.paths {
                            if path.extension().and_then(|s| s.to_str()) == Some("md") {
                                let _ = sync_tx.send(path);
                            }
                        }
                    }
                    _ => {}
                }
            }
        },
        Config::default(),
    )?;

    watcher.watch(dir_path, RecursiveMode::Recursive)?;

    // Convertir les événements synchrones en asynchrones
    tokio::spawn(async move {
        while let Ok(path) = sync_rx.recv() {
            let _ = tx.send(path).await;
        }
    });

    // Garder le watcher actif
    std::mem::forget(watcher);

    Ok(())
}
