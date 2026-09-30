//! Rust port of `packages/app/src/updater.ts` (opencode v1.18.30).
//!
//! Source 17 lines: `UpdaterState` union + `UpdaterPlatform` interface.
//! `Accessor<UpdaterState>` is modelled as an explicit `get_state` closure;
//! async methods are modelled synchronously with `UpdaterError` since the
//! Tauri/updater backend is unavailable here.
//! Original file: `packages/app/src/updater.ts`

#![allow(dead_code)]

/// Mirrors `UpdaterState`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UpdaterState {
    Disabled,
    Idle,
    Checking,
    Downloading {
        version: String,
        percent: Option<u8>,
    },
    Ready {
        version: String,
    },
    UpToDate,
    Installing {
        version: String,
    },
    Error {
        message: String,
    },
}

impl UpdaterState {
    /// Mirrors the `status` discriminant string.
    pub fn status(&self) -> &'static str {
        match self {
            UpdaterState::Disabled => "disabled",
            UpdaterState::Idle => "idle",
            UpdaterState::Checking => "checking",
            UpdaterState::Downloading { .. } => "downloading",
            UpdaterState::Ready { .. } => "ready",
            UpdaterState::UpToDate => "up-to-date",
            UpdaterState::Installing { .. } => "installing",
            UpdaterState::Error { .. } => "error",
        }
    }

    pub fn new() -> Self {
        UpdaterState::Idle
    }

    /// Explicit state-machine transition (mirrors SolidJS `Accessor` updates).
    pub fn transition(&mut self, next: UpdaterState) {
        *self = next;
    }
}

impl Default for UpdaterState {
    fn default() -> Self {
        UpdaterState::new()
    }
}

// PROVISIONAL: pending Tauri updater backend — mirrors `packages/app/src/updater.ts` (`UpdaterPlatform`).
/// Mirrors `UpdaterPlatform` (`state` accessor + async `check`/`install`).
pub trait UpdaterPlatform {
    fn state(&self) -> UpdaterState;
    fn check(&mut self) -> Result<UpdaterState, String>;
    fn install(&mut self) -> Result<(), String>;
}
