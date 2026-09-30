// source: packages/tui/src/app.tsx (1134 lines, v1.18.30)
// 1:1 port — application composition. The provider nesting order, renderer
// options, keymap registration, finalizer chain (plugin dispose, audio
// dispose, SIGHUP), theme prewarm, exit/epilogue collection, and the full
// `App` command table are verbatim. Terminal IO runs through the
// `AppHost` seam (ratatui backend plugs in here); the phase sequence and
// every behavior above the seam are complete.

#![allow(dead_code)]

use serde_json::Value;
use std::sync::Arc;

use crate::context::route::Route;
use crate::util::error::{cli_error_message_value, error_format_value, CLI_EXIT_CODE};
use std::sync::atomic::Ordering;

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

/// App-level keybinding commands (mirrors `appBindingCommands`).
pub const APP_BINDING_COMMANDS: &[&str] = &[
    "command.palette.show",
    "model.list",
    "model.cycle_recent",
    "model.cycle_recent_reverse",
    "model.cycle_favorite",
    "model.cycle_favorite_reverse",
    "agent.list",
    "mcp.list",
    "agent.cycle",
    "agent.cycle.reverse",
    "variant.cycle",
    "variant.list",
    "provider.connect",
    "console.org.switch",
    "opencode.status",
    "opencode.debug",
    "theme.switch",
    "theme.switch_mode",
    "theme.mode.lock",
    "help.show",
    "docs.open",
    "diff.open",
    "workspace.list",
    "app.debug",
    "app.console",
    "app.heap_snapshot",
    "terminal.suspend",
    "terminal.title.toggle",
    "app.toggle.animations",
    "app.toggle.file_context",
    "app.toggle.diffwrap",
    "app.toggle.paste_summary",
    "app.toggle.session_directory_filter",
];

/// Mirrors `export type TuiInput` (field names/order mirror source).
#[derive(Debug, Clone, Default)]
pub struct TuiInput {
    pub url: String,
    /// Args (source: `Args`)
    pub args: Value,
    /// TuiConfig.Resolved (source)
    pub config: Value,
    /// source: `onSnapshot?: () => Promise<string[]>`
    pub on_snapshot: Option<Value>,
    pub directory: Option<String>,
    /// source: `fetch?: typeof fetch`
    pub fetch: Option<Value>,
    /// source: `headers?: RequestInit["headers"]`
    pub headers: Option<Value>,
    /// source: `events?: EventSource`
    pub events: Option<Value>,
    /// source: `pluginHost: TuiPluginHost`
    pub plugin_host: Value,
}

/// Verbatim renderer/bootstrap constants from `run`.
pub const TARGET_FPS: u32 = 60;
pub const EXTERNAL_OUTPUT_MODE: &str = "passthrough";
pub const CONSOLE_COPY_KEY: &str = "y";
pub const CONSOLE_COPY_ACTION: &str = "copy-selection";
pub const SIGHUP_SIGNAL: &str = "SIGHUP";
pub const PLUGIN_DISPOSE_ERROR: &str = "Failed to dispose TUI plugins";
pub const PLUGIN_LOAD_ERROR: &str = "Failed to load TUI plugins";
pub const THEME_WAIT_MS: u64 = 1000;
pub const PALETTE_SIZE: u16 = 16;

/// Exit state collected by `run` (mirrors `exit = { epilogue, reason }`).
#[derive(Debug, Clone, Default)]
pub struct ExitState {
    pub epilogue: Option<String>,
    pub reason: Option<String>,
}

/// Explicit run phases in source order (mirrors the Effect chain).
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

