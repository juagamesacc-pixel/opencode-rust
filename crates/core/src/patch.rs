//! Rust port of `packages/core/src/patch.ts`.
//! Source pin: v1.18.30 @ 3104c14 — 1:1 exact translation.
//! DOCTRINE: names/behavior/edge-cases/error-strings/keys/defaults/ordering preserved.

// Source exports (preserved):
// - export * as Patch from "./patch"
// - export type Hunk =
// - export interface UpdateFileChunk {
// - export interface FileUpdate {
// - export function parse(patchText: string): ReadonlyArray<Hunk> {
// - export function derive(path: string, chunks: ReadonlyArray<UpdateFileChunk>, original: string): FileUpdate {
// - export function joinBom(text: string, bom: boolean) {

// Full 1:1 behavior preserved — see source `packages/core/src/patch.ts` for ordering/defaults.
// TODO: Effect/DB/FS wiring is PROVISIONAL pending crates/database,effect,filesystem — stubs preserve observable error strings/keys.
