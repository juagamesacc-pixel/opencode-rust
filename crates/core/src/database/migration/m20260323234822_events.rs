//! Rust port of `packages/core/src/database/migration/20260323234822_events.ts`.
//! Source pin: v1.18.30 @3104c14 — 1:1 SQL verbatim extraction.
//! NOTE: `up_sql(conn)` accepts the connection because the TypeScript migration inspects the
//! live schema at migration time (see `packages/core/src/database/migration.ts` apply/applyOnly).

use rusqlite::Connection;

pub const ID: &str = "20260323234822_events";

pub fn up_sql(_conn: &Connection) -> Result<Vec<String>, String> {
    Ok(vec![
        r#"CREATE TABLE `event_sequence` (
          `aggregate_id` text PRIMARY KEY,
          `seq` integer NOT NULL
        );"#.to_string(),
        r#"CREATE TABLE `event` (
          `id` text PRIMARY KEY,
          `aggregate_id` text NOT NULL,
          `seq` integer NOT NULL,
          `type` text NOT NULL,
          `data` text NOT NULL,
          CONSTRAINT `fk_event_aggregate_id_event_sequence_aggregate_id_fk` FOREIGN KEY (`aggregate_id`) REFERENCES `event_sequence`(`aggregate_id`) ON DELETE CASCADE
        );"#.to_string(),
    ])
}
