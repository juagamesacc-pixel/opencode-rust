//! Rust port of `packages/core/src/database/migration/20260611192811_lush_chimera.ts`.
//! Source pin: v1.18.30 @3104c14 — 1:1 SQL verbatim extraction.
//! NOTE: `up_sql(conn)` accepts the connection because the TypeScript migration inspects the
//! live schema at migration time (see `packages/core/src/database/migration.ts` apply/applyOnly).

use rusqlite::Connection;

pub const ID: &str = "20260611192811_lush_chimera";

pub fn up_sql(_conn: &Connection) -> Result<Vec<String>, String> {
    Ok(vec![
        r#"DROP INDEX IF EXISTS `credential_connector_active_idx`;"#.to_string(),
        r#"DROP TABLE `credential`;"#.to_string(),
        r#"CREATE TABLE `credential` (
          `id` text PRIMARY KEY,
          `integration_id` text,
          `label` text NOT NULL,
          `value` text NOT NULL,
          `connector_id` text,
          `method_id` text,
          `active` integer,
          `time_created` integer NOT NULL,
          `time_updated` integer NOT NULL
        );"#
        .to_string(),
    ])
}
