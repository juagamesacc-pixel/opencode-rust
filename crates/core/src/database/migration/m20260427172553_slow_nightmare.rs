//! Rust port of `packages/core/src/database/migration/20260427172553_slow_nightmare.ts`.
//! Source pin: v1.18.30 @3104c14 — 1:1 SQL verbatim extraction.
//! NOTE: `up_sql(conn)` accepts the connection because the TypeScript migration inspects the
//! live schema at migration time (see `packages/core/src/database/migration.ts` apply/applyOnly).

use rusqlite::Connection;

pub const ID: &str = "20260427172553_slow_nightmare";

pub fn up_sql(_conn: &Connection) -> Result<Vec<String>, String> {
    Ok(vec![
        r#"CREATE TABLE `session_message` (
          `id` text PRIMARY KEY,
          `session_id` text NOT NULL,
          `type` text NOT NULL,
          `time_created` integer NOT NULL,
          `time_updated` integer NOT NULL,
          `data` text NOT NULL,
          CONSTRAINT `fk_session_message_session_id_session_id_fk` FOREIGN KEY (`session_id`) REFERENCES `session`(`id`) ON DELETE CASCADE
        );"#.to_string(),
        r#"DROP INDEX IF EXISTS `session_entry_session_idx`;"#.to_string(),
        r#"DROP INDEX IF EXISTS `session_entry_session_type_idx`;"#.to_string(),
        r#"DROP INDEX IF EXISTS `session_entry_time_created_idx`;"#.to_string(),
        r#"CREATE INDEX `session_message_session_idx` ON `session_message` (`session_id`);"#.to_string(),
        r#"CREATE INDEX `session_message_session_type_idx` ON `session_message` (`session_id`,`type`);"#.to_string(),
        r#"CREATE INDEX `session_message_time_created_idx` ON `session_message` (`time_created`);"#.to_string(),
        r#"DROP TABLE `session_entry`;"#.to_string(),
    ])
}
