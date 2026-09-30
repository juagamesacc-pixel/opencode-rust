//! Rust port of `packages/core/src/project/sql.ts` — ProjectTable + ProjectDirectoryTable
//! descriptors and ProjectTable CRUD. Directory behavior lives in `super::directories`.
//! Source pin: v1.18.30 @ 3104c14 — 1:1 exact translation.

use rusqlite::Connection;

/// Source: `export const ProjectTable = sqliteTable("project", {...})` verbatim.
pub const PROJECT_TABLE: &str = "project";
/// Source: `export const ProjectDirectoryTable = sqliteTable("project_directory", {...})` verbatim.
pub const PROJECT_DIRECTORY_TABLE: &str = "project_directory";

/// Source `ProjectTable.$inferSelect` — `sandboxes` is `absoluteArrayColumn()` (JSON text).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectRow {
    pub id: String,
    /// `absoluteColumn()` — absolute path (kept verbatim as stored).
    pub worktree: String,
    pub vcs: Option<String>,
    pub name: Option<String>,
    pub icon_url: Option<String>,
    pub icon_url_override: Option<String>,
    pub icon_color: Option<String>,
    pub time_created: i64,
    pub time_updated: i64,
    pub time_initialized: Option<i64>,
    /// `absoluteArrayColumn().notNull()` — JSON-encoded array of absolute paths.
    pub sandboxes: String,
    /// `commands: text({ mode: "json" }).$type<{ start?: string }>()` — JSON text.
    pub commands: Option<String>,
}

fn project_row_from(query: &rusqlite::Row<'_>) -> rusqlite::Result<ProjectRow> {
    Ok(ProjectRow {
        id: query.get(0)?,
        worktree: query.get(1)?,
        vcs: query.get(2)?,
        name: query.get(3)?,
        icon_url: query.get(4)?,
        icon_url_override: query.get(5)?,
        icon_color: query.get(6)?,
        time_created: query.get(7)?,
        time_updated: query.get(8)?,
        time_initialized: query.get(9)?,
        sandboxes: query.get(10)?,
        commands: query.get(11)?,
    })
}

const PROJECT_COLUMNS: &str = "id, worktree, vcs, name, icon_url, icon_url_override, icon_color, time_created, time_updated, time_initialized, sandboxes, commands";

/// Helper mirroring `JSON.stringify(paths)` for the `sandboxes` JSON column.
pub fn sandboxes_to_json(paths: &[&str]) -> String {
    serde_json::to_string(paths).unwrap_or_else(|_| "[]".to_string())
}

/// Source `db.insert(ProjectTable).values({...}).onConflictDoNothing().run()` (setup pattern).
pub fn insert_project(conn: &Connection, row: &ProjectRow) -> Result<(), String> {
    conn.execute(
        &format!(
            "INSERT OR IGNORE INTO \"{PROJECT_TABLE}\" ({PROJECT_COLUMNS}) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)"
        ),
        rusqlite::params![
            row.id,
            row.worktree,
            row.vcs,
            row.name,
            row.icon_url,
            row.icon_url_override,
            row.icon_color,
            row.time_created,
            row.time_updated,
            row.time_initialized,
            row.sandboxes,
            row.commands
        ],
    )
    .map(|_| ())
    .map_err(|e| e.to_string())
}

/// Source `db.select().from(ProjectTable).where(eq(ProjectTable.id, id)).get()`.
pub fn get_project(conn: &Connection, id: &str) -> Result<Option<ProjectRow>, String> {
    conn.query_row(
        &format!("SELECT {PROJECT_COLUMNS} FROM \"{PROJECT_TABLE}\" WHERE id = ?1"),
        [id],
        project_row_from,
    )
    .map(Some)
    .or_else(|e| match e {
        rusqlite::Error::QueryReturnedNoRows => Ok(None),
        other => Err(other.to_string()),
    })
}

/// Source `db.select().from(ProjectTable).where(eq(ProjectTable.worktree, worktree)).get()`.
pub fn get_project_by_worktree(
    conn: &Connection,
    worktree: &str,
) -> Result<Option<ProjectRow>, String> {
    conn.query_row(
        &format!("SELECT {PROJECT_COLUMNS} FROM \"{PROJECT_TABLE}\" WHERE worktree = ?1"),
        [worktree],
        project_row_from,
    )
    .map(Some)
    .or_else(|e| match e {
        rusqlite::Error::QueryReturnedNoRows => Ok(None),
        other => Err(other.to_string()),
    })
}

/// Source `db.delete(ProjectTable).where(eq(ProjectTable.id, id)).run()`.
pub fn delete_project(conn: &Connection, id: &str) -> Result<bool, String> {
    conn.execute(
        &format!("DELETE FROM \"{PROJECT_TABLE}\" WHERE id = ?1"),
        [id],
    )
    .map(|n| n > 0)
    .map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::database::database::open_in_memory;

    fn seed(conn: &Connection) {
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS project (id text PRIMARY KEY, worktree text NOT NULL, vcs text, name text, icon_url text, icon_url_override text, icon_color text, time_created integer NOT NULL, time_updated integer NOT NULL, time_initialized integer, sandboxes text NOT NULL, commands text);",
        )
        .unwrap();
    }

    fn row(id: &str) -> ProjectRow {
        ProjectRow {
            id: id.to_string(),
            worktree: format!("/worktree/{id}"),
            vcs: None,
            name: None,
            icon_url: None,
            icon_url_override: None,
            icon_color: None,
            time_created: 1,
            time_updated: 1,
            time_initialized: None,
            sandboxes: sandboxes_to_json(&[]),
            commands: None,
        }
    }

    #[test]
    fn inserts_and_reads_project_by_id_and_worktree() {
        let conn = open_in_memory().unwrap();
        seed(&conn);
        insert_project(&conn, &row("proj_1")).unwrap();
        insert_project(&conn, &row("proj_1")).unwrap(); // onConflictDoNothing → ignored
        assert_eq!(
            get_project(&conn, "proj_1").unwrap().unwrap().worktree,
            "/worktree/proj_1"
        );
        assert_eq!(
            get_project_by_worktree(&conn, "/worktree/proj_1")
                .unwrap()
                .unwrap()
                .id,
            "proj_1"
        );
        assert!(get_project(&conn, "missing").unwrap().is_none());
    }

    #[test]
    fn deletes_project() {
        let conn = open_in_memory().unwrap();
        seed(&conn);
        insert_project(&conn, &row("proj_1")).unwrap();
        assert!(delete_project(&conn, "proj_1").unwrap());
        assert!(!delete_project(&conn, "proj_1").unwrap());
        assert!(get_project(&conn, "proj_1").unwrap().is_none());
    }
}
