//! Rust port of `packages/core/src/database/migration/20260622142730_simplify_session_context_epoch.ts`.
//! Source pin: v1.18.30 @3104c14 — 1:1 SQL verbatim extraction.
//! NOTE: `up_sql(conn)` accepts the connection because the TypeScript migration inspects the
//! live schema at migration time (see `packages/core/src/database/migration.ts` apply/applyOnly).

use rusqlite::Connection;

pub const ID: &str = "20260622142730_simplify_session_context_epoch";

pub fn up_sql(_conn: &Connection) -> Result<Vec<String>, String> {
    Ok(vec![
        r#"ALTER TABLE `session_context_epoch` DROP COLUMN `agent`;"#.to_string(),
        r#"ALTER TABLE `session_context_epoch` DROP COLUMN `replacement_seq`;"#.to_string(),
        r#"ALTER TABLE `session_context_epoch` DROP COLUMN `revision`;"#.to_string(),
    ])
}
