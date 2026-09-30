//! Rust port of `packages/core/src/tool/builtins.ts` — built-in registry aggregation.

pub const BUILTIN_TOOLS: &[&str] = &[
    "bash",
    "read",
    "write",
    "edit",
    "grep",
    "glob",
    "webfetch",
    "websearch",
    "todowrite",
    "apply_patch",
    "question",
    "skill",
];

pub fn is_builtin(name: &str) -> bool {
    BUILTIN_TOOLS.contains(&name)
}

// PROVISIONAL pending Location node wiring — list above is verbatim from source.
