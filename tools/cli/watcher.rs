// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad CLI watcher. Most parts proudly taken from https://github.com/typst/typst/blob/main/crates/typst-cli/src/watch.rs

use std::{iter, path::PathBuf, sync::mpsc::Receiver, time::Duration};

use notify::{Event, RecommendedWatcher};

/// Watches file system activity.
pub struct Watcher {
    /// The underlying watcher.
    watcher: RecommendedWatcher,
    /// Notify event receiver.
    rx: Receiver<notify::Result<Event>>,
    /// Keeps track of which paths are watched via `watcher`. The boolean is
    /// used during updating for mark-and-sweep garbage collection of paths we
    /// should unwatch.
    watched: std::collections::HashMap<PathBuf, bool>,
    /// A set of files that should be watched, but don't exist. We manually poll
    /// for those.
    missing: std::collections::HashSet<PathBuf>,
}

impl Watcher {
    /// How long to wait for a shortly following file system event when
    /// watching.
    const BATCH_TIMEOUT: Duration = Duration::from_millis(100);

    /// The maximum time we spend batching events before quitting wait().
    const STARVE_TIMEOUT: Duration = Duration::from_millis(500);

    /// The interval in which we poll when falling back to poll watching
    /// due to missing files.
    const POLL_INTERVAL: Duration = Duration::from_millis(300);

    /// Create a new, blank watcher.
    pub fn new() -> anyhow::Result<Self> {
        // Setup file watching.
        let (tx, rx) = std::sync::mpsc::channel();
        use notify::Watcher;

        // Set the poll interval to something more eager than the default. That
        // default seems a bit excessive for our purposes at around 30s.
        // Depending on feedback, some tuning might still be in order. Note that
        // this only affects a tiny number of systems. Most do not use the
        // [`notify::PollWatcher`].
        let config = notify::Config::default().with_poll_interval(Self::POLL_INTERVAL);
        let watcher = RecommendedWatcher::new(tx, config)?;

        Ok(Self {
            rx,
            watcher,
            watched: std::collections::HashMap::new(),
            missing: std::collections::HashSet::new(),
        })
    }

    /// Update the watching to watch exactly the listed files.
    ///
    /// Files that are not yet watched will be watched. Files that are already
    /// watched, but don't need to be watched anymore, will be unwatched.
    pub fn update(&mut self, iter: impl IntoIterator<Item = PathBuf>) -> anyhow::Result<()> {
        use notify::{RecursiveMode, Watcher};

        // Mark all files as not "seen" so that we may unwatch them if they
        // aren't in the dependency list.
        for seen in self.watched.values_mut() {
            *seen = false;
        }

        // Reset which files are missing.
        self.missing.clear();

        // Retrieve the dependencies of the last compilation and watch new paths
        // that weren't watched yet.
        for path in iter {
            // We can't watch paths that don't exist with notify-rs. Instead, we
            // add those to a `missing` set and fall back to manual poll
            // watching.
            if !path.exists() {
                self.missing.insert(path);
                continue;
            }

            // Watch the path if it's not already watched.
            if !self.watched.contains_key(&path) {
                self.watcher.watch(&path, RecursiveMode::NonRecursive)?;
            }

            // Mark the file as "seen" so that we don't unwatch it.
            self.watched.insert(path, true);
        }

        // Unwatch old paths that don't need to be watched anymore.
        self.watched.retain(|path, &mut seen| {
            if !seen {
                self.watcher.unwatch(path).ok();
            }
            seen
        });

        Ok(())
    }

    /// Wait until there is a change to a watched path.
    pub fn wait(&mut self) -> anyhow::Result<()> {
        use notify::Watcher;
        use std::time::Instant;

        loop {
            // Wait for an initial event. If there are missing files, we need to
            // poll those regularly to check whether they are created, so we
            // wait with a smaller timeout.
            let first = self.rx.recv_timeout(if self.missing.is_empty() {
                Duration::MAX
            } else {
                Self::POLL_INTERVAL
            });

            // Watch for file system events. If multiple events happen
            // consecutively all within a certain duration, then they are
            // bunched up without a recompile in-between. This helps against
            // some editors' remove & move behavior. Events are also only
            // watched until a certain point, to hinder a barrage of events from
            // preventing recompilations.
            let mut relevant = false;
            let batch_start = Instant::now();
            for event in first
                .into_iter()
                .chain(iter::from_fn(|| {
                    self.rx.recv_timeout(Self::BATCH_TIMEOUT).ok()
                }))
                .take_while(|_| batch_start.elapsed() <= Self::STARVE_TIMEOUT)
            {
                let event = event?;

                if !Self::is_relevant_event_kind(&event.kind) {
                    continue;
                }

                // Workaround for notify-rs' implicit unwatch on remove/rename
                // (triggered by some editors when saving files) with the
                // inotify backend. By keeping track of the potentially
                // unwatched files, we can allow those we still depend on to be
                // watched again later on.
                if matches!(
                    event.kind,
                    notify::EventKind::Remove(notify::event::RemoveKind::File)
                        | notify::EventKind::Modify(notify::event::ModifyKind::Name(
                            notify::event::RenameMode::From
                        ))
                ) {
                    for path in &event.paths {
                        // Remove affected path from the watched map to restart
                        // watching on it later again.
                        self.watcher.unwatch(path).ok();
                        self.watched.remove(path);
                    }
                }

                relevant = true;
            }

            // If we found a relevant event or if any of the missing files now
            // exists, stop waiting.
            if relevant || self.missing.iter().any(|path| path.exists()) {
                return Ok(());
            }
        }
    }

    /// Whether a kind of watch event is relevant for compilation.
    fn is_relevant_event_kind(kind: &notify::EventKind) -> bool {
        match kind {
            notify::EventKind::Any => true,
            notify::EventKind::Access(_) => false,
            notify::EventKind::Create(_) => true,
            notify::EventKind::Modify(kind) => match kind {
                notify::event::ModifyKind::Any => true,
                notify::event::ModifyKind::Data(_) => true,
                notify::event::ModifyKind::Metadata(_) => false,
                notify::event::ModifyKind::Name(_) => true,
                notify::event::ModifyKind::Other => false,
            },
            notify::EventKind::Remove(_) => true,
            notify::EventKind::Other => false,
        }
    }
}