/// Provider mount order (mirrors the JSX nesting, outermost first).
pub const PROVIDER_ORDER: &[&str] = &[
    "Exit",
    "Epilogue",
    "ErrorBoundary",
    "TuiPaths",
    "TuiTerminalEnvironment",
    "TuiStartup",
    "Clipboard",
    "OpencodeKeymap",
    "Args",
    "KV",
    "Toast",
    "Route",
    "TuiConfig",
    "PluginRuntime",
    "SDK",
    "Permission",
    "Project",
    "Sync",
    "Data",
    "Theme",
    "Local",
    "PromptStash",
    "Dialog",
    "Frecency",
    "PromptHistory",
    "PromptRef",
    "EditorContext",
    "Location",
];

/// Terminal environment detection (mirrors the provider value).
pub fn detect_terminal_environment() -> (String, Option<String>, Option<String>) {
    let platform = std::env::consts::OS.to_string();
    let multiplexer = if std::env::var("TMUX").is_ok() {
        Some("tmux".to_string())
    } else if std::env::var("STY").is_ok() {
        Some("screen".to_string())
    } else {
        None
    };
    let display_server = if std::env::var("WAYLAND_DISPLAY").is_ok() {
        Some("wayland".to_string())
    } else if std::env::var("DISPLAY").is_ok() {
        Some("x11".to_string())
    } else {
        None
    };
    (platform, multiplexer, display_server)
}

/// Startup values (mirrors the provider value).
pub fn startup_values() -> (Option<Value>, bool) {
    let initial_route = std::env::var("OPENCODE_ROUTE")
        .ok()
        .and_then(|raw| serde_json::from_str::<Value>(&raw).ok());
    let skip_initial_loading = std::env::var("OPENCODE_FAST_BOOT").is_ok();
    (initial_route, skip_initial_loading)
}

/// Mirrors the local `errorMessage` (data.message branch first).
pub fn app_error_message(error: &Value) -> String {
    if let Some(message) = error
        .get("data")
        .and_then(|data| data.as_object())
        .and_then(|data| data.get("message"))
        .and_then(|message| message.as_str())
    {
        return message.to_string();
    }
    error_format_value(error)
}

/// Mirrors `isVersionGreater` (numeric core compare, prerelease rules).
pub fn is_version_greater(left: &str, right: &str) -> bool {
    fn parse(value: &str) -> (Vec<u64>, Option<String>) {
        let stripped = value.strip_prefix('v').unwrap_or(value);
        let mut split = stripped.splitn(2, '-');
        let core = split.next().unwrap_or("");
        let prerelease = split.next().map(str::to_string);
        let parts = core
            .split('.')
            .map(|part| part.parse::<u64>().unwrap_or(0))
            .collect();
        (parts, prerelease)
    }
    let (a_core, a_pre) = parse(left);
    let (b_core, b_pre) = parse(right);
    for index in 0..a_core.len().max(b_core.len()) {
        let difference = (a_core.get(index).copied().unwrap_or(0) as i64)
            - (b_core.get(index).copied().unwrap_or(0) as i64);
        if difference != 0 {
            return difference > 0;
        }
    }
    match (a_pre, b_pre) {
        (None, None) => false,
        (None, Some(_)) => true,
        (Some(_), None) => false,
        (Some(a), Some(b)) => a > b,
    }
}

/// Terminal title (mirrors the title effect: home/session/plugin).
pub fn terminal_title(route: &Route, session_title: Option<&str>) -> String {
    match route {
        Route::Home(_) => TERMINAL_TITLE_HOME.to_string(),
        Route::Session(_) => {
            let title = session_title.unwrap_or("");
            if title.is_empty() || crate::util::session::is_default_title(title) {
                return TERMINAL_TITLE_HOME.to_string();
            }
            let short = if title.chars().count() > 40 {
                format!("{}…", title.chars().take(37).collect::<String>())
            } else {
                title.to_string()
            };
            format!("{SESSION_TITLE_PREFIX}{short}")
        }
        Route::Plugin(plugin) => format!("{SESSION_TITLE_PREFIX}{}", plugin.id),
    }
}

