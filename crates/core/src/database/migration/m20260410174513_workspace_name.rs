//! Rust port of `packages/core/src/database/migration/20260410174513_workspace-name.ts`.
//! Source pin: v1.18.30 @3104c14 — 1:1 SQL verbatim extraction.
//! NOTE: `up_sql(conn)` accepts the connection because the TypeScript migration inspects the
//! live schema at migration time (see `packages/core/src/database/migration.ts` apply/applyOnly).

use rusqlite::Connection;

pub const ID: &str = "20260410174513_workspace-name";

pub fn up_sql(conn: &Connection) -> Result<Vec<String>, String> {
    // source: const columns = yield* tx.all(`PRAGMA table_info(`workspace`)`)
    //         const name = columns.some((column) => column.name === "name") ? "`name`" : "''"
    let mut stmt = conn
        .prepare("PRAGMA table_info(`workspace`)")
        .map_err(|e| e.to_string())?;
    let columns = stmt
        .query_map([], |row| row.get::<_, String>(1))
        .map_err(|e| e.to_string())?;
    let mut has_name = false;
    for column in columns {
        if column.map_err(|e| e.to_string())? == "name" {
            has_name = true;
        }
    }
    let name = if has_name { "`name`" } else { "''" };
    let insert = format!(
        "INSERT INTO `__new_workspace`(`id`, `type`, `branch`, `name`, `directory`, `extra`, `project_id`) SELECT `id`, `type`, `branch`, {}, `directory`, `extra`, `project_id` FROM `workspace`;",
        name
    );
    Ok(vec![
        r#"PRAGMA foreign_keys=OFF;"#.to_string(),
        r#"CREATE TABLE `__new_workspace` (
          `id` text PRIMARY KEY,
          `type` text NOT NULL,
          `name` text DEFAULT '' NOT NULL,
          `branch` text,
          `directory` text,
          `extra` text,
          `project_id` text NOT NULL,
          CONSTRAINT `fk_workspace_project_id_project_id_fk` FOREIGN KEY (`project_id`) REFERENCES `project`(`id`) ON DELETE CASCADE
        );"#.to_string(),
        insert,
        r#"DROP TABLE `workspace`;"#.to_string(),
        r#"ALTER TABLE `__new_workspace` RENAME TO `workspace`;"#.to_string(),
        r#"PRAGMA foreign_keys=ON;"#.to_string(),
    ])
}
