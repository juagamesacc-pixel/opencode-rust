//! Rust port of `src/renderer/wsl/connections.ts` (opencode v1.18.30).
//!
//! `WslServersState` mirrors the `@opencode-ai/app/wsl/types` shape the
//! source imports; the canonical definition lives in the app package (see
//! `crate::preload::types`).
//!
//! Original file: `packages/desktop/src/renderer/wsl/connections.ts`

use crate::preload::types::{WslServerRuntime, WslServersState};

pub const CONNECTION_TYPE_SIDECAR: &str = "sidecar";
pub const CONNECTION_VARIANT_WSL: &str = "wsl";
pub const DEFAULT_CONNECTION_LABEL: &str = "WSL";
pub const DEFAULT_STARTUP_SERVER: &str = "sidecar";

pub struct WslConnectionHttp {
    pub url: String,
    pub username: Option<String>,
    pub password: Option<String>,
}

pub struct WslConnection {
    pub display_name: String,
    pub label: String,
    pub connection_type: String,
    pub variant: String,
    pub distro: String,
    pub http: WslConnectionHttp,
}

pub fn ready_wsl_connections(state: Option<&WslServersState>, label: &str) -> Vec<WslConnection> {
    let Some(state) = state else {
        return Vec::new();
    };
    state
        .servers
        .iter()
        .filter_map(|item| match &item.runtime {
            WslServerRuntime::Ready {
                url,
                username,
                password,
            } => Some(WslConnection {
                display_name: item.config.distro.clone(),
                label: label.to_string(),
                connection_type: CONNECTION_TYPE_SIDECAR.to_string(),
                variant: CONNECTION_VARIANT_WSL.to_string(),
                distro: item.config.distro.clone(),
                http: WslConnectionHttp {
                    url: url.clone(),
                    // Mirrors `?? undefined`: null/None stays absent.
                    username: username.clone(),
                    password: password.clone(),
                },
            }),
            _ => None,
        })
        .collect()
}

pub fn available_startup_server(
    default_server: Option<&str>,
    state: Option<&WslServersState>,
) -> String {
    let key = default_server.unwrap_or(DEFAULT_STARTUP_SERVER);
    if !key.starts_with("wsl:") {
        return key.to_string();
    }
    let ready = state.map(|state| {
        state.servers.iter().any(|item| {
            item.config.id == key && matches!(item.runtime, WslServerRuntime::Ready { .. })
        })
    });
    if ready == Some(true) {
        return key.to_string();
    }
    DEFAULT_STARTUP_SERVER.to_string()
}

#[cfg(test)]
mod tests {
    // Mirrors `src/renderer/wsl/connections.test.ts`
    // (`describe("WSL desktop connections")`).
    use super::*;
    use crate::preload::types::{WslServerConfig, WslServerItem};
    use std::collections::HashMap;

    fn state(kind: &str) -> WslServersState {
        let runtime = match kind {
            "ready" => WslServerRuntime::Ready {
                url: "http://127.0.0.1:4096".to_string(),
                username: Some("opencode".to_string()),
                password: Some("secret".to_string()),
            },
            "failed" => WslServerRuntime::Failed {
                message: "boom".to_string(),
            },
            "starting" => WslServerRuntime::Starting,
            _ => WslServerRuntime::Stopped,
        };
        WslServersState {
            runtime: None,
            installed: Vec::new(),
            online: Vec::new(),
            distro_probes: HashMap::new(),
            opencode_checks: HashMap::new(),
            pending_restart: false,
            servers: vec![WslServerItem {
                config: WslServerConfig {
                    id: "wsl:Debian".to_string(),
                    distro: "Debian".to_string(),
                },
                runtime,
            }],
            job: None,
        }
    }

    #[test]
    fn publishes_a_wsl_server_only_after_it_reports_ready() {
        assert!(ready_wsl_connections(Some(&state("starting")), "WSL").is_empty());
        assert!(ready_wsl_connections(Some(&state("failed")), "WSL").is_empty());
        assert!(ready_wsl_connections(Some(&state("stopped")), "WSL").is_empty());
        let ready = ready_wsl_connections(Some(&state("ready")), "WSL");
        assert_eq!(ready.len(), 1);
        assert_eq!(ready[0].display_name, "Debian");
        assert_eq!(ready[0].label, "WSL");
    }

    #[test]
    fn uses_the_renderer_translation_for_the_wsl_connection_label() {
        let ready = ready_wsl_connections(Some(&state("ready")), "Translated WSL");
        assert_eq!(ready[0].label, "Translated WSL");
    }

    #[test]
    fn does_not_block_desktop_startup_on_a_configured_wsl_default() {
        let key = "wsl:Debian";
        assert_eq!(available_startup_server(Some(key), None), "sidecar");
        assert_eq!(
            available_startup_server(Some(key), Some(&state("starting"))),
            "sidecar"
        );
        assert_eq!(
            available_startup_server(Some(key), Some(&state("ready"))),
            key
        );
    }
}
