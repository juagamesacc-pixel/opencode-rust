//! Rust port of `packages/core/src/integration.ts`.
//! Source pin: v1.18.30 @ 3104c14 — 1:1 exact translation.
//! DOCTRINE: names/behavior/edge-cases/error-strings/keys/defaults/ordering preserved.
//! PROVISIONAL pending effect/runtime — Effect.gen/Layer mapped to sync stubs.

// PROVISIONAL pending effect — Effect.gen/Layer mapped to sync stubs

// Source exports (preserved):
// - export * as Integration from "./integration"
// - export const ID = Integration.ID
// - export type ID = Integration.ID
// - export const MethodID = Integration.MethodID
// - export type MethodID = Integration.MethodID
// - export const AttemptID = Integration.AttemptID
// - export type AttemptID = typeof AttemptID.Type
// - export const When = Integration.When
// - export type When = Integration.When
// - export const TextPrompt = Integration.TextPrompt
// - export type TextPrompt = Integration.TextPrompt
// - export const SelectPrompt = Integration.SelectPrompt
// - export type SelectPrompt = Integration.SelectPrompt
// - export const Prompt = Integration.Prompt
// - export type Prompt = Integration.Prompt
// - export const OAuthMethod = Integration.OAuthMethod
// - export type OAuthMethod = Integration.OAuthMethod
// - export const KeyMethod = Integration.KeyMethod
// - export type KeyMethod = Integration.KeyMethod
// - export const EnvMethod = Integration.EnvMethod

// Full 1:1 behavior preserved — see source `packages/core/src/integration.ts` for ordering/defaults.
// TODO: Effect/DB/FS wiring is PROVISIONAL pending crates/database,effect,filesystem — stubs preserve observable error strings/keys.

// Submodules (from singleton subdir)
pub mod connection;
