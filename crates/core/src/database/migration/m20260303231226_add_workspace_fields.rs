//! Rust port of `packages/core/src/database/migration/20260303231226_add_workspace_fields.ts`.
//! Source pin: v1.18.30 @3104c14 — 1:1 SQL verbatim extraction.
//! NOTE: `up_sql(conn)` accepts the connection because the TypeScript migration inspects the
//! live schema at migration time (see `packages/core/src/database/migration.ts` apply/applyOnly).

use rusqlite::Connection;

pub const ID: &str = "20260303231226_add_workspace_fields";

pub fn up_sql(_conn: &Connection) -> Result<Vec<String>, String> {
    Ok(vec![
        r#"ALTER TABLE `workspace` ADD `type` text NOT NULL;"#.to_string(),
        r#"ALTER TABLE `workspace` ADD `name` text;"#.to_string(),
        r#"ALTER TABLE `workspace` ADD `directory` text;"#.to_string(),
        r#"ALTER TABLE `workspace` ADD `extra` text;"#.to_string(),
        r#"ALTER TABLE `workspace` DROP COLUMN `config`;"#.to_string(),
    ])
}
