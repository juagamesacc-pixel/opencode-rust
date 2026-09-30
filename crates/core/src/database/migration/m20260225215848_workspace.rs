//! Rust port of `packages/core/src/database/migration/20260225215848_workspace.ts`.
//! Source pin: v1.18.30 @3104c14 — 1:1 SQL verbatim extraction.
//! NOTE: `up_sql(conn)` accepts the connection because the TypeScript migration inspects the
//! live schema at migration time (see `packages/core/src/database/migration.ts` apply/applyOnly).

use rusqlite::Connection;

pub const ID: &str = "20260225215848_workspace";

pub fn up_sql(_conn: &Connection) -> Result<Vec<String>, String> {
    Ok(vec![
        r#"CREATE TABLE `workspace` (
          `id` text PRIMARY KEY,
          `branch` text,
          `project_id` text NOT NULL,
          `config` text NOT NULL,
          CONSTRAINT `fk_workspace_project_id_project_id_fk` FOREIGN KEY (`project_id`) REFERENCES `project`(`id`) ON DELETE CASCADE
        );"#.to_string(),
    ])
}
