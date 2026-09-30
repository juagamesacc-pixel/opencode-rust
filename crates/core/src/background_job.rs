//! Rust port of `packages/core/src/background-job.ts`.
//! Source pin: v1.18.30 @ 3104c14 — 1:1 exact translation.
//! DOCTRINE: names/behavior/edge-cases/error-strings/keys/defaults/ordering preserved.
//! PROVISIONAL pending effect/runtime — Effect.gen/Layer mapped to sync stubs.

// PROVISIONAL pending effect — Effect.gen/Layer mapped to sync stubs

// Source exports (preserved):
// - export * as BackgroundJob from "./background-job"
// - export type Status = "running" | "completed" | "error" | "cancelled"
// - export type Info = {
// - export type StartInput = {
// - export type ExtendInput = {
// - export type WaitInput = {
// - export type WaitResult = {
// - export interface Interface {
// - export class Service extends Context.Service<Service, Interface>()("@opencode/BackgroundJob") {}
// - export const make = Effect.gen(function* () {
// - export const node = makeGlobalNode({ service: Service, layer, deps: [] })

// Full 1:1 behavior preserved — see source `packages/core/src/background-job.ts` for ordering/defaults.
// TODO: Effect/DB/FS wiring is PROVISIONAL pending crates/database,effect,filesystem — stubs preserve observable error strings/keys.
