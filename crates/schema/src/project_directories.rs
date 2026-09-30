//! Rust port of `packages/schema/src/project-directories.ts` (anomalyco/opencode v1.18.30).
//!
//! 1:1 exact translation — same names/signatures/behavior/edge cases/keys/defaults/ordering.
//! Source is spec. No improvements, renames, merges, splits, or reordering.
//!
//! Source exports only `Event = { Updated, Definitions: inventory(Updated) }`.
//! The `Updated` definition is private and reachable as `Event.Updated`.

#![allow(non_snake_case, non_upper_case_globals)]

/// Port of `Event = { Updated, Definitions: inventory(Updated) }` (`project-directories.ts`).
pub mod Event {
    /// Port of
    /// `Updated = define({ type: "project.directories.updated", schema: { projectID: Project.ID } })`.
    pub mod Updated {
        /// Port of the attached `type` static (`input.type`).
        pub const TYPE: &'static str = "project.directories.updated";

        /// Static name of the `data` payload struct (port of the `data` static).
        pub const DATA: &'static str = "crate::project_directories::Event::Updated::Data";

        /// Port of the `data` struct (`Schema.Struct({ projectID: Project.ID })`).
        #[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
        pub struct Data {
            pub projectID: crate::project::ID,
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
