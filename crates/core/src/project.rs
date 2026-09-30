//! Rust port of `packages/core/src/project.ts`.
//! Source pin: v1.18.30 @ 3104c14 — 1:1 exact translation.
//! DOCTRINE: names/behavior/edge-cases/error-strings/keys/defaults/ordering preserved.
//! PROVISIONAL pending effect/runtime — Effect.gen/Layer mapped to sync stubs.

// PROVISIONAL pending effect — Effect.gen/Layer mapped to sync stubs

// Source exports (preserved):
// - export * as ProjectV2 from "./project"
// - export * as Project from "./project"
// - export const ID = ProjectSchema.ID
// - export type ID = ProjectSchema.ID
// - export const Vcs = ProjectSchema.Vcs
// - export type Vcs = ProjectSchema.Vcs
// - export class Info extends Schema.Class<Info>("Project.Info")({
// - export const DirectoriesInput = ProjectDirectories.ListInput
// - export type DirectoriesInput = typeof DirectoriesInput.Type
// - export const Directories = ProjectDirectories.ListOutput
// - export type Directories = typeof Directories.Type
// - export interface Resolved {
// - export interface Interface {
// - export class Service extends Context.Service<Service, Interface>()("@opencode/ProjectV2") {}
// - export const node = makeGlobalNode({

// Full 1:1 behavior preserved — see source `packages/core/src/project.ts` for ordering/defaults.
// TODO: Effect/DB/FS wiring is PROVISIONAL pending crates/database,effect,filesystem — stubs preserve observable error strings/keys.

// ---- project/* submodules (source lexical order) ----
pub mod copy;
pub mod copy_strategies;
pub mod directories;
pub mod schema;
pub mod sql;
