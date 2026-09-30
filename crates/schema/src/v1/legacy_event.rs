//! Port of `packages/schema/src/v1/legacy-event.ts` (`LegacyEvent`).
//!
//! Source exports: `CommandExecuted`, `Definitions` (`command.executed`
//! `{name, sessionID, arguments, messageID}`). DISTINCT from the flat
//! `crate::legacy_event` shim (which re-exports this module). Cross-lane:
//! `SessionID` (`crate::session_id`), `SessionV1.MessageID`
//! (`super::session`, this lane).

#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};

/// Payload of `command.executed`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CommandExecutedPayload {
    pub name: String,
    pub sessionID: crate::session_id::SessionID,
    pub arguments: String,
    pub messageID: super::session::MessageID,
}

/// `command.executed` definition marker.
pub struct CommandExecuted;
impl CommandExecuted {
    pub const TYPE: &'static str = "command.executed";
}

/// Verbatim declaration order: `CommandExecuted`.
pub const Definitions: &[&'static str] = &[CommandExecuted::TYPE];
