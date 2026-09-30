//! Rust port of `src/main/wsl/servers.ts` (opencode v1.18.30).
//!
//! Fully ported: `wslServerIdForDistro`, `initialState`, persisted-server
//! normalization (`normalizePersistedServer`), `opencodeCheck`,
//! `distroProbeReady`, and `startupFailure`. PROVISIONAL: the async
//! `WslServersController` (job/abort bookkeeping, start-attempt
//! invalidation, background checks — needs an async runtime) and the
//! `electron-store`-backed persistence. The controller keeps its exact
//! method names/parameters/return shapes with `unimplemented!()` bodies;
//! the sync-observable surface (`new`, `getState`, `subscribe`,
//! `unsubscribe`) is implemented.
//!
//! The four async `servers.test.ts` controller cases (stale background /
//! startup checks, parallel `probeAddable`, non-executable filtering) have
//! no executable equivalent without the async runtime — recorded as
//! skipped-with-reason (plan §10 R3), not dropped.
//!
//! Original file: `packages/desktop/src/main/wsl/servers.ts`

use crate::main::native_translations::native_t;
use crate::main::store_keys::WSL_SERVERS_KEY;
use crate::preload::types::{
    WslDistroProbe, WslOpencodeCheck, WslServerConfig, WslServersEvent, WslServersState,
};
use std::collections::HashMap;

// Note: the source re-exports its `preload/types` imports for callers
// (`export type { WslInstalledDistro, … }`); in Rust those types are used
// from `crate::preload::types` directly, so no re-export is needed.

pub fn wsl_server_id_for_distro(distro: &str) -> String {
    format!("wsl:{}", distro)
}

pub fn initial_state() -> WslServersState {
    WslServersState {
        runtime: None,
        installed: Vec::new(),
        online: Vec::new(),
        distro_probes: HashMap::new(),
        opencode_checks: HashMap::new(),
        pending_restart: false,
        servers: Vec::new(),
        job: None,
    }
}

/// Mirrors `normalizePersistedServer` (the source returns 0-or-1 items for
/// `flatMap`; `None` is the empty case).
pub fn normalize_persisted_server(value: &serde_json::Value) -> Option<WslServerConfig> {
    let record = value.as_object()?;
    let distro = record.get("distro")?.as_str()?;
    if distro.is_empty() {
        return None;
    }
    let id = match record.get("id").and_then(|id| id.as_str()) {
        Some(id) if !id.is_empty() => id.to_string(),
        _ => wsl_server_id_for_distro(distro),
    };
    Some(WslServerConfig {
        id,
        distro: distro.to_string(),
    })
}

pub fn opencode_check(
    distro: &str,
    resolved_path: Option<&str>,
    version: Option<&str>,
    expected_version: &str,
) -> WslOpencodeCheck {
    let resolved_path = resolved_path.map(str::to_string);
    if resolved_path.is_none() {
        return WslOpencodeCheck {
            distro: distro.to_string(),
            resolved_path: None,
            version: None,
            expected_version: expected_version.to_string(),
            matches_desktop: None,
            error: Some(native_t("desktop.wsl.error.opencodeMissing", &[])),
        };
    }
    let version = version.map(str::to_string);
    if version.is_none() {
        return WslOpencodeCheck {
            distro: distro.to_string(),
            resolved_path,
            version: None,
            expected_version: expected_version.to_string(),
            matches_desktop: None,
            error: Some(native_t("desktop.wsl.error.opencodeCannotRun", &[])),
        };
    }
    let version = version.unwrap_or_default();
    let matches_desktop = version == expected_version;
    WslOpencodeCheck {
        distro: distro.to_string(),
        resolved_path,
        version: Some(version),
        expected_version: expected_version.to_string(),
        matches_desktop: Some(matches_desktop),
        error: None,
    }
}

pub fn distro_probe_ready(probe: Option<&WslDistroProbe>) -> bool {
    match probe {
        Some(probe) => probe.can_execute && probe.has_bash && probe.has_curl,
        None => false,
    }
}

pub fn startup_failure(code: Option<i32>, signal: Option<&str>) -> String {
    native_t(
        "desktop.wsl.error.serverExited",
        &[
            (
                "code",
                code.map(|code| code.to_string())
                    .unwrap_or_else(|| "null".to_string()),
            ),
            ("signal", signal.unwrap_or("null").to_string()),
        ],
    )
}

// PROVISIONAL(packages/desktop/src/main/wsl/servers.ts): `electron-store`
// (`getStore()` + `WSL_SERVERS_KEY`) for
// `readPersistedServers`/`writePersistedServers`.
pub fn read_persisted_servers() -> Vec<WslServerConfig> {
    let _ = WSL_SERVERS_KEY;
    unimplemented!("electron-store binding")
}

// PROVISIONAL(packages/desktop/src/main/wsl/servers.ts): `electron-store`.
pub fn write_persisted_servers(_servers: &[WslServerConfig]) {
    unimplemented!("electron-store binding")
}

