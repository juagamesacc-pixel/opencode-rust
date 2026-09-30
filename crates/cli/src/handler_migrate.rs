//! Port of `packages/cli/src/commands/handlers/migrate.ts` (v1.18.30 @3104c14).
//!
//! Source: `export default Runtime.handler(Commands.commands.migrate,
//! (_input) => Effect.log("No migrations to run."))` (5 lines).
//!
//! 1:1 notes: the handler ignores its input and logs exactly
//! `"No migrations to run."` (Effect `Effect.log` -> info-level log line).

/// Port of the migrate handler: returns the exact log line.
pub fn log_message() -> &'static str {
    "No migrations to run."
}
