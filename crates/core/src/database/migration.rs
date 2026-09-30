//! Rust port of `packages/core/src/database/migration.ts` + `packages/core/src/database/migration` barrel.
//! Source pin: v1.18.30 @3104c14 — 1:1 exact translation.

use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};

use rusqlite::Connection;

pub const SERVICE_ID: &str = "DatabaseMigration";

#[derive(Debug, Clone, Copy)]
pub struct Migration {
    pub id: &'static str,
    pub up_sql: fn(&Connection) -> Result<Vec<String>, String>,
}

pub const LOCK_DESC: &str = "Semaphore.makeUnsafe(1)";
pub const ERROR_NOT_EMPTY_NO_SESSION: &str = "Database is not empty and has no session table";
pub fn legacy_timestamp_error(created_at: i64) -> String {
    format!(
        "Legacy migration timestamp {} does not match any known migration",
        created_at
    )
}

/// `Date.now()` — milliseconds since the Unix epoch, matching Effect/TS calls.
pub(crate) fn now_ms() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock before epoch")
        .as_millis() as i64
}

/// Serializes concurrent embedded initialization for one database path (source lock: Semaphore.makeUnsafe(1)).
static APPLY_LOCK: Mutex<()> = Mutex::new(());

// ---- migration/* submodules (verbatim SQL per migration, see generation) ----
pub mod m20260127222353_familiar_lady_ursula;
pub mod m20260211171708_add_project_commands;
pub mod m20260213144116_wakeful_the_professor;
pub mod m20260225215848_workspace;
pub mod m20260227213759_add_session_workspace_id;
pub mod m20260228203230_blue_harpoon;
pub mod m20260303231226_add_workspace_fields;
pub mod m20260309230000_move_org_to_state;
pub mod m20260312043431_session_message_cursor;
pub mod m20260323234822_events;
pub mod m20260410174513_workspace_name;
pub mod m20260413175956_chief_energizer;
pub mod m20260423070820_add_icon_url_override;
pub mod m20260427172553_slow_nightmare;
pub mod m20260428004200_add_session_path;
pub mod m20260501142318_next_venus;
pub mod m20260504145000_add_sync_owner;
pub mod m20260507164347_add_workspace_time;
pub mod m20260510033149_session_usage;
pub mod m20260511000411_data_migration_state;
pub mod m20260511173437_session_metadata;
pub mod m20260601010001_normalize_storage_paths;
pub mod m20260601202201_amazing_prowler;
pub mod m20260602002951_lowly_union_jack;
pub mod m20260602182828_add_project_directories;
pub mod m20260603001617_session_message_projection_indexes;
pub mod m20260603040000_session_message_projection_order;
pub mod m20260603141458_session_input_inbox;
pub mod m20260603160727_jittery_ezekiel_stane;
pub mod m20260604172448_event_sourced_session_input;
pub mod m20260605003541_add_session_context_snapshot;
pub mod m20260605042240_add_context_epoch_agent;
pub mod m20260611035744_credential;
pub mod m20260611192811_lush_chimera;
pub mod m20260612174303_project_dir_strategy;
pub mod m20260622142730_simplify_session_context_epoch;
pub mod m20260622170816_reset_v2_session_state;
pub mod m20260622202450_simplify_session_input;

