//! Rust port of `src/main/server.ts` (opencode v1.18.30).
//!
//! Fully ported: `SidecarMessage`, the service/timeout constants, the
//! `preferAppEnv` env overlay keys (and the [`prefer_app_env`] entry itself,
//! modulo the shell-resolve runtime), the health-check URL construction, and
//! `createSidecarEnv` filtering (`DEBUG` dropped, `LD_PRELOAD` dropped on
//! Linux) — the env pieces over an injected map mirroring `process.env`.
//! PROVISIONAL: `spawnLocalServer` (`utilityProcess.fork` + message/timeout
//! flow), `checkHealth` (`fetch`), the `SidecarListener`/`HealthCheck`
//! structs, and the store-backed default-server-url accessors
//! (`electron-store`).
//!
//! Original file: `packages/desktop/src/main/server.ts`

use std::collections::HashMap;

pub const SIDECAR_SERVICE_NAME: &str = "opencode server";
pub const SIDECAR_START_STALL_TIMEOUT_MS: u64 = 60_000;
pub const SIDECAR_STOP_TIMEOUT_MS: u64 = 6_000;
pub const HEALTH_POLL_INTERVAL_MS: u64 = 100;
pub const HEALTH_REQUEST_TIMEOUT_MS: u64 = 3000;

pub enum SidecarMessage {
    Ready,
    Stopped,
    Error {
        message: String,
        stack: Option<String>,
    },
}

pub struct SerializedError {
    pub message: String,
    pub stack: Option<String>,
}

/// Mirrors the file-local `serializeError(error)`.
pub fn serialize_error(message: String, stack: Option<String>) -> SerializedError {
    SerializedError { message, stack }
}

pub struct SpawnLocalServerOptions {
    pub user_data_path: String,
    pub on_stdout: Option<Box<dyn FnMut(String)>>,
    pub on_stderr: Option<Box<dyn FnMut(String)>>,
    pub on_exit: Option<Box<dyn FnMut(i32)>>,
}

// PROVISIONAL(packages/desktop/src/main/server.ts): needs
// `utilityProcess.fork`, timers, and `fetch`. Preserved source error text,
// byte-identical:
// - "Sidecar did not become ready within {SIDECAR_START_STALL_TIMEOUT}ms: {sidecar}"
// - "Sidecar exited before ready with code {code}"
// - "Sidecar exited before health check passed with code {code}"
// - "utility process gone reason={reason} exitCode={exitCode}"
// - "utility process error: {message}".
#[allow(dead_code)]
const STALL_MESSAGE_PREFIX: &str = "Sidecar did not become ready within";
#[allow(dead_code)]
const EXIT_BEFORE_READY_PREFIX: &str = "Sidecar exited before ready with code";
#[allow(dead_code)]
const EXIT_BEFORE_HEALTHY_PREFIX: &str = "Sidecar exited before health check passed with code";

/// Mirrors the `preferAppEnv` overlay (`Object.assign` on `process.env`,
/// expressed as a mutation of the passed map). The shell-env lookup and
/// the `getLogger()` call stay PROVISIONAL; the key set is exact.
pub fn apply_app_env(
    env: &mut HashMap<String, String>,
    user_data_path: &str,
    shell_env: Option<&HashMap<String, String>>,
) {
    if let Some(shell_env) = shell_env {
        for (key, value) in shell_env {
            env.insert(key.clone(), value.clone());
        }
    }
    env.insert(
        "OPENCODE_EXPERIMENTAL_ICON_DISCOVERY".to_string(),
        "true".to_string(),
    );
    env.insert(
        "OPENCODE_EXPERIMENTAL_FILEWATCHER".to_string(),
        "true".to_string(),
    );
    env.insert("OPENCODE_CLIENT".to_string(), "desktop".to_string());
    if !env.contains_key("XDG_STATE_HOME") {
        env.insert("XDG_STATE_HOME".to_string(), user_data_path.to_string());
    }
}

/// Mirrors the health-check URL construction in the `wait` closure
/// (`http://{hostname}:{port}` + `/api/health` and `/global/health`).
pub fn health_urls(hostname: &str, port: u16) -> [String; 2] {
    let base = format!("http://{}:{}", hostname, port);
    [
        format!("{}/api/health", base),
        format!("{}/global/health", base),
    ]
}

/// Mirrors `createSidecarEnv()`: defined values stringified, `DEBUG`
/// dropped, `LD_PRELOAD` dropped on Linux.
pub fn create_sidecar_env(
    vars: &HashMap<String, Option<String>>,
    platform: &str,
) -> HashMap<String, String> {
    let mut env: HashMap<String, String> = vars
        .iter()
        .filter_map(|(key, value)| value.clone().map(|value| (key.clone(), value)))
        .collect();
    env.remove("DEBUG");
    if platform == "linux" {
        env.remove("LD_PRELOAD");
    }
    env
}

// PROVISIONAL(packages/desktop/src/main/server.ts): needs `fetch` with
// `AbortSignal.timeout(3000)` and Basic `opencode:{password}` auth.
pub fn check_health(_url: &str, _password: Option<&str>) -> bool {
    unimplemented!("fetch binding")
}

