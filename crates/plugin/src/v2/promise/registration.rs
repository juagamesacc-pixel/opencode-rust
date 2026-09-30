// source: packages/plugin/src/v2/promise/registration.ts
#![allow(dead_code)]

//! Rust port of `packages/plugin/src/v2/promise/registration.ts` (opencode v1.18.30).
//!
//! Source 11 lines. Exports: `Registration`, `Reload`, `Hooks` Promise variant (`=> Promise<Registration>`).

/// Mirrors `Registration { readonly dispose: () => Promise<void> }` Promise variant verbatim.
#[derive(Clone, Debug, PartialEq)]
pub struct Registration;

impl Registration {
    pub const DISPOSE: &'static str = "dispose";
}

/// Mirrors `Reload { readonly reload: () => Promise<void> }` Promise variant verbatim.
#[derive(Clone, Debug, PartialEq)]
pub struct Reload;

impl Reload {
    pub const RELOAD: &'static str = "reload";
}

/// Mirrors `Hooks<Spec> = { [Name in keyof Spec]: (callback: (input: Spec[Name]) => Promise<void> | void) => Promise<Registration> }` Promise variant verbatim.
pub type Hooks = serde_json::Value;
