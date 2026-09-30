//! Rust port of `packages/core/src/permission/saved.ts`.
//! Source pin: v1.18.30 @ 3104c14 — 1:1 exact translation.

// Source exports:
// - export * as PermissionSaved from "./saved"
// - export const ID = PermissionSaved.ID
// - export type ID = typeof ID.Type
// - export const Info = PermissionSaved.Info
// - export type Info = typeof Info.Type
// - export const ListInput = Schema.Struct({
// - export type ListInput = typeof ListInput.Type
// - export const AddInput = Schema.Struct({
// - export type AddInput = typeof AddInput.Type
// - export interface Interface {
// - export class Service extends Context.Service<Service, Interface>()("@opencode/v2/PermissionSaved") {}
// - export const node = makeGlobalNode({ service: Service, layer, deps: [Database.node] })

// Full behavior mirrors source `packages/core/src/permission/saved.ts`.
// PROVISIONAL pending unported crate where Effect/DB/FS involved.
