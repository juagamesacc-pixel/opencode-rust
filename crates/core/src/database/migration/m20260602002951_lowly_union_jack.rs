//! Rust port of `packages/core/src/database/migration/20260602002951_lowly_union_jack.ts`.
//! Source pin: v1.18.30 @3104c14 — 1:1 SQL verbatim extraction.
//! NOTE: `up_sql(conn)` accepts the connection because the TypeScript migration inspects the
//! live schema at migration time (see `packages/core/src/database/migration.ts` apply/applyOnly).

use rusqlite::Connection;

pub const ID: &str = "20260602002951_lowly_union_jack";

pub fn up_sql(_conn: &Connection) -> Result<Vec<String>, String> {
    Ok(vec![
        r#"CREATE TABLE `permission` (
          `id` text PRIMARY KEY,
          `project_id` text NOT NULL,
          `action` text NOT NULL,
          `resource` text NOT NULL,
          `time_created` integer NOT NULL,
          `time_updated` integer NOT NULL,
          CONSTRAINT `fk_permission_project_id_project_id_fk` FOREIGN KEY (`project_id`) REFERENCES `project`(`id`) ON DELETE CASCADE
        );"#.to_string(),
        r#"CREATE UNIQUE INDEX `permission_project_action_resource_idx` ON `permission` (`project_id`,`action`,`resource`);"#.to_string(),
    ])
}
