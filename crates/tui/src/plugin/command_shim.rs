// source: packages/tui/src/plugin/command-shim.ts (109 lines, v1.18.30)
// 1:1 port — legacy `api.command` bridge for v1 plugins (remove in v2).
// Effect/SolidJS → explicit state: module `warned` set behind OnceLock;
// `console.warn` → eprintln! same tag; dialog mount ops on the stack case
// are recorded as explicit ops + synced size/depth snapshot (the app owns
// the live DialogStack and drains/applies ops); keymap/keybinds arrive as
// explicit traits/structs. Names, strings, field order, and branch logic
// verbatim.

#![allow(dead_code)]

use std::collections::HashSet;
use std::sync::{Arc, Mutex, OnceLock};

use crate::config::keybind::{BindingKey, BindingLookup, COMMAND_MAP};
use crate::keymap::COMMAND_PALETTE_COMMAND;
use crate::ui::dialog::{DialogSize, DialogStack};

/// Mirrors `COMMAND_PALETTE_SHOW` (equals `COMMAND_PALETTE_COMMAND`
/// from `crate::keymap`; kept as its own const like the source).
pub const COMMAND_PALETTE_SHOW: &str = "command.palette.show";

/// Mirrors module `warned = new Set<string>()`.
fn warned() -> &'static Mutex<HashSet<String>> {
    static WARNED: OnceLock<Mutex<HashSet<String>>> = OnceLock::new();
    WARNED.get_or_init(|| Mutex::new(HashSet::new()))
}

/// Mirrors `warnCommandShim` — warns v1 plugins about the deprecated API.
fn warn_command_shim(api: &str, replacement: &str) {
    // Mirrors console.warn("[tui.plugin] deprecated TUI plugin API", {...}).
    eprintln!(
        "[tui.plugin] deprecated TUI plugin API {{ api: {}, replacement: {} }}",
        api, replacement
    );
}

/// Mirrors `warnOnce(api, replacement, warn)` — first call wins per api.
fn warn_once(api: &str, replacement: &str, warn: fn(&str, &str)) {
    let mut guard = warned().lock().unwrap_or_else(|e| e.into_inner());
    if guard.contains(api) {
        return;
    }
    guard.insert(api.to_string());
    drop(guard);
    warn(api, replacement);
}

/// Explicit dialog ops recorded for the stack case (the app drains these
/// against the live DialogStack; mirrors the delegated method calls).
#[derive(Debug)]
pub enum DialogOp {
    Replace { has_on_close: bool },
    Clear,
    SetSize(DialogSize),
}

/// Live size/depth snapshot synced from the stack by the app.
/// Getters mirror `dialog.size` / `dialog.stack.length` / `length > 0`.
#[derive(Debug, Clone)]
pub struct StackDialogSnapshot {
    pub size: DialogSize,
    pub depth: usize,
}

impl StackDialogSnapshot {
    pub fn sync_from_stack(&mut self, stack: &DialogStack) {
        self.size = stack.size();
        self.depth = stack.stack_len();
    }

    pub fn open(&self) -> bool {
        self.depth > 0
    }
}

/// Mirrors `CommandShimDialog = DialogContext | LegacyDialog`.
/// Stack case: snapshot + op log. Legacy case: behavior-compatible handle.
#[derive(Debug, Clone)]
pub struct ShimDialog {
    inner: Arc<Mutex<ShimDialogInner>>,
}

#[derive(Debug, Default)]
struct ShimDialogInner {
    size: DialogSize,
    depth: usize,
    ops: Vec<DialogOp>,
}

impl Default for ShimDialog {
    fn default() -> Self {
        Self::new()
    }
}

