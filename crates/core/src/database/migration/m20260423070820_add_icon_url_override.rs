//! Rust port of `packages/core/src/database/migration/20260423070820_add_icon_url_override.ts`.
//! Source pin: v1.18.30 @3104c14 — 1:1 SQL verbatim extraction.
//! NOTE: `up_sql(conn)` accepts the connection because the TypeScript migration inspects the
//! live schema at migration time (see `packages/core/src/database/migration.ts` apply/applyOnly).

use rusqlite::Connection;

pub const ID: &str = "20260423070820_add_icon_url_override";

pub fn up_sql(_conn: &Connection) -> Result<Vec<String>, String> {
    Ok(vec![r#"ALTER TABLE `project` ADD `icon_url_override` text;
        UPDATE `project` SET `icon_url_override` = `icon_url` WHERE `icon_url` IS NOT NULL;"#
        .to_string()])
}
