//! Rust port of `packages/core/src/database/migration/20260511173437_session-metadata.ts`.
//! Source pin: v1.18.30 @3104c14 — 1:1 SQL verbatim extraction.
//! NOTE: `up_sql(conn)` accepts the connection because the TypeScript migration inspects the
//! live schema at migration time (see `packages/core/src/database/migration.ts` apply/applyOnly).

use rusqlite::Connection;

pub const ID: &str = "20260511173437_session-metadata";

pub fn up_sql(conn: &Connection) -> Result<Vec<String>, String> {
    // source verbatim: this column briefly shipped again under
    // 20260530232709_lovely_romulus — skip when already present.
    let mut stmt = conn
        .prepare("PRAGMA table_info(`session`)")
        .map_err(|e| e.to_string())?;
    let columns = stmt
        .query_map([], |row| row.get::<_, String>(1))
        .map_err(|e| e.to_string())?;
    for column in columns {
        if column.map_err(|e| e.to_string())? == "metadata" {
            return Ok(vec![]);
        }
    }
    Ok(vec![
        r#"ALTER TABLE `session` ADD `metadata` text;"#.to_string()
    ])
}
