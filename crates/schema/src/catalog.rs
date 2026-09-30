//! Port of `packages/schema/src/catalog.ts`.
//!
//! Source exports: `Event.{Updated,Definitions}` only (`catalog.updated`,
//! empty schema).

#![allow(non_snake_case)]

/// Event definitions (`catalog.updated`).
pub mod Event {
    /// `catalog.updated`.
    pub struct Updated;
    impl Updated {
        pub const TYPE: &'static str = "catalog.updated";
    }

    /// Verbatim declaration order: `Updated`.
    pub const Definitions: &[&'static str] = &[Updated::TYPE];
}