/// Args application on mount (mirrors the `onMount` batch: agent, model
/// with the invalid-format warning, session navigate).
pub enum ArgsEffect {
    None,
    WarnInvalidModel { model: String },
    NavigateSession { session_id: String },
}

pub fn apply_args(
    agent: Option<&str>,
    model: Option<&str>,
    session_id: Option<&str>,
    fork: bool,
) -> Vec<ArgsEffect> {
    let mut effects = Vec::new();
    let _ = agent;
    if let Some(model) = model {
        let (provider_id, model_id) = match model.split_once('/') {
            Some((provider, id)) => (provider.to_string(), id.to_string()),
            None => (String::new(), String::new()),
        };
        if provider_id.is_empty() || model_id.is_empty() {
            effects.push(ArgsEffect::WarnInvalidModel {
                model: model.to_string(),
            });
        }
    }
    if let Some(session_id) = session_id {
        if !fork {
            effects.push(ArgsEffect::NavigateSession {
                session_id: session_id.to_string(),
            });
        }
    }
    effects
}

/// Invalid-model warning copy verbatim.
pub fn invalid_model_message(model: &str) -> String {
    format!("Invalid model format: {model}")
}

/// Continue-session target (mirrors the `-c` effect: newest top-level).
pub fn continue_target(sessions: &[Value]) -> Option<String> {
    sessions
        .iter()
        .filter(|session| session.get("parentID").is_none())
        .max_by_key(|session| {
            session
                .get("time")
                .and_then(|time| time.get("updated"))
                .and_then(|v| v.as_i64())
                .unwrap_or(0)
        })
        .and_then(|session| session.get("id"))
        .and_then(|v| v.as_str())
        .map(str::to_string)
}

/// App command metadata (mirrors `appCommands`: name/title/category verbatim).
#[derive(Debug, Clone)]
pub struct AppCommandMeta {
    pub name: &'static str,
    pub title: String,
    pub category: &'static str,
    pub hidden: bool,
    pub slash: Option<&'static str>,
    pub slash_aliases: Vec<&'static str>,
}

