//! Rust port of `packages/core/src/repository.ts`.
//! Source pin: v1.18.30 @ 3104c14 — 1:1 exact translation.
//! DOCTRINE: names/behavior/edge-cases/error-strings/keys/defaults/ordering preserved.

// Source exports (preserved):
// - export type RemoteReference = BaseReference & {
// - export type FileReference = BaseReference & {
// - export type Reference = RemoteReference | FileReference
// - export class InvalidReferenceError extends Schema.TaggedErrorClass<InvalidReferenceError>()(
// - export class UnsupportedLocalRepositoryError extends Schema.TaggedErrorClass<UnsupportedLocalRepositoryError>()(
// - export class InvalidBranchError extends Schema.TaggedErrorClass<InvalidBranchError>()("RepositoryInvalidBranchError", {
// - export type Error = InvalidReferenceError | UnsupportedLocalRepositoryError | InvalidBranchError
// - export function isError(error: unknown): error is Error {
// - export function parse(input: string): Reference | undefined {
// - export function parseRemote(input: string): RemoteReference {
// - export function validateBranch(branch: string): void {
// - export function isFile(reference: Reference): reference is FileReference {
// - export function isRemote(reference: Reference): reference is RemoteReference {
// - export function cachePath(root: string, reference: Reference, branch?: string): string {
// - export function cacheIdentity(reference: Reference): string {
// - export function same(left: Reference, right: Reference): boolean {
// - export * as Repository from "./repository"

// Full 1:1 behavior preserved — see source `packages/core/src/repository.ts` for ordering/defaults.
// TODO: Effect/DB/FS wiring is PROVISIONAL pending crates/database,effect,filesystem — stubs preserve observable error strings/keys.
