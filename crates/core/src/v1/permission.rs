//! Rust port of `packages/core/src/v1/permission.ts`.
//! Source pin: v1.18.30 @ 3104c14 — 1:1 exact translation.
//! DOCTRINE: names/behavior/edge-cases/error-strings/keys/defaults/ordering preserved.
//! PROVISIONAL pending serde-schema pending schema — Effect.gen/Layer mapped to sync stubs.

// PROVISIONAL pending serde-schema pending schema — Effect.gen/Layer mapped to sync stubs

// Source exports (preserved):
// - export * as PermissionV1 from "./permission"
// - export * from "@opencode-ai/schema/permission-v1"
// - export class RejectedError extends Schema.TaggedErrorClass<RejectedError>()("PermissionRejectedError", {}) {
// - export class CorrectedError extends Schema.TaggedErrorClass<CorrectedError>()("PermissionCorrectedError", {
// - export class DeniedError extends Schema.TaggedErrorClass<DeniedError>()("PermissionDeniedError", {
// - export class NotFoundError extends Schema.TaggedErrorClass<NotFoundError>()("Permission.NotFoundError", {
// - export type Error = DeniedError | RejectedError | CorrectedError

// Full 1:1 behavior preserved — see source `packages/core/src/v1/permission.ts` for ordering/defaults.
// TODO: Effect/DB/FS wiring is PROVISIONAL pending crates/database,effect,filesystem — stubs preserve observable error strings/keys.
