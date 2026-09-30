//! Rust port of `packages/core/src/credential/sql.ts` (+ CRUD from `packages/core/src/credential.ts`).
//! Source pin: v1.18.30 @ 3104c14 — 1:1 exact translation.

use rusqlite::Connection;
use serde::{Deserialize, Serialize};

/// Source: `export const CredentialTable = sqliteTable("credential", {...})` verbatim
pub const TABLE: &str = "credential";

/// Row shape of `credential` (source `$inferSelect`): `value` is stored as JSON text.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CredentialRow {
    pub id: String,
    /// nullable in schema (`integration_id: text()`)
    pub integration_id: Option<String>,
    pub label: String,
    /// `value: text({ mode: "json" })` — JSON-encoded credential value
    pub value: String,
    /// nullable in schema
    pub connector_id: Option<String>,
    /// nullable in schema
    pub method_id: Option<String>,
    /// `active: integer({ mode: "boolean" })` — nullable
    pub active: Option<bool>,
    pub time_created: i64,
    pub time_updated: i64,
}

fn row_from(query: &rusqlite::Row<'_>) -> rusqlite::Result<CredentialRow> {
    Ok(CredentialRow {
        id: query.get(0)?,
        integration_id: query.get(1)?,
        label: query.get(2)?,
        value: query.get(3)?,
        connector_id: query.get(4)?,
        method_id: query.get(5)?,
        active: query.get::<_, Option<i64>>(6)?.map(|v| v != 0),
        time_created: query.get(7)?,
        time_updated: query.get(8)?,
    })
}

const COLUMNS: &str =
    "id, integration_id, label, value, connector_id, method_id, active, time_created, time_updated";

/// Returns every stored credential, newest-first? — no: source `orderBy(asc(time_created))`.
pub fn all(conn: &Connection) -> Result<Vec<CredentialRow>, String> {
    let mut stmt = conn
        .prepare(&format!(
            "SELECT {COLUMNS} FROM \"{TABLE}\" ORDER BY time_created ASC"
        ))
        .map_err(|e| e.to_string())?;
    let rows = stmt.query_map([], row_from).map_err(|e| e.to_string())?;
    let mut out = Vec::new();
    for r in rows {
        out.push(r.map_err(|e| e.to_string())?);
    }
    Ok(out)
}

/// Stored credentials belonging to one integration — source `orderBy(asc(time_created))`.
pub fn list(conn: &Connection, integration_id: &str) -> Result<Vec<CredentialRow>, String> {
    let mut stmt = conn
        .prepare(&format!(
            "SELECT {COLUMNS} FROM \"{TABLE}\" WHERE integration_id = ?1 ORDER BY time_created ASC"
        ))
        .map_err(|e| e.to_string())?;
    let rows = stmt
        .query_map([integration_id], row_from)
        .map_err(|e| e.to_string())?;
    let mut out = Vec::new();
    for r in rows {
        out.push(r.map_err(|e| e.to_string())?);
    }
    Ok(out)
}

/// One stored credential by ID (undefined when missing).
pub fn get(conn: &Connection, id: &str) -> Result<Option<CredentialRow>, String> {
    let mut stmt = conn
        .prepare(&format!("SELECT {COLUMNS} FROM \"{TABLE}\" WHERE id = ?1"))
        .map_err(|e| e.to_string())?;
    let mut rows = stmt.query_map([id], row_from).map_err(|e| e.to_string())?;
    match rows.next() {
        Some(r) => Ok(Some(r.map_err(|e| e.to_string())?)),
        None => Ok(None),
    }
}

/// `create` — replaces any credential for an integration (delete-then-insert, one transaction)
/// and returns the new record. `label` defaults to `"default"`.
pub fn create(
    conn: &mut Connection,
    id: &str,
    integration_id: &str,
    label: &str,
    value: &str,
) -> Result<(), String> {
    let tx = conn.transaction().map_err(|e| e.to_string())?;
    tx.execute(
        "DELETE FROM \"credential\" WHERE integration_id = ?1",
        [integration_id],
    )
    .map_err(|e| e.to_string())?;
    let now = crate::database::migration::now_ms();
    tx.execute(
        "INSERT INTO \"credential\" (id, integration_id, label, value, time_created, time_updated, active) VALUES (?1, ?2, ?3, ?4, ?5, ?6, NULL)",
        rusqlite::params![id, integration_id, label, value, now, now],
    )
    .map_err(|e| e.to_string())?;
    tx.commit().map_err(|e| e.to_string())
}

/// `update` — updates the label or secret value of a stored credential; no-op when both are undefined.
pub fn update(
    conn: &Connection,
    id: &str,
    label: Option<&str>,
    value: Option<&str>,
) -> Result<(), String> {
    if label.is_none() && value.is_none() {
        return Ok(());
    }
    conn.execute(
        "UPDATE \"credential\" SET label = COALESCE(?1, label), value = COALESCE(?2, value), time_updated = ?3 WHERE id = ?4",
        rusqlite::params![label, value, crate::database::migration::now_ms(), id],
    )
    .map(|_| ())
    .map_err(|e| e.to_string())
}

/// `remove` — removes a stored credential by id.
pub fn remove(conn: &Connection, id: &str) -> Result<(), String> {
    conn.execute("DELETE FROM \"credential\" WHERE id = ?1", [id])
        .map(|_| ())
        .map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mem() -> Result<Connection, String> {
        crate::database::database::open_in_memory()
    }

    #[test]
    fn create_replaces_integration_credential() {
        let mut conn = mem().unwrap();
        create(&mut conn, "a", "int1", "default", "\"v1\"").unwrap();
        create(&mut conn, "b", "int1", "default", "\"v2\"").unwrap();
        let rows = all(&conn).unwrap();
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].id, "b");
        assert_eq!(rows[0].value, "\"v2\"");
    }

    #[test]
    fn get_and_update() {
        let mut conn = mem().unwrap();
        create(&mut conn, "a", "int1", "default", "\"v1\"").unwrap();
        let fetched = get(&conn, "a").unwrap().unwrap();
        assert_eq!(fetched.label, "default");
        update(&conn, "a", Some("new"), Some("\"v2\"")).unwrap();
        let updated = get(&conn, "a").unwrap().unwrap();
        assert_eq!(updated.label, "new");
        assert_eq!(updated.value, "\"v2\"");
        // no-op update (both None) leaves row intact
        update(&conn, "a", None, None).unwrap();
        let again = get(&conn, "a").unwrap().unwrap();
        assert_eq!(again.value, "\"v2\"");
    }

    #[test]
    fn list_filters_by_integration() {
        let mut conn = mem().unwrap();
        create(&mut conn, "a", "int1", "default", "\"v1\"").unwrap();
        create(&mut conn, "b", "int2", "default", "\"v2\"").unwrap();
        let only = list(&conn, "int1").unwrap();
        assert_eq!(only.len(), 1);
        assert_eq!(only[0].id, "a");
        remove(&conn, "a").unwrap();
        assert!(get(&conn, "a").unwrap().is_none());
    }
}
