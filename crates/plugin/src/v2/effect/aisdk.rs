// source: packages/plugin/src/v2/effect/aisdk.ts
#![allow(dead_code)]

//! Rust port of `packages/plugin/src/v2/effect/aisdk.ts` (opencode v1.18.30).
//!
//! Source 18 lines. Exports: `AISDKHooks`.
//!
//! PROVISIONAL: `@ai-sdk/provider` (`LanguageModelV3`) pending ai-sdk crate,
//! `@opencode-ai/sdk/v2/types` (`ModelV2Info`) pending `crates/sdk`, `effect` Hooks pending effect runtime.

/// Mirrors `AISDKHooks = Hooks<{ sdk: {...}, language: {...} }>` verbatim keys.
pub const AISDK_HOOKS_KEYS: &[&str] = &["sdk", "language"];

/// Mirrors `AISDKHooks` sdk event fields verbatim.
pub const AISDK_SDK_EVENT_FIELDS: &[&str] = &["model", "package", "options", "sdk"];

/// Mirrors `AISDKHooks` language event fields verbatim.
pub const AISDK_LANGUAGE_EVENT_FIELDS: &[&str] = &["model", "sdk", "options", "language"];

/// Mirrors `AISDKHooks` brand.
pub type AISDKHooks = serde_json::Value;

/// Mirrors `LanguageModelV3` brand from `@ai-sdk/provider`.
pub type LanguageModelV3 = serde_json::Value;

/// Mirrors `ModelV2Info` brand.
pub type ModelV2Info = serde_json::Value;

/// PROVISIONAL: `@ai-sdk/provider` pending ai-sdk crate.
pub mod aisdk_provisional {
    pub const PACKAGE: &str = "@ai-sdk/provider";
    pub const LANGUAGE_MODEL_V3: &str = "LanguageModelV3";
    pub const PENDING_CRATE: &str = "ai-sdk";
}
/// PROVISIONAL: `@opencode-ai/sdk/v2/types` pending `crates/sdk`.
pub mod sdk_provisional {
    pub const PACKAGE: &str = "@opencode-ai/sdk/v2/types";
    pub const MODEL_V2_INFO: &str = "ModelV2Info";
    pub const PENDING_CRATE: &str = "crates/sdk";
}
