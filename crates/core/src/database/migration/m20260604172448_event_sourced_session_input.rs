//! Rust port of `packages/core/src/database/migration/20260604172448_event_sourced_session_input.ts`.
//! Source pin: v1.18.30 @3104c14 — 1:1 SQL verbatim extraction.
//! NOTE: `up_sql(conn)` accepts the connection because the TypeScript migration inspects the
//! live schema at migration time (see `packages/core/src/database/migration.ts` apply/applyOnly).

use rusqlite::Connection;

pub const ID: &str = "20260604172448_event_sourced_session_input";

pub fn up_sql(_conn: &Connection) -> Result<Vec<String>, String> {
    Ok(vec![
        r#"DELETE FROM `session_input`;"#.to_string(),
        r#"DELETE FROM `session_message`;"#.to_string(),
        r#"DELETE FROM `event`;"#.to_string(),
        r#"DELETE FROM `event_sequence`;"#.to_string(),
        r#"UPDATE `session` SET `workspace_id` = NULL;"#.to_string(),
        r#"DELETE FROM `workspace`;"#.to_string(),
        r#"DROP INDEX IF EXISTS `event_aggregate_seq_idx`;"#.to_string(),
        r#"CREATE UNIQUE INDEX `event_aggregate_seq_idx` ON `event` (`aggregate_id`,`seq`);"#.to_string(),
        r#"DROP INDEX IF EXISTS `session_message_session_seq_idx`;"#.to_string(),
        r#"CREATE UNIQUE INDEX `session_message_session_seq_idx` ON `session_message` (`session_id`,`seq`);"#.to_string(),
        r#"PRAGMA foreign_keys=OFF;"#.to_string(),
        r#"CREATE TABLE `__new_session_input` (
          `id` text PRIMARY KEY,
          `session_id` text NOT NULL,
          `prompt` text NOT NULL,
          `delivery` text NOT NULL,
          `admitted_seq` integer NOT NULL,
          `promoted_seq` integer,
          `time_created` integer NOT NULL,
          CONSTRAINT `fk_session_input_session_id_session_id_fk` FOREIGN KEY (`session_id`) REFERENCES `session`(`id`) ON DELETE CASCADE
        );"#.to_string(),
        r#"DROP TABLE `session_input`;"#.to_string(),
        r#"ALTER TABLE `__new_session_input` RENAME TO `session_input`;"#.to_string(),
        r#"PRAGMA foreign_keys=ON;"#.to_string(),
        r#"CREATE INDEX `session_input_session_pending_delivery_seq_idx` ON `session_input` (`session_id`,`promoted_seq`,`delivery`,`admitted_seq`);"#.to_string(),
        r#"CREATE UNIQUE INDEX `session_input_session_admitted_seq_idx` ON `session_input` (`session_id`,`admitted_seq`);"#.to_string(),
        r#"CREATE UNIQUE INDEX `session_input_session_promoted_seq_idx` ON `session_input` (`session_id`,`promoted_seq`);"#.to_string(),
    ])
}
