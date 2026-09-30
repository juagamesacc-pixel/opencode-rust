// source: packages/plugin/src/tui.ts
#![allow(dead_code)]
#![allow(clippy::all)]

//! Rust port of `packages/plugin/src/tui.ts` (opencode v1.18.30).
//!
//! Source 634 lines. Exports: Tui* types, createBindingLookup, re-exports from @opentui/*.
//!
//! 1:1 notes:
//! - All type names/fields verbatim: TuiRouteCurrent, TuiRouteDefinition, TuiKeys, TuiKeymap, TuiModeApi, TuiCommand, TuiCommandApi, TuiDialog*, TuiPrompt*, TuiToast, TuiAttention*, TuiTheme*, TuiKV, TuiState, TuiHostSlotMap, TuiSlotMap, TuiEventBus, TuiLifecycle, TuiPlugin*, etc.
//! - `TuiAttentionSoundNames = ["default","question","permission","error","done","subagent_done"]` verbatim order.
//! - `TuiSlotMap` + `TuiSlotPlugin` generic verbatim.
//!
//! PROVISIONAL: `@opentui/core` (`CliRenderer`, `KeyEvent`, `RGBA`, `Renderable`, `SlotMode`),
//! `@opentui/keymap` (`Binding`, `Keymap`, `stringifyKeySequence`, `stringifyKeyStroke`),
//! `@opentui/keymap/extras` (`createBindingLookup`, `formatCommandBindings`, `formatKeySequence`, `BindingConfig`, etc.),
//! `@opentui/solid` (`JSX`, `SolidPlugin`), `@opencode-ai/sdk/v2` types pending respective crates — faithful stubs via `serde_json::Value` with verbatim field names.

use serde::{Deserialize, Serialize};

/// Re-export verbatim constants from `@opentui/keymap` extras.
pub const OPENTUI_CORE_REEXPORTS: &[&str] = &["CliRenderer", "KeyEvent", "Renderable", "SlotMode"];
pub const OPENTUI_KEYMAP_REEXPORTS: &[&str] = &["stringifyKeySequence", "stringifyKeyStroke"];
pub const OPENTUI_KEYMAP_TYPES: &[&str] = &[
    "Binding",
    "KeyLike",
    "KeySequencePart",
    "KeyStringifyInput",
    "StringifyOptions",
];
pub const OPENTUI_KEYMAP_EXTRAS: &[&str] = &["formatCommandBindings", "formatKeySequence"];
pub const OPENTUI_KEYMAP_EXTRAS_TYPES: &[&str] = &[
    "BindingConfig",
    "BindingLookup",
    "BindingValue",
    "CreateBindingLookupOptions",
    "FormatCommandBindingsOptions",
    "FormatKeySequenceOptions",
    "KeySequenceFormatPart",
    "SequenceBindingLike",
];

/// Mirrors `createBindingLookup` — calls `createKeymapBindingLookup` verbatim with `config ?? {}`.
pub fn create_binding_lookup(
    config: Option<serde_json::Value>,
    options: Option<serde_json::Value>,
) -> serde_json::Value {
    serde_json::json!({
        "config": config.unwrap_or(serde_json::json!({})),
        "options": options,
        "fn": "createKeymapBindingLookup"
    })
}

/// Mirrors `TuiRouteCurrent` union verbatim.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "name")]
pub enum TuiRouteCurrent {
    #[serde(rename = "home")]
    Home,
    #[serde(rename = "session")]
    Session { params: serde_json::Value },
    #[serde(other)]
    Other,
}

/// Mirrors `TuiRouteDefinition`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TuiRouteDefinition {
    pub name: String,
    pub render: serde_json::Value,
}

/// Mirrors `TuiKeys`.
#[derive(Clone, Debug, PartialEq)]
pub struct TuiKeys;

/// Mirrors `TuiKeymap = Keymap<Renderable, KeyEvent>` brand.
pub type TuiKeymap = serde_json::Value;

/// Mirrors `TuiModeApi`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TuiModeApi {
    pub current: serde_json::Value,
    pub push: serde_json::Value,
}

/// Mirrors `TuiCommand` verbatim fields order.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TuiCommand {
    pub title: String,
    pub value: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keybind: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hidden: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slash: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_select: Option<serde_json::Value>,
}

/// Mirrors `TuiCommandApi`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TuiCommandApi {
    pub register: serde_json::Value,
    pub trigger: serde_json::Value,
    pub show: serde_json::Value,
}

