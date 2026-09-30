//! Rust port of `packages/core/src/v1/session.ts`.
//! Source pin: v1.18.30 @ 3104c14 — 1:1 exact translation.
//! DOCTRINE: names/behavior/edge-cases/error-strings/keys/defaults/ordering preserved.
//! PROVISIONAL pending serde-schema pending schema — Effect.gen/Layer mapped to sync stubs.

// PROVISIONAL pending serde-schema pending schema — Effect.gen/Layer mapped to sync stubs

// Source exports (preserved):
// - export * as SessionV1 from "./session"
// - export {
// - export const OutputLengthError = NamedError.create("MessageOutputLengthError", {})
// - export const AuthError = NamedError.create("ProviderAuthError", { providerID: Schema.String, message: Schema.String })
// - export const AbortedError = NamedError.create("MessageAbortedError", { message: Schema.String })
// - export const StructuredOutputError = NamedError.create("StructuredOutputError", {
// - export const APIError = NamedError.create("APIError", {
// - export type APIError = Schema.Schema.Type<typeof APIError.Schema>
// - export const ContextOverflowError = NamedError.create("ContextOverflowError", {
// - export const ContentFilterError = NamedError.create("ContentFilterError", { message: Schema.String })

// Full 1:1 behavior preserved — see source `packages/core/src/v1/session.ts` for ordering/defaults.
// TODO: Effect/DB/FS wiring is PROVISIONAL pending crates/database,effect,filesystem — stubs preserve observable error strings/keys.
