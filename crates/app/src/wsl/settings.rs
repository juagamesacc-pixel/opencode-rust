//! Rust port of `packages/app/src/wsl/settings.tsx` (opencode v1.18.30).
//!
//! Source 173 lines: `isWslServer`, `AddServerMenu`, `useFilteredWslServers`,
//! `WslServerSettings` (UI) + server-row integration. SolidJS/UI wiring is
//! PROVISIONAL; pure helpers + i18n keys verbatim.
//! Original file: `packages/app/src/wsl/settings.tsx`

#![allow(dead_code)]

// PROVISIONAL: pending solid-js/fuzzysort/tanstack-query/ui — mirrors `packages/app/src/wsl/settings.tsx`.

use crate::wsl::types::WslServerConfig;

/// Mirrors `ServerConnection.Any` discriminator check (`type === "sidecar" && variant === "wsl"`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServerConnectionRef {
    pub conn_type: String,
    pub variant: Option<String>,
}

/// Mirrors `isWslServer(server)`.
pub fn is_wsl_server(conn_type: &str, variant: Option<&str>) -> bool {
    conn_type == "sidecar" && variant == Some("wsl")
}

/// Mirrors the `useFilteredWslServers` filter input (verbatim keys for fuzzysort).
pub fn filter_wsl_servers(distros: Vec<WslServerConfig>, query: &str) -> Vec<WslServerConfig> {
    let q = query.trim().to_lowercase();
    if q.is_empty() {
        return distros;
    }
    // PROVISIONAL: fuzzysort modelled as substring match — mirrors `packages/app/src/wsl/settings.tsx`.
    distros
        .into_iter()
        .filter(|item| {
            item.distro.to_lowercase().contains(&q) || item.id.to_lowercase().contains(&q)
        })
        .collect()
}

/// Mirrors the `AddServerMenu` i18n keys (verbatim).
pub const ADD_SERVER_MENU_KEYS: &[&str] = &["dialog.server.add.button", "wsl.server.add"];

/// Mirrors the `WslServerSettings` row i18n keys (verbatim order).
pub const WSL_SERVER_SETTINGS_KEYS: &[&str] = &[
    "common.requestFailed",
    "wsl.server.label",
    "dialog.server.status.default",
    "wsl.server.updating",
    "common.moreOptions",
    "wsl.server.menu.label",
    "wsl.server.retryStart",
    "dialog.server.menu.default",
    "dialog.server.menu.defaultRemove",
    "dialog.server.menu.delete",
];

/// Mirrors the `WslServerSettings` descriptor (props-free, render is PROVISIONAL).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WslServerSettingsDescriptor {
    pub distro: String,
    pub is_default: bool,
    pub can_default: bool,
    pub opencode_action: Option<String>,
    pub busy: bool,
}

impl WslServerSettingsDescriptor {
    pub fn new(distro: &str) -> Self {
        Self {
            distro: distro.to_string(),
            is_default: false,
            can_default: false,
            opencode_action: None,
            busy: false,
        }
    }
}
