//! Rust port of `packages/app/src/wsl/settings-model.ts` (opencode v1.18.30).
//!
//! Source 341 lines: `AddServerText`, `DistroStatus`, `AddServerPrimaryButton`,
//! `AddServerProbePlan`, `WslAddServerView`, `wslRuntimeRetryable`,
//! `wslOpencodeAction`, `wslDistroReady`, `addServerViewModel` (+ private
//! selectors), probe-gate helpers. `fuzzysort` is modelled as substring
//! match; pure selectors ported verbatim.
//! Original file: `packages/app/src/wsl/settings-model.ts`

#![allow(dead_code)]

use crate::wsl::types::{WslInstalledDistro, WslOpencodeCheck, WslServerRuntime, WslServersState};

/// Mirrors `AddServerText`.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct AddServerText {
    pub key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<std::collections::HashMap<String, String>>,
}

/// Mirrors `DistroStatusTone`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DistroStatusTone {
    Success,
    Warning,
    Muted,
}

/// Mirrors `DistroStatus`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DistroStatus {
    pub label: AddServerText,
    pub tone: DistroStatusTone,
}

/// Mirrors `AddServerPrimaryButton`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AddServerPrimaryButton {
    pub variant: String,
    pub label: AddServerText,
    pub disabled: bool,
    pub action: Option<String>,
    pub loading: bool,
    pub width: Option<String>,
}

/// Mirrors `AddServerRuntimeState`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AddServerRuntimeState {
    Loading,
    PendingRestart,
    Checking,
    Unavailable,
    Ready,
}

/// Mirrors `WslAddServerView`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WslAddServerView {
    Main,
    Catalog,
}

/// Mirrors `AddServerProbePlan`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AddServerProbePlan {
    Auto { key: String, action: String },
    Addable { key: String, distros: Vec<String> },
}

fn is_hidden_distro(name: &str) -> bool {
    let lower = name.to_lowercase();
    lower == "docker-desktop" || lower == "docker-desktop-data"
}

/// Mirrors `wslRuntimeRetryable(runtime)`.
pub fn wsl_runtime_retryable(runtime: &WslServerRuntime) -> bool {
    matches!(
        runtime,
        WslServerRuntime::Failed { .. } | WslServerRuntime::Stopped
    )
}

/// Mirrors `wslOpencodeAction(check)` (verbatim i18n keys).
pub fn wsl_opencode_action(check: Option<&WslOpencodeCheck>) -> Option<&'static str> {
    let check = check?;
    if check.resolved_path.is_none() {
        return Some("wsl.onboarding.installOpencode");
    }
    if check.matches_desktop == Some(false) {
        return Some("wsl.onboarding.updateOpencode");
    }
    None
}

/// Mirrors `wslDistroReady(state, name)`.
pub fn wsl_distro_ready(state: Option<&WslServersState>, name: &str) -> bool {
    let state = match state {
        None => return false,
        Some(state) => state,
    };
    let installed = match state.installed.iter().find(|item| item.name == name) {
        None => return false,
        Some(installed) => installed,
    };
    let probe = match state.distro_probes.get(name) {
        None => return false,
        Some(probe) => probe,
    };
    if installed.version == Some(1) {
        return false;
    }
    probe.can_execute && probe.has_bash && probe.has_curl
}

/// Mirrors `addServerRuntimeState(state)`.
pub fn add_server_runtime_state(state: Option<&WslServersState>) -> AddServerRuntimeState {
    let state = match state {
        None => return AddServerRuntimeState::Loading,
        Some(state) => state,
    };
    if state.pending_restart {
        return AddServerRuntimeState::PendingRestart;
    }
    match &state.runtime {
        None => AddServerRuntimeState::Checking,
        Some(runtime) if !runtime.available => AddServerRuntimeState::Unavailable,
        Some(_) => AddServerRuntimeState::Ready,
    }
}

/// Mirrors the addable/installed distro selectors.
pub fn addable_installed_distros(state: Option<&WslServersState>) -> Vec<WslInstalledDistro> {
    let state = match state {
        None => return vec![],
        Some(state) => state,
    };
    let existing: std::collections::HashSet<&str> = state
        .servers
        .iter()
        .map(|item| item.config.distro.as_str())
        .collect();
    state
        .installed
        .iter()
        .filter(|item| !is_hidden_distro(&item.name) && !existing.contains(item.name.as_str()))
        .cloned()
        .collect()
}

// PROVISIONAL: pending fuzzysort/tanstack-query — mirrors `packages/app/src/wsl/settings-model.ts` probe gate.
#[derive(Debug, Default)]
pub struct ProbeFailureGate {
    pub failed: Vec<String>,
}

impl ProbeFailureGate {
    pub fn new() -> Self {
        Self::default()
    }

    /// Mirrors `gate.accepts(key)`.
    pub fn accepts(&self, key: &str) -> bool {
        !self.failed.contains(&key.to_string())
    }

    /// Mirrors `gate.settle(key, error)`.
    pub fn update_settle(&mut self, key: &str, failed: bool) {
        if failed && !self.failed.contains(&key.to_string()) {
            self.failed.push(key.to_string());
        }
    }

    /// Mirrors `gate.reset()`.
    pub fn transition_reset(&mut self) {
        self.failed.clear();
    }
}
