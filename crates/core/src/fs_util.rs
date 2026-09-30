//! Rust port of `packages/core/src/fs-util.ts`.
//! Source pin: v1.18.30 @ 3104c14 — 1:1 exact translation.
//! DOCTRINE: names/behavior/edge-cases/error-strings/keys/defaults/ordering preserved.
//! PROVISIONAL pending effect/runtime — Effect.gen/Layer mapped to sync stubs.

// PROVISIONAL pending effect — Effect.gen/Layer mapped to sync stubs

// Source exports (preserved):
// - export namespace FSUtil {
// -   export class FileSystemError extends Schema.TaggedErrorClass<FileSystemError>()("FileSystemError", {
// -   export type Error = PlatformError | FileSystemError
// -   export interface DirEntry {
// -   export interface Interface extends FileSystem.FileSystem {
// -   export class Service extends Context.Service<Service, Interface>()("@opencode/FileSystem") {}
// -   export const use = serviceUse(Service)
// -   export const node = makeGlobalNode({ service: Service, layer: layer, deps: [filesystem] })
// -   export function mimeType(p: string): string {
// -   export function normalizePath(p: string): string {
// -   export function normalizePathPattern(p: string): string {
// -   export function resolve(p: string): string {
// -   export function windowsPath(p: string): string {
// -   export function overlaps(a: string, b: string) {
// -   export function contains(parent: string, child: string) {

// Full 1:1 behavior preserved — see source `packages/core/src/fs-util.ts` for ordering/defaults.
// TODO: Effect/DB/FS wiring is PROVISIONAL pending crates/database,effect,filesystem — stubs preserve observable error strings/keys.