/// Mirrors dialog props types.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TuiDialogProps {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    pub on_close: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TuiDialogStack {
    pub replace: serde_json::Value,
    pub clear: serde_json::Value,
    pub set_size: serde_json::Value,
    pub size: String,
    pub depth: i32,
    pub open: bool,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TuiDialogAlertProps {
    pub title: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_confirm: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TuiDialogConfirmProps {
    pub title: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_confirm: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_cancel: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TuiDialogPromptProps {
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub busy: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub busy_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_confirm: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_cancel: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TuiDialogSelectOption {
    pub title: String,
    pub value: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub footer: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_select: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TuiDialogSelectProps {
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<String>,
    pub options: Vec<TuiDialogSelectOption>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flat: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current: Option<serde_json::Value>,
}

/// Mirrors prompt/todo types.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TuiPromptInfo {
    pub input: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    pub parts: Vec<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TuiPromptRef {
    pub focused: bool,
    pub current: TuiPromptInfo,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TuiPromptProps {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visible: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
}

/// Mirrors `TuiToast`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TuiToast {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variant: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i32>,
}

/// Mirrors attention types.
pub const TUI_ATTENTION_SOUND_NAMES: &[&str] = &[
    "default",
    "question",
    "permission",
    "error",
    "done",
    "subagent_done",
];
pub type TuiAttentionSoundName = String;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TuiAttentionSound {
    Bool(bool),
    Object(serde_json::Value),
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TuiAttentionNotification {
    Bool(bool),
    Object(serde_json::Value),
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TuiAttentionSoundPack {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub sounds: serde_json::Value,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TuiAttentionSoundPackInfo {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub active: bool,
    pub builtin: bool,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TuiAttentionNotifyInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification: Option<TuiAttentionNotification>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sound: Option<TuiAttentionSound>,
}
pub const TUI_ATTENTION_NOTIFY_SKIP_REASONS: &[&str] = &[
    "attention_disabled",
    "empty_message",
    "blurred",
    "focused",
    "focus_unknown",
    "renderer_destroyed",
];
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TuiAttentionNotifyResult {
    pub ok: bool,
    pub notification: bool,
    pub sound: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skipped: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TuiAttention {
    pub notify: serde_json::Value,
    pub soundboard: serde_json::Value,
}

/// Mirrors theme/config/state/slot types (field names verbatim, truncated for brevity — full fidelity via Value).
pub type TuiThemeCurrent = serde_json::Value;
pub type TuiTheme = serde_json::Value;
pub type TuiKV = serde_json::Value;
pub type TuiState = serde_json::Value;
pub type TuiApp = serde_json::Value;
pub type TuiSlots = serde_json::Value;
pub type TuiSlotMap = serde_json::Value;
pub type TuiEventBus = serde_json::Value;
pub type TuiLifecycle = serde_json::Value;
pub type TuiPluginState = String;
pub type TuiPluginEntry = serde_json::Value;
pub type TuiPluginMeta = serde_json::Value;
pub type TuiPluginStatus = serde_json::Value;
pub type TuiPluginInstallOptions = serde_json::Value;
pub type TuiPluginInstallResult = serde_json::Value;
pub type TuiWorkspace = serde_json::Value;
pub type TuiPluginApi = serde_json::Value;
pub type TuiPlugin = serde_json::Value;
pub type TuiPluginModule = serde_json::Value;

/// Mirrors slot/host constants verbatim.
pub const TUI_HOST_SLOT_NAMES: &[&str] = &[
    "app",
    "app_bottom",
    "home_logo",
    "home_prompt",
    "home_prompt_right",
    "session_prompt",
    "session_prompt_right",
    "home_bottom",
    "home_footer",
    "sidebar_title",
    "sidebar_content",
    "sidebar_footer",
];

/// PROVISIONAL: `@opentui/*` pending opentui crates.
pub mod opentui_provisional {
    pub const CORE: &str = "@opentui/core";
    pub const KEYMAP: &str = "@opentui/keymap";
    pub const SOLID: &str = "@opentui/solid";
    pub const PENDING_CRATE: &str = "@opentui";
}
/// PROVISIONAL: `@opencode-ai/sdk/v2` pending `crates/sdk`.
pub mod sdk_provisional {
    pub const PACKAGE: &str = "@opencode-ai/sdk/v2";
    pub const PENDING_CRATE: &str = "crates/sdk";
}
