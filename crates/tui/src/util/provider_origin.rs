// source: packages/tui/src/util/provider-origin.ts (7 lines, v1.18.30)
// 1:1 port — membership test over a list or set of console-managed ids.

#![allow(dead_code)]

use std::collections::HashSet;

/// Mirrors `isConsoleManagedProvider`.
pub fn is_console_managed_provider(console_managed: &[String], provider_id: &str) -> bool {
    console_managed.iter().any(|item| item == provider_id)
}

/// Set-backed variant (the TS `ReadonlySet` branch).
pub fn is_console_managed_provider_set(console_managed: &HashSet<String>, provider_id: &str) -> bool {
    console_managed.contains(provider_id)
}