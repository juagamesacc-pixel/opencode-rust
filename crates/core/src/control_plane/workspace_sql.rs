//! Rust port of `packages/core/src/control-plane/workspace.sql.ts` descriptor + CRUD consumed by
//! `packages/core/src/session/projector.ts` and the control-plane workspace service.
//! Source pin: v1.18.30 @ 3104c14 — 1:1 exact translation.

use rusqlite::Connection;

/// Source: `export const WorkspaceTable = sqliteTable("workspace", {...})` verbatim.
pub const WORKSPACE_TABLE: &str = "workspace";

/// Source `WorkspaceTable.$inferSelect` — `extra` is `text({ mode: "json" })`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkspaceRow {
    pub id: String,
    pub type_: String,
    /// `.notNull().default("")`
    pub name: String,
    pub branch: Option<String>,
    pub directory: Option<String>,
    /// JSON text.
    pub extra: Option<String>,
    pub project_id: String,
    /// `.notNull().$default(() => Date.now())`
    pub time_used: i64,
}

fn row_from(query: &rusqlite::Row<'_>) -> rusqlite::Result<WorkspaceRow> {
    Ok(WorkspaceRow {
        id: query.get(0)?,
        type_: query.get(1)?,
        name: query.get(2)?,
        branch: query.get(3)?,
        directory: query.get(4)?,
        extra: query.get(5)?,
        project_id: query.get(6)?,
        time_used: query.get(7)?,
    })
}

const COLUMNS: &str = "id, type, name, branch, directory, extra, project_id, time_used";

/// Source `db.select().from(WorkspaceTable).where(eq(WorkspaceTable.id, id)).get()`.
pub fn get(conn: &Connection, id: &str) -> Result<Option<WorkspaceRow>, String> {
    conn.query_row(
        &format!("SELECT {COLUMNS} FROM \"{WORKSPACE_TABLE}\" WHERE id = ?1"),
        [id],
        row_from,
    )
    .map(Some)
    .or_else(|e| match e {
        rusqlite::Error::QueryReturnedNoRows => Ok(None),
        other => Err(other.to_string()),
    })
}

/// Source `db.select().from(WorkspaceTable).where(eq(WorkspaceTable.project_id, id)).all()`.
pub fn get_by_project(conn: &Connection, project_id: &str) -> Result<Vec<WorkspaceRow>, String> {
    let mut stmt = conn
        .prepare(&format!(
            "SELECT {COLUMNS} FROM \"{WORKSPACE_TABLE}\" WHERE project_id = ?1"
        ))
        .map_err(|e| e.to_string())?;
    let rows = stmt
        .query_map([project_id], row_from)
        .map_err(|e| e.to_string())?;
    let mut out = Vec::new();
    for r in rows {
        out.push(r.map_err(|e| e.to_string())?);
    }
    Ok(out)
}

/// Source `db.insert(WorkspaceTable).values({...})` — `name`/`time_used` mirror the SQL defaults
/// when omitted at the call site (`.default("")` / `.$default(() => Date.now())`).
pub fn insert(conn: &Connection, row: &WorkspaceRow) -> Result<(), String> {
    conn.execute(
        &format!(
            "INSERT INTO \"{WORKSPACE_TABLE}\" ({COLUMNS}) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)"
        ),
        rusqlite::params![
            row.id,
            row.type_,
            row.name,
            row.branch,
            row.directory,
            row.extra,
            row.project_id,
            row.time_used
        ],
    )
    .map(|_| ())
    .map_err(|e| e.to_string())
}

/// Source `session/projector.ts`: `update(WorkspaceTable).set({ time_used }).where(eq(id))`.
pub fn update_time_used(conn: &Connection, id: &str, time_used: i64) -> Result<bool, String> {
    conn.execute(
        &format!("UPDATE \"{WORKSPACE_TABLE}\" SET time_used = ?2 WHERE id = ?1"),
        rusqlite::params![id, time_used],
    )
    .map(|n| n > 0)
    .map_err(|e| e.to_string())
}

/// Source `db.delete(WorkspaceTable).where(eq(WorkspaceTable.id, id)).run()`.
pub fn delete(conn: &Connection, id: &str) -> Result<bool, String> {
    conn.execute(
        &format!("DELETE FROM \"{WORKSPACE_TABLE}\" WHERE id = ?1"),
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
            "CREATE TABLE IF NOT EXISTS project (id text PRIMARY KEY, worktree text NOT NULL, sandboxes text NOT NULL, time_created integer NOT NULL, time_updated integer NOT NULL);
             CREATE TABLE IF NOT EXISTS workspace (id text PRIMARY KEY, type text NOT NULL, name text DEFAULT '' NOT NULL, branch text, directory text, extra text, project_id text NOT NULL, time_used integer NOT NULL, CONSTRAINT fk_workspace_project_id_project_id_fk FOREIGN KEY (project_id) REFERENCES project(id) ON DELETE CASCADE);",
        )
        .unwrap();
        conn.execute(
            "INSERT INTO project (id, worktree, sandboxes, time_created, time_updated) VALUES ('proj_1', '/p', '[]', 1, 1)",
            [],
        )
        .unwrap();
    }

    fn row(id: &str) -> WorkspaceRow {
        WorkspaceRow {
            id: id.to_string(),
            type_: "local".to_string(),
            name: String::new(),
            branch: None,
            directory: None,
            extra: None,
            project_id: "proj_1".to_string(),
            time_used: 1,
        }
    }

    #[test]
    fn inserts_reads_and_lists_by_project() {
        let conn = open_in_memory().unwrap();
        seed(&conn);
        insert(&conn, &row("wrk_1")).unwrap();
        insert(&conn, &row("wrk_2")).unwrap();
        assert_eq!(get(&conn, "wrk_1").unwrap().unwrap().project_id, "proj_1");
        assert_eq!(get_by_project(&conn, "proj_1").unwrap().len(), 2);
        assert!(get(&conn, "missing").unwrap().is_none());
    }

    #[test]
    fn updates_time_used_and_deletes() {
        let conn = open_in_memory().unwrap();
        seed(&conn);
        insert(&conn, &row("wrk_1")).unwrap();
        assert!(update_time_used(&conn, "wrk_1", 42).unwrap());
        assert_eq!(get(&conn, "wrk_1").unwrap().unwrap().time_used, 42);
        assert!(!update_time_used(&conn, "missing", 42).unwrap());
        assert!(delete(&conn, "wrk_1").unwrap());
        assert!(!delete(&conn, "wrk_1").unwrap());
    }
}
