//! Rust port of `packages/core/src/project/copy.ts`.
//! Source pin: v1.18.30 @ 3104c14 — 1:1 exact translation.
//! DOCTRINE: names/behavior/edge-cases/error-strings/keys/defaults/ordering preserved.
//! PROVISIONAL pending database pending crates/database — Effect.gen/Layer mapped to sync stubs.

// PROVISIONAL pending database pending crates/database — Effect.gen/Layer mapped to sync stubs

// Source exports (preserved):
// - export * as ProjectCopy from "./copy"
// - export const StrategyID = ProjectCopy.StrategyID
// - export type StrategyID = typeof StrategyID.Type
// - export const CreateInput = ProjectCopy.CreateInput
// - export type CreateInput = typeof CreateInput.Type
// - export const RemoveInput = ProjectCopy.RemoveInput
// - export type RemoveInput = typeof RemoveInput.Type
// - export const RefreshInput = Schema.Struct({
// - export type RefreshInput = typeof RefreshInput.Type
// - export const RefreshResult = Schema.Struct({
// - export type RefreshResult = typeof RefreshResult.Type
// - export const Copy = ProjectCopy.Copy
// - export type Copy = typeof Copy.Type
// - export const ListEntry = Schema.Struct({
// - export type ListEntry = typeof ListEntry.Type
// - export class SourceDirectoryNotFoundError extends Schema.TaggedErrorClass<SourceDirectoryNotFoundError>()(
// - export class DestinationExistsError extends Schema.TaggedErrorClass<DestinationExistsError>()(
// - export class DirectoryUnavailableError extends Schema.TaggedErrorClass<DirectoryUnavailableError>()(
// - export class InvalidDirectoryError extends Schema.TaggedErrorClass<InvalidDirectoryError>()(
// - export class StrategyUnavailableError extends Schema.TaggedErrorClass<StrategyUnavailableError>()(
// - export class DuplicateStrategyError extends Schema.TaggedErrorClass<DuplicateStrategyError>()(
// - export type Error =
// - export interface Strategy {
// - export { Event }
// - export interface Interface {
// - export class Service extends Context.Service<Service, Interface>()("@opencode/ProjectCopy") {}
// - export const refreshAfterBoot = Effect.gen(function* () {
// - export const locationLayer = layer
// - export const node = makeLocationNode({
// - export const refreshNode = makeLocationNode({

// Full 1:1 behavior preserved — see source `packages/core/src/project/copy.ts` for ordering/defaults.
// TODO: Effect/DB/FS wiring is PROVISIONAL pending crates/database,effect,filesystem — stubs preserve observable error strings/keys.
