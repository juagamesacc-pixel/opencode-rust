//! Rust port of `packages/core/src/database/migration/20260603001617_session_message_projection_indexes.ts`.
//! Source pin: v1.18.30 @3104c14 — 1:1 SQL verbatim extraction.
//! NOTE: `up_sql(conn)` accepts the connection because the TypeScript migration inspects the
//! live schema at migration time (see `packages/core/src/database/migration.ts` apply/applyOnly).

use rusqlite::Connection;

pub const ID: &str = "20260603001617_session_message_projection_indexes";

pub fn up_sql(_conn: &Connection) -> Result<Vec<String>, String> {
    Ok(vec![
        r#"DROP INDEX IF EXISTS `session_message_session_idx`;"#.to_string(),
        r#"DROP INDEX IF EXISTS `session_message_session_type_idx`;"#.to_string(),
        r#"CREATE INDEX `event_aggregate_seq_idx` ON `event` (`aggregate_id`,`seq`);"#.to_string(),
        r#"CREATE INDEX `session_message_session_time_created_id_idx` ON `session_message` (`session_id`,`time_created`,`id`);"#.to_string(),
        r#"CREATE INDEX `session_message_session_type_time_created_id_idx` ON `session_message` (`session_id`,`type`,`time_created`,`id`);"#.to_string(),
    ])
}
