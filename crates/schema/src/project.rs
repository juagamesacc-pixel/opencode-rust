//! Rust port of `packages/schema/src/project.ts` (anomalyco/opencode v1.18.30).
//!
//! 1:1 exact translation — same names/signatures/behavior/edge cases/keys/defaults/ordering.
//! Source is spec. No improvements, renames, merges, splits, or reordering.
//!
//! `ID = ProjectID` (re-export of the branded project identifier). `Vcs` is the
//! literal `"git"`. `Icon`, `Commands`, `Time` and `Info` are structs with
//! package-`optional` fields omitted when absent. The `project.updated` event
//! payload is the spread of `Info.fields` (a distinct struct with the same
//! fields), following the crate's event pattern.

#![allow(non_snake_case, non_upper_case_globals)]

use serde::{Deserialize, Serialize};

/// `Project.ID` (= `ProjectID`).
pub type ID = crate::project_id::ProjectID;

/// Identifier `Project.Vcs` — `Schema.Literal("git")`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Vcs {
    #[serde(rename = "git")]
    Git,
}

/// Identifier `Project.Icon`. Field order is verbatim: url, override, color.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Icon {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "override", skip_serializing_if = "Option::is_none")]
    pub r#override: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
}

/// Identifier `Project.Commands`. Field order is verbatim: start.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Commands {
    /// "Startup script to run when creating a new workspace (worktree)".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<String>,
}

/// Identifier `Project.Time`. Field order is verbatim: created, updated, initialized.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Time {
    pub created: crate::schema_primitives::NonNegativeInt,
    pub updated: crate::schema_primitives::NonNegativeInt,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initialized: Option<crate::schema_primitives::NonNegativeInt>,
}

/// Identifier `Project` (`Project.Info`). Field order is verbatim:
/// id, worktree, vcs, name, icon, commands, time, sandboxes.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Info {
    pub id: ID,
    pub worktree: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vcs: Option<Vcs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<Icon>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commands: Option<Commands>,
    pub time: Time,
    pub sandboxes: Vec<String>,
}

/// Port of `Event = { Updated, Definitions: inventory(Updated) }` (`project.ts`).
pub mod Event {
    /// Port of `Updated = define({ type: "project.updated", schema: Info.fields })`.
    pub mod Updated {
        /// Port of the attached `type` static (`input.type`).
        pub const TYPE: &'static str = "project.updated";

        /// Static name of the `data` payload struct (port of the `data` static,
        /// i.e. `Schema.Struct(Info.fields)` — a distinct struct with the same
        /// fields, not an alias of `Info`).
        pub const DATA: &'static str = "crate::project::Event::Updated::Data";

        /// Port of the `data` struct (`Schema.Struct(Info.fields)`).
        /// Field order and keys match `Info` exactly.
        #[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
        pub struct Data {
            pub id: super::super::ID,
            pub worktree: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub vcs: Option<super::super::Vcs>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub name: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub icon: Option<super::super::Icon>,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub commands: Option<super::super::Commands>,
            pub time: super::super::Time,
            pub sandboxes: Vec<String>,
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

    /// Port of `Definitions = inventory(Updated)`. Order is verbatim.
    pub const Definitions: &[crate::event::Definition] = &[crate::event::Definition {
        r#type: Updated::TYPE,
        durable: None,
        data: Updated::DATA,
    }];
}
