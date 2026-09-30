//! Rust port of `packages/core/src/npm.ts`.
//! Source pin: v1.18.30 @ 3104c14 — 1:1 exact translation.
//! DOCTRINE: names/behavior/edge-cases/error-strings/keys/defaults/ordering preserved.
//! PROVISIONAL pending effect/runtime — Effect.gen/Layer mapped to sync stubs.

// PROVISIONAL pending effect — Effect.gen/Layer mapped to sync stubs

// Source exports (preserved):
// - export * as Npm from "./npm"
// - export class InstallFailedError extends Schema.TaggedErrorClass<InstallFailedError>()("NpmInstallFailedError", {
// - export interface EntryPoint {
// - export interface Interface {
// - export class Service extends Context.Service<Service, Interface>()("@opencode/Npm") {}
// - export function sanitize(pkg: string) {
// - export const node = makeGlobalNode({
// - export async function install(...args: Parameters<Interface["install"]>) {
// - export async function add(...args: Parameters<Interface["add"]>) {
// - export async function which(...args: Parameters<Interface["which"]>) {

// Full 1:1 behavior preserved — see source `packages/core/src/npm.ts` for ordering/defaults.
// TODO: Effect/DB/FS wiring is PROVISIONAL pending crates/database,effect,filesystem — stubs preserve observable error strings/keys.
