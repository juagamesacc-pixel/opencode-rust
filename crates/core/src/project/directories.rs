//! Rust port of `packages/core/src/project/directories.ts` — ProjectDirectories behavior over
//! `ProjectDirectoryTable`. Service layer / Effect wiring is ported at the crate boundary.
//! Source pin: v1.18.30 @ 3104c14 — 1:1 exact translation (SQL semantics preserved).

use rusqlite::Connection;

use super::sql::PROJECT_DIRECTORY_TABLE;
use crate::database::migration::now_ms;

/// Source `ProjectDirectories.Directory` — a single project directory row.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Directory {
    pub directory: String,
    pub strategy: Option<String>,
}

/// Source `ProjectDirectories.CreateInput` (`behavior` optional: "replace" | "ignore").
#[derive(Debug, Clone)]
pub struct CreateInput {
    pub project_id: String,
    pub directory: String,
    pub strategy: Option<String>,
    pub behavior: Option<String>,
}

/// Source `ProjectDirectories.RemoveInput`/`contains`/`get` key.
#[derive(Debug, Clone)]
pub struct DirectoryKey {
    pub project_id: String,
    pub directory: String,
}

fn directory_row_from(query: &rusqlite::Row<'_>) -> rusqlite::Result<Directory> {
    Ok(Directory {
        directory: query.get(0)?,
        strategy: query.get(1)?,
    })
}

/// Source `ProjectDirectories.create`: insert with `onConflictDoNothing` (default "ignore") or
/// `onConflictDoUpdate` (+ setWhere) when `behavior === "replace"`. Returns whether a row was
/// inserted/updated — `returning({ directory }).get()` semantics.
pub fn create(conn: &Connection, input: &CreateInput) -> Result<bool, String> {
    let sql = match input.behavior.as_deref() {
        Some("replace") => format!(
            "INSERT INTO \"{PROJECT_DIRECTORY_TABLE}\" (project_id, directory, strategy, time_created) VALUES (?1, ?2, ?3, ?4) ON CONFLICT(project_id, directory) DO UPDATE SET strategy = excluded.strategy WHERE (excluded.strategy IS NOT NULL AND (strategy IS NULL OR strategy != excluded.strategy)) OR (excluded.strategy IS NULL AND strategy IS NOT NULL) RETURNING directory"
        ),
        _ => format!(
            "INSERT INTO \"{PROJECT_DIRECTORY_TABLE}\" (project_id, directory, strategy, time_created) VALUES (?1, ?2, ?3, ?4) ON CONFLICT DO NOTHING RETURNING directory"
        ),
    };
    conn.query_row(
        &sql,
        rusqlite::params![input.project_id, input.directory, input.strategy, now_ms()],
        |row| row.get::<_, String>(0),
    )
    .map(|_| true)
    .or_else(|e| match e {
        rusqlite::Error::QueryReturnedNoRows => Ok(false),
        other => Err(other.to_string()),
    })
}

/// Source `ProjectDirectories.remove`: `delete(...).where(and(eq(project_id), eq(directory)))`
/// with `returning({ directory }).get()` — removed or not.
pub fn remove(conn: &Connection, key: &DirectoryKey) -> Result<bool, String> {
    conn.query_row(
        &format!(
            "DELETE FROM \"{PROJECT_DIRECTORY_TABLE}\" WHERE project_id = ?1 AND directory = ?2 RETURNING directory"
        ),
        rusqlite::params![key.project_id, key.directory],
        |row| row.get::<_, String>(0),
    )
    .map(|_| true)
    .or_else(|e| match e {
        rusqlite::Error::QueryReturnedNoRows => Ok(false),
        other => Err(other.to_string()),
    })
}

/// Source `ProjectDirectories.list`: `orderBy(desc(time_created), asc(directory))`.
pub fn list(conn: &Connection, project_id: &str) -> Result<Vec<Directory>, String> {
    let mut stmt = conn
        .prepare(&format!(
            "SELECT directory, strategy FROM \"{PROJECT_DIRECTORY_TABLE}\" WHERE project_id = ?1 ORDER BY time_created DESC, directory ASC"
        ))
        .map_err(|e| e.to_string())?;
    let rows = stmt
        .query_map([project_id], directory_row_from)
        .map_err(|e| e.to_string())?;
    let mut out = Vec::new();
    for r in rows {
        out.push(r.map_err(|e| e.to_string())?);
    }
    Ok(out)
}

/// Source `ProjectDirectories.contains` — row presence check.
pub fn contains(conn: &Connection, key: &DirectoryKey) -> Result<bool, String> {
    conn.query_row(
        &format!(
            "SELECT directory FROM \"{PROJECT_DIRECTORY_TABLE}\" WHERE project_id = ?1 AND directory = ?2"
        ),
        rusqlite::params![key.project_id, key.directory],
        |row| row.get::<_, String>(0),
    )
    .map(|_| true)
    .or_else(|e| match e {
        rusqlite::Error::QueryReturnedNoRows => Ok(false),
        other => Err(other.to_string()),
    })
}

