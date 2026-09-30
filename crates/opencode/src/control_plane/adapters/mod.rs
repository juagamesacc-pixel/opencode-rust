// source: src/control-plane/adapters/index.ts — exports: getAdapter,
// listAdapters, registeredAdapters, registerAdapter
// (BUILTIN { worktree }, custom-then-builtin precedence, `Unknown workspace
// adapter: ${type}` verbatim).

/// source: BUILTIN adapters — verbatim keys.
pub const BUILTIN_ADAPTERS: &[&str] = &["worktree"];

/// source: `Unknown workspace adapter: ${type}` — verbatim.
pub fn unknown_adapter_message(type_: &str) -> String {
    format!("Unknown workspace adapter: {}", type_)
}
