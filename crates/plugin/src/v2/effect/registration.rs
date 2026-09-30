// source: packages/plugin/src/v2/effect/registration.ts
#![allow(dead_code)]

//! Rust port of `packages/plugin/src/v2/effect/registration.ts` (opencode v1.18.30).
//!
//! Source 15 lines. Exports: `Registration`, `Reload`, `Hooks`.
//!
//! PROVISIONAL: `effect` (`Effect`, `Scope`) pending effect runtime.

/// Mirrors `Registration { readonly dispose: Effect.Effect<void> }` verbatim.
#[derive(Clone, Debug, PartialEq)]
pub struct Registration;

impl Registration {
    pub const DISPOSE: &'static str = "dispose";
}

/// Mirrors `Reload { readonly reload: () => Effect.Effect<void> }` verbatim.
#[derive(Clone, Debug, PartialEq)]
pub struct Reload;

impl Reload {
    pub const RELOAD: &'static str = "reload";
}

/// Mirrors `Hooks<Spec> = { [Name in keyof Spec]: (callback: (input: Spec[Name]) => Effect.Effect<void> | void) => Effect.Effect<Registration, never, Scope.Scope> }` verbatim.
pub type Hooks = serde_json::Value;

/// PROVISIONAL: `effect` pending.
pub mod effect_provisional {
    pub const PACKAGE: &str = "effect";
    pub const EFFECT: &str = "Effect.Effect";
    pub const SCOPE: &str = "Scope.Scope";
    pub const PENDING_CRATE: &str = "effect";
}