/// Source `ProjectDirectories.get` — one `Directory` or `undefined`.
pub fn get(conn: &Connection, key: &DirectoryKey) -> Result<Option<Directory>, String> {
    conn.query_row(
        &format!(
            "SELECT directory, strategy FROM \"{PROJECT_DIRECTORY_TABLE}\" WHERE project_id = ?1 AND directory = ?2"
        ),
        rusqlite::params![key.project_id, key.directory],
        directory_row_from,
    )
    .map(Some)
    .or_else(|e| match e {
        rusqlite::Error::QueryReturnedNoRows => Ok(None),
        other => Err(other.to_string()),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::database::database::open_in_memory;
    use crate::project::sql::{insert_project, sandboxes_to_json, ProjectRow};

    fn seed(conn: &Connection) {
        // project_directory has an FK to project — seed the parent row first.
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS project (id text PRIMARY KEY, worktree text NOT NULL, vcs text, name text, icon_url text, icon_url_override text, icon_color text, time_created integer NOT NULL, time_updated integer NOT NULL, time_initialized integer, sandboxes text NOT NULL, commands text);
             CREATE TABLE IF NOT EXISTS project_directory (project_id text NOT NULL, directory text NOT NULL, type text, strategy text, time_created integer NOT NULL, CONSTRAINT project_directory_pk PRIMARY KEY(project_id, directory), CONSTRAINT fk_project_directory_project_id_project_id_fk FOREIGN KEY (project_id) REFERENCES project(id) ON DELETE CASCADE);",
        )
        .unwrap();
    }

    fn seed_project(conn: &Connection) {
        insert_project(
            conn,
            &ProjectRow {
                id: "project-directories".to_string(),
                worktree: "/tmp/project-directories".to_string(),
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
            },
        )
        .unwrap();
    }

    #[test]
    fn creates_once_and_ignores_conflicts() {
        let conn = open_in_memory().unwrap();
        seed(&conn);
        seed_project(&conn);
        let key = DirectoryKey {
            project_id: "project-directories".to_string(),
            directory: "/tmp/project-directories".to_string(),
        };
        assert!(create(
            &conn,
            &CreateInput {
                project_id: key.project_id.clone(),
                directory: key.directory.clone(),
                strategy: None,
                behavior: None
            }
        )
        .unwrap());
        assert!(!create(
            &conn,
            &CreateInput {
                project_id: key.project_id.clone(),
                directory: key.directory.clone(),
                strategy: Some("git_worktree".to_string()),
                behavior: None
            }
        )
        .unwrap());
        assert_eq!(
            list(&conn, "project-directories").unwrap(),
            vec![Directory {
                directory: "/tmp/project-directories".to_string(),
                strategy: None
            }]
        );
    }

    #[test]
    fn replaces_the_strategy_when_requested() {
        let conn = open_in_memory().unwrap();
        seed(&conn);
        seed_project(&conn);
        let mk = |strategy: Option<&str>, behavior: Option<&str>| CreateInput {
            project_id: "project-directories".to_string(),
            directory: "/tmp/project-directories".to_string(),
            strategy: strategy.map(|s| s.to_string()),
            behavior: behavior.map(|s| s.to_string()),
        };
        // source test sequence: create old/strategy, replace new/strategy (true), again (false),
        // replace with NO strategy (true), again (false), replace back new/strategy (true).
        assert!(create(&conn, &mk(Some("old/strategy"), None)).unwrap());
        assert!(create(&conn, &mk(Some("new/strategy"), Some("replace"))).unwrap());
        assert!(!create(&conn, &mk(Some("new/strategy"), Some("replace"))).unwrap());
        assert!(create(&conn, &mk(None, Some("replace"))).unwrap());
        assert!(!create(&conn, &mk(None, Some("replace"))).unwrap());
        assert!(create(&conn, &mk(Some("new/strategy"), Some("replace"))).unwrap());
        assert_eq!(
            list(&conn, "project-directories").unwrap(),
            vec![Directory {
                directory: "/tmp/project-directories".to_string(),
                strategy: Some("new/strategy".to_string())
            }]
        );
    }

    #[test]
    fn remove_contains_and_get() {
        let conn = open_in_memory().unwrap();
        seed(&conn);
        seed_project(&conn);
        let key = DirectoryKey {
            project_id: "project-directories".to_string(),
            directory: "/tmp/project-directories".to_string(),
        };
        assert!(!contains(&conn, &key).unwrap());
        assert!(create(
            &conn,
            &CreateInput {
                project_id: key.project_id.clone(),
                directory: key.directory.clone(),
                strategy: Some("git_worktree".to_string()),
                behavior: None
            }
        )
        .unwrap());
        assert!(contains(&conn, &key).unwrap());
        assert_eq!(
            get(&conn, &key).unwrap(),
            Some(Directory {
                directory: "/tmp/project-directories".to_string(),
                strategy: Some("git_worktree".to_string())
            })
        );
        assert!(remove(&conn, &key).unwrap());
        assert!(!remove(&conn, &key).unwrap());
        assert!(!contains(&conn, &key).unwrap());
    }
}
