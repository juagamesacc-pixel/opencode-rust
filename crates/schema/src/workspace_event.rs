//! Port of `packages/schema/src/workspace-event.ts`.
//!
//! Workspace domain events. 1:1 exact translation — same names/signatures/
//! behavior/edge-cases/error-strings/keys/defaults/ordering. Source is spec.
//!
//! This module itself is the `WorkspaceEvent` namespace: source line 1
//! (`export * as WorkspaceEvent from "./workspace-event"`) is the
//! self-namespace pattern, so `crate::workspace_event` plays that role in
//! Rust; no extra item is emitted for it.
//!
//! Per-event mapping (`Event.define` → `TYPE` const + payload struct +
//! `definition()` fn; `Event.inventory` → `Definitions` const): each event
//! lives in a submodule named exactly after the source const (`Ready`,
//! `Failed`, `Status`), and `Definitions` lists them in source order.
//! `type` strings are verbatim (`workspace.ready`, `workspace.failed`,
//! `workspace.status`). None of the three events is durable.

#![allow(non_snake_case, non_upper_case_globals)]

use crate::workspace_id::WorkspaceID;

/// Port of `ConnectionStatus` (`Schema.Struct({ workspaceID: WorkspaceID, status: Literals([...]) })`,
/// annotated with identifier `"WorkspaceEvent.ConnectionStatus"` in
/// `workspace-event.ts`). Field order and keys are verbatim.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ConnectionStatus {
    pub workspaceID: WorkspaceID,
    pub status: ConnectionStatusStatus,
}

/// Closed literal set `["connected", "connecting", "disconnected", "error"]`
/// of `ConnectionStatus.status` (`workspace-event.ts`).
///
/// The field type is anonymous in source; the Rust name composes the owning
/// struct + field. Variant order and strings are verbatim; deserialization
/// rejects any other value (port of `Schema.Literals`).
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ConnectionStatusStatus {
    #[serde(rename = "connected")]
    Connected,
    #[serde(rename = "connecting")]
    Connecting,
    #[serde(rename = "disconnected")]
    Disconnected,
    #[serde(rename = "error")]
    Error,
}

/// Port of `Ready = Event.define({ type: "workspace.ready", schema: { name: Schema.String } })`
/// (`workspace-event.ts`).
pub mod Ready {
    /// Port of the attached `type` static (`input.type`).
    pub const TYPE: &'static str = "workspace.ready";

    /// Static name of the `data` payload struct (port of the `data` static,
    /// i.e. `Schema.Struct(input.schema)`).
    pub const DATA: &'static str = "crate::workspace_event::Ready::Data";

    /// Port of the `data` struct (`Schema.Struct({ name: Schema.String })`).
    #[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
    pub struct Data {
        pub name: String,
    }

    /// Port of the `define(...)` descriptor for this event.
    pub fn definition() -> crate::event::Definition {
        crate::event::define(crate::event::DefineInput {
            r#type: TYPE,
            durable: None,
            data: DATA,
        })
    }
}

/// Port of `Failed = Event.define({ type: "workspace.failed", schema: { message: Schema.String } })`
/// (`workspace-event.ts`).
pub mod Failed {
    /// Port of the attached `type` static (`input.type`).
    pub const TYPE: &'static str = "workspace.failed";

    /// Static name of the `data` payload struct (port of the `data` static,
    /// i.e. `Schema.Struct(input.schema)`).
    pub const DATA: &'static str = "crate::workspace_event::Failed::Data";

    /// Port of the `data` struct (`Schema.Struct({ message: Schema.String })`).
    #[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
    pub struct Data {
        pub message: String,
    }

    /// Port of the `define(...)` descriptor for this event.
    pub fn definition() -> crate::event::Definition {
        crate::event::define(crate::event::DefineInput {
            r#type: TYPE,
            durable: None,
            data: DATA,
        })
    }
}

/// Port of `Status = Event.define({ type: "workspace.status", schema: ConnectionStatus.fields })`
/// (`workspace-event.ts`).
pub mod Status {
    /// Port of the attached `type` static (`input.type`).
    pub const TYPE: &'static str = "workspace.status";

    /// Static name of the `data` payload struct (port of the `data` static,
    /// i.e. `Schema.Struct(ConnectionStatus.fields)` — a distinct struct with
    /// the same fields, not an alias of `ConnectionStatus`).
    pub const DATA: &'static str = "crate::workspace_event::Status::Data";

    /// Port of the `data` struct (`Schema.Struct(ConnectionStatus.fields)`).
    /// Field order and keys match `ConnectionStatus`.
    #[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
    pub struct Data {
        pub workspaceID: super::WorkspaceID,
        pub status: super::ConnectionStatusStatus,
    }

    /// Port of the `define(...)` descriptor for this event.
    pub fn definition() -> crate::event::Definition {
        crate::event::define(crate::event::DefineInput {
            r#type: TYPE,
            durable: None,
            data: DATA,
        })
    }
}

/// Port of `Definitions = Event.inventory(Ready, Failed, Status)`
/// (`workspace-event.ts`). Order is verbatim. (Name stays verbatim per
/// doctrine — parity over lint — hence the `non_upper_case_globals` allow.)
pub const Definitions: &[crate::event::Definition] = &[
    crate::event::Definition {
        r#type: Ready::TYPE,
        durable: None,
        data: Ready::DATA,
    },
    crate::event::Definition {
        r#type: Failed::TYPE,
        durable: None,
        data: Failed::DATA,
    },
    crate::event::Definition {
        r#type: Status::TYPE,
        durable: None,
        data: Status::DATA,
    },
];
pub mod Event {
    pub use super::Definitions;
}
