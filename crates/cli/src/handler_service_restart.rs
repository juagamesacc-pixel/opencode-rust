//! Port of `packages/cli/src/commands/handlers/service/restart.ts`
//! (v1.18.30 @3104c14).
//!
//! Source: `export default Runtime.handler(
//! Commands.commands.service.commands.restart,
//! Effect.fn("cli.service.restart")(function* () {
//!   const daemon = yield* Daemon.Service
//!   yield* daemon.stop()
//!   process.stdout.write((yield* daemon.start()) + EOL)
//! }))` (14 lines). Stop, then print fresh `start()` URL + EOL.

/// Output: fresh `start()` URL with exactly one trailing EOL.
pub fn format_output(url: &str, eol: &str) -> String {
    format!("{url}{eol}")
}