// PROVISIONAL(packages/desktop/src/main/server.ts): needs
// `electron-store` (`getStore()` + `DEFAULT_SERVER_URL_KEY`).
pub fn get_default_server_url() -> Option<String> {
    unimplemented!("electron-store binding")
}

// PROVISIONAL(packages/desktop/src/main/server.ts): needs
// `electron-store` (`getStore()` + `DEFAULT_SERVER_URL_KEY`).
pub fn set_default_server_url(_url: Option<&str>) {
    unimplemented!("electron-store binding")
}

// PROVISIONAL(packages/desktop/src/main/server.ts): needs
// `utilityProcess.fork(sidecar.js)` plus the ready/timeout/exit flow.
// The return type mirrors the source's `{ listener, health }`.
pub fn spawn_local_server(
    _hostname: &str,
    _port: u16,
    _password: &str,
    _options: SpawnLocalServerOptions,
) -> SpawnLocalServerResult {
    unimplemented!("Electron utilityProcess.fork binding")
}

/// Mirrors `type HealthCheck = { wait: Promise<void> }` — the promise resolved
/// once either health endpoint responds (or rejected on timeout).
///
/// PROVISIONAL(packages/desktop/src/main/server.ts): the `wait` promise is the
/// async health race; no async runtime binding exists in the port.
#[derive(Debug, Default)]
pub struct HealthCheck;

/// Mirrors `type SidecarListener = { stop: () => Promise<void> }`.
pub struct SidecarListener {
    pub stop: Option<Box<dyn FnMut()>>,
}

/// The `{ listener, health }` result of `spawnLocalServer`.
pub struct SpawnLocalServerResult {
    pub listener: SidecarListener,
    pub health: HealthCheck,
}

/// Mirrors `preferAppEnv(userDataPath)`: loads the shell env (or `None` when
/// no shell resolves — the source's `process.platform === "win32" ? null :
/// getUserShell()` short-circuits before the probe either way), overlays the
/// desktop defaults on the current process env, and returns the shell env.
///
/// PROVISIONAL(packages/desktop/src/main/server.ts): `getUserShell()` /
/// `loadShellEnv(shell, getLogger())` need the shell subprocess runtime; the
/// overlay itself is ported in [`apply_app_env`].
pub fn prefer_app_env(user_data_path: &str) -> Option<HashMap<String, String>> {
    let shell_env: Option<HashMap<String, String>> = None;
    let mut env = std::env::vars().collect::<HashMap<String, String>>();
    apply_app_env(&mut env, user_data_path, shell_env.as_ref());
    for (key, value) in env {
        std::env::set_var(key, value);
    }
    shell_env
}

#[cfg(test)]
mod tests {
    // No `src/main/server.test.ts` exists in the source; the cases below
    // pin the ported pure helpers to the source's inline behavior.
    use super::*;

    #[test]
    fn apply_app_env_sets_desktop_defaults() {
        let mut env = HashMap::from([("XDG_STATE_HOME".to_string(), "/keep".to_string())]);
        apply_app_env(
            &mut env,
            "/data",
            Some(&HashMap::from([("PATH".to_string(), "/shell".to_string())])),
        );
        assert_eq!(
            env.get("OPENCODE_CLIENT").map(String::as_str),
            Some("desktop")
        );
        assert_eq!(
            env.get("OPENCODE_EXPERIMENTAL_ICON_DISCOVERY")
                .map(String::as_str),
            Some("true")
        );
        assert_eq!(
            env.get("OPENCODE_EXPERIMENTAL_FILEWATCHER")
                .map(String::as_str),
            Some("true")
        );
        assert_eq!(env.get("XDG_STATE_HOME").map(String::as_str), Some("/keep"));
        assert_eq!(env.get("PATH").map(String::as_str), Some("/shell"));
    }

    #[test]
    fn health_urls_cover_both_endpoints() {
        assert_eq!(
            health_urls("127.0.0.1", 4096),
            [
                "http://127.0.0.1:4096/api/health".to_string(),
                "http://127.0.0.1:4096/global/health".to_string(),
            ]
        );
    }

    #[test]
    fn create_sidecar_env_drops_debug_and_linux_ld_preload() {
        let vars = HashMap::from([
            ("DEBUG".to_string(), Some("1".to_string())),
            ("LD_PRELOAD".to_string(), Some("/lib".to_string())),
            ("PATH".to_string(), Some("/bin".to_string())),
            ("EMPTY".to_string(), None),
        ]);
        let linux = create_sidecar_env(&vars, "linux");
        assert!(!linux.contains_key("DEBUG"));
        assert!(!linux.contains_key("LD_PRELOAD"));
        assert_eq!(linux.get("PATH").map(String::as_str), Some("/bin"));
        assert!(!linux.contains_key("EMPTY"));
        let macos = create_sidecar_env(&vars, "darwin");
        assert_eq!(macos.get("LD_PRELOAD").map(String::as_str), Some("/lib"));
    }
}
