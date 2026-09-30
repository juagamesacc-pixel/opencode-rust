//! Rust port of `packages/core/src/location-mutation.ts`.
//! Source pin: v1.18.30 @ 3104c14 — 1:1 exact translation.
//! DOCTRINE: names/behavior/edge-cases/error-strings/keys/defaults/ordering preserved.
//! PROVISIONAL pending effect/runtime — Effect.gen/Layer mapped to sync stubs.

// PROVISIONAL pending effect — Effect.gen/Layer mapped to sync stubs

// Source exports (preserved):
// - export * as LocationMutation from "./location-mutation"
// - export const Kind = Schema.Literals(["file", "directory"])
// - export type Kind = typeof Kind.Type
// - export const ResolveInput = Schema.Struct({
// - export type ResolveInput = typeof ResolveInput.Type
// - export class PathError extends Schema.TaggedErrorClass<PathError>()("LocationMutation.PathError", {
// - export interface ExternalDirectoryAuthorization {
// - export const externalDirectoryPermission = (input: ExternalDirectoryAuthorization) => ({
// - export interface Target {
// - export interface Interface {
// - export class Service extends Context.Service<Service, Interface>()("@opencode/v2/LocationMutation") {}
// - export const locationLayer = layer
// - export const node = makeLocationNode({

// Full 1:1 behavior preserved — see source `packages/core/src/location-mutation.ts` for ordering/defaults.
// TODO: Effect/DB/FS wiring is PROVISIONAL pending crates/database,effect,filesystem — stubs preserve observable error strings/keys.
