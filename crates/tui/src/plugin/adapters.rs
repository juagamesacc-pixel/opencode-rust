// source: packages/tui/src/plugin/adapters.tsx (355 lines, v1.18.30)
// 1:1 port — full TUI API surface for plugins. SolidJS hooks/JSX → explicit
// state: hook returns arrive as concrete seam types (DialogStack snapshot
// via ShimDialog, KvStore, RouteStore, SdkContext, SyncStore, ThemeContext,
// ToastState, BindingLookup, OpencodeModeStack, MemoryEventSource); JSX
// components become explicit mount/request values applied by the app
// (Dialog mount via DialogStack, alert/confirm/prompt via show_*,
// select via SelectState, Prompt/Slot as data). Every method name, string,
// message, default, and ordering verbatim; `slots.register` and
// `theme.install` throw with the source messages (as Err); `plugins.*`
// stubs return []/false/{ok:false,...} verbatim.

#![allow(dead_code)]

use std::collections::HashMap;
use std::sync::Arc;

use serde_json::Value;
use tokio::sync::oneshot;

use crate::config::keybind::{BindingLookup, KeyStroke};
use crate::context::event::MemoryEventSource;
use crate::context::kv::KvStore;
use crate::context::route::{HomeRoute, PluginRoute, Route, RouteStore, SessionRoute};
use crate::context::sdk::{SdkClient, SdkContext};
use crate::context::sync::SyncStore;
use crate::context::theme::{ThemeContext, ThemeMode};
use crate::keymap::{format_key_bindings, format_key_sequence, OpencodeModeStack};
use crate::prompt::history::PromptInfo;
use crate::ui::dialog::{DialogContent, DialogSize, DialogStack};
use crate::ui::dialog_alert::show_alert;
use crate::ui::dialog_confirm::{show_confirm, ConfirmResult};
use crate::ui::dialog_prompt::{show_prompt, PromptProps, PromptResult};
use crate::ui::dialog_select::SelectOption;
use crate::ui::toast::{ToastInput, ToastState};
use crate::util::selection::ToastVariant;

use super::api::{PluginRoutes, RouteDefinition, Unregister};
use super::command_shim::{create_command_shim, CommandShim, KeymapShim, ShimDialog};
use super::slots::{SlotManager, SlotView};

/// Re-exported like the source (`export type { RouteMap }`, `export {...}`).
pub use super::api::{RouteEntry, RouteMap};

/// Mirrors the `Input` — all 15+ hook/config values in source order.
pub struct AdaptersInput {
    pub version: String,
    pub keybinds: BindingLookup,
    pub dialog: ShimDialog,
    pub keymap_shim: Box<dyn KeymapShim>,
    pub keymap_modes: OpencodeModeStack,
    pub kv: KvStore,
    pub route: RouteStore,
    pub routes: PluginRoutes,
    pub event: MemoryEventSource,
    pub sdk: SdkContext,
    pub sync: SyncStore,
    pub sync_path: String,
    pub theme: ThemeContext,
    pub toast: ToastState,
    pub renderer: Value,
    pub attention: Value,
    pub slots: SlotManager,
}

// ---------------------------------------------------------------------------
// route helpers (verbatim branches)
// ---------------------------------------------------------------------------

/// Mirrors `routeNavigate` — home/session/plugin branches verbatim.
pub fn route_navigate(route: &mut RouteStore, name: &str, params: Option<&HashMap<String, Value>>) {
    if name == "home" {
        route.navigate(Route::Home(HomeRoute { prompt: None }));
        return;
    }
    if name == "session" {
        let session_id = params
            .and_then(|p| p.get("sessionID"))
            .and_then(|v| v.as_str());
        let session_id = match session_id {
            Some(id) => id.to_string(),
            None => return,
        };
        route.navigate(Route::Session(SessionRoute {
            session_id,
            prompt: None,
        }));
        return;
    }
    route.navigate(Route::Plugin(PluginRoute {
        id: name.to_string(),
        data: params
            .map(|p| Value::Object(p.iter().map(|(k, v)| (k.clone(), v.clone())).collect())),
    }));
}

/// Mirrors `route.current` shapes.
#[derive(Debug, Clone)]
pub enum RouteCurrent {
    Home,
    Session {
        session_id: String,
        prompt: Option<PromptInfo>,
    },
    Plugin {
        name: String,
        params: Option<Value>,
    },
}

