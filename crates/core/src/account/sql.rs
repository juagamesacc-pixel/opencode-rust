//! Rust port of `packages/core/src/account/sql.ts` descriptors + CRUD helpers consumed by
//! `packages/opencode/src/account/repo.ts` (ACCOUNT_STATE_ID = 1 semantics).
//! Source pin: v1.18.30 @ 3104c14 — 1:1 exact translation.

use rusqlite::Connection;

/// Source: `export const AccountTable = sqliteTable("account", {...})` verbatim.
pub const ACCOUNT_TABLE: &str = "account";
/// Source: `export const AccountStateTable = sqliteTable("account_state", {...})` verbatim.
pub const ACCOUNT_STATE_TABLE: &str = "account_state";
/// Source: `export const ControlAccountTable = sqliteTable("control_account", {...})` verbatim.
pub const CONTROL_ACCOUNT_TABLE: &str = "control_account";

/// Source `AccountTable.$inferSelect` — `packages/opencode/src/account/repo.ts` AccountRow.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AccountRow {
    pub id: String,
    pub email: String,
    pub url: String,
    pub access_token: String,
    pub refresh_token: String,
    pub token_expiry: Option<i64>,
    pub time_created: i64,
    pub time_updated: i64,
}

/// Source `AccountStateTable.$inferSelect`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AccountStateRow {
    pub id: i64,
    pub active_account_id: Option<String>,
    pub active_org_id: Option<String>,
}

/// Source `ControlAccountTable.$inferSelect`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ControlAccountRow {
    pub email: String,
    pub url: String,
    pub access_token: String,
    pub refresh_token: String,
    pub token_expiry: Option<i64>,
    pub active: bool,
    pub time_created: i64,
    pub time_updated: i64,
}

fn account_row_from(query: &rusqlite::Row<'_>) -> rusqlite::Result<AccountRow> {
    Ok(AccountRow {
        id: query.get(0)?,
        email: query.get(1)?,
        url: query.get(2)?,
        access_token: query.get(3)?,
        refresh_token: query.get(4)?,
        token_expiry: query.get(5)?,
        time_created: query.get(6)?,
        time_updated: query.get(7)?,
    })
}

const ACCOUNT_COLUMNS: &str =
    "id, email, url, access_token, refresh_token, token_expiry, time_created, time_updated";

/// Source `repo.list`: `select().from(AccountTable).all()` — insertion order, no ORDER BY.
pub fn list_accounts(conn: &Connection) -> Result<Vec<AccountRow>, String> {
    let mut stmt = conn
        .prepare(&format!(
            "SELECT {ACCOUNT_COLUMNS} FROM \"{ACCOUNT_TABLE}\""
        ))
        .map_err(|e| e.to_string())?;
    let rows = stmt
        .query_map([], account_row_from)
        .map_err(|e| e.to_string())?;
    let mut out = Vec::new();
    for r in rows {
        out.push(r.map_err(|e| e.to_string())?);
    }
    Ok(out)
}

/// Source `repo.getRow`: `select().from(AccountTable).where(eq(id)).get()`.
pub fn get_account(conn: &Connection, id: &str) -> Result<Option<AccountRow>, String> {
    conn.query_row(
        &format!("SELECT {ACCOUNT_COLUMNS} FROM \"{ACCOUNT_TABLE}\" WHERE id = ?1"),
        [id],
        account_row_from,
    )
    .map(Some)
    .or_else(|e| match e {
        rusqlite::Error::QueryReturnedNoRows => Ok(None),
        other => Err(other.to_string()),
    })
}

/// Source `repo.persistAccount` insert branch (upsert on `id`, `time_updated` refreshed).
pub fn upsert_account(conn: &Connection, account: &AccountRow) -> Result<(), String> {
    conn.execute(
        &format!(
            "INSERT INTO \"{ACCOUNT_TABLE}\" ({ACCOUNT_COLUMNS}) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8) ON CONFLICT(id) DO UPDATE SET email = ?2, url = ?3, access_token = ?4, refresh_token = ?5, token_expiry = ?6, time_updated = ?8"
        ),
        rusqlite::params![
            account.id,
            account.email,
            account.url,
            account.access_token,
            account.refresh_token,
            account.token_expiry,
            account.time_created,
            account.time_updated
        ],
    )
    .map(|_| ())
    .map_err(|e| e.to_string())
}

/// Source `repo.persistToken`: update `access_token`/`refresh_token`/`token_expiry` where `id`.
pub fn update_token(
    conn: &Connection,
    id: &str,
    access_token: &str,
    refresh_token: &str,
    token_expiry: Option<i64>,
    now: i64,
) -> Result<(), String> {
    conn.execute(
        &format!(
            "UPDATE \"{ACCOUNT_TABLE}\" SET access_token = ?2, refresh_token = ?3, token_expiry = ?4, time_updated = ?5 WHERE id = ?1"
        ),
        rusqlite::params![id, access_token, refresh_token, token_expiry, now],
    )
    .map(|_| ())
    .map_err(|e| e.to_string())
}

/// Source `repo.remove` delete branch: `delete(AccountTable).where(eq(id))`.
pub fn delete_account(conn: &Connection, id: &str) -> Result<(), String> {
    conn.execute(
        &format!("DELETE FROM \"{ACCOUNT_TABLE}\" WHERE id = ?1"),
        [id],
    )
    .map(|_| ())
    .map_err(|e| e.to_string())
}