impl ShimDialog {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(ShimDialogInner::default())),
        }
    }

    /// Mirrors `createCommandShimDialog` stack branch — snapshot the stack.
    pub fn from_stack(stack: &DialogStack) -> Self {
        let dialog = Self::new();
        {
            let mut guard = dialog.inner.lock().unwrap_or_else(|e| e.into_inner());
            guard.size = stack.size();
            guard.depth = stack.stack_len();
        }
        dialog
    }

    /// Mirrors the `"stack" in dialog` guard — legacy handles pass through.
    pub fn legacy(size: DialogSize, depth: usize) -> Self {
        let dialog = Self::new();
        {
            let mut guard = dialog.inner.lock().unwrap_or_else(|e| e.into_inner());
            guard.size = size;
            guard.depth = depth;
        }
        dialog
    }

    /// Mirrors `replace(render, onClose)`.
    pub fn replace(&self, _render: Box<dyn FnMut(&mut DialogStack) + Send>, on_close: Option<Box<dyn FnMut() + Send>>) {
        let mut guard = self.inner.lock().unwrap_or_else(|e| e.into_inner());
        guard.ops.push(DialogOp::Replace {
            has_on_close: on_close.is_some(),
        });
    }

    /// Mirrors `clear()`.
    pub fn clear(&self) {
        self.inner
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .ops
            .push(DialogOp::Clear);
    }

    /// Mirrors `setSize(size)`.
    pub fn set_size(&self, size: DialogSize) {
        let mut guard = self.inner.lock().unwrap_or_else(|e| e.into_inner());
        guard.size = size;
        guard.ops.push(DialogOp::SetSize(size));
    }

    /// Mirrors `get size()`.
    pub fn size(&self) -> DialogSize {
        self.inner.lock().unwrap_or_else(|e| e.into_inner()).size
    }

    /// Mirrors `get depth()` — `dialog.stack.length`.
    pub fn depth(&self) -> usize {
        self.inner.lock().unwrap_or_else(|e| e.into_inner()).depth
    }

    /// Mirrors `get open()` — `dialog.stack.length > 0`.
    pub fn open(&self) -> bool {
        self.depth() > 0
    }

    /// Drain recorded ops (app applies them to the live stack).
    pub fn drain_ops(&self) -> Vec<DialogOp> {
        std::mem::take(
            &mut self
                .inner
                .lock()
                .unwrap_or_else(|e| e.into_inner())
                .ops,
        )
    }

    /// Re-sync size/depth from the live stack.
    pub fn sync_from_stack(&self, stack: &DialogStack) {
        let mut guard = self.inner.lock().unwrap_or_else(|e| e.into_inner());
        guard.size = stack.size();
        guard.depth = stack.stack_len();
    }
}

/// Mirrors `TuiCommand` fields read by the shim.
pub struct TuiCommand {
    pub value: String,
    pub title: String,
    pub description: Option<String>,
    pub category: Option<String>,
    pub suggested: Option<bool>,
    pub hidden: Option<bool>,
    pub enabled: Option<bool>,
    pub slash_name: Option<String>,
    pub slash_aliases: Vec<String>,
    pub keybind: Option<String>,
    pub on_select: Option<Box<dyn FnMut(ShimDialog) + Send>>,
}

/// Mirrors the object built by `toCommand` — field order verbatim:
/// namespace/name/title/desc/category/suggested/hidden/enabled/
/// slashName/slashAliases/run. `namespace` is always `"palette"`.
pub struct ShimRegisteredCommand {
    pub namespace: &'static str,
    pub name: String,
    pub title: String,
    pub desc: Option<String>,
    pub category: Option<String>,
    pub suggested: Option<bool>,
    pub hidden: Option<bool>,
    pub enabled: Option<bool>,
    pub slash_name: Option<String>,
    pub slash_aliases: Vec<String>,
    pub run: Arc<Mutex<Option<Box<dyn FnMut(ShimDialog) + Send>>>>,
}

impl ShimRegisteredCommand {
    /// Mirrors `run() { return item.onSelect?.(dialog) }`.
    pub fn run(&self, dialog: ShimDialog) {
        if let Some(on_select) = self
            .run
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .as_mut()
        {
            on_select(dialog);
        }
    }
}

/// Mirrors one mapped binding (`{...binding, cmd, desc}`) or the fallback
/// (`{key, cmd, desc}`).
#[derive(Debug, Clone)]
pub struct ShimBinding {
    pub key: String,
    pub cmd: String,
    pub desc: Option<String>,
}

/// Mirrors a `registerLayer` layer — commands + bindings.
pub struct KeymapLayer {
    pub commands: Vec<ShimRegisteredCommand>,
    pub bindings: Vec<ShimBinding>,
}

/// Explicit keymap seam (`useOpencodeKeymap` surface used by the shim).
pub trait KeymapShim {
    fn register_layer(&mut self, layer: KeymapLayer) -> Box<dyn FnOnce() + Send>;
    fn dispatch_command(&mut self, name: &str);
}

/// Blanket forwarding so boxed trait objects satisfy the seam.
impl<K: KeymapShim + ?Sized> KeymapShim for Box<K> {
    fn register_layer(&mut self, layer: KeymapLayer) -> Box<dyn FnOnce() + Send> {
        (**self).register_layer(layer)
    }

    fn dispatch_command(&mut self, name: &str) {
        (**self).dispatch_command(name);
    }
}

fn binding_key_display(key: &BindingKey) -> String {
    match key {
        BindingKey::Str(s) => s.clone(),
        BindingKey::Stroke(stroke) => {
            let mut parts = Vec::new();
            if stroke.ctrl == Some(true) {
                parts.push("ctrl".to_string());
            }
            if stroke.shift == Some(true) {
                parts.push("shift".to_string());
            }
            if stroke.meta == Some(true) {
                parts.push("meta".to_string());
            }
            if stroke.super_ == Some(true) {
                parts.push("super".to_string());
            }
            if stroke.hyper == Some(true) {
                parts.push("hyper".to_string());
            }
            parts.push(stroke.name.clone());
            parts.join("+")
        }
    }
}

