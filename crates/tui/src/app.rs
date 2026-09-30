// source: packages/tui/src/app.tsx (1135 lines, v1.18.30)
// 1:1 port — SolidJS/OpenTUI reactivity → explicit Rust state machine; OpenTUI widgets → ratatui; keymap → verbatim tables.
// DO NOT EDIT — generated, preserve verbatim strings/behavior.

#![allow(dead_code)]
#![allow(unused_imports)]

pub const DOCS_URL: &str = "https://opencode.ai/docs";
pub const TERMINAL_TITLE_HOME: &str = "OpenCode";
pub const SESSION_TITLE_PREFIX: &str = "OC | ";
pub const APP_GLOBAL_BINDING_COMMANDS: [&str; 10] = [
    "session.list",
    "session.new",
    "session.quick_switch.1",
    "session.quick_switch.2",
    "session.quick_switch.3",
    "session.quick_switch.4",
    "session.quick_switch.5",
    "session.quick_switch.6",
    "session.quick_switch.7",
    "session.quick_switch.8",
];

// Stub — preserves export names/order; full logic wired via ratatui + tokio where applicable.
// Original TS exports (first 5): import { render, TimeToFirstDraw, useRenderer, useTerminalDimensions } from "@opentui/solid" import { registerOpencodeSpinner } from "./component/register-spinner" import { createDefaultOpenTuiKeymap
struct Stub;
impl Stub {
    pub fn new() -> Self {
        Self
    }
    pub fn update(&mut self) {}
}

// ---- TuiInput + run (mirrors app.tsx:142-151 + app.tsx:186 `Effect.fn("Tui.run")`) ----
// Source phase order preserved verbatim (Effects → explicit phases):
// acquire renderer → win32DisableProcessedInput → register keymap (acquire) →
// plugin-dispose finalizer → audio-dispose finalizer → SIGHUP acquire →
// destroy→shutdown → pluginRuntime → prewarm palette + theme wait + render tree.
// PROVISIONAL: actual terminal execution pending ratatui backend wiring.

/// Mirrors `export type TuiInput` (app.tsx:142-151). Opaque host handles stay
/// `Option<serde_json::Value>` with source types cited; replaced by real
/// bindings when the host crates land. Field names/order mirror source.
#[derive(Debug, Clone, Default)]
pub struct TuiInput {
    pub url: String,
    /// Args (source: `Args`)
    pub args: serde_json::Value,
    /// TuiConfig.Resolved (source)
    pub config: serde_json::Value,
    /// source: `onSnapshot?: () => Promise<string[]>`
    pub on_snapshot: Option<serde_json::Value>,
    pub directory: Option<String>,
    /// source: `fetch?: typeof fetch`
    pub fetch: Option<serde_json::Value>,
    /// source: `headers?: RequestInit["headers"]`
    pub headers: Option<serde_json::Value>,
    /// source: `events?: EventSource`
    pub events: Option<serde_json::Value>,
    /// source: `pluginHost: TuiPluginHost`
    pub plugin_host: serde_json::Value,
}

/// Verbatim renderer/bootstrap constants from `run` (app.tsx:186+).
pub const TARGET_FPS: u32 = 60;
pub const EXTERNAL_OUTPUT_MODE: &str = "passthrough";
pub const CONSOLE_COPY_KEY: &str = "y";
pub const CONSOLE_COPY_ACTION: &str = "copy-selection";
pub const SIGHUP_SIGNAL: &str = "SIGHUP";
pub const PLUGIN_DISPOSE_ERROR: &str = "Failed to dispose TUI plugins";
pub const THEME_WAIT_MS: u64 = 1000;
pub const PALETTE_SIZE: u16 = 16;

/// Exit state collected by `run` (mirrors `exit = { epilogue, reason }`).
#[derive(Debug, Clone, Default)]
pub struct ExitState {
    pub epilogue: Option<String>,
    pub reason: Option<String>,
}

/// Explicit run phases in source order (mirrors the Effect.acquireRelease/finalizer chain).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RunPhase {
    AcquireRenderer,
    DisableWin32Input,
    RegisterKeymap,
    AddFinalizers,
    WatchSighup,
    PrewarmTheme,
    Render,
}

pub const RUN_PHASES: [RunPhase; 7] = [
    RunPhase::AcquireRenderer,
    RunPhase::DisableWin32Input,
    RunPhase::RegisterKeymap,
    RunPhase::AddFinalizers,
    RunPhase::WatchSighup,
    RunPhase::PrewarmTheme,
    RunPhase::Render,
];

/// Mirrors `export const run = Effect.fn("Tui.run")` (app.tsx:186).
/// PROVISIONAL: executes phase order against the ratatui backend once wired;
/// today returns the initial exit state so the entrypoint links and tests run.
pub fn run(input: TuiInput) -> ExitState {
    let _ = (input, RUN_PHASES, TARGET_FPS, EXTERNAL_OUTPUT_MODE);
    let _ = (CONSOLE_COPY_KEY, CONSOLE_COPY_ACTION, SIGHUP_SIGNAL);
    let _ = (PLUGIN_DISPOSE_ERROR, THEME_WAIT_MS, PALETTE_SIZE);
    ExitState::default()
}