/// Mirrors `routeCurrent(route)`.
pub fn route_current(route: &RouteStore) -> RouteCurrent {
    match route.data() {
        Route::Home(_) => RouteCurrent::Home,
        Route::Session(s) => RouteCurrent::Session {
            session_id: s.session_id.clone(),
            prompt: s.prompt.clone(),
        },
        Route::Plugin(p) => RouteCurrent::Plugin {
            name: p.id.clone(),
            params: p.data.clone(),
        },
    }
}

// ---------------------------------------------------------------------------
// select option mapping (verbatim)
// ---------------------------------------------------------------------------

/// Mirrors `TuiDialogSelectOption` (options passed by plugins).
pub struct TuiDialogSelectOption {
    pub title: String,
    pub value: Value,
    pub description: Option<String>,
    pub footer: Option<String>,
    pub category: Option<String>,
    pub disabled: bool,
    pub on_select: Option<Box<dyn FnMut() + Send>>,
}

/// Mirrors `mapOption` — spreads fields, wraps onSelect as no-arg.
pub fn map_option(mut item: TuiDialogSelectOption) -> SelectOption {
    let has_on_select = item.on_select.is_some();
    // Drain parity with `() => item.onSelect?.()` (args dropped, optional).
    let _ = item.on_select.take();
    SelectOption {
        title: item.title,
        value: item.value,
        description: item.description,
        footer: item.footer,
        category: item.category,
        disabled: item.disabled,
        has_on_select,
        ..Default::default()
    }
}

/// Mirrors `pickOption` — back to the plugin shape (no onSelect).
pub fn pick_option(item: &SelectOption) -> TuiDialogSelectOption {
    TuiDialogSelectOption {
        title: item.title.clone(),
        value: item.value.clone(),
        description: item.description.clone(),
        footer: item.footer.clone(),
        category: item.category.clone(),
        disabled: item.disabled,
        on_select: None,
    }
}

/// Mirrors `mapOptionCb` — wraps an optional callback with pickOption.
pub type SelectChangeCallback = Box<dyn FnMut(&SelectOption) + Send>;

pub fn map_option_cb(
    cb: Option<Box<dyn FnMut(TuiDialogSelectOption) + Send>>,
) -> Option<SelectChangeCallback> {
    cb.map(|mut inner| {
        let boxed: SelectChangeCallback =
            Box::new(move |item: &SelectOption| inner(pick_option(item)));
        boxed
    })
}

// ---------------------------------------------------------------------------
// state snapshot (mirrors `stateApi(sync)`)
// ---------------------------------------------------------------------------

/// Mirrors `{ branch, default_branch }` vcs view.
#[derive(Debug, Clone)]
pub struct VcsView {
    pub branch: Value,
    pub default_branch: Value,
}

/// Mirrors the object built by `stateApi` — explicit snapshot.
#[derive(Debug, Clone)]
pub struct StateSnapshot {
    pub ready: bool,
    pub config: Value,
    pub provider: Vec<Value>,
    pub path: String,
    pub vcs: Option<VcsView>,
    pub session: Vec<Value>,
    pub session_status: HashMap<String, Value>,
    pub session_diff: HashMap<String, Vec<Value>>,
    pub todo: HashMap<String, Vec<Value>>,
    pub message: HashMap<String, Vec<Value>>,
    pub part: HashMap<String, Vec<Value>>,
    pub permission: HashMap<String, Vec<Value>>,
    pub question: HashMap<String, Vec<Value>>,
    pub lsp: Vec<Value>,
    pub mcp: Vec<Value>,
}

