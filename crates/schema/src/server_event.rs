//! Port of `packages/schema/src/server-event.ts`.
//!
//! Source exports: `Connected`, `Disposed`, `Definitions` (`server.connected`,
//! `global.disposed`, both empty schemas). Uses the `Event.define` namespace
//! form in source.

#![allow(non_snake_case)]

/// `server.connected` definition marker.
pub struct Connected;
impl Connected {
    pub const TYPE: &'static str = "server.connected";
}

/// `global.disposed` definition marker.
pub struct Disposed;
impl Disposed {
    pub const TYPE: &'static str = "global.disposed";
}

/// Verbatim declaration order: `Connected`, `Disposed`.
pub const Definitions: &[&'static str] = &[Connected::TYPE, Disposed::TYPE];
