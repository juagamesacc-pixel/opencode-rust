// source: packages/plugin/src/v2/effect/event.ts
#![allow(dead_code)]

//! Rust port of `packages/plugin/src/v2/effect/event.ts` (opencode v1.18.30).
//!
//! Source 10 lines. Exports: `EventMap`, `Event`.
//!
//! PROVISIONAL: `effect` (`Stream`) pending effect runtime, `@opencode-ai/sdk/v2/types` (`Event as SDKEvent`) pending `crates/sdk`.

/// Mirrors `EventMap = { [Item in SDKEvent as Item["type"]]: Item }` verbatim.
pub type EventMap = serde_json::Value;

/// Mirrors `Event` interface verbatim: `subscribe<Type extends keyof EventMap>(type: Type): Stream.Stream<EventMap[Type]>`.
#[derive(Clone, Debug, PartialEq)]
pub struct Event;

impl Event {
    pub const SUBSCRIBE: &'static str = "subscribe";
}

/// PROVISIONAL: `effect` Stream pending effect runtime.
pub mod effect_provisional {
    pub const PACKAGE: &str = "effect";
    pub const STREAM: &str = "Stream.Stream";
    pub const PENDING_CRATE: &str = "effect";
}
/// PROVISIONAL: `@opencode-ai/sdk/v2/types` pending `crates/sdk`.
pub mod sdk_provisional {
    pub const PACKAGE: &str = "@opencode-ai/sdk/v2/types";
    pub const EVENT: &str = "Event";
    pub const PENDING_CRATE: &str = "crates/sdk";
}
