//! Rust port of `src/main/wsl/ipc.ts` (opencode v1.18.30).
//!
//! Fully ported: every `wsl-servers-*` channel name (byte-identical) and
//! the unavailable-state builder used on non-Windows platforms. PROVISIONAL:
//! `registerWslIpcHandlers` itself (`ipcMain.handle`, sender tracking,
//! `app.once("will-quit")`); the channel→controller routing is preserved in
//! the function docs.
//!
//! Original file: `packages/desktop/src/main/wsl/ipc.ts`

use crate::main::native_translations::native_t;
use crate::preload::types::{WslRuntimeCheck, WslServersState};
use std::collections::HashMap;

pub const CHANNEL_WSL_SERVERS_SUBSCRIBE: &str = "wsl-servers-subscribe";
pub const CHANNEL_WSL_SERVERS_UNSUBSCRIBE: &str = "wsl-servers-unsubscribe";
pub const CHANNEL_WSL_SERVERS_GET_STATE: &str = "wsl-servers-get-state";
pub const CHANNEL_WSL_SERVERS_PROBE_RUNTIME: &str = "wsl-servers-probe-runtime";
pub const CHANNEL_WSL_SERVERS_REFRESH_DISTROS: &str = "wsl-servers-refresh-distros";
pub const CHANNEL_WSL_SERVERS_INSTALL_WSL: &str = "wsl-servers-install-wsl";
pub const CHANNEL_WSL_SERVERS_INSTALL_DISTRO: &str = "wsl-servers-install-distro";
pub const CHANNEL_WSL_SERVERS_PROBE_ADDABLE: &str = "wsl-servers-probe-addable";
pub const CHANNEL_WSL_SERVERS_INSTALL_OPENCODE: &str = "wsl-servers-install-opencode";
pub const CHANNEL_WSL_SERVERS_OPEN_TERMINAL: &str = "wsl-servers-open-terminal";
pub const CHANNEL_WSL_SERVERS_ADD: &str = "wsl-servers-add";
pub const CHANNEL_WSL_SERVERS_REMOVE: &str = "wsl-servers-remove";
pub const CHANNEL_WSL_SERVERS_START: &str = "wsl-servers-start";
pub const EVENT_WSL_SERVERS_EVENT: &str = "wsl-servers-event";

/// Mirrors the `state()` builder in `registerUnavailableWslIpcHandlers`.
pub fn unavailable_servers_state() -> WslServersState {
    WslServersState {
        runtime: Some(WslRuntimeCheck {
            available: false,
            version: None,
            error: Some(native_t("desktop.wsl.error.windowsOnly", &[])),
        }),
        installed: Vec::new(),
        online: Vec::new(),
        distro_probes: HashMap::new(),
        opencode_checks: HashMap::new(),
        pending_restart: false,
        servers: Vec::new(),
        job: None,
    }
}

// PROVISIONAL(packages/desktop/src/main/wsl/ipc.ts): needs
// `ipcMain.handle` (all 13 channels above), `event.sender` tracking, and
// `app.once("will-quit")`. Routing preserved from the source:
// - `wsl-servers-subscribe` → per-sender `controller.subscribe`, fan-out
//   `wsl-servers-event`,sender-destroyed cleanup
// - `wsl-servers-unsubscribe` → drop the sender subscription
// - `wsl-servers-get-state` → `controller.getState()`
// - `wsl-servers-probe-runtime` → `controller.probeRuntime()`
// - `wsl-servers-refresh-distros` → `controller.refreshDistros()`
// - `wsl-servers-install-wsl` → `controller.installWsl()`
// - `wsl-servers-install-distro` → `controller.installDistro(requireWslIpcString("distro", name))`
// - `wsl-servers-probe-addable` → `controller.probeAddable(requireWslIpcStrings("distro", distros))`
// - `wsl-servers-install-opencode` → `controller.installOpencode(requireWslIpcString("distro", name))`
// - `wsl-servers-open-terminal` → `controller.openTerminal(requireWslIpcString("distro", name))`
// - `wsl-servers-add` → `controller.addServer(requireWslIpcString("distro", distro))`
// - `wsl-servers-remove` → `controller.removeServer(requireWslIpcString("server id", id))`
// - `wsl-servers-start` → `controller.startServer(requireWslIpcString("server id", id))`
// Non-Windows platforms register the unavailable variants instead
// (`nativeT("desktop.wsl.error.windowsOnly")`).
pub fn register_wsl_ipc_handlers() {
    unimplemented!("Electron ipcMain binding")
}
