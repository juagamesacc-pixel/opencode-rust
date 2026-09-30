//! Port of `packages/cli/src/commands/handlers/default.ts` (v1.18.30 @3104c14).
//!
//! Source: `export default Runtime.handler(Commands, Effect.gen(...))`
//! (13 lines): resolve `Daemon.Service`, `yield* daemon.transport()`,
//! dynamic `import("../../tui")`, `yield* runTui(transport)`.
//!
//! 1:1 notes: `$` default handler for the root node; same three steps in the
//! same order (service -> transport -> runTui). Dynamic import becomes the
//! `tui` module reference (lazy boundary preserved via the handler table).
//!
//! File mapping: `default.ts` -> `handler_default.rs` (`default` needs no
//! escape as a file stem, but the `handler_` prefix keeps the flat pilot
//! layout unambiguous; recorded in PORTING_MAP.md).

/// The three steps of the default handler, in source order.
pub const STEPS: [&str; 3] = ["daemon.service", "daemon.transport", "runTui"];
