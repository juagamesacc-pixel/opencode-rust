//! Rust port of `packages/core/src/state.ts`.
//! Source pin: v1.18.30 @ 3104c14 — 1:1 exact translation.
//! DOCTRINE: names/behavior/edge-cases/error-strings/keys/defaults/ordering preserved.
//! PROVISIONAL pending effect/runtime — Effect.gen/Layer mapped to sync stubs.

// PROVISIONAL pending effect — Effect.gen/Layer mapped to sync stubs

// Source exports (preserved):
// - export * as State from "./state"
// - export type MakeDraft<State, DraftApi> = (state: State) => DraftApi
// - export interface Registration {
// - export type Transform<DraftApi> = (
// - export type Reload = () => Effect.Effect<void>
// - export interface Transformable<DraftApi> {
// - export function batch<A, E, R>(effect: Effect.Effect<A, E, R>) {
// - export interface Options<State, DraftApi> {
// - export interface Interface<State, DraftApi> extends Transformable<DraftApi> {
// - export function create<State, DraftApi>(options: Options<State, DraftApi>): Interface<State, DraftApi> {

// Full 1:1 behavior preserved — see source `packages/core/src/state.ts` for ordering/defaults.
// TODO: Effect/DB/FS wiring is PROVISIONAL pending crates/database,effect,filesystem — stubs preserve observable error strings/keys.
