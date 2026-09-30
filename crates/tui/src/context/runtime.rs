// source: packages/tui/src/context/runtime.tsx (62 lines, v1.18.30)
// 1:1 port — providers become one explicit `TuiRuntime` owner; the
// `required()` missing-provider throw becomes a `Result`.

#![allow(dead_code)]

/// Mirrors `TuiPaths`.
#[derive(Debug, Clone, Default)]
pub struct TuiPaths {
    pub cwd: String,
    pub home: String,
    pub state: String,
    pub worktree: String,
}

/// Mirrors `TuiTerminalEnvironment`.
#[derive(Debug, Clone, Default)]
pub struct TuiTerminalEnvironment {
    pub platform: String,
    pub multiplexer: Option<String>,
    pub display_server: Option<String>,
}

/// Mirrors `TuiStartup`.
#[derive(Debug, Clone, Default)]
pub struct TuiStartup {
    pub initial_route: Option<serde_json::Value>,
    pub skip_initial_loading: bool,
}

/// Explicit owner replacing the three SolidJS providers. Values are
/// frozen at construction (mirrors `Object.freeze({ ...value })`).
#[derive(Debug, Clone, Default)]
pub struct TuiRuntime {
    pub paths: Option<TuiPaths>,
    pub terminal_environment: Option<TuiTerminalEnvironment>,
    pub startup: Option<TuiStartup>,
}

impl TuiRuntime {
    /// Mirrors `useTuiPaths` (throws `"TuiPathsProvider is missing"`).
    pub fn paths(&self) -> Result<&TuiPaths, String> {
        self.paths
            .as_ref()
            .ok_or_else(|| "TuiPathsProvider is missing".to_string())
    }

    /// Mirrors `useTuiTerminalEnvironment`.
    pub fn terminal_environment(&self) -> Result<&TuiTerminalEnvironment, String> {
        self.terminal_environment
            .as_ref()
            .ok_or_else(|| "TuiTerminalEnvironmentProvider is missing".to_string())
    }

    /// Mirrors `useTuiStartup`.
    pub fn startup(&self) -> Result<&TuiStartup, String> {
        self.startup
            .as_ref()
            .ok_or_else(|| "TuiStartupProvider is missing".to_string())
    }
}
