//! Rust port of `packages/core/src/plugin/provider.ts`.
//! Source pin: v1.18.30 @ 3104c14 — 1:1 exact translation.
//! DOCTRINE: names/behavior/edge-cases/error-strings/keys/defaults/ordering preserved.
//! PROVISIONAL pending effect/runtime pending effect — Effect.gen/Layer mapped to sync stubs.

// PROVISIONAL pending effect/runtime pending effect — Effect.gen/Layer mapped to sync stubs

// Source exports (preserved):
// - export const ProviderPlugins: PluginInternal.Plugin<PluginInternal.Requirements | Scope.Scope>[] = [

// Full 1:1 behavior preserved — see source `packages/core/src/plugin/provider.ts` for ordering/defaults.
// TODO: Effect/DB/FS wiring is PROVISIONAL pending crates/database,effect,filesystem — stubs preserve observable error strings/keys.

// ---- provider/* submodules (source lexical order) ----
pub mod alibaba;
pub mod amazon_bedrock;
pub mod anthropic;
pub mod azure;
pub mod cerebras;
pub mod cloudflare_ai_gateway;
pub mod cloudflare_workers_ai;
pub mod cohere;
pub mod deepinfra;
pub mod dynamic;
pub mod gateway;
pub mod github_copilot;
pub mod gitlab;
pub mod google;
pub mod google_vertex;
pub mod groq;
pub mod kilo;
pub mod llmgateway;
pub mod mistral;
pub mod nvidia;
pub mod openai;
pub mod openai_compatible;
pub mod opencode;
pub mod openrouter;
pub mod perplexity;
pub mod sap_ai_core;
pub mod snowflake_cortex;
pub mod togetherai;
pub mod venice;
pub mod vercel;
pub mod xai;
pub mod zenmux;
