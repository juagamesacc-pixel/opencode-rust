//! Port of `packages/schema/src/workspace.ts`.
//!
//! Re-export facade: `ID = WorkspaceID`, `Event = WorkspaceEvent`. 1:1 exact
//! translation — same names/ordering. Source is spec.
//!
//! This module itself is the `Workspace` namespace: source line 1
//! (`export * as Workspace from "./workspace"`) is the self-namespace
//! pattern, so `crate::workspace` plays that role in Rust; no extra item is
//! emitted for it.

#![allow(non_snake_case)]

/// Port of `export const ID = WorkspaceID` + `export type ID = WorkspaceID`
/// (`workspace.ts`). A single re-export covers both (structs are type + value).
pub use crate::workspace_id::WorkspaceID as ID;

/// Port of `export const Event = WorkspaceEvent` (`workspace.ts`): the event
/// namespace for this domain.
pub use crate::workspace_event as Event;
