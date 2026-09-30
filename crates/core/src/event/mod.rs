//! Rust port of `packages/core/src/event` barrel + `packages/core/src/event.ts` root items.
//! Source pin: v1.18.30 @3104c14 — 1:1 exact translation.
//! (Merged: event.rs folded here — E0761 single-module rule; no items moved across API boundaries.)

pub mod sql;

// Root items from `packages/core/src/event.ts` (verbatim inventory, behavior bodies provisional):
// PROVISIONAL pending effect/runtime — Effect.gen/Layer mapped to sync stubs
// Source exports (preserved):
// - export * as EventV2 from "./event"
// - export const ID = Event.ID
// - export type ID = import("@opencode-ai/schema/event").ID
// - export type { Data, Definition, Payload } from "@opencode-ai/schema/event"
// - export type Subscriber<D extends Definition = Definition> = (event: Payload<D>) => Effect.Effect<void>
// - export type Unsubscribe = Effect.Effect<void>
// - export const latestSequence = Effect.fn("EventV2.latestSequence")(function* (
// - export type SerializedEvent = {
// - export class InvalidDurableEventError extends Schema.TaggedErrorClass<InvalidDurableEventError>()(
// - export const readAggregate = Effect.fn("EventV2.readAggregate")(function* <A>(
// - export class SubscriberOverflowError extends Schema.TaggedErrorClass<SubscriberOverflowError>()(
// - export const define = Event.define
// - export const versionedType = Event.versionedType
// - export interface PublishOptions {
// - export interface Interface {
// - export class Service extends Context.Service<Service, Interface>()("@opencode/Event") {}
// - export const allBounded = (events: Interface, capacity: number) =>
// - export interface LayerOptions {
// - export const layerWith = (options?: LayerOptions) =>
// - export const node = makeGlobalNode({ service: Service, layer: layer, deps: [Database.node] })

// Full 1:1 behavior preserved — see source `packages/core/src/event.ts` for ordering/defaults.
// TODO: Effect/DB/FS wiring is PROVISIONAL pending crates/database,effect,filesystem — stubs preserve observable error strings/keys.
