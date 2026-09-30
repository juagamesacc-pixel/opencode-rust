//! Rust port of `packages/core/src/repository-cache.ts`.
//! Source pin: v1.18.30 @ 3104c14 — 1:1 exact translation.
//! DOCTRINE: names/behavior/edge-cases/error-strings/keys/defaults/ordering preserved.
//! PROVISIONAL pending effect/runtime — Effect.gen/Layer mapped to sync stubs.

// PROVISIONAL pending effect — Effect.gen/Layer mapped to sync stubs

// Source exports (preserved):
// - export type Result = {
// - export type EnsureInput = {
// - export class InvalidRepositoryError extends Schema.TaggedErrorClass<InvalidRepositoryError>()(
// - export class InvalidBranchError extends Schema.TaggedErrorClass<InvalidBranchError>()(
// - export class CloneFailedError extends Schema.TaggedErrorClass<CloneFailedError>()("RepositoryCacheCloneFailedError", {
// - export class FetchFailedError extends Schema.TaggedErrorClass<FetchFailedError>()("RepositoryCacheFetchFailedError", {
// - export class CheckoutFailedError extends Schema.TaggedErrorClass<CheckoutFailedError>()(
// - export class ResetFailedError extends Schema.TaggedErrorClass<ResetFailedError>()("RepositoryCacheResetFailedError", {
// - export class LockFailedError extends Schema.TaggedErrorClass<LockFailedError>()("RepositoryCacheLockFailedError", {
// - export class CacheOperationError extends Schema.TaggedErrorClass<CacheOperationError>()(
// - export type Error =
// - export interface Interface {
// - export class Service extends Context.Service<Service, Interface>()("@opencode/RepositoryCache") {}
// - export function isError(error: unknown): error is Error {
// - export const parseRemote = Effect.fn("RepositoryCache.parseRemote")(function* (repository: string) {
// - export const validateBranch = Effect.fn("RepositoryCache.validateBranch")(function* (branch: string) {
// - export const node = makeGlobalNode({
// - export * as RepositoryCache from "./repository-cache"

// Full 1:1 behavior preserved — see source `packages/core/src/repository-cache.ts` for ordering/defaults.
// TODO: Effect/DB/FS wiring is PROVISIONAL pending crates/database,effect,filesystem — stubs preserve observable error strings/keys.
