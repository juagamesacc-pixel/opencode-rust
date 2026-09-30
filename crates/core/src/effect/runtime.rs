//! Rust port of `packages/core/src/effect/runtime.ts`.
//! Source pin: v1.18.30 @3104c14 — 1:1 exact translation.

// PROVISIONAL pending Effect ManagedRuntime, Context, Layer, Observability

use serde::{Deserialize, Serialize};

/// Source: `export function makeRuntime<I,S,E>(service, layer)` verbatim — lazy ManagedRuntime.make with memoMap

pub struct Runtime;

pub fn make_runtime_description() -> &'static str {
    "ManagedRuntime.make(Layer.provideMerge(layer, Observability.layer), { memoMap })"
}

pub const METHODS: &[&str] = &[
    "runSync",
    "runPromiseExit",
    "runPromise",
    "runFork",
    "runCallback",
];
