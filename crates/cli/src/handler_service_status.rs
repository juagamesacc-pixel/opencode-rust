//! Port of `packages/cli/src/commands/handlers/service/status.ts`
//! (v1.18.30 @3104c14).
//!
//! Source: `export default Runtime.handler(
//! Commands.commands.service.commands.status,
//! Effect.fn("cli.service.status")(function* () {
//!   const url = yield* (yield* Daemon.Service).status()
//!   process.stdout.write((url ? `running ${url}` : "stopped") + EOL)
//! }))` (13 lines).
//!
//! 1:1 notes: `"running {url}"` when Some, `"stopped"` when None, + EOL.

/// Port of the status line (line 11, verbatim strings).
pub fn format_output(url: Option<&str>, eol: &str) -> String {
    match url {
        Some(u) => format!("running {u}{eol}"),
        None => format!("stopped{eol}"),
    }
}
