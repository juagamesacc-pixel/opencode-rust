//! Port of `packages/schema/src/models-dev.ts`.
//!
//! Source exports: `Event.{Refreshed,Definitions}` only (`models-dev.refreshed`,
//! empty schema).

#![allow(non_snake_case)]

/// Event definitions (`models-dev.refreshed`).
pub mod Event {
    /// `models-dev.refreshed`.
    pub struct Refreshed;
    impl Refreshed {
        pub const TYPE: &'static str = "models-dev.refreshed";
    }

    /// Verbatim declaration order: `Refreshed`.
    pub const Definitions: &[&'static str] = &[Refreshed::TYPE];
}
