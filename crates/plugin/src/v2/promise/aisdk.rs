// source: packages/plugin/src/v2/promise/aisdk.ts
#![allow(dead_code)]

//! Rust port of `packages/plugin/src/v2/promise/aisdk.ts` (opencode v1.18.30).
//!
//! Source 18 lines. Exports: `AISDKHooks` Promise variant (same shape as effect but Promise).

/// Mirrors `AISDKHooks` Promise variant — keys verbatim: sdk, language.
pub const AISDK_HOOKS_KEYS: &[&str] = &["sdk", "language"];

/// Mirrors `AISDKHooks` brand (Promise).
pub type AISDKHooks = serde_json::Value;