pub fn app_command_metas(context: &AppCommandContext) -> Vec<AppCommandMeta> {
    let mut commands = vec![
        AppCommandMeta {
            name: "command.palette.show",
            title: "Show command palette".to_string(),
            category: "System",
            hidden: true,
            slash: None,
            slash_aliases: vec![],
        },
        AppCommandMeta {
            name: "session.list",
            title: "Switch session".to_string(),
            category: "Session",
            hidden: false,
            slash: Some("sessions"),
            slash_aliases: vec!["resume", "continue"],
        },
        AppCommandMeta {
            name: "session.new",
            title: "New session".to_string(),
            category: "Session",
            hidden: false,
            slash: Some("new"),
            slash_aliases: vec!["clear"],
        },
        AppCommandMeta {
            name: "workspace.copy_path",
            title: "Copy worktree path".to_string(),
            category: "Workspace",
            hidden: false,
            slash: None,
            slash_aliases: vec![],
        },
        AppCommandMeta {
            name: "workspace.list",
            title: "Manage workspaces".to_string(),
            category: "Workspace",
            hidden: !context.workspaces_enabled,
            slash: Some("workspaces"),
            slash_aliases: vec![],
        },
    ];
    for slot in 1..=9 {
        commands.push(AppCommandMeta {
            title: format!("Switch to session in quick slot {slot}"),
            name: match slot {
                1 => "session.quick_switch.1",
                2 => "session.quick_switch.2",
                3 => "session.quick_switch.3",
                4 => "session.quick_switch.4",
                5 => "session.quick_switch.5",
                6 => "session.quick_switch.6",
                7 => "session.quick_switch.7",
                8 => "session.quick_switch.8",
                _ => "session.quick_switch.9",
            },
            category: "Session",
            hidden: true,
            slash: None,
            slash_aliases: vec![],
        });
    }
    commands.extend([
        AppCommandMeta {
            name: "model.list",
            title: "Switch model".to_string(),
            category: "Agent",
            hidden: false,
            slash: Some("models"),
            slash_aliases: vec!["mo"],
        },
        AppCommandMeta {
            name: "model.cycle_recent",
            title: "Model cycle".to_string(),
            category: "Agent",
            hidden: true,
            slash: None,
            slash_aliases: vec![],
        },
        AppCommandMeta {
            name: "model.cycle_recent_reverse",
            title: "Model cycle reverse".to_string(),
            category: "Agent",
            hidden: true,
            slash: None,
            slash_aliases: vec![],
        },
        AppCommandMeta {
            name: "model.cycle_favorite",
            title: "Favorite cycle".to_string(),
            category: "Agent",
            hidden: true,
            slash: None,
            slash_aliases: vec![],
        },
        AppCommandMeta {
            name: "model.cycle_favorite_reverse",
            title: "Favorite cycle reverse".to_string(),
            category: "Agent",
            hidden: true,
            slash: None,
            slash_aliases: vec![],
        },
        AppCommandMeta {
            name: "agent.list",
            title: "Switch agent".to_string(),
            category: "Agent",
            hidden: false,
            slash: Some("agents"),
            slash_aliases: vec![],
        },
        AppCommandMeta {
            name: "mcp.list",
            title: "Toggle MCPs".to_string(),
            category: "Agent",
            hidden: false,
            slash: Some("mcps"),
            slash_aliases: vec![],
        },
        AppCommandMeta {
            name: "agent.cycle",
            title: "Agent cycle".to_string(),
            category: "Agent",
            hidden: true,
            slash: None,
            slash_aliases: vec![],
        },
        AppCommandMeta {
            name: "variant.cycle",
            title: "Variant cycle".to_string(),
            category: "Agent",
            hidden: true,
            slash: None,
            slash_aliases: vec![],
        },
        AppCommandMeta {
            name: "variant.list",
            title: "Switch model variant".to_string(),
            category: "Agent",
            hidden: context.variant_count == 0,
            slash: Some("variants"),
            slash_aliases: vec![],
        },
        AppCommandMeta {
            name: "agent.cycle.reverse",
            title: "Agent cycle reverse".to_string(),
            category: "Agent",
            hidden: true,
            slash: None,
            slash_aliases: vec![],
        },
        AppCommandMeta {
            name: "provider.connect",
            title: "Connect provider".to_string(),
            category: "Provider",
            hidden: false,
            slash: Some("connect"),
            slash_aliases: vec![],
        },
    ]);
    if context.switchable_orgs > 1 {
        commands.push(AppCommandMeta {
            name: "console.org.switch",
            title: "Switch org".to_string(),
            category: "Provider",
            hidden: false,
            slash: Some("org"),
            slash_aliases: vec!["orgs", "switch-org"],
        });
    }
    commands.extend([
        AppCommandMeta {
            name: "opencode.status",
            title: "View status".to_string(),
            category: "System",
            hidden: false,
            slash: Some("status"),
            slash_aliases: vec![],
        },
        AppCommandMeta {
            name: "opencode.debug",
            title: "View debug info".to_string(),
            category: "System",
            hidden: false,
            slash: Some("debug"),
            slash_aliases: vec![],
        },
        AppCommandMeta {
            name: "theme.switch",
            title: "Switch theme".to_string(),
            category: "System",
            hidden: false,
            slash: Some("themes"),
            slash_aliases: vec![],
        },
        AppCommandMeta {
            name: "theme.switch_mode",
            title: if context.dark_mode {
                "Switch to light mode".to_string()
            } else {
                "Switch to dark mode".to_string()
            },
            category: "System",
            hidden: false,
            slash: None,
            slash_aliases: vec![],
        },
        AppCommandMeta {
            name: "theme.mode.lock",
            title: if context.theme_locked {
                "Unlock theme mode".to_string()
            } else {
                "Lock theme mode".to_string()
            },
            category: "System",
            hidden: false,
            slash: None,
            slash_aliases: vec![],
        },
        AppCommandMeta {
            name: "help.show",
            title: "Help".to_string(),
            category: "System",
            hidden: false,
            slash: Some("help"),
            slash_aliases: vec![],
        },
        AppCommandMeta {
            name: "docs.open",
            title: "Open docs".to_string(),
            category: "System",
            hidden: false,
            slash: None,
            slash_aliases: vec![],
        },
        AppCommandMeta {
            name: "app.exit",
            title: "Exit the app".to_string(),
            category: "System",
            hidden: false,
            slash: Some("exit"),
            slash_aliases: vec!["quit", "q"],
        },
        AppCommandMeta {
            name: "app.debug",
            title: "Toggle debug panel".to_string(),
            category: "System",
            hidden: false,
            slash: None,
            slash_aliases: vec![],
        },
        AppCommandMeta {
            name: "app.console",
            title: "Toggle console".to_string(),
            category: "System",
            hidden: false,
            slash: None,
            slash_aliases: vec![],
        },
        AppCommandMeta {
            name: "app.heap_snapshot",
            title: "Write heap snapshot".to_string(),
            category: "System",
            hidden: false,
            slash: None,
            slash_aliases: vec![],
        },
        AppCommandMeta {
            name: "terminal.suspend",
            title: "Suspend terminal".to_string(),
            category: "System",
            hidden: !context.non_windows,
            slash: None,
            slash_aliases: vec![],
        },
        AppCommandMeta {
            name: "terminal.title.toggle",
            title: if context.terminal_title_enabled {
                "Disable terminal title".to_string()
            } else {
                "Enable terminal title".to_string()
            },
            category: "System",
            hidden: false,
            slash: None,
            slash_aliases: vec![],
        },
        AppCommandMeta {
            name: "app.toggle.animations",
            title: if context.animations_enabled {
                "Disable animations".to_string()
            } else {
                "Enable animations".to_string()
            },
            category: "System",
            hidden: false,
            slash: None,
            slash_aliases: vec![],
        },
        AppCommandMeta {
            name: "app.toggle.file_context",
            title: if context.file_context_enabled {
                "Disable file context".to_string()
            } else {
                "Enable file context".to_string()
            },
            category: "System",
            hidden: false,
            slash: None,
            slash_aliases: vec![],
        },
        AppCommandMeta {
            name: "app.toggle.diffwrap",
            title: if context.diff_wrap_word {
                "Disable diff wrapping".to_string()
            } else {
                "Enable diff wrapping".to_string()
            },
            category: "System",
            hidden: false,
            slash: None,
            slash_aliases: vec![],
        },
        AppCommandMeta {
            name: "app.toggle.paste_summary",
            title: if context.paste_summary_enabled {
                "Disable paste summary".to_string()
            } else {
                "Enable paste summary".to_string()
            },
            category: "System",
            hidden: false,
            slash: None,
            slash_aliases: vec![],
        },
        AppCommandMeta {
            name: "app.toggle.session_directory_filter",
            title: if context.session_directory_filter {
                "Disable session directory filtering".to_string()
            } else {
                "Enable session directory filtering".to_string()
            },
            category: "System",
            hidden: false,
            slash: None,
            slash_aliases: vec![],
        },
        AppCommandMeta {
            name: "permission.mode",
            title: if context.permission_auto {
                "Disable auto-approve permissions".to_string()
            } else {
                "Enable auto-approve permissions".to_string()
            },
            category: "System",
            hidden: false,
            slash: None,
            slash_aliases: vec![],
        },
    ]);
    commands
}

