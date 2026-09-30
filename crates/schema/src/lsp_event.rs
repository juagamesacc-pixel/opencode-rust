//! Port of `packages/schema/src/lsp-event.ts`.
//!
//! Source exports: `Updated`, `Definitions` (`lsp.updated`, empty schema).
//! Uses the `Event.define` namespace form in source.

#![allow(non_snake_case)]

/// `lsp.updated` definition marker.
pub struct Updated;
impl Updated {
    pub const TYPE: &'static str = "lsp.updated";
}

/// Verbatim declaration order: `Updated`.
pub const Definitions: &[&'static str] = &[Updated::TYPE];