/// Mirrors `WslServersController` (the `createWslServersController`
/// return). Async methods are PROVISIONAL (need an async runtime with
/// `AbortController`, timers, and the `wsl.exe`/`node-pty` bindings);
/// preserved source log text, byte-identical:
/// - "wsl opencode check failed"
/// - "wsl sidecar starting"
/// - "wsl sidecar ready"
/// - "wsl sidecar exited"
/// - "wsl sidecar failed to start".
pub struct WslServersController {
    app_version: String,
    state: WslServersState,
    listeners: HashMap<u64, Box<dyn Fn(WslServersEvent)>>,
    next_listener_id: u64,
}

impl WslServersController {
    pub fn new(app_version: String) -> Self {
        Self {
            app_version,
            state: initial_state(),
            listeners: HashMap::new(),
            next_listener_id: 0,
        }
    }

    pub fn app_version(&self) -> &str {
        &self.app_version
    }

    pub fn get_state(&self) -> &WslServersState {
        &self.state
    }

    pub fn subscribe(&mut self, listener: impl Fn(WslServersEvent) + 'static) -> u64 {
        let id = self.next_listener_id;
        self.next_listener_id += 1;
        self.listeners.insert(id, Box::new(listener));
        id
    }

    pub fn unsubscribe(&mut self, id: u64) {
        self.listeners.remove(&id);
    }

    pub fn initialize(&mut self) {
        unimplemented!("async runtime binding")
    }

    pub fn probe_runtime(&mut self) {
        unimplemented!("async runtime binding")
    }

    pub fn refresh_distros(&mut self) {
        unimplemented!("async runtime binding")
    }

    pub fn install_wsl(&mut self) {
        unimplemented!("async runtime binding")
    }

    pub fn install_distro(&mut self, _name: &str) {
        unimplemented!("async runtime binding")
    }

    pub fn probe_addable(&mut self, _distros: Vec<String>) {
        unimplemented!("async runtime binding")
    }

    pub fn install_opencode(&mut self, _name: &str) {
        unimplemented!("async runtime binding")
    }

    pub fn open_terminal(&mut self, _name: &str) {
        unimplemented!("async runtime binding")
    }

    pub fn add_server(&mut self, _distro: &str) -> WslServerConfig {
        unimplemented!("async runtime binding")
    }

    pub fn remove_server(&mut self, _id: &str) {
        unimplemented!("async runtime binding")
    }

    pub fn start_server(&mut self, _id: &str) {
        unimplemented!("async runtime binding")
    }

    pub fn stop_all(&mut self) {
        unimplemented!("async runtime binding")
    }
}

// PROVISIONAL(packages/desktop/src/main/wsl/servers.ts): mirrors
// `createWslServersController(appVersion, spawnSidecar, options?)`.
pub fn create_wsl_servers_controller(_app_version: String) -> WslServersController {
    unimplemented!("async runtime binding")
}

#[cfg(test)]
mod tests {
    // The pure-helper assertions from `src/main/wsl/servers.test.ts` that
    // target `./servers` directly: none — the file's sync assertions cover
    // `./policy` and `./startup` (ported in those modules) plus the four
    // async controller cases recorded above as skipped-with-reason. The
    // cases below pin the remaining ported helpers.
    use super::*;

    #[test]
    fn server_id_for_distro_prefixes_wsl() {
        assert_eq!(wsl_server_id_for_distro("Debian"), "wsl:Debian");
    }

    #[test]
    fn normalizes_persisted_servers() {
        assert_eq!(
            normalize_persisted_server(
                &serde_json::json!({ "id": "wsl:Debian", "distro": "Debian" })
            ),
            Some(WslServerConfig {
                id: "wsl:Debian".to_string(),
                distro: "Debian".to_string(),
            })
        );
        assert_eq!(
            normalize_persisted_server(&serde_json::json!({ "distro": "Debian" })),
            Some(WslServerConfig {
                id: "wsl:Debian".to_string(),
                distro: "Debian".to_string(),
            })
        );
        assert_eq!(
            normalize_persisted_server(&serde_json::json!({ "distro": "" })),
            None
        );
        assert_eq!(normalize_persisted_server(&serde_json::json!(null)), None);
    }

    #[test]
    fn opencode_check_reports_missing_unrunnable_and_match() {
        let missing = opencode_check("Debian", None, None, "1.16.2");
        assert_eq!(missing.matches_desktop, None);
        assert!(missing.error.is_some());

        let unrunnable = opencode_check("Debian", Some("/bin/opencode"), None, "1.16.2");
        assert_eq!(unrunnable.matches_desktop, None);
        assert!(unrunnable.error.is_some());

        let matched = opencode_check("Debian", Some("/bin/opencode"), Some("1.16.2"), "1.16.2");
        assert_eq!(matched.matches_desktop, Some(true));
        assert_eq!(matched.error, None);

        let mismatched = opencode_check("Debian", Some("/bin/opencode"), Some("1.14.35"), "1.16.2");
        assert_eq!(mismatched.matches_desktop, Some(false));
    }

    #[test]
    fn distro_probe_ready_requires_execute_bash_and_curl() {
        assert!(!distro_probe_ready(None));
        assert!(distro_probe_ready(Some(&WslDistroProbe {
            name: "Debian".to_string(),
            can_execute: true,
            has_bash: true,
            has_curl: true,
            error: None,
        })));
        assert!(!distro_probe_ready(Some(&WslDistroProbe {
            name: "Ubuntu".to_string(),
            can_execute: true,
            has_bash: true,
            has_curl: false,
            error: None,
        })));
    }
}
