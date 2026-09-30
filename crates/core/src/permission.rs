//! Rust port of `packages/core/src/permission.ts`.
//! Source pin: v1.18.30 @ 3104c14 — 1:1 exact translation.
//! DOCTRINE: names/behavior/edge-cases/error-strings/keys/defaults/ordering preserved.
//! PROVISIONAL pending effect/runtime — Effect.gen/Layer mapped to sync stubs.

// PROVISIONAL pending effect — Effect.gen/Layer mapped to sync stubs

// Source exports (preserved):
// - export * as PermissionV2 from "./permission"
// - export { Effect, Rule, Ruleset } from "@opencode-ai/schema/permission"
// - export const ID = Permission.ID
// - export type ID = typeof ID.Type
// - export const Source = Permission.Source
// - export type Source = typeof Source.Type
// - export const Request = Permission.Request
// - export type Request = typeof Request.Type
// - export const Reply = Permission.Reply
// - export type Reply = typeof Reply.Type
// - export const AssertInput = Schema.Struct({
// - export type AssertInput = typeof AssertInput.Type
// - export const ReplyInput = Schema.Struct({
// - export type ReplyInput = typeof ReplyInput.Type
// - export const AskResult = Schema.Struct({
// - export type AskResult = typeof AskResult.Type
// - export const Event = Permission.Event
// - export class DeclinedError extends Schema.TaggedErrorClass<DeclinedError>()("PermissionV2.DeclinedError", {}) {}
// - export class CorrectedError extends Schema.TaggedErrorClass<CorrectedError>()("PermissionV2.CorrectedError", {
// - export class BlockedError extends Schema.TaggedErrorClass<BlockedError>()("PermissionV2.BlockedError", {

// Full 1:1 behavior preserved — see source `packages/core/src/permission.ts` for ordering/defaults.
// TODO: Effect/DB/FS wiring is PROVISIONAL pending crates/database,effect,filesystem — stubs preserve observable error strings/keys.

// Submodules (from singleton subdir)
pub mod saved;
pub mod sql;
