// source: packages/plugin/src/v2/effect/filesystem.ts
#![allow(dead_code)]

//! Rust port of `packages/plugin/src/v2/effect/filesystem.ts` (opencode v1.18.30).
//!
//! Source 17 lines. Exports: `FileSystem` with read/list/find/glob.
//!
//! PROVISIONAL: `effect` (`Effect`) pending effect runtime, `@opencode-ai/sdk/v2/types` (`FileSystemEntry`) pending `crates/sdk`.

/// Mirrors `FileSystem` method names verbatim.
pub const FILESYSTEM_METHODS: &[&str] = &["read", "list", "find", "glob"];

/// Mirrors `FileSystem` descriptor.
#[derive(Clone, Debug, PartialEq)]
pub struct FileSystem;

impl FileSystem {
    pub const READ: &'static str = "read";
    pub const LIST: &'static str = "list";
    pub const FIND: &'static str = "find";
    pub const GLOB: &'static str = "glob";
}

/// Mirrors `FileSystemEntry` brand.
pub type FileSystemEntry = serde_json::Value;

/// PROVISIONAL: `effect` pending.
pub mod effect_provisional {
    pub const PACKAGE: &str = "effect";
    pub const EFFECT: &str = "Effect.Effect";
    pub const PENDING_CRATE: &str = "effect";
}
