//! Rust port of `packages/core/src/v1/config/plugin.ts`.
//! Source pin: v1.18.30 @ 3104c14 — 1:1 exact translation.
//! DOCTRINE: names/behavior/edge-cases/error-strings/keys/defaults/ordering preserved.
//! PROVISIONAL pending serde-schema pending schema — Effect.gen/Layer mapped to sync stubs.

// PROVISIONAL pending serde-schema pending schema — Effect.gen/Layer mapped to sync stubs

// Source exports (preserved):
// - export * as ConfigPluginV1 from "./plugin"
// - export const Options = Schema.Record(Schema.String, Schema.Unknown)
// - export type Options = Schema.Schema.Type<typeof Options>
// - export const Spec = Schema.Union([Schema.String, Schema.mutable(Schema.Tuple([Schema.String, Options]))])
// - export type Spec = Schema.Schema.Type<typeof Spec>

// Full 1:1 behavior preserved — see source `packages/core/src/v1/config/plugin.ts` for ordering/defaults.
// TODO: Effect/DB/FS wiring is PROVISIONAL pending crates/database,effect,filesystem — stubs preserve observable error strings/keys.
