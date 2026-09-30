// source: src/lsp/launch.ts — exports: spawn (args-or-opts overload,
// piped stdio triple, "Process output not available" guard verbatim).
// PROVISIONAL: process spawning via @/util/process descriptor.

/// source: stdio triple pipe — verbatim.
pub const STDIO: &[&str] = &["pipe", "pipe", "pipe"];

/// source: "Process output not available" — verbatim (mirrors util/process).
pub const NO_OUTPUT: &str = "Process output not available";
