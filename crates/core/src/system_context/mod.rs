//! Rust port of `packages/core/src/system-context`.

pub mod builtins;
pub mod registry;

pub fn context_header() -> &'static str {
    "System Context"
}

// PROVISIONAL pending system context algebra wiring.
