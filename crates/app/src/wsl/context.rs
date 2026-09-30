//! Rust port of `packages/app/src/wsl/context.tsx` (opencode v1.18.30).
//!
//! Source 36 lines: `wslServersQueryKey`, `useWslServers` /
//! `WslServersProvider` (`createSimpleContext`, tanstack-query subscription).
//! Reactivity is PROVISIONAL; query key + subscription semantics preserved.
//! Original file: `packages/app/src/wsl/context.tsx`

#![allow(dead_code)]

/// Mirrors `wslServersQueryKey` (verbatim).
pub const WSL_SERVERS_QUERY_KEY: &[&str] = &["platform", "wslServers"];

// PROVISIONAL: pending solid-query/ui-context — mirrors `packages/app/src/wsl/context.tsx`.
/// Mirrors the `WslServers` context handle.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct WslServersContext {
    pub subscribed: bool,
}

impl WslServersContext {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn update_subscribe(&mut self) {
        self.subscribed = true;
    }

    pub fn transition_unsubscribe(&mut self) {
        self.subscribed = false;
    }
}
