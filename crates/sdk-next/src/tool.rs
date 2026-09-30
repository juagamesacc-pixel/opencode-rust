//! Rust port of `packages/sdk-next/src/tool.ts` (opencode v1.18.30).
//!
//! Source 2 lines. Exports: re-exports from `@opencode-ai/core/tool/tool`.
//!
//! 1:1 notes:
//! - `export { Failure, RegistrationError, make } from "@opencode-ai/core/tool/tool"` — names/order verbatim.
//! - `export type { AnyTool, Content, Context, Definition } from "@opencode-ai/core/tool/tool"` — type names/order verbatim.
//! - This file is a pure re-export barrel; no local logic.
//!
//! PROVISIONAL: `@opencode-ai/core/tool/tool` is pending `crates/core` — faithful stub with same IDs/names.

// ---------------------------------------------------------------------------
// Re-export descriptors verbatim (PROVISIONAL — pending crates/core)
// ---------------------------------------------------------------------------

/// Tool failure tag verbatim: `"Failure"` from `core/tool/tool`.
pub const FAILURE_TAG: &str = "Failure";
/// Registration error tag verbatim: `"RegistrationError"`.
pub const REGISTRATION_ERROR_TAG: &str = "RegistrationError";
/// Tool factory fn name verbatim: `"make"`.
pub const MAKE_FN: &str = "make";

/// Mirrors `AnyTool` type brand (opaque handle).
pub type AnyTool = serde_json::Value;

/// Mirrors `Content` type brand.
pub type Content = serde_json::Value;

/// Mirrors `Context` type brand.
pub type Context = serde_json::Value;

/// Mirrors `Definition` type brand.
pub type Definition = serde_json::Value;

/// Mirrors `Failure` error tag.
#[derive(Clone, Debug, PartialEq)]
pub struct Failure {
    pub _tag: &'static str,
}

impl Failure {
    pub const TAG: &'static str = FAILURE_TAG;
}

/// Mirrors `RegistrationError` error tag.
#[derive(Clone, Debug, PartialEq)]
pub struct RegistrationError {
    pub _tag: &'static str,
}

impl RegistrationError {
    pub const TAG: &'static str = REGISTRATION_ERROR_TAG;
}

/// Mirrors `Tool` namespace descriptor (re-exported as `Tool` in `index.ts` `export * as Tool`).
#[derive(Clone, Debug, PartialEq)]
pub struct Tool;

impl Tool {
    pub const MODULE: &'static str = "@opencode-ai/core/tool/tool";
    pub const EXPORTS: &'static [&'static str] = &["Failure", "RegistrationError", "make"];
    pub const TYPE_EXPORTS: &'static [&'static str] =
        &["AnyTool", "Content", "Context", "Definition"];
}

/// PROVISIONAL: core tool provisional stub.
pub mod core_provisional {
    pub const SERVICE_MODULE: &str = "@opencode-ai/core/tool/tool";
    pub const PENDING_CRATE: &str = "crates/core";
}
