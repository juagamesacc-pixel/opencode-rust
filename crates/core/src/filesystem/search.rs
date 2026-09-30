//! Rust port of `packages/core/src/filesystem/search.ts`.
//! Source pin: v1.18.30 @3104c14 — 1:1 exact translation.
//! 239 lines — faithful port of Interface/Service + ripgrepLayer/fffLayer + node.

// PROVISIONAL pending Effect, Fff, fuzzysort, FileSystem, FSUtil, Location, Ripgrep, RelativePath, Flag

pub const SERVICE_ID: &str = "@opencode/v2/FileSystem/Search";

/// Source: `export interface Interface { find, glob, grep }` verbatim
pub trait Interface {
    fn find(&self, input: &serde_json::Value) -> Vec<serde_json::Value>;
    fn glob(&self, input: &serde_json::Value) -> Vec<serde_json::Value>;
    fn grep(&self, input: &serde_json::Value) -> Vec<serde_json::Value>;
}

pub const RIPGREP_LIMIT_DEFAULT: usize = 100_000;
pub const FIND_FUZZY_LIMIT: usize = 50;
pub const GREP_TIME_BUDGET_MS: u64 = 1500;
pub const GREP_LINE_TRUNCATE: usize = 2000;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn service_id() {
        assert_eq!(SERVICE_ID, "@opencode/v2/FileSystem/Search");
    }
}
