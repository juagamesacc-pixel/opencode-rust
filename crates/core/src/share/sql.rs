//! Rust port of `packages/core/src/share/sql.ts` (+ CRUD from `packages/opencode/src/share/share-next.ts`).
//! Source pin: v1.18.30 @ 3104c14 — 1:1 exact translation.

use rusqlite::Connection;

/// Source: `export const SessionShareTable = sqliteTable("session_share", {...})` verbatim.
pub const TABLE: &str = "session_share";

/// Row shape of `session_share` (source `$inferSelect`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SessionShareRow {
    pub session_id: String,
    pub id: String,
    pub secret: String,
    pub url: String,
    pub time_created: i64,
    pub time_updated: i64,
}

fn row_from(query: &rusqlite::Row<'_>) -> rusqlite::Result<SessionShareRow> {
    Ok(SessionShareRow {
        session_id: query.get(0)?,
        id: query.get(1)?,
        secret: query.get(2)?,
        url: query.get(3)?,
        time_created: query.get(4)?,
        time_updated: query.get(5)?,
    })
}

const COLUMNS: &str = "session_id, id, secret, url, time_created, time_updated";

/// Source `share.get(sessionID)`: `select().from(SessionShareTable).where(eq(session_id))` → row
/// or `undefined`.
pub fn get(conn: &Connection, session_id: &str) -> Result<Option<SessionShareRow>, String> {
    conn.query_row(
        &format!("SELECT {COLUMNS} FROM \"{TABLE}\" WHERE session_id = ?1"),
        [session_id],
        row_from,
    )
    .map(Some)
    .or_else(|e| match e {
        rusqlite::Error::QueryReturnedNoRows => Ok(None),
        other => Err(other.to_string()),
    })
}

/// Source `share.create`: insert `(session_id, id, secret, url)` with
/// `onConflictDoUpdate(target: session_id)` setting `id`/`secret`/`url`.
pub fn upsert(
    conn: &Connection,
    session_id: &str,
    id: &str,
    secret: &str,
    url: &str,
    now: i64,
) -> Result<(), String> {
    conn.execute(
        &format!(
            "INSERT INTO \"{TABLE}\" (session_id, id, secret, url, time_created, time_updated) VALUES (?1, ?2, ?3, ?4, ?5, ?5) ON CONFLICT(session_id) DO UPDATE SET id = ?2, secret = ?3, url = ?4, time_updated = ?5"
        ),
        rusqlite::params![session_id, id, secret, url, now],
    )
    .map(|_| ())
    .map_err(|e| e.to_string())
}

/// Source `share.remove`: `delete(SessionShareTable).where(eq(session_id))` → removed or not.
pub fn delete(conn: &Connection, session_id: &str) -> Result<bool, String> {
    conn.execute(
        &format!("DELETE FROM \"{TABLE}\" WHERE session_id = ?1"),
        [session_id],
    )
    .map(|n| n > 0)
    .map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::database::database::open_in_memory;

    #[test]
    fn upsert_replaces_share_for_session() {
        let conn = open_in_memory().unwrap();
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS session_share (session_id text PRIMARY KEY, id text NOT NULL, secret text NOT NULL, url text NOT NULL, time_created integer NOT NULL, time_updated integer NOT NULL);",
        )
        .unwrap();
        // Parent rows for FK session_share.session_id → session(id) → project(id).
        conn.execute_batch(
            "INSERT OR IGNORE INTO project (id, worktree, time_created, time_updated, sandboxes) VALUES ('proj_share', '/p', 1, 1, '[]');
             INSERT OR IGNORE INTO session (id, project_id, slug, directory, title, version, time_created, time_updated) VALUES ('session', 'proj_share', 's', '/p', 't', 'v', 1, 1);",
        )
        .unwrap();
        upsert(&conn, "session", "share_1", "secret_1", "url_1", 1).unwrap();
        upsert(&conn, "session", "share_2", "secret_2", "url_2", 2).unwrap();
        let row = get(&conn, "session").unwrap().unwrap();
        assert_eq!(row.id, "share_2");
        assert_eq!(row.secret, "secret_2");
        assert_eq!(row.url, "url_2");
        assert_eq!(row.time_created, 1);
        assert_eq!(row.time_updated, 2);
        assert!(get(&conn, "missing").unwrap().is_none());
    }

    #[test]
    fn delete_removes_share() {
        let conn = open_in_memory().unwrap();
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS session_share (session_id text PRIMARY KEY, id text NOT NULL, secret text NOT NULL, url text NOT NULL, time_created integer NOT NULL, time_updated integer NOT NULL);",
        )
        .unwrap();
        // Parent rows for FK session_share.session_id → session(id) → project(id).
        conn.execute_batch(
            "INSERT OR IGNORE INTO project (id, worktree, time_created, time_updated, sandboxes) VALUES ('proj_share', '/p', 1, 1, '[]');
             INSERT OR IGNORE INTO session (id, project_id, slug, directory, title, version, time_created, time_updated) VALUES ('session', 'proj_share', 's', '/p', 't', 'v', 1, 1);",
        )
        .unwrap();
        upsert(&conn, "session", "share_1", "secret_1", "url_1", 1).unwrap();
        assert!(delete(&conn, "session").unwrap());
        assert!(!delete(&conn, "session").unwrap());
        assert!(get(&conn, "session").unwrap().is_none());
    }
}