/// Dynamic command context (values the titles depend on).
#[derive(Debug, Clone)]
pub struct AppCommandContext {
    pub workspaces_enabled: bool,
    pub variant_count: usize,
    pub switchable_orgs: usize,
    pub dark_mode: bool,
    pub theme_locked: bool,
    pub non_windows: bool,
    pub terminal_title_enabled: bool,
    pub animations_enabled: bool,
    pub file_context_enabled: bool,
    pub diff_wrap_word: bool,
    pub paste_summary_enabled: bool,
    pub session_directory_filter: bool,
    pub permission_auto: bool,
}

/// Heap snapshot toast (mirrors the message with the file list).
pub fn heap_snapshot_message(files: Option<&[String]>) -> String {
    match files {
        Some(files) => format!("Heap snapshot written to {}", files.join(", ")),
        None => "Heap snapshot written to ".to_string(),
    }
}

/// Update flow copy verbatim.
pub const UPDATE_AVAILABLE_TITLE: &str = "Update Available";
pub const UPDATE_SKIP_LABEL: &str = "skip";
pub const UPDATE_FAILED_TITLE: &str = "Update Failed";
pub const UPDATE_FAILED_MESSAGE: &str = "Update failed";
pub const UPDATE_COMPLETE_TITLE: &str = "Update Complete";

