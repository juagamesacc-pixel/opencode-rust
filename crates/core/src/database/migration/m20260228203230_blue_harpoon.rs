//! Rust port of `packages/core/src/database/migration/20260228203230_blue_harpoon.ts`.
//! Source pin: v1.18.30 @3104c14 — 1:1 SQL verbatim extraction.
//! NOTE: `up_sql(conn)` accepts the connection because the TypeScript migration inspects the
//! live schema at migration time (see `packages/core/src/database/migration.ts` apply/applyOnly).

use rusqlite::Connection;

pub const ID: &str = "20260228203230_blue_harpoon";

pub fn up_sql(_conn: &Connection) -> Result<Vec<String>, String> {
    Ok(vec![
        r#"CREATE TABLE `account` (
          `id` text PRIMARY KEY,
          `email` text NOT NULL,
          `url` text NOT NULL,
          `access_token` text NOT NULL,
          `refresh_token` text NOT NULL,
          `token_expiry` integer,
          `selected_org_id` text,
          `time_created` integer NOT NULL,
          `time_updated` integer NOT NULL
        );"#.to_string(),
        r#"CREATE TABLE `account_state` (
          `id` integer PRIMARY KEY NOT NULL,
          `active_account_id` text,
          FOREIGN KEY (`active_account_id`) REFERENCES `account`(`id`) ON UPDATE no action ON DELETE set null
        );"#.to_string(),
    ])
}
