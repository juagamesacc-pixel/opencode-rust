//! Rust port of `packages/core/src/database/migration/20260611035744_credential.ts`.
//! Source pin: v1.18.30 @3104c14 — 1:1 SQL verbatim extraction.
//! NOTE: `up_sql(conn)` accepts the connection because the TypeScript migration inspects the
//! live schema at migration time (see `packages/core/src/database/migration.ts` apply/applyOnly).

use rusqlite::Connection;

pub const ID: &str = "20260611035744_credential";

pub fn up_sql(_conn: &Connection) -> Result<Vec<String>, String> {
    Ok(vec![
        r#"CREATE TABLE `credential` (
          `id` text PRIMARY KEY,
          `connector_id` text NOT NULL,
          `method_id` text NOT NULL,
          `label` text NOT NULL,
          `value` text NOT NULL,
          `active` integer DEFAULT false NOT NULL,
          `time_created` integer NOT NULL,
          `time_updated` integer NOT NULL
        );"#.to_string(),
        r#"CREATE UNIQUE INDEX `credential_connector_active_idx` ON `credential` (`connector_id`) WHERE "credential"."active" = 1;"#.to_string(),
    ])
}