/// Mirrors `TuiKeybind.CommandMap[key] ?? key`.
fn resolve_command(keybind: &str) -> &str {
    for (alias, command) in COMMAND_MAP {
        if *alias == keybind {
            return command;
        }
    }
    keybind
}

/// Mirrors `toCommand(item, dialog)` — captures the shim dialog for `run`.
/// Mirrors `toCommand(item, dialog)` — captures `onSelect`; the shim dialog
/// is supplied at `run(dialog)` call time (mirrors `item.onSelect?.(dialog)`).
pub fn to_command(item: TuiCommand, _dialog: ShimDialog) -> ShimRegisteredCommand {
    let on_select = item.on_select;
    let run: Arc<Mutex<Option<Box<dyn FnMut(ShimDialog) + Send>>>> =
        Arc::new(Mutex::new(on_select));
    ShimRegisteredCommand {
        namespace: "palette",
        name: item.value,
        title: item.title,
        desc: item.description,
        category: item.category,
        suggested: item.suggested,
        hidden: item.hidden,
        enabled: item.enabled,
        slash_name: item.slash_name,
        slash_aliases: item.slash_aliases,
        run,
    }
}

/// Mirrors `toBindings(commands, keybinds)` — per command with a keybind:
/// resolved keybind present in lookup → mapped bindings with
/// `desc: binding.desc ?? item.title`; else fallback
/// `{key: keybind, cmd: value, desc: title}`.
pub fn to_bindings(commands: &[TuiCommandRef<'_>], keybinds: &BindingLookup) -> Vec<ShimBinding> {
    let mut out = Vec::new();
    for item in commands {
        let keybind = match &item.keybind {
            Some(k) => k,
            None => continue,
        };
        let resolved = resolve_command(keybind);
        if keybinds.has(resolved) {
            for binding in keybinds.get(resolved) {
                out.push(ShimBinding {
                    key: binding_key_display(&binding.key),
                    cmd: item.value.clone(),
                    desc: Some(
                        binding
                            .desc
                            .clone()
                            .unwrap_or_else(|| item.title.clone()),
                    ),
                });
            }
        } else {
            out.push(ShimBinding {
                key: keybind.clone(),
                cmd: item.value.clone(),
                desc: Some(item.title.clone()),
            });
        }
    }
    out
}

/// Borrowed view of a command for `to_bindings` (avoids moving on_select).
pub struct TuiCommandRef<'a> {
    pub value: &'a str,
    pub title: &'a str,
    pub keybind: &'a Option<String>,
}

/// Mirrors `createCommandShim(keymap, dialog, keybinds)` — `{register,
/// trigger, show}` with the three `warnOnce` pairs verbatim.
pub struct CommandShim<K> {
    keymap: K,
    dialog: ShimDialog,
}

impl<K: KeymapShim> CommandShim<K> {
    pub fn new(keymap: K, dialog: ShimDialog) -> Self {
        Self { keymap, dialog }
    }

    /// Mirrors `register(cb)` — warns, maps commands + bindings, registers.
    pub fn register(
        &mut self,
        commands: Vec<TuiCommand>,
        keybinds: &BindingLookup,
    ) -> Box<dyn FnOnce() + Send> {
        warn_once(
            "api.command.register",
            "api.keymap.registerLayer({ commands, bindings })",
            warn_command_shim,
        );
        let refs: Vec<TuiCommandRef<'_>> = commands
            .iter()
            .map(|item| TuiCommandRef {
                value: &item.value,
                title: &item.title,
                keybind: &item.keybind,
            })
            .collect();
        let bindings = to_bindings(&refs, keybinds);
        let dialog = self.dialog.clone();
        let mapped: Vec<ShimRegisteredCommand> = commands
            .into_iter()
            .map(|item| to_command(item, dialog.clone()))
            .collect();
        self.keymap.register_layer(KeymapLayer {
            commands: mapped,
            bindings,
        })
    }

    /// Mirrors `trigger(value)` — warns, dispatches.
    pub fn trigger(&mut self, value: &str) {
        warn_once(
            "api.command.trigger",
            "api.keymap.dispatchCommand(name)",
            warn_command_shim,
        );
        let _ = COMMAND_PALETTE_COMMAND;
        self.keymap.dispatch_command(value);
    }

    /// Mirrors `show()` — warns, dispatches the palette command.
    pub fn show(&mut self) {
        warn_once(
            "api.command.show",
            "api.keymap.dispatchCommand(\"command.palette.show\")",
            warn_command_shim,
        );
        self.keymap.dispatch_command(COMMAND_PALETTE_SHOW);
    }
}

/// Mirrors `createCommandShim` factory.
pub fn create_command_shim<K: KeymapShim>(keymap: K, dialog: ShimDialog) -> CommandShim<K> {
    CommandShim::new(keymap, dialog)
}
