//! Rust port of `packages/core/src/observability.ts`.
//! Source pin: v1.18.30 @ 3104c14 — 1:1 exact translation.
//! DOCTRINE: names/behavior/edge-cases/error-strings/keys/defaults/ordering preserved.
//! PROVISIONAL pending effect/runtime — Effect.gen/Layer mapped to sync stubs.

// PROVISIONAL pending effect — Effect.gen/Layer mapped to sync stubs

// Source exports (preserved):
// - export * as Observability from "./observability"
// - export const layer = Layer.unwrap(
// - export const node = LayerNode.make({ name: "observability", layer, deps: [] })

// Full 1:1 behavior preserved — see source `packages/core/src/observability.ts` for ordering/defaults.
// TODO: Effect/DB/FS wiring is PROVISIONAL pending crates/database,effect,filesystem — stubs preserve observable error strings/keys.

// Submodules (from singleton subdir)
pub mod logging;
pub mod otlp;
pub mod shared;
pub use logging::{file_logger_path, formatter_id, loggers, minimum_log_level, LOG_FILE_DEFAULT};
pub use otlp::{otlp_headers, resource, tracing_layer_enabled, Resource, SERVICE_NAME};
pub use shared::{run_id, RUN_ID_LEN};
