//! Rust port of `packages/core/src/database/migration/20260511000411_data_migration_state.ts`.
//! Source pin: v1.18.30 @3104c14 — 1:1 SQL verbatim extraction.
//! NOTE: `up_sql(conn)` accepts the connection because the TypeScript migration inspects the
//! live schema at migration time (see `packages/core/src/database/migration.ts` apply/applyOnly).

use rusqlite::Connection;

pub const ID: &str = "20260511000411_data_migration_state";

pub fn up_sql(_conn: &Connection) -> Result<Vec<String>, String> {
    Ok(vec![r#"CREATE TABLE `data_migration` (
          `name` text PRIMARY KEY,
          `time_completed` integer NOT NULL
        );"#
    .to_string()])
}
