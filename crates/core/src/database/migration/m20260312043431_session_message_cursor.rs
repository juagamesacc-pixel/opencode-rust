//! Rust port of `packages/core/src/database/migration/20260312043431_session_message_cursor.ts`.
//! Source pin: v1.18.30 @3104c14 — 1:1 SQL verbatim extraction.
//! NOTE: `up_sql(conn)` accepts the connection because the TypeScript migration inspects the
//! live schema at migration time (see `packages/core/src/database/migration.ts` apply/applyOnly).

use rusqlite::Connection;

pub const ID: &str = "20260312043431_session_message_cursor";

pub fn up_sql(_conn: &Connection) -> Result<Vec<String>, String> {
    Ok(vec![
        r#"DROP INDEX IF EXISTS `message_session_idx`;"#.to_string(),
        r#"DROP INDEX IF EXISTS `part_message_idx`;"#.to_string(),
        r#"CREATE INDEX `message_session_time_created_id_idx` ON `message` (`session_id`,`time_created`,`id`);"#.to_string(),
        r#"CREATE INDEX `part_message_id_id_idx` ON `part` (`message_id`,`id`);"#.to_string(),
    ])
}