/// `migrations` from `migration.gen.ts` — declaration order preserved verbatim.
pub fn all() -> Vec<Migration> {
    vec![
        Migration {
            id: m20260127222353_familiar_lady_ursula::ID,
            up_sql: m20260127222353_familiar_lady_ursula::up_sql,
        },
        Migration {
            id: m20260211171708_add_project_commands::ID,
            up_sql: m20260211171708_add_project_commands::up_sql,
        },
        Migration {
            id: m20260213144116_wakeful_the_professor::ID,
            up_sql: m20260213144116_wakeful_the_professor::up_sql,
        },
        Migration {
            id: m20260225215848_workspace::ID,
            up_sql: m20260225215848_workspace::up_sql,
        },
        Migration {
            id: m20260227213759_add_session_workspace_id::ID,
            up_sql: m20260227213759_add_session_workspace_id::up_sql,
        },
        Migration {
            id: m20260228203230_blue_harpoon::ID,
            up_sql: m20260228203230_blue_harpoon::up_sql,
        },
        Migration {
            id: m20260303231226_add_workspace_fields::ID,
            up_sql: m20260303231226_add_workspace_fields::up_sql,
        },
        Migration {
            id: m20260309230000_move_org_to_state::ID,
            up_sql: m20260309230000_move_org_to_state::up_sql,
        },
        Migration {
            id: m20260312043431_session_message_cursor::ID,
            up_sql: m20260312043431_session_message_cursor::up_sql,
        },
        Migration {
            id: m20260323234822_events::ID,
            up_sql: m20260323234822_events::up_sql,
        },
        Migration {
            id: m20260410174513_workspace_name::ID,
            up_sql: m20260410174513_workspace_name::up_sql,
        },
        Migration {
            id: m20260413175956_chief_energizer::ID,
            up_sql: m20260413175956_chief_energizer::up_sql,
        },
        Migration {
            id: m20260423070820_add_icon_url_override::ID,
            up_sql: m20260423070820_add_icon_url_override::up_sql,
        },
        Migration {
            id: m20260427172553_slow_nightmare::ID,
            up_sql: m20260427172553_slow_nightmare::up_sql,
        },
        Migration {
            id: m20260428004200_add_session_path::ID,
            up_sql: m20260428004200_add_session_path::up_sql,
        },
        Migration {
            id: m20260501142318_next_venus::ID,
            up_sql: m20260501142318_next_venus::up_sql,
        },
        Migration {
            id: m20260504145000_add_sync_owner::ID,
            up_sql: m20260504145000_add_sync_owner::up_sql,
        },
        Migration {
            id: m20260507164347_add_workspace_time::ID,
            up_sql: m20260507164347_add_workspace_time::up_sql,
        },
        Migration {
            id: m20260510033149_session_usage::ID,
            up_sql: m20260510033149_session_usage::up_sql,
        },
        Migration {
            id: m20260511000411_data_migration_state::ID,
            up_sql: m20260511000411_data_migration_state::up_sql,
        },
        Migration {
            id: m20260511173437_session_metadata::ID,
            up_sql: m20260511173437_session_metadata::up_sql,
        },
        Migration {
            id: m20260601010001_normalize_storage_paths::ID,
            up_sql: m20260601010001_normalize_storage_paths::up_sql,
        },
        Migration {
            id: m20260601202201_amazing_prowler::ID,
            up_sql: m20260601202201_amazing_prowler::up_sql,
        },
        Migration {
            id: m20260602002951_lowly_union_jack::ID,
            up_sql: m20260602002951_lowly_union_jack::up_sql,
        },
        Migration {
            id: m20260602182828_add_project_directories::ID,
            up_sql: m20260602182828_add_project_directories::up_sql,
        },
        Migration {
            id: m20260603001617_session_message_projection_indexes::ID,
            up_sql: m20260603001617_session_message_projection_indexes::up_sql,
        },
        Migration {
            id: m20260603040000_session_message_projection_order::ID,
            up_sql: m20260603040000_session_message_projection_order::up_sql,
        },
        Migration {
            id: m20260603141458_session_input_inbox::ID,
            up_sql: m20260603141458_session_input_inbox::up_sql,
        },
        Migration {
            id: m20260603160727_jittery_ezekiel_stane::ID,
            up_sql: m20260603160727_jittery_ezekiel_stane::up_sql,
        },
        Migration {
            id: m20260604172448_event_sourced_session_input::ID,
            up_sql: m20260604172448_event_sourced_session_input::up_sql,
        },
        Migration {
            id: m20260605003541_add_session_context_snapshot::ID,
            up_sql: m20260605003541_add_session_context_snapshot::up_sql,
        },
        Migration {
            id: m20260605042240_add_context_epoch_agent::ID,
            up_sql: m20260605042240_add_context_epoch_agent::up_sql,
        },
        Migration {
            id: m20260611035744_credential::ID,
            up_sql: m20260611035744_credential::up_sql,
        },
        Migration {
            id: m20260611192811_lush_chimera::ID,
            up_sql: m20260611192811_lush_chimera::up_sql,
        },
        Migration {
            id: m20260612174303_project_dir_strategy::ID,
            up_sql: m20260612174303_project_dir_strategy::up_sql,
        },
        Migration {
            id: m20260622142730_simplify_session_context_epoch::ID,
            up_sql: m20260622142730_simplify_session_context_epoch::up_sql,
        },
        Migration {
            id: m20260622170816_reset_v2_session_state::ID,
            up_sql: m20260622170816_reset_v2_session_state::up_sql,
        },
        Migration {
            id: m20260622202450_simplify_session_input::ID,
            up_sql: m20260622202450_simplify_session_input::up_sql,
        },
    ]
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TableName {
    pub name: String,
}

fn table_names(conn: &Connection) -> Result<Vec<TableName>, String> {
    let mut stmt = conn
        .prepare("SELECT name FROM sqlite_master WHERE type = 'table' AND name NOT LIKE 'sqlite_%'")
        .map_err(|e| e.to_string())?;
    let rows = stmt
        .query_map([], |row| Ok(TableName { name: row.get(0)? }))
        .map_err(|e| e.to_string())?;
    let mut out = Vec::new();
    for r in rows {
        out.push(r.map_err(|e| e.to_string())?);
    }
    Ok(out)
}

fn completed_ids(conn: &Connection) -> Result<Vec<String>, String> {
    let mut stmt = conn
        .prepare("SELECT id FROM \"migration\"")
        .map_err(|e| e.to_string())?;
    let rows = stmt
        .query_map([], |row| row.get::<_, String>(0))
        .map_err(|e| e.to_string())?;
    let mut out = Vec::new();
    for r in rows {
        out.push(r.map_err(|e| e.to_string())?);
    }
    Ok(out)
}

fn insert_completed(conn: &Connection, id: &str) -> Result<(), String> {
    conn.execute(
        "INSERT INTO \"migration\" (id, time_completed) VALUES (?1, ?2)",
        rusqlite::params![id, now_ms()],
    )
    .map(|_| ())
    .map_err(|e| e.to_string())
}

/// Port of `DatabaseMigration.apply(db)` — fresh installs build the push schema and seed the
/// journal; installs with a `session` table replay tracked migrations via `apply_only`.
pub fn apply(conn: &mut Connection) -> Result<(), String> {
    let _guard = APPLY_LOCK
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let tables = table_names(conn)?;
    if tables.iter().any(|t| t.name == "session") {
        return apply_only(conn, &all());
    }
    if !tables.is_empty() {
        return Err(ERROR_NOT_EMPTY_NO_SESSION.to_string());
    }
    let tx = conn.transaction().map_err(|e| e.to_string())?;
    for statement in super::schema_gen::up_sql_statements() {
        tx.execute_batch(statement).map_err(|e| e.to_string())?;
    }
    tx.execute_batch(
        "CREATE TABLE \"migration\" (id TEXT PRIMARY KEY, time_completed INTEGER NOT NULL)",
    )
    .map_err(|e| e.to_string())?;
    for m in all() {
        tx.execute(
            "INSERT INTO \"migration\" (id, time_completed) VALUES (?1, ?2)",
            rusqlite::params![m.id, now_ms()],
        )
        .map_err(|e| e.to_string())?;
    }
    tx.commit().map_err(|e| e.to_string())
}

/// Port of `DatabaseMigration.applyOnly(db, input)`.
pub fn apply_only(conn: &mut Connection, input: &[Migration]) -> Result<(), String> {
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS \"migration\" (id TEXT PRIMARY KEY, time_completed INTEGER NOT NULL)",
    )
    .map_err(|e| e.to_string())?;
    let mut completed: std::collections::HashSet<String> =
        completed_ids(conn)?.into_iter().collect();
    if completed.is_empty() {
        // Existing installs used Drizzle's migration journal. Seed the new journal once so
        // TypeScript migrations don't replay old SQL.
        let drizzle_exists: Option<String> = conn
            .query_row(
                "SELECT name FROM sqlite_master WHERE type = 'table' AND name = '__drizzle_migrations'",
                [],
                |row| row.get(0),
            )
            .ok();
        if drizzle_exists.is_some() {
            let named: bool = conn
                .prepare("SELECT name FROM pragma_table_info('__drizzle_migrations')")
                .map_err(|e| e.to_string())?
                .query_map([], |row| row.get::<_, String>(0))
                .map_err(|e| e.to_string())?
                .filter_map(|r| r.ok())
                .any(|column| column == "name");
            if named {
                conn.execute(
                    "INSERT OR IGNORE INTO \"migration\" (id, time_completed) SELECT name, ?1 FROM \"__drizzle_migrations\" WHERE name IS NOT NULL",
                    rusqlite::params![now_ms()],
                )
                .map_err(|e| e.to_string())?;
            } else {
                let mut stmt = conn
                    .prepare(
                        "SELECT created_at, strftime('%Y%m%d%H%M%S', created_at / 1000, 'unixepoch') AS prefix FROM \"__drizzle_migrations\" WHERE created_at IS NOT NULL",
                    )
                    .map_err(|e| e.to_string())?;
                let rows = stmt
                    .query_map([], |row| {
                        Ok((row.get::<_, i64>(0)?, row.get::<_, String>(1)?))
                    })
                    .map_err(|e| e.to_string())?;
                let mut entries = Vec::new();
                for r in rows {
                    entries.push(r.map_err(|e| e.to_string())?);
                }
                for (created_at, prefix) in entries {
                    let migration = match input
                        .iter()
                        .find(|m| m.id.starts_with(&format!("{}_", prefix)))
                    {
                        Some(m) => m,
                        None => return Err(legacy_timestamp_error(created_at)),
                    };
                    conn.execute(
                        "INSERT OR IGNORE INTO \"migration\" (id, time_completed) VALUES (?1, ?2)",
                        rusqlite::params![migration.id, now_ms()],
                    )
                    .map_err(|e| e.to_string())?;
                }
            }
            completed = completed_ids(conn)?.into_iter().collect();
        }
    }
    for migration in input {
        if completed.contains(migration.id) {
            continue;
        }
        let tx = conn.transaction().map_err(|e| e.to_string())?;
        let statements = (migration.up_sql)(&tx).map_err(|e| e.to_string())?;
        for statement in &statements {
            tx.execute_batch(statement).map_err(|e| e.to_string())?;
        }
        tx.execute(
            "INSERT INTO \"migration\" (id, time_completed) VALUES (?1, ?2)",
            rusqlite::params![migration.id, now_ms()],
        )
        .map_err(|e| e.to_string())?;
        tx.commit().map_err(|e| e.to_string())?;
    }
    Ok(())
}
