//! Rust port of `packages/schema/src/plugin.ts` (anomalyco/opencode v1.18.30).
//!
//! 1:1 exact translation — same names/signatures/behavior/edge cases/keys/defaults/ordering.
//! Source is spec. No improvements, renames, merges, splits, or reordering.
//!
//! Source exports only `ID`; the `Added` definition is private and reachable as
//! `Event.Added` / `Event.Definitions` (the `Event = { Added, Definitions }`
//! object). Following the crate's event pattern, each event is a submodule of
//! the `Event` module with a `TYPE` const, payload `Data` struct and
//! `definition()` fn; `Definitions` lists them in source order. The event is
//! not durable.

#![allow(non_snake_case, non_upper_case_globals)]

use serde::{Deserialize, Serialize};
use std::ops::Deref;

/// Brand `Plugin.ID` — transparent newtype, wire format stays bare string.
///
/// No refinement check in source (`Schema.String.pipe(Schema.brand("Plugin.ID"))`).
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ID(pub String);

impl ID {
    pub fn new(value: String) -> Self {
        ID(value)
    }

    pub fn into_inner(self) -> String {
        self.0
    }
}

impl Deref for ID {
    type Target = str;

    fn deref(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for ID {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for ID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for ID {
    fn from(value: String) -> Self {
        ID(value)
    }
}

impl From<&str> for ID {
    fn from(value: &str) -> Self {
        ID(value.to_string())
    }
}

/// Port of `Event = { Added, Definitions: inventory(Added) }` (`plugin.ts`).
pub mod Event {
    /// Port of `Added = define({ type: "plugin.added", schema: { id: ID } })`.
    pub mod Added {
        /// Port of the attached `type` static (`input.type`).
        pub const TYPE: &'static str = "plugin.added";

        /// Static name of the `data` payload struct (port of the `data` static).
        pub const DATA: &'static str = "crate::plugin::Event::Added::Data";

        /// Port of the `data` struct (`Schema.Struct({ id: ID })`).
        #[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
        pub struct Data {
            pub id: super::super::ID,
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

    /// Port of `Definitions = inventory(Added)`. Order is verbatim.
    pub const Definitions: &[crate::event::Definition] = &[crate::event::Definition {
        r#type: Added::TYPE,
        durable: None,
        data: Added::DATA,
    }];
}
