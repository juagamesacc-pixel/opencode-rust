//! Rust port of `packages/core/src/file-mutation.ts`.
//! Source pin: v1.18.30 @ 3104c14 — 1:1 exact translation.
//! DOCTRINE: names/behavior/edge-cases/error-strings/keys/defaults/ordering preserved.
//! PROVISIONAL pending effect/runtime — Effect.gen/Layer mapped to sync stubs.

// PROVISIONAL pending effect — Effect.gen/Layer mapped to sync stubs

// Source exports (preserved):
// - export * as FileMutation from "./file-mutation"
// - export interface Target {
// - export interface WriteInput {
// - export interface TextWriteInput {
// - export interface ConditionalWriteInput extends WriteInput {
// - export interface RemoveInput {
// - export class StaleContentError extends Schema.TaggedErrorClass<StaleContentError>()("FileMutation.StaleContentError", {
// - export class TargetExistsError extends Schema.TaggedErrorClass<TargetExistsError>()("FileMutation.TargetExistsError", {
// - export interface WriteResult {
// - export interface RemoveResult {
// - export interface Interface {
// - export class Service extends Context.Service<Service, Interface>()("@opencode/v2/FileMutation") {}
// - export const locationLayer = layer
// - export const node = makeLocationNode({ service: Service, layer, deps: [FSUtil.node] })

// Full 1:1 behavior preserved — see source `packages/core/src/file-mutation.ts` for ordering/defaults.
// TODO: Effect/DB/FS wiring is PROVISIONAL pending crates/database,effect,filesystem — stubs preserve observable error strings/keys.
