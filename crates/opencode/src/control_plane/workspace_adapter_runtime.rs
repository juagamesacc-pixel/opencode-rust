// source: src/control-plane/workspace-adapter-runtime.ts — exports:
// target, configure, create, list, remove, WorkspaceAdapterRuntime
// (instance+workspace context capture, list ?? [] fallback verbatim).

/// source: runtime fns — verbatim names/order.
pub const RUNTIME_FNS: &[&str] = &["target", "configure", "create", "list", "remove"];
