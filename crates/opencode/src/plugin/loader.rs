// source: src/plugin/loader.ts — exports: PluginLoader (Plan, Resolved,
// Missing, Loaded, Report, stages; "missing package.json or index file"
// retry rule verbatim).
// PROVISIONAL: npm/importCompat resolution as descriptors.

/// source: stages — verbatim.
pub const STAGES: &[&str] = &["install", "entry", "compatibility", "load"];

/// source: "missing package.json or index file" install-retry rule — verbatim.
pub const RETRYABLE_RESOLVE_MESSAGE: &str = "missing package.json or index file";

/// source: isRetryableResolveError() — stage === install && includes. Verbatim.
pub fn is_retryable_resolve_error(stage: &str, message: &str) -> bool {
    stage == "install" && message.contains(RETRYABLE_RESOLVE_MESSAGE)
}
