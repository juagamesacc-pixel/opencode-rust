//! Rust port of `packages/app/src/wsl/types.ts` (opencode v1.18.30).
//!
//! Source 85 lines: `WslRuntimeCheck`, `WslInstalledDistro`,
//! `WslOnlineDistro`, `WslDistroProbe`, `WslOpencodeCheck`,
//! `WslServerConfig`, `WslServerRuntime`, `WslServerItem`, `WslJob`,
//! `WslServersState`, `WslServersEvent`, `WslServersPlatform`.
//! Async platform methods are modelled as descriptor enums (backend pending).
//! Original file: `packages/app/src/wsl/types.ts`

#![allow(dead_code)]

/// Mirrors `WslRuntimeCheck`.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct WslRuntimeCheck {
    pub available: bool,
    pub version: Option<String>,
    pub error: Option<String>,
}

/// Mirrors `WslInstalledDistro`.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct WslInstalledDistro {
    pub name: String,
    pub version: Option<u32>,
    #[serde(rename = "isDefault")]
    pub is_default: bool,
}

/// Mirrors `WslOnlineDistro`.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct WslOnlineDistro {
    pub name: String,
    pub label: String,
}

/// Mirrors `WslDistroProbe`.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct WslDistroProbe {
    pub name: String,
    #[serde(rename = "canExecute")]
    pub can_execute: bool,
    #[serde(rename = "hasBash")]
    pub has_bash: bool,
    #[serde(rename = "hasCurl")]
    pub has_curl: bool,
    pub error: Option<String>,
}

/// Mirrors `WslOpencodeCheck`.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct WslOpencodeCheck {
    pub distro: String,
    #[serde(rename = "resolvedPath")]
    pub resolved_path: Option<String>,
    pub version: Option<String>,
    #[serde(rename = "expectedVersion")]
    pub expected_version: Option<String>,
    #[serde(rename = "matchesDesktop")]
    pub matches_desktop: Option<bool>,
    pub error: Option<String>,
}

/// Mirrors `WslServerConfig`.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct WslServerConfig {
    pub id: String,
    pub distro: String,
}

/// Mirrors `WslServerRuntime`.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(tag = "kind", rename_all = "lowercase")]
pub enum WslServerRuntime {
    Starting,
    Ready {
        url: String,
        username: Option<String>,
        password: Option<String>,
    },
    Failed {
        message: String,
    },
    Stopped,
}

/// Mirrors `WslServerItem`.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct WslServerItem {
    pub config: WslServerConfig,
    pub runtime: WslServerRuntime,
}

/// Mirrors `WslJob`.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub enum WslJob {
    Runtime {
        #[serde(rename = "startedAt")]
        started_at: i64,
    },
    Distros {
        #[serde(rename = "startedAt")]
        started_at: i64,
    },
    #[serde(rename = "install-wsl")]
    InstallWsl {
        #[serde(rename = "startedAt")]
        started_at: i64,
    },
    #[serde(rename = "install-distro")]
    InstallDistro {
        distro: String,
        #[serde(rename = "startedAt")]
        started_at: i64,
    },
    #[serde(rename = "probe-addable")]
    ProbeAddable {
        distros: Vec<String>,
        #[serde(rename = "startedAt")]
        started_at: i64,
    },
    #[serde(rename = "install-opencode")]
    InstallOpencode {
        distro: String,
        #[serde(rename = "startedAt")]
        started_at: i64,
    },
}

/// Mirrors `WslServersState`.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct WslServersState {
    pub runtime: Option<WslRuntimeCheck>,
    pub installed: Vec<WslInstalledDistro>,
    pub online: Vec<WslOnlineDistro>,
    #[serde(rename = "distroProbes")]
    pub distro_probes: std::collections::HashMap<String, WslDistroProbe>,
    #[serde(rename = "opencodeChecks")]
    pub opencode_checks: std::collections::HashMap<String, WslOpencodeCheck>,
    #[serde(rename = "pendingRestart")]
    pub pending_restart: bool,
    pub servers: Vec<WslServerItem>,
    pub job: Option<WslJob>,
}

/// Mirrors `WslServersEvent`.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(tag = "type")]
pub enum WslServersEvent {
    #[serde(rename = "state")]
    State { state: WslServersState },
}

// PROVISIONAL: pending wsl platform backend — mirrors `packages/app/src/wsl/types.ts` (`WslServersPlatform`).
/// Mirrors the `WslServersPlatform` method names (verbatim).
pub const WSL_SERVERS_PLATFORM_METHODS: &[&str] = &[
    "getState",
    "subscribe",
    "probeRuntime",
    "refreshDistros",
    "installWsl",
    "installDistro",
    "probeAddable",
    "installOpencode",
    "openTerminal",
    "addServer",
    "removeServer",
    "startServer",
];

/// Mirrors `WslServersPlatform` as an explicit descriptor.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WslServersPlatform {
    pub available: bool,
}

impl WslServersPlatform {
    pub fn new(available: bool) -> Self {
        Self { available }
    }
}
