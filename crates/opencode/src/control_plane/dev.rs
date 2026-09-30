// source: src/control-plane/dev/debug-workspace-plugin.ts — exports:
// DebugWorkspacePlugin (default), DEV_DATA_FILE, waitForHealth rules verbatim.

/// source: DEV_DATA_FILE — verbatim.
pub const DEV_DATA_FILE: &str = "/tmp/opencode-workspace-dev-data.json";
/// source: temp suffix ".tmp" — verbatim.
pub const DEV_DATA_TEMP_SUFFIX: &str = ".tmp";

/// source: health path "/global/health" — verbatim.
pub const HEALTH_PATH: &str = "/global/health";
/// source: wait budget 30_000ms, poll 250ms — verbatim.
pub const HEALTH_TIMEOUT_MS: u64 = 30_000;
pub const HEALTH_POLL_MS: u64 = 250;

/// source: `Timed out waiting for debug server health check at ${url}` — verbatim.
pub fn health_timeout_message(url: &str) -> String {
    format!("Timed out waiting for debug server health check at {}", url)
}

/// source: port range 5000..9001 — verbatim.
pub const PORT_MIN: u32 = 5000;
pub const PORT_MAX: u32 = 9001;

/// source: adapter type "debug", name "Debug", description — verbatim.
pub const DEBUG_TYPE: &str = "debug";
pub const DEBUG_NAME: &str = "Debug";
pub const DEBUG_DESCRIPTION: &str = "Create a debugging server";