/// Source `repo.current` state read: `select().from(AccountStateTable).where(eq(id, 1)).get()`.
pub fn get_state(conn: &Connection) -> Result<Option<AccountStateRow>, String> {
    conn.query_row(
        &format!(
            "SELECT id, active_account_id, active_org_id FROM \"{ACCOUNT_STATE_TABLE}\" WHERE id = 1"
        ),
        [],
        |row| {
            Ok(AccountStateRow {
                id: row.get(0)?,
                active_account_id: row.get(1)?,
                active_org_id: row.get(2)?,
            })
        },
    )
    .map(Some)
    .or_else(|e| match e {
        rusqlite::Error::QueryReturnedNoRows => Ok(None),
        other => Err(other.to_string()),
    })
}

/// Source `repo.state`: upsert `account_state` row with `ACCOUNT_STATE_ID = 1`.
pub fn upsert_state(
    conn: &Connection,
    active_account_id: Option<&str>,
    active_org_id: Option<&str>,
) -> Result<(), String> {
    conn.execute(
        &format!(
            "INSERT INTO \"{ACCOUNT_STATE_TABLE}\" (id, active_account_id, active_org_id) VALUES (1, ?1, ?2) ON CONFLICT(id) DO UPDATE SET active_account_id = ?1, active_org_id = ?2"
        ),
        rusqlite::params![active_account_id, active_org_id],
    )
    .map(|_| ())
    .map_err(|e| e.to_string())
}

/// Source `repo.current`: resolve the active account + `active_org_id` (or none).
pub fn resolve_active(conn: &Connection) -> Result<Option<(AccountRow, Option<String>)>, String> {
    let state = match get_state(conn)? {
        Some(state) => state,
        None => return Ok(None),
    };
    let account_id = match state.active_account_id {
        Some(id) => id,
        None => return Ok(None),
    };
    let account = match get_account(conn, &account_id)? {
        Some(account) => account,
        None => return Ok(None),
    };
    Ok(Some((account, state.active_org_id)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::database::database::open_in_memory;

    fn seed(conn: &Connection) {
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS account (id text PRIMARY KEY, email text NOT NULL, url text NOT NULL, access_token text NOT NULL, refresh_token text NOT NULL, token_expiry integer, time_created integer NOT NULL, time_updated integer NOT NULL);
             CREATE TABLE IF NOT EXISTS account_state (id integer PRIMARY KEY, active_account_id text, active_org_id text);
             CREATE TABLE IF NOT EXISTS control_account (email text NOT NULL, url text NOT NULL, access_token text NOT NULL, refresh_token text NOT NULL, token_expiry integer, active integer NOT NULL, time_created integer NOT NULL, time_updated integer NOT NULL, CONSTRAINT control_account_pk PRIMARY KEY(email, url));",
        )
        .unwrap();
    }

    fn row(id: &str) -> AccountRow {
        AccountRow {
            id: id.to_string(),
            email: format!("{id}@test"),
            url: "https://api.openai.com".to_string(),
            access_token: "access".to_string(),
            refresh_token: "refresh".to_string(),
            token_expiry: None,
            time_created: 1,
            time_updated: 1,
        }
    }

    #[test]
    fn persists_and_resolves_active_account() {
        let conn = open_in_memory().unwrap();
        seed(&conn);
        upsert_account(&conn, &row("acc_1")).unwrap();
        upsert_account(&conn, &row("acc_2")).unwrap();
        assert_eq!(list_accounts(&conn).unwrap().len(), 2);
        assert_eq!(get_account(&conn, "acc_1").unwrap().unwrap().id, "acc_1");

        upsert_state(&conn, Some("acc_2"), Some("org_9")).unwrap();
        let (account, org_id) = resolve_active(&conn).unwrap().unwrap();
        assert_eq!(account.id, "acc_2");
        assert_eq!(org_id.as_deref(), Some("org_9"));

        // source `use`: switch active account.
        upsert_state(&conn, Some("acc_1"), None).unwrap();
        let (account, org_id) = resolve_active(&conn).unwrap().unwrap();
        assert_eq!(account.id, "acc_1");
        assert_eq!(org_id, None);
    }

    #[test]
    fn updates_token_and_removes_account() {
        let conn = open_in_memory().unwrap();
        seed(&conn);
        upsert_account(&conn, &row("acc_1")).unwrap();
        update_token(
            &conn,
            "acc_1",
            "access2",
            "refresh2",
            Some(1_700_000_000),
            5,
        )
        .unwrap();
        let updated = get_account(&conn, "acc_1").unwrap().unwrap();
        assert_eq!(updated.access_token, "access2");
        assert_eq!(updated.token_expiry, Some(1_700_000_000));
        assert_eq!(updated.time_updated, 5);

        upsert_state(&conn, Some("acc_1"), None).unwrap();
        delete_account(&conn, "acc_1").unwrap();
        assert!(get_account(&conn, "acc_1").unwrap().is_none());
        // source `remove` clears state first, then deletes the account row.
        assert!(get_state(&conn).unwrap().is_some());
    }
}
