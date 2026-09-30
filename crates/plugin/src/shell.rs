// source: packages/plugin/src/shell.ts
#![allow(dead_code)]

//! Rust port of `packages/plugin/src/shell.ts` (opencode v1.18.30).
//!
//! Source 136 lines. Exports: `ShellFunction`, `ShellExpression`, `BunShell`, `BunShellPromise`, `BunShellOutput`, `BunShellError`.
//!
//! 1:1 notes:
//! - All method names/signatures verbatim: braces, escape, env, cwd, nothrow, throws, quiet, lines, text, json, arrayBuffer, blob, bytes.
//! - `ShellExpression` union verbatim: toString | Array | string | {raw} | ReadableStream.
//!
//! PROVISIONAL: `BunShell` is Bun runtime (`bun` package) — descriptor stubs pending bun runtime.
//! `Buffer`, `ReadableStream`, `WritableStream`, `Blob` are JS runtime types — stubs via `serde_json::Value`.

use serde::{Deserialize, Serialize};

/// Mirrors `ShellFunction = (input: Uint8Array) => Uint8Array`.
pub type ShellFunction = serde_json::Value;

/// Mirrors `ShellExpression` union.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ShellExpression {
    String(String),
    Array(Vec<ShellExpression>),
    Raw { raw: String },
    Stream(serde_json::Value),
    ToString(serde_json::Value),
}

/// Mirrors `BunShell` interface method names verbatim.
pub const BUN_SHELL_METHODS: &[&str] = &["braces", "escape", "env", "cwd", "nothrow", "throws"];

/// Mirrors `BunShellPromise` method names verbatim.
pub const BUN_SHELL_PROMISE_METHODS: &[&str] = &[
    "cwd",
    "env",
    "quiet",
    "lines",
    "text",
    "json",
    "arrayBuffer",
    "blob",
    "nothrow",
    "throws",
];

/// Mirrors `BunShellOutput` fields verbatim.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BunShellOutput {
    pub stdout: serde_json::Value,
    pub stderr: serde_json::Value,
    #[serde(rename = "exitCode")]
    pub exit_code: i32,
}

/// Mirrors `BunShellPromise` output methods verbatim.
pub const BUN_SHELL_OUTPUT_METHODS: &[&str] = &["text", "json", "arrayBuffer", "bytes", "blob"];

/// Mirrors `BunShell` descriptor.
#[derive(Clone, Debug, PartialEq)]
pub struct BunShell;

impl BunShell {
    pub const BRACES: &'static str = "braces";
    pub const ESCAPE: &'static str = "escape";
    pub const ENV: &'static str = "env";
    pub const CWD: &'static str = "cwd";
    pub const NOTHROW: &'static str = "nothrow";
    pub const THROWS: &'static str = "throws";
}

/// Mirrors `BunShellPromise` descriptor.
#[derive(Clone, Debug, PartialEq)]
pub struct BunShellPromise;

impl BunShellPromise {
    pub const STDIN: &'static str = "stdin";
}

/// Mirrors `BunShellError = Error & BunShellOutput`.
pub type BunShellError = serde_json::Value;

/// PROVISIONAL: `bun` runtime pending.
pub mod bun_provisional {
    pub const PACKAGE: &str = "bun";
    pub const PENDING_CRATE: &str = "bun";
}