/// Mirrors `stateApi(sync)` field-by-field.
pub fn state_snapshot(sync: &SyncStore, path: &str) -> StateSnapshot {
    // Mirrors `get vcs()` — undefined when no vcs data.
    let vcs = sync.vcs.as_ref().map(|v| VcsView {
        branch: v.get("branch").cloned().unwrap_or(Value::Null),
        default_branch: v.get("default_branch").cloned().unwrap_or(Value::Null),
    });
    // Mirrors `lsp()` — [{id, root, status}].
    let lsp: Vec<Value> = sync
        .lsp
        .iter()
        .map(|item| {
            serde_json::json!({
                "id": item.get("id").cloned().unwrap_or(Value::Null),
                "root": item.get("root").cloned().unwrap_or(Value::Null),
                "status": item.get("status").cloned().unwrap_or(Value::Null),
            })
        })
        .collect();
    // Mirrors `mcp()` — sorted by name, error only when failed.
    let mut mcp_entries: Vec<(&String, &Value)> = sync.mcp.iter().collect();
    mcp_entries.sort_by_key(|(a, _)| *a);
    let mcp: Vec<Value> = mcp_entries
        .into_iter()
        .map(|(name, item)| {
            let status = item.get("status").cloned().unwrap_or(Value::Null);
            let failed = status.as_str() == Some("failed");
            serde_json::json!({
                "name": name,
                "status": status,
                "error": if failed { item.get("error").cloned().unwrap_or(Value::Null) } else { Value::Null },
            })
        })
        .collect();
    StateSnapshot {
        ready: sync.ready(),
        config: sync.config.clone(),
        provider: sync.provider.clone(),
        path: path.to_string(),
        vcs,
        session: sync.session.clone(),
        session_status: sync.session_status.clone(),
        session_diff: sync.session_diff.clone(),
        todo: sync.todo.clone(),
        message: sync.message.clone(),
        part: sync.part.clone(),
        permission: sync.permission.clone(),
        question: sync.question.clone(),
        lsp,
        mcp,
    }
}

impl StateSnapshot {
    /// Mirrors `session.count()`.
    pub fn session_count(&self) -> usize {
        self.session.len()
    }

    /// Mirrors `session.get(sessionID)`.
    pub fn session_get(&self, session_id: &str) -> Option<&Value> {
        self.session
            .iter()
            .find(|s| s.get("id").and_then(|v| v.as_str()) == Some(session_id))
    }

    /// Mirrors `session.diff(sessionID)` — drops items with undefined file.
    pub fn session_diff(&self, session_id: &str) -> Vec<Value> {
        self.session_diff
            .get(session_id)
            .map(|items| {
                items
                    .iter()
                    .filter(|item| item.get("file").is_some())
                    .cloned()
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Mirrors `session.todo/messages/status/permission/question(sessionID)`.
    pub fn session_todo(&self, session_id: &str) -> Vec<Value> {
        self.todo.get(session_id).cloned().unwrap_or_default()
    }

    pub fn session_messages(&self, session_id: &str) -> Vec<Value> {
        self.message.get(session_id).cloned().unwrap_or_default()
    }

    pub fn session_status(&self, session_id: &str) -> Option<&Value> {
        self.session_status.get(session_id)
    }

    pub fn session_permission(&self, session_id: &str) -> Vec<Value> {
        self.permission.get(session_id).cloned().unwrap_or_default()
    }

    pub fn session_question(&self, session_id: &str) -> Vec<Value> {
        self.question.get(session_id).cloned().unwrap_or_default()
    }

    /// Mirrors `part(messageID)`.
    pub fn part(&self, message_id: &str) -> Vec<Value> {
        self.part.get(message_id).cloned().unwrap_or_default()
    }
}

// ---------------------------------------------------------------------------
// ui request values (JSX → explicit data + mount helpers)
// ---------------------------------------------------------------------------

/// Mirrors `<DialogUI size onClose>` — mounted by the app on the stack.
pub struct DialogMount {
    pub size: DialogSize,
    pub on_close: Option<Box<dyn FnMut() + Send>>,
    pub content: Box<dyn DialogContent>,
}

pub fn mount_dialog(stack: &mut DialogStack, mount: DialogMount) {
    stack.replace(mount.content, mount.on_close);
}

/// Mirrors `<DialogAlert {...}/>` — mounts and resolves like `show`.
pub fn ui_dialog_alert(
    stack: &mut DialogStack,
    title: &str,
    message: &str,
) -> oneshot::Receiver<()> {
    show_alert(stack, title, message)
}

/// Mirrors `<DialogConfirm {...}/>` .
pub fn ui_dialog_confirm(
    stack: &mut DialogStack,
    title: &str,
    message: &str,
    label: Option<String>,
) -> oneshot::Receiver<ConfirmResult> {
    show_confirm(stack, title, message, label)
}

/// Mirrors `<DialogPrompt {...}/>` (description passed through).
pub fn ui_dialog_prompt(
    stack: &mut DialogStack,
    title: &str,
    props: PromptProps,
) -> oneshot::Receiver<PromptResult> {
    show_prompt(stack, title, props)
}

/// Mirrors `<DialogSelect {...}/>` option plumbing — builds the select state;
/// the app renders it (SelectState is the shared type).
pub fn ui_dialog_select_state(
    title: &str,
    options: Vec<TuiDialogSelectOption>,
) -> crate::ui::dialog_select::SelectState {
    let mapped: Vec<SelectOption> = options.into_iter().map(map_option).collect();
    crate::ui::dialog_select::SelectState::new(title, mapped)
}

/// Mirrors `<Prompt .../>` props — rendered by the app (lane 3 owns the
/// component); submit handler carried explicitly.
pub struct PromptRequest {
    pub session_id: String,
    pub visible: bool,
    pub disabled: bool,
    pub hint: Option<String>,
    pub right: Option<String>,
    pub show_placeholder: Option<bool>,
    pub placeholders: Vec<String>,
    pub on_submit: Option<Box<dyn FnMut(String) + Send>>,
}

/// Mirrors `plugins.install` result.
#[derive(Debug, Clone)]
pub struct PluginInstallResult {
    pub ok: bool,
    pub message: Option<String>,
}

// ---------------------------------------------------------------------------
// adapter views (each TuiPluginApi section)
// ---------------------------------------------------------------------------

/// Mirrors `api.route` — register/navigate/current.
pub struct RouteApi<'a> {
    routes: &'a mut PluginRoutes,
    route: &'a mut RouteStore,
}

impl<'a> RouteApi<'a> {
    pub fn register(&mut self, list: Vec<RouteDefinition>) -> Unregister {
        self.routes.register(list)
    }

    pub fn navigate(&mut self, name: &str, params: Option<&HashMap<String, Value>>) {
        route_navigate(self.route, name, params);
    }

    pub fn current(&self) -> RouteCurrent {
        route_current(self.route)
    }
}

/// Mirrors `api.kv`.
pub struct KvApi<'a> {
    kv: &'a mut KvStore,
}

impl<'a> KvApi<'a> {
    pub fn get(&self, key: &str, fallback: Option<Value>) -> Value {
        self.kv.get(key, fallback)
    }

