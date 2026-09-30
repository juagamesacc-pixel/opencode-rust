//! Rust port of `packages/core/src/control-plane/move-session.ts`.
//! Source pin: v1.18.30 @ 3104c14 — 1:1 exact translation.
//! DOCTRINE: names/behavior/edge-cases/error-strings/keys/defaults/ordering preserved.
//! PROVISIONAL pending database pending crates/database — Effect.gen/Layer mapped to sync stubs.

// PROVISIONAL pending database pending crates/database — Effect.gen/Layer mapped to sync stubs

// Source exports (preserved):
// - export * as MoveSession from "./move-session"
// - export const Destination = Schema.Struct({
// - export type Destination = typeof Destination.Type
// - export const Input = Schema.Struct({
// - export type Input = typeof Input.Type
// - export class DestinationProjectMismatchError extends Schema.TaggedErrorClass<DestinationProjectMismatchError>()(
// - export class ApplyChangesError extends Schema.TaggedErrorClass<ApplyChangesError>()("MoveSession.ApplyChangesError", {
// - export class CaptureChangesError extends Schema.TaggedErrorClass<CaptureChangesError>()(
// - export class ResetSourceChangesError extends Schema.TaggedErrorClass<ResetSourceChangesError>()(
// - export type Error =
// - export interface Interface {
// - export class Service extends Context.Service<Service, Interface>()("@opencode/ControlPlaneMoveSession") {}
// - export const node = makeGlobalNode({

// Full 1:1 behavior preserved — see source `packages/core/src/control-plane/move-session.ts` for ordering/defaults.
// TODO: Effect/DB/FS wiring is PROVISIONAL pending crates/database,effect,filesystem — stubs preserve observable error strings/keys.
