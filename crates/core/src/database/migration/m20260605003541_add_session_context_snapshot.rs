//! Rust port of `packages/core/src/database/migration/20260605003541_add_session_context_snapshot.ts`.
//! Source pin: v1.18.30 @3104c14 — 1:1 SQL verbatim extraction.
//! NOTE: `up_sql(conn)` accepts the connection because the TypeScript migration inspects the
//! live schema at migration time (see `packages/core/src/database/migration.ts` apply/applyOnly).

use rusqlite::Connection;

pub const ID: &str = "20260605003541_add_session_context_snapshot";

pub fn up_sql(_conn: &Connection) -> Result<Vec<String>, String> {
    Ok(vec![
        r#"CREATE TABLE `session_context_epoch` (
          `session_id` text PRIMARY KEY,
          `baseline` text NOT NULL,
          `snapshot` text NOT NULL,
          `baseline_seq` integer NOT NULL,
          `replacement_seq` integer,
          `revision` integer DEFAULT 0 NOT NULL,
          CONSTRAINT `fk_session_context_epoch_session_id_session_id_fk` FOREIGN KEY (`session_id`) REFERENCES `session`(`id`) ON DELETE CASCADE
        );"#.to_string(),
    ])
}