    pub fn set(&mut self, key: &str, value: Value) {
        self.kv.set(key, value);
    }

    pub fn ready(&self) -> bool {
        self.kv.ready()
    }
}

/// Mirrors `api.theme` — current/selected/has/set/install/mode/ready.
pub struct ThemeApi<'a> {
    theme: &'a mut ThemeContext,
}

impl<'a> ThemeApi<'a> {
    pub fn current(&self) -> String {
        self.theme.selected().to_string()
    }

    pub fn selected(&self) -> String {
        self.theme.selected().to_string()
    }

    pub fn has(&self, name: &str) -> bool {
        self.theme.theme_names().iter().any(|n| n == name)
    }

    pub fn set(&mut self, name: &str) -> bool {
        self.theme.set(name)
    }

    pub fn install(&self, _json_path: &str) -> Result<(), String> {
        Err("theme.install is only available in plugin context".to_string())
    }

    pub fn mode(&self) -> ThemeMode {
        self.theme.mode()
    }

    pub fn ready(&self) -> bool {
        self.theme.ready()
    }
}

/// Mirrors `api.slots` — register always throws with the source message.
pub struct SlotsApi;

impl SlotsApi {
    pub fn register(&self, _plugin: Value) -> Result<(), String> {
        Err("slots.register is only available in plugin context".to_string())
    }
}

/// Mirrors `api.plugins` — list/activate/deactivate/add/install verbatim.
pub struct PluginsApi;

impl PluginsApi {
    pub fn list(&self) -> Vec<Value> {
        vec![]
    }

    pub fn activate(&self, _id: &str) -> bool {
        false
    }

    pub fn deactivate(&self, _id: &str) -> bool {
        false
    }

    pub fn add(&self, _spec: &str) -> bool {
        false
    }

    pub fn install(&self, _spec: &str, _options: Option<Value>) -> PluginInstallResult {
        PluginInstallResult {
            ok: false,
            message: Some("plugins.install is only available in plugin context".to_string()),
        }
    }
}

// ---------------------------------------------------------------------------
// top-level adapters object (mirrors createTuiApiAdapters return)
// ---------------------------------------------------------------------------

