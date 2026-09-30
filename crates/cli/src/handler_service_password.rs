//! Port of `packages/cli/src/commands/handlers/service/password.ts`
//! (v1.18.30 @3104c14).
//!
//! Source: `export default Runtime.handler(
//! Commands.commands.service.commands.password,
//! Effect.fn("cli.service.password")(function* (input) {
//!   const daemon = yield* Daemon.Service
//!   const value = Option.getOrUndefined(input.value)
//!   if (value !== undefined) yield* daemon.stop()
//!   process.stdout.write((yield* daemon.password(value)) + EOL)
//! }))` (16 lines).
//!
//! 1:1 notes: optional positional `value`; when a value is given the daemon
//! is stopped BEFORE (re)setting the password (rotation semantics);
//! prints the resulting password + EOL.

/// Whether the handler stops the daemon first (source line 13).
pub fn should_stop_first(value: Option<&str>) -> bool {
    value.is_some()
}

/// Output: resulting password with exactly one trailing EOL.
pub fn format_output(password: &str, eol: &str) -> String {
    format!("{password}{eol}")
}
