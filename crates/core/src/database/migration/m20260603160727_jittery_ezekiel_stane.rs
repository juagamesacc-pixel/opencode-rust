//! Rust port of `packages/core/src/database/migration/20260603160727_jittery_ezekiel_stane.ts`.
//! Source pin: v1.18.30 @3104c14 — 1:1 SQL verbatim extraction.
//! NOTE: `up_sql(conn)` accepts the connection because the TypeScript migration inspects the
//! live schema at migration time (see `packages/core/src/database/migration.ts` apply/applyOnly).

use rusqlite::Connection;

pub const ID: &str = "20260603160727_jittery_ezekiel_stane";

pub fn up_sql(_conn: &Connection) -> Result<Vec<String>, String> {
    Ok(vec![
        r#"DROP INDEX IF EXISTS `session_input_session_pending_seq_idx`;"#.to_string(),
        r#"CREATE INDEX IF NOT EXISTS `event_aggregate_type_seq_idx` ON `event` (`aggregate_id`,`type`,`seq`);"#.to_string(),
        r#"CREATE INDEX IF NOT EXISTS `session_input_session_pending_delivery_seq_idx` ON `session_input` (`session_id`,`promoted_seq`,`delivery`,`seq`);"#.to_string(),
        r#"CREATE INDEX IF NOT EXISTS `session_message_session_time_created_id_idx` ON `session_message` (`session_id`,`time_created`,`id`);"#.to_string(),
    ])
}