/// Field bundles that borrow the input (mirrors the returned object shape).
pub struct TuiApiAdapters<'a> {
    pub version: &'a str,
    pub keymap_modes: &'a mut OpencodeModeStack,
    pub keybinds: &'a BindingLookup,
    pub kv: KvApi<'a>,
    pub route: RouteApi<'a>,
    pub dialog: &'a ShimDialog,
    pub theme: ThemeApi<'a>,
    pub toast: &'a mut ToastState,
    pub slots_api: SlotsApi,
    pub plugins_api: PluginsApi,
    pub state: StateSnapshot,
    pub event: &'a MemoryEventSource,
    pub sdk_client: Arc<dyn SdkClient>,
    pub renderer: &'a Value,
    pub attention: &'a Value,
    pub slot_view: SlotView,
}

impl<'a> TuiApiAdapters<'a> {
    /// Mirrors `app.version`.
    pub fn app_version(&self) -> &str {
        self.version
    }

    /// Mirrors `keys.formatSequence`.
    pub fn format_sequence(&self, parts: &[KeyStroke]) -> String {
        format_key_sequence(parts, self.keybinds)
    }

    /// Mirrors `keys.formatBindings`.
    pub fn format_bindings(&self, bindings: &[crate::config::keybind::Binding]) -> String {
        format_key_bindings(bindings, self.keybinds)
    }

    /// Mirrors `mode.current/push`.
    pub fn mode_current(&self) -> String {
        self.keymap_modes.current()
    }

    pub fn mode_push(&mut self, mode: &str) -> Option<u64> {
        self.keymap_modes.push(mode)
    }

    /// Mirrors `get tuiConfig`.
    pub fn tui_config_keybinds(&self) -> &BindingLookup {
        self.keybinds
    }

    /// Mirrors `ui.toast(...)` — `variant ?? "info"`, duration passthrough.
    pub fn toast(
        &mut self,
        title: Option<String>,
        message: String,
        variant: Option<ToastVariant>,
        duration: Option<u64>,
    ) {
        self.toast.show(ToastInput {
            title,
            message,
            variant: variant.or(Some(ToastVariant::Info)),
            duration_ms: duration,
        });
    }

    /// Mirrors `get client` — the SDK client handle.
    pub fn client(&self) -> Arc<dyn SdkClient> {
        Arc::clone(&self.sdk_client)
    }

    /// Mirrors `ui.Slot` — current slot view for the app to render.
    pub fn slot(&self) -> &SlotView {
        &self.slot_view
    }
}

/// Mirrors `createTuiApiAdapters(input)` — builds the command shim plus the
/// borrowed adapter views. (The outer `lifecycle` is added by
/// `createTuiApi`; see `api.rs`.)
pub struct AdaptersSession<'a> {
    pub command: CommandShim<Box<dyn KeymapShim>>,
    pub api: TuiApiAdapters<'a>,
}

pub fn create_tui_api_adapters<'a>(input: &'a mut AdaptersInput) -> AdaptersSession<'a> {
    let slot_view = input.slots.view();
    let state = state_snapshot(&input.sync, &input.sync_path);
    let sdk_client = Arc::clone(&input.sdk.client);
    let dialog_clone = input.dialog.clone();
    let keymap_shim = std::mem::replace(&mut input.keymap_shim, empty_keymap_shim());
    let command = create_command_shim(keymap_shim, dialog_clone);
    AdaptersSession {
        command,
        api: TuiApiAdapters {
            version: &input.version,
            keymap_modes: &mut input.keymap_modes,
            keybinds: &input.keybinds,
            kv: KvApi { kv: &mut input.kv },
            route: RouteApi {
                routes: &mut input.routes,
                route: &mut input.route,
            },
            dialog: &input.dialog,
            theme: ThemeApi {
                theme: &mut input.theme,
            },
            toast: &mut input.toast,
            slots_api: SlotsApi,
            plugins_api: PluginsApi,
            state,
            event: &input.event,
            sdk_client,
            renderer: &input.renderer,
            attention: &input.attention,
            slot_view,
        },
    }
}

fn empty_keymap_shim() -> Box<dyn KeymapShim> {
    struct Empty;
    impl KeymapShim for Empty {
        fn register_layer(
            &mut self,
            _layer: super::command_shim::KeymapLayer,
        ) -> Box<dyn FnOnce() + Send> {
            Box::new(|| {})
        }
        fn dispatch_command(&mut self, _name: &str) {}
    }
    Box::new(Empty)
}
