//! Rust port of `packages/core/src/database/migration/20260602182828_add_project_directories.ts`.
//! Source pin: v1.18.30 @3104c14 — 1:1 SQL verbatim extraction.
//! NOTE: `up_sql(conn)` accepts the connection because the TypeScript migration inspects the
//! live schema at migration time (see `packages/core/src/database/migration.ts` apply/applyOnly).

use rusqlite::Connection;

pub const ID: &str = "20260602182828_add_project_directories";

pub fn up_sql(_conn: &Connection) -> Result<Vec<String>, String> {
    Ok(vec![
        r#"CREATE TABLE `project_directory` (
          `project_id` text NOT NULL,
          `directory` text NOT NULL,
          `type` text NOT NULL,
          `time_created` integer NOT NULL,
          CONSTRAINT `project_directory_pk` PRIMARY KEY(`project_id`, `directory`),
          CONSTRAINT `fk_project_directory_project_id_project_id_fk` FOREIGN KEY (`project_id`) REFERENCES `project`(`id`) ON DELETE CASCADE
        );"#.to_string(),
    ])
}
