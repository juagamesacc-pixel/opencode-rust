//! Rust port of `packages/core/src/database/migration/20260612174303_project_dir_strategy.ts`.
//! Source pin: v1.18.30 @3104c14 — 1:1 SQL verbatim extraction.
//! NOTE: `up_sql(conn)` accepts the connection because the TypeScript migration inspects the
//! live schema at migration time (see `packages/core/src/database/migration.ts` apply/applyOnly).

use rusqlite::Connection;

pub const ID: &str = "20260612174303_project_dir_strategy";

pub fn up_sql(_conn: &Connection) -> Result<Vec<String>, String> {
    Ok(vec![
        r#"ALTER TABLE `project_directory` ADD `strategy` text;"#.to_string(),
        r#"PRAGMA foreign_keys=OFF;"#.to_string(),
        r#"CREATE TABLE `__new_project_directory` (
          `project_id` text NOT NULL,
          `directory` text NOT NULL,
          `type` text,
          `strategy` text,
          `time_created` integer NOT NULL,
          CONSTRAINT `project_directory_pk` PRIMARY KEY(`project_id`, `directory`),
          CONSTRAINT `fk_project_directory_project_id_project_id_fk` FOREIGN KEY (`project_id`) REFERENCES `project`(`id`) ON DELETE CASCADE
        );"#.to_string(),
        r#"INSERT INTO `__new_project_directory`(`project_id`, `directory`, `type`, `time_created`) SELECT `project_id`, `directory`, `type`, `time_created` FROM `project_directory`;"#.to_string(),
        r#"DROP TABLE `project_directory`;"#.to_string(),
        r#"ALTER TABLE `__new_project_directory` RENAME TO `project_directory`;"#.to_string(),
        r#"PRAGMA foreign_keys=ON;"#.to_string(),
    ])
}
