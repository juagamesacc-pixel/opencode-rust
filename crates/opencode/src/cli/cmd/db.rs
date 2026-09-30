// source: src/cli/cmd/db.ts — exports: [DbCommand]
// PROVISIONAL pending external `yargs` (host-provided; no new dep)
// PROVISIONAL pending crates/core: `@opencode-ai/core/database/database`
// PROVISIONAL pending external `effect` (host-provided; no new dep)
/// verbatim strings (source order, quoted for V2 audit):
/// - "$0 [query]"
/// - "open an interactive sqlite3 shell or run a query"
/// - "SQL query to execute"
/// - "Output format"
/// - "Cli.db.query"
/// - "print the database path"
/// - "Cli.db.path"
/// - "database tools"
/// source: `export const DbCommand` — shape as JSON value; CI verifies.
pub type DbCommand = serde_json::Value;
