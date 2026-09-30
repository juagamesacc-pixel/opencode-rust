//! Port of `packages/schema/src/tui-event.ts`.
//!
//! Source exports: `PromptAppend`, `CommandExecute`, `ToastShow`,
//! `SessionSelect`, `Definitions`. Uses the `Event.define` namespace form.
//! Cross-lane: `SessionID` is `crate::session_id::SessionID`,
//! `PositiveInt` is `crate::schema_primitives::PositiveInt` (Lane A).
//!
//! Notes: `CommandExecute.command` is a closed literal set unioned with
//! `String` in source (i.e. any string is valid) — ported as `String` with the
//! 17 known commands listed in `CommandExecute::KNOWN_COMMANDS` for reference.
//! `ToastShow.duration` decodes to `5000` when absent (source-private
//! `DEFAULT_TOAST_DURATION` via `withDecodingDefault`).

#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};

fn default_toast_duration() -> crate::schema_primitives::PositiveInt {
    5000
}

/// Payload of `tui.prompt.append`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PromptAppendPayload {
    pub text: String,
}

/// `tui.prompt.append` definition marker.
pub struct PromptAppend;
impl PromptAppend {
    pub const TYPE: &'static str = "tui.prompt.append";
}

/// Payload of `tui.command.execute`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CommandExecutePayload {
    pub command: String,
}

/// `tui.command.execute` definition marker.
pub struct CommandExecute;
impl CommandExecute {
    pub const TYPE: &'static str = "tui.command.execute";

    /// Known commands from the source literal set (any string remains valid).
    pub const KNOWN_COMMANDS: &[&'static str] = &[
        "session.list",
        "session.new",
        "session.share",
        "session.interrupt",
        "session.compact",
        "session.page.up",
        "session.page.down",
        "session.line.up",
        "session.line.down",
        "session.half.page.up",
        "session.half.page.down",
        "session.first",
        "session.last",
        "prompt.clear",
        "prompt.submit",
        "agent.cycle",
    ];
}

/// `tui.toast.show` `variant` closed set.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ToastVariant {
    #[serde(rename = "info")]
    Info,
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "warning")]
    Warning,
    #[serde(rename = "error")]
    Error,
}

/// Payload of `tui.toast.show`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ToastShowPayload {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    pub message: String,
    pub variant: ToastVariant,
    #[serde(default = "default_toast_duration")]
    pub duration: crate::schema_primitives::PositiveInt,
}

/// `tui.toast.show` definition marker.
pub struct ToastShow;
impl ToastShow {
    pub const TYPE: &'static str = "tui.toast.show";
}

/// Payload of `tui.session.select`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SessionSelectPayload {
    pub sessionID: crate::session_id::SessionID,
}

/// `tui.session.select` definition marker.
pub struct SessionSelect;
impl SessionSelect {
    pub const TYPE: &'static str = "tui.session.select";
}

/// Verbatim declaration order: `PromptAppend`, `CommandExecute`, `ToastShow`,
/// `SessionSelect`.
pub const Definitions: &[&'static str] = &[
    PromptAppend::TYPE,
    CommandExecute::TYPE,
    ToastShow::TYPE,
    SessionSelect::TYPE,
];
