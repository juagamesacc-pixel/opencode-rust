//! Port of `packages/cli/src/commands/handlers/service/start.ts`
//! (v1.18.30 @3104c14).
//!
//! Source: `export default Runtime.handler(
//! Commands.commands.service.commands.start,
//! Effect.fn("cli.service.start")(function* () {
//!   process.stdout.write((yield* (yield* Daemon.Service).start()) + EOL)
//! }))` (12 lines). Prints `daemon.start()` URL + EOL.

/// Output: `start()` URL with exactly one trailing EOL.
pub fn format_output(url: &str, eol: &str) -> String {
    format!("{url}{eol}")
}
