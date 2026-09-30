// source: packages/tui/src/routes/home/session-destination.tsx (41 lines, v1.18.30)
// 1:1 port — optional override with sync-path/cwd fallback.

#![allow(dead_code)]

/// Mirrors `HomeSessionDestination`.
#[derive(Debug, Clone)]
pub enum HomeSessionDestination {
    Directory {
        directory: String,
        subdirectory: bool,
    },
    New,
}

/// Mirrors the destination context value.
#[derive(Debug, Clone, Default)]
pub struct HomeSessionDestinationState {
    selected: Option<HomeSessionDestination>,
}

impl HomeSessionDestinationState {
    /// Mirrors the `destination` memo (override or directory fallback).
    pub fn destination(&self, sync_directory: &str, cwd: &str) -> HomeSessionDestination {
        self.selected
            .clone()
            .unwrap_or(HomeSessionDestination::Directory {
                directory: if sync_directory.is_empty() {
                    cwd.to_string()
                } else {
                    sync_directory.to_string()
                },
                subdirectory: false,
            })
    }

    pub fn selected(&self) -> Option<&HomeSessionDestination> {
        self.selected.as_ref()
    }

    pub fn set_destination(&mut self, destination: HomeSessionDestination) {
        self.selected = Some(destination);
    }

    /// Mirrors `clear`.
    pub fn clear(&mut self) {
        self.selected = None;
    }
}
