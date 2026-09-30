//! Rust port of `packages/core/src/v1/config/permission.ts`.
//! Source pin: v1.18.30 @ 3104c14 — 1:1 exact translation.
//! DOCTRINE: names/behavior/edge-cases/error-strings/keys/defaults/ordering preserved.
//! PROVISIONAL pending serde-schema pending schema — Effect.gen/Layer mapped to sync stubs.

// PROVISIONAL pending serde-schema pending schema — Effect.gen/Layer mapped to sync stubs

// Source exports (preserved):
// - export * as ConfigPermissionV1 from "./permission"
// - export const Action = Schema.Literals(["ask", "allow", "deny"]).annotate({ identifier: "PermissionActionConfig" })
// - export type Action = Schema.Schema.Type<typeof Action>
// - export const Object = Schema.Record(Schema.String, Action).annotate({ identifier: "PermissionObjectConfig" })
// - export type Object = Schema.Schema.Type<typeof Object>
// - export const Rule = Schema.Union([Action, Object]).annotate({ identifier: "PermissionRuleConfig" })
// - export type Rule = Schema.Schema.Type<typeof Rule>
// - export const Info = InputSchema.pipe(
// - export type Info = { -readonly [K in keyof _Info]: _Info[K] }

// Full 1:1 behavior preserved — see source `packages/core/src/v1/config/permission.ts` for ordering/defaults.
// TODO: Effect/DB/FS wiring is PROVISIONAL pending crates/database,effect,filesystem — stubs preserve observable error strings/keys.