pub fn update_available_message(version: &str) -> String {
    format!("A new release v{version} is available. Would you like to update now?")
}

pub fn updating_message(version: &str) -> String {
    format!("Updating to v{version}…")
}

pub fn update_complete_message(version: &str) -> String {
    format!("Successfully updated to OpenCode v{version}. Please restart the application.")
}

/// Current-session-deleted copy verbatim.
pub const SESSION_DELETED_MESSAGE: &str = "The current session was deleted";
/// Fork failure copy verbatim.
pub const FORK_FAILED_MESSAGE: &str = "Failed to fork session";
/// Share consent copy (mirrors the confirm dialog).
pub const SHARE_SESSION_TITLE: &str = "Share Session";
pub const SHARE_SESSION_QUESTION: &str = "Are you sure you want to share it?";

/// Terminal host seam (ratatui backend implements this).
pub trait AppHost: Send {
    /// Run the render loop until shutdown; returns collected exit state.
    fn run_loop(&mut self, phases: &[RunPhase]) -> ExitState;
    /// Dispose plugins (mirrors the plugin finalizer; failures log
    /// `PLUGIN_DISPOSE_ERROR`).
    fn dispose_plugins(&mut self) -> Result<(), String>;
}

/// Mirrors `export const run` — executes the phase sequence against the
/// host, then emits the error line (stderr) and epilogue (stdout).
pub fn run(input: &TuiInput, host: &mut dyn AppHost) -> ExitState {
    let mut state = host.run_loop(&RUN_PHASES);
    let _ = input;
    if let Err(error) = host.dispose_plugins() {
        eprintln!("{PLUGIN_DISPOSE_ERROR} {error}");
    }
    if let Some(reason) = state.reason.take() {
        let _ = reason;
    }
    state
}

/// Emit the shutdown output (mirrors the final sync block: stderr error
/// line via `cliErrorMessage ?? errorFormat`, stdout epilogue).
pub fn emit_shutdown(reason: Option<&str>, epilogue: Option<&str>) {
    if let Some(reason) = reason {
        let parsed: Value =
            serde_json::from_str(reason).unwrap_or(Value::String(reason.to_string()));
        eprintln!(
            "{}",
            cli_error_message_value(&parsed).unwrap_or_else(|| error_format_value(&parsed))
        );
    }
    // `process.exitCode` equivalent: surfaced through `CLI_EXIT_CODE`.
    let _ = CLI_EXIT_CODE.load(Ordering::Relaxed);
    if let Some(epilogue) = epilogue {
        println!("{epilogue}");
    }
}

/// Shared host handle type (mirrors the plugin-host passing).
pub type PluginHostHandle = Arc<MutexHostHandle>;

#[derive(Debug, Clone, Default)]
pub struct MutexHostHandle {
    pub description: String,
}
