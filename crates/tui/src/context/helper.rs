// source: packages/tui/src/context/helper.tsx (26 lines, v1.18.30)
// 1:1 port — SolidJS context providers become explicit owner structs;
// the `Show when={ready}` gate becomes `gate_ready`.

#![allow(dead_code)]

/// Mirrors the `use()` missing-provider error (`"${name} context must be
/// used within a context provider"`).
pub fn context_missing(name: &str) -> String {
    format!("{name} context must be used within a context provider")
}

/// Mirrors the provider `Show` gate: children render once
/// `ready === undefined || ready === true`.
pub fn gate_ready(ready: Option<bool>) -> bool {
    ready.unwrap_or(true)
}

/// Contexts that expose readiness (mirrors `init.ready`).
pub trait HasReady {
    fn ready(&self) -> Option<bool>;
}
