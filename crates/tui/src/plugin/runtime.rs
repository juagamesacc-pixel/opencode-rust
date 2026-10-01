// source: packages/tui/src/plugin/runtime.tsx (81 lines, v1.18.30)
// 1:1 port — SolidJS context/signals → explicit state: commands/status
// signals → struct fields + update(); Context/Provider → thread-local scope
// set during provider execution; usePluginRuntime throws outside provider
// (mirrored as Err with the same message). Empty-command defaults and the
// install message verbatim.

#![allow(dead_code)]

use std::cell::RefCell;

use serde_json::Value;

use super::api::{create_plugin_routes, PluginRoutes};
use super::slots::{create_slots, HostPluginApi, HostSlots, SlotManager, SlotView};

/// Mirrors `TuiPluginInstallOptions`.
#[derive(Debug, Clone, Default)]
pub struct TuiPluginInstallOptions {
    pub force: Option<bool>,
}

/// Mirrors `TuiPluginInstallResult`.
#[derive(Debug, Clone)]
pub struct TuiPluginInstallResult {
    pub ok: bool,
    pub message: Option<String>,
}

/// Mirrors `TuiPluginStatus`.
#[derive(Debug, Clone, Default)]
pub struct TuiPluginStatus {
    pub id: String,
    pub status: String,
}

/// Install handler shape (mirrors the `install` command signature).
pub type InstallFn =
    Box<dyn FnMut(&str, Option<TuiPluginInstallOptions>) -> TuiPluginInstallResult + Send>;

/// Mirrors `PluginRuntimeCommands` — activate/deactivate/add/install.
pub struct PluginRuntimeCommands {
    activate_fn: Box<dyn FnMut(&str) -> bool + Send>,
    deactivate_fn: Box<dyn FnMut(&str) -> bool + Send>,
    add_fn: Box<dyn FnMut(&str) -> bool + Send>,
    install_fn: InstallFn,
}

impl PluginRuntimeCommands {
    /// Mirrors `emptyCommands` — false/false/false/`{ok:false, message}`.
    pub fn empty() -> Self {
        Self {
            activate_fn: Box::new(|_| false),
            deactivate_fn: Box::new(|_| false),
            add_fn: Box::new(|_| false),
            install_fn: Box::new(|_, _| TuiPluginInstallResult {
                ok: false,
                message: Some("Plugin runtime is not available.".to_string()),
            }),
        }
    }

    pub fn activate(&mut self, id: &str) -> bool {
        (self.activate_fn)(id)
    }

    pub fn deactivate(&mut self, id: &str) -> bool {
        (self.deactivate_fn)(id)
    }

    pub fn add(&mut self, spec: &str) -> bool {
        (self.add_fn)(spec)
    }

    pub fn install(
        &mut self,
        spec: &str,
        options: Option<TuiPluginInstallOptions>,
    ) -> TuiPluginInstallResult {
        (self.install_fn)(spec, options)
    }
}

impl Default for PluginRuntimeCommands {
    fn default() -> Self {
        Self::empty()
    }
}

/// Partial update input (mirrors `update(input)`).
#[derive(Default)]
pub struct RuntimeUpdate {
    pub commands: Option<PluginRuntimeCommands>,
    pub status: Option<Vec<TuiPluginStatus>>,
}

/// Mirrors the return of `createPluginRuntime()` — Slot/routes/commands/
/// status/update/clear/setupSlots with explicit state.
pub struct PluginRuntime {
    slots: SlotManager,
    pub routes: PluginRoutes,
    pub commands: PluginRuntimeCommands,
    pub status: Vec<TuiPluginStatus>,
}

impl PluginRuntime {
    pub fn new() -> Self {
        Self {
            slots: create_slots(),
            routes: create_plugin_routes(),
            commands: PluginRuntimeCommands::empty(),
            status: Vec::new(),
        }
    }

    /// Mirrors the `Slot` export — current slot view.
    pub fn slot_view(&self) -> SlotView {
        self.slots.view()
    }

    /// Mirrors `update(input)` — applies present fields only.
    pub fn update(&mut self, input: RuntimeUpdate) {
        if let Some(commands) = input.commands {
            self.commands = commands;
        }
        if let Some(status) = input.status {
            self.status = status;
        }
    }

    /// Mirrors `clear()` — resets commands/status/slots.
    pub fn clear(&mut self) {
        self.commands = PluginRuntimeCommands::empty();
        self.status = Vec::new();
        self.slots.clear();
    }

    /// Mirrors `setupSlots(api)`.
    pub fn setup_slots(&self, api: &HostPluginApi) -> HostSlots {
        self.slots.setup(api)
    }
}

impl Default for PluginRuntime {
    fn default() -> Self {
        Self::new()
    }
}

/// Mirrors `createPluginRuntime()`.
pub fn create_plugin_runtime() -> PluginRuntime {
    PluginRuntime::new()
}

/// Mirrors `PluginRuntime = ReturnType<typeof createPluginRuntime>`.
pub type PluginRuntimeHandle = PluginRuntime;

/// Mirrors the `start` input — `{ api, config, runtime, dispose? }`.
pub struct PluginHostStartInput {
    pub api: Value,
    pub config: Value,
    pub dispose: Option<Box<dyn FnOnce() + Send>>,
}

/// Mirrors `TuiPluginHost` — `{ start, dispose }`.
pub trait TuiPluginHost: Send {
    fn start(&mut self, input: PluginHostStartInput);
    fn dispose(&mut self);
}

thread_local! {
    static PLUGIN_RUNTIME_SCOPE: RefCell<Option<*const PluginRuntime>> = const { RefCell::new(None) };
}

/// Mirrors `PluginRuntimeProvider` — runs `f` with `runtime` in scope.
/// (SolidJS `<Context.Provider>` → explicit scope around the call.)
pub fn with_plugin_runtime<R>(runtime: &PluginRuntime, f: impl FnOnce() -> R) -> R {
    PLUGIN_RUNTIME_SCOPE.with(|scope| {
        let prev = scope.replace(Some(runtime as *const PluginRuntime));
        let out = f();
        scope.replace(prev);
        out
    })
}

/// Mirrors `usePluginRuntime()` — returns the scoped runtime or the same
/// error message the source throws outside the provider.
pub fn use_plugin_runtime<R>(f: impl FnOnce(&PluginRuntime) -> R) -> Result<R, String> {
    PLUGIN_RUNTIME_SCOPE.with(|scope| {
        let ptr = *scope.borrow();
        match ptr {
            // SAFETY: pointer is valid for the duration of with_plugin_runtime.
            Some(p) => Ok(f(unsafe { &*p })),
            None => Err("usePluginRuntime must be used within PluginRuntimeProvider".to_string()),
        }
    })
}
