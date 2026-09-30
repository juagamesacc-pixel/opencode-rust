//! Rust port of `packages/core/src/database/migration/20260603141458_session_input_inbox.ts`.
//! Source pin: v1.18.30 @3104c14 — 1:1 SQL verbatim extraction.
//! NOTE: `up_sql(conn)` accepts the connection because the TypeScript migration inspects the
//! live schema at migration time (see `packages/core/src/database/migration.ts` apply/applyOnly).

use rusqlite::Connection;

pub const ID: &str = "20260603141458_session_input_inbox";

pub fn up_sql(_conn: &Connection) -> Result<Vec<String>, String> {
    Ok(vec![
        r#"CREATE TABLE `session_input` (
          `seq` integer PRIMARY KEY AUTOINCREMENT,
          `id` text NOT NULL UNIQUE,
          `session_id` text NOT NULL,
          `prompt` text NOT NULL,
          `delivery` text NOT NULL,
          `promoted_seq` integer,
          `time_created` integer NOT NULL,
          CONSTRAINT `fk_session_input_session_id_session_id_fk` FOREIGN KEY (`session_id`) REFERENCES `session`(`id`) ON DELETE CASCADE
        );"#.to_string(),
        r#"CREATE INDEX `session_input_session_pending_seq_idx` ON `session_input` (`session_id`,`promoted_seq`,`seq`);"#.to_string(),
    ])
}
