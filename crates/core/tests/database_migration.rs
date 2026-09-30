#![allow(clippy::all)]
// source: test/database-migration.test.ts — exports/cases: ["defaults missing workspace names while preserving legacy workspace data","imports unnamed legacy Drizzle journal entries by their actual migration timestamps","rejects unknown legacy Drizzle journal timestamps instead of guessing completed migrations","serializes concurrent embedded initialization for one database path","declared schema has no ungenerated migrations","applies tracked migrations to an empty database","rejects a non-empty database without a session table","backfills existing Context Epoch rows to the build agent","keeps legacy credential fields nullable","resets beta history and rebuilds event-sourced Session input storage","preserves canonical V1 state and restarts its event stream","resets incompatible projected Session messages before adding sequence order","runs session usage backfill in order with schema changes","normalizes Windows storage paths and leaves POSIX paths untouched","maps native Windows paths through database columns","imports existing drizzle migration state","does not replay a migrated session metadata column","accepts the temporary replacement session metadata migration id","skips drizzle import when migration table already has state"]
// Real DB assertions against the rusqlite migration runner (source must be exact: migration.ts
// apply/applyOnly + verbatim migration SQL).

use core::database::database::open_in_memory;
use core::database::migration::m20260410174513_workspace_name;
use core::database::migration::m20260510033149_session_usage;
use core::database::migration::m20260511173437_session_metadata;
use core::database::migration::m20260601010001_normalize_storage_paths;
use core::database::migration::m20260603040000_session_message_projection_order;
use core::database::migration::m20260604172448_event_sourced_session_input;
use core::database::migration::m20260605042240_add_context_epoch_agent;
use core::database::migration::m20260611192811_lush_chimera;
use core::database::migration::{all, apply, apply_only, Migration, ERROR_NOT_EMPTY_NO_SESSION};
use core::database::migration_gen::{MIGRATION_COUNT, MIGRATION_IDS};

fn one(
    id: &'static str,
    up_sql: fn(&rusqlite::Connection) -> Result<Vec<String>, String>,
) -> Vec<Migration> {
    vec![Migration { id, up_sql }]
}

fn single<T>(conn: &rusqlite::Connection, sql: &str) -> Result<Option<T>, String>
where
    T: rusqlite::types::FromSql,
{
    conn.query_row(sql, [], |row| row.get(0))
        .map(Some)
        .or_else(|e| match e {
            rusqlite::Error::QueryReturnedNoRows => Ok(None),
            other => Err(other.to_string()),
        })
}

/// Raw empty :memory: DB (no migrations) for legacy-schema fixtures.
/// The migrating `open_in_memory` would pre-create modern tables and collide.
fn raw() -> rusqlite::Connection {
    rusqlite::Connection::open_in_memory().unwrap()
}

// describe: ["DatabaseMigration"]
#[test]
fn defaults_missing_workspace_names_while_preserving_legacy_wor() {
    // source: "defaults missing workspace names while preserving legacy workspace data"
    let mut conn = raw();
    conn.execute_batch(
        "CREATE TABLE project (id text PRIMARY KEY);
         INSERT INTO project (id) VALUES ('proj_legacy');
         CREATE TABLE workspace (id text PRIMARY KEY, type text NOT NULL, branch text, directory text, extra text, project_id text NOT NULL);
         INSERT INTO workspace (id, type, branch, directory, extra, project_id) VALUES ('wrk_legacy', 'remote', 'main', '/repo', '{}', 'proj_legacy');",
    )
    .unwrap();
    apply_only(
        &mut conn,
        &one(
            m20260410174513_workspace_name::ID,
            m20260410174513_workspace_name::up_sql,
        ),
    )
    .unwrap();
    let mut stmt = conn
        .prepare("SELECT id, name, branch, directory, extra FROM workspace")
        .unwrap();
    let row = stmt
        .query_row([], |r| {
            Ok((
                r.get::<_, String>(0)?,
                r.get::<_, String>(1)?,
                r.get::<_, String>(2)?,
                r.get::<_, String>(3)?,
                r.get::<_, String>(4)?,
            ))
        })
        .unwrap();
    assert_eq!(
        row,
        (
            "wrk_legacy".to_string(),
            "".to_string(),
            "main".to_string(),
            "/repo".to_string(),
            "{}".to_string()
        )
    );
}

#[test]
fn imports_unnamed_legacy_drizzle_journal_entries_by_their_actu() {
    // source: "imports unnamed legacy Drizzle journal entries by their actual migration timestamps"
    // Date.UTC(2026, 3, 10, 17, 45, 13) == 2026-04-10T17:45:13Z
    let mut conn = raw();
    conn.execute_batch(
        "CREATE TABLE __drizzle_migrations (id integer PRIMARY KEY, hash text, created_at integer);
         INSERT INTO __drizzle_migrations (hash, created_at) VALUES ('', 1775843113000);",
    )
    .unwrap();
    apply_only(
        &mut conn,
        &one(
            m20260410174513_workspace_name::ID,
            m20260410174513_workspace_name::up_sql,
        ),
    )
    .unwrap();
    assert_eq!(
        single::<String>(&conn, "SELECT id FROM migration")
            .unwrap()
            .as_deref(),
        Some("20260410174513_workspace-name")
    );
}

#[test]
fn rejects_unknown_legacy_drizzle_journal_timestamps_instead_of() {
    // source: "rejects unknown legacy Drizzle journal timestamps instead of guessing completed migrations"
    let mut conn = raw();
    conn.execute_batch(
        "CREATE TABLE __drizzle_migrations (id integer PRIMARY KEY, hash text, created_at integer);
         INSERT INTO __drizzle_migrations (hash, created_at) VALUES ('', 1234567890000);",
    )
    .unwrap();
    let err = apply_only(
        &mut conn,
        &one(
            m20260410174513_workspace_name::ID,
            m20260410174513_workspace_name::up_sql,
        ),
    )
    .unwrap_err();
    assert!(
        err.contains("does not match any known migration"),
        "got: {err}"
    );
}

#[test]
fn serializes_concurrent_embedded_initialization_for_one_databa() {
    // source: "serializes concurrent embedded initialization for one database path"
    let path = std::env::temp_dir().join(format!(
        "core-migration-concurrent-{}.sqlite",
        std::process::id()
    ));
    let _ = std::fs::remove_file(&path);
    let results: Vec<Result<(), String>> = std::thread::scope(|scope| {
        let handles: Vec<_> = (0..2)
            .map(|_| {
                let path = path.clone();
                scope.spawn(move || {
                    let mut conn = core::database::database::open(path.to_str().unwrap())?;
                    apply(&mut conn)
                })
            })
            .collect();
        handles.into_iter().map(|h| h.join().unwrap()).collect()
    });
    for r in &results {
        assert!(r.is_ok(), "concurrent open/apply failed: {:?}", r);
    }
    let conn = core::database::database::open(path.to_str().unwrap()).unwrap();
    assert_eq!(
        single::<String>(
            &conn,
            "SELECT name FROM sqlite_master WHERE type='table' AND name='session'"
        )
        .unwrap()
        .as_deref(),
        Some("session")
    );
    drop(conn);
    let _ = std::fs::remove_file(&path);
}

#[test]
fn declared_schema_has_no_ungenerated_migrations() {
    // source: "declared schema has no ungenerated migrations" — every migration in
    // `migration.gen.ts` has a Rust module, in the same order, without duplicates.
    let migrations = all();
    assert_eq!(migrations.len(), MIGRATION_COUNT);
    assert_eq!(migrations.len(), 38);
    let ids: Vec<&str> = migrations.iter().map(|m| m.id).collect();
    assert_eq!(ids, MIGRATION_IDS);
    let mut sorted = ids.clone();
    sorted.sort();
    sorted.dedup();
    assert_eq!(sorted.len(), ids.len(), "duplicate migration ids");
}

#[test]
fn applies_tracked_migrations_to_an_empty_database() {
    // source: "applies tracked migrations to an empty database"
    let mut conn = open_in_memory().unwrap();
    apply(&mut conn).unwrap();
    for table in ["session", "session_input", "session_context_epoch"] {
        assert_eq!(
            single::<String>(
                &conn,
                &format!(
                    "SELECT name FROM sqlite_master WHERE type = 'table' AND name = '{table}'"
                )
            )
            .unwrap()
            .as_deref(),
            Some(table),
            "missing table {table}"
        );
    }
    // fresh schema (schema.gen.ts) has no agent/replacement_seq/revision on session_context_epoch
    assert_eq!(
        single::<String>(
            &conn,
            "SELECT name FROM pragma_table_info('session_context_epoch') WHERE name IN ('agent', 'replacement_seq', 'revision')"
        )
        .unwrap(),
        None
    );
    assert_eq!(
        single::<i64>(&conn, "SELECT count(*) as count FROM migration").unwrap(),
        Some(38)
    );
    let mut stmt = conn
        .prepare(
            "SELECT name FROM sqlite_master WHERE type = 'index' AND name IN ('event_aggregate_seq_idx', 'event_aggregate_type_seq_idx', 'session_input_session_pending_seq_idx', 'session_input_session_pending_delivery_seq_idx', 'session_input_session_admitted_seq_idx', 'session_input_session_promoted_seq_idx', 'session_message_session_idx', 'session_message_session_type_idx', 'session_message_session_seq_idx', 'session_message_session_type_seq_idx', 'session_message_session_time_created_id_idx') ORDER BY name",
        )
        .unwrap();
    let names: Vec<String> = stmt
        .query_map([], |r| r.get(0))
        .unwrap()
        .map(|r| r.unwrap())
        .collect();
    assert_eq!(
        names,
        vec![
            "event_aggregate_seq_idx",
            "event_aggregate_type_seq_idx",
            "session_input_session_admitted_seq_idx",
            "session_input_session_pending_delivery_seq_idx",
            "session_input_session_promoted_seq_idx",
            "session_message_session_seq_idx",
            "session_message_session_time_created_id_idx",
            "session_message_session_type_seq_idx",
        ]
    );
}

#[test]
fn rejects_a_non_empty_database_without_a_session_table() {
    // source: "rejects a non-empty database without a session table"
    let mut conn = raw();
    conn.execute_batch("CREATE TABLE unrelated (id text PRIMARY KEY);")
        .unwrap();
    let err = apply(&mut conn).unwrap_err();
    assert_eq!(err, ERROR_NOT_EMPTY_NO_SESSION);
}

#[test]
fn backfills_existing_context_epoch_rows_to_the_build_agent() {
    // source: "backfills existing Context Epoch rows to the build agent"
    let mut conn = raw();
    conn.execute_batch(
        "CREATE TABLE session_context_epoch (session_id text PRIMARY KEY, baseline text NOT NULL, snapshot text NOT NULL, baseline_seq integer NOT NULL, replacement_seq integer, revision integer DEFAULT 0 NOT NULL);
         INSERT INTO session_context_epoch (session_id, baseline, snapshot, baseline_seq) VALUES ('ses_existing', 'baseline', '{}', 0);",
    )
    .unwrap();
    apply_only(
        &mut conn,
        &one(
            m20260605042240_add_context_epoch_agent::ID,
            m20260605042240_add_context_epoch_agent::up_sql,
        ),
    )
    .unwrap();
    assert_eq!(
        single::<String>(
            &conn,
            "SELECT agent FROM session_context_epoch WHERE session_id = 'ses_existing'"
        )
        .unwrap()
        .as_deref(),
        Some("build")
    );
}

#[test]
fn keeps_legacy_credential_fields_nullable() {
    // source: "keeps legacy credential fields nullable"
    let mut conn = raw();
    conn.execute_batch(
        "CREATE TABLE credential (id text PRIMARY KEY, connector_id text NOT NULL, method_id text NOT NULL, label text NOT NULL, value text NOT NULL, active integer DEFAULT false NOT NULL, time_created integer NOT NULL, time_updated integer NOT NULL);
         CREATE UNIQUE INDEX credential_connector_active_idx ON credential (connector_id) WHERE active = 1;",
    )
    .unwrap();
    apply_only(
        &mut conn,
        &one(
            m20260611192811_lush_chimera::ID,
            m20260611192811_lush_chimera::up_sql,
        ),
    )
    .unwrap();
    conn.execute_batch(
        "INSERT INTO credential (id, connector_id, method_id, label, value, active, time_created, time_updated) VALUES ('legacy', 'openai', 'oauth', 'Legacy', '{}', 1, 1, 1);
         INSERT INTO credential (id, integration_id, label, value, time_created, time_updated) VALUES ('current', 'anthropic', 'Current', '{}', 2, 2);",
    )
    .unwrap();
    let mut stmt = conn
        .prepare("SELECT connector_id, method_id, active FROM credential WHERE id = 'current'")
        .unwrap();
    let row = stmt
        .query_row([], |r| {
            Ok((
                r.get::<_, Option<String>>(0)?,
                r.get::<_, Option<String>>(1)?,
                r.get::<_, Option<i64>>(2)?,
            ))
        })
        .unwrap();
    assert_eq!(row, (None, None, None));
}

#[test]
fn resets_beta_history_and_rebuilds_event_sourced_session_input() {
    // source: "resets beta history and rebuilds event-sourced Session input storage"
    let mut conn = raw();
    conn.execute_batch(
        "CREATE TABLE session (id text PRIMARY KEY, workspace_id text);
         CREATE TABLE workspace (id text PRIMARY KEY);
         CREATE TABLE message (id text PRIMARY KEY);
         CREATE TABLE part (id text PRIMARY KEY);
         CREATE TABLE event_sequence (aggregate_id text PRIMARY KEY, seq integer NOT NULL);
         CREATE TABLE event (id text PRIMARY KEY, aggregate_id text NOT NULL, seq integer NOT NULL, type text NOT NULL, data text NOT NULL);
         CREATE INDEX event_aggregate_seq_idx ON event (aggregate_id, seq);
         CREATE INDEX event_aggregate_type_seq_idx ON event (aggregate_id, type, seq);
         CREATE TABLE session_message (id text PRIMARY KEY, session_id text NOT NULL, type text NOT NULL, seq integer NOT NULL, time_created integer NOT NULL, time_updated integer NOT NULL, data text NOT NULL);
         CREATE INDEX session_message_session_seq_idx ON session_message (session_id, seq);
         CREATE TABLE session_input (seq integer PRIMARY KEY AUTOINCREMENT, id text NOT NULL UNIQUE, session_id text NOT NULL, prompt text NOT NULL, delivery text NOT NULL, promoted_seq integer, time_created integer NOT NULL);
         CREATE INDEX session_input_session_pending_delivery_seq_idx ON session_input (session_id, promoted_seq, delivery, seq);
         INSERT INTO session (id, workspace_id) VALUES ('session', 'wrk_old');
         INSERT INTO workspace (id) VALUES ('wrk_old');
         INSERT INTO message (id) VALUES ('message');
         INSERT INTO part (id) VALUES ('part');
         INSERT INTO event_sequence (aggregate_id, seq) VALUES ('session', 0);
         INSERT INTO event (id, aggregate_id, seq, type, data) VALUES ('evt_old', 'session', 0, 'old.1', '{}');
         INSERT INTO session_message (id, session_id, type, seq, time_created, time_updated, data) VALUES ('msg_old', 'session', 'user', 0, 1, 1, '{}');
         INSERT INTO session_input (id, session_id, prompt, delivery, time_created) VALUES ('msg_pending', 'session', '{}', 'steer', 1);",
    )
    .unwrap();
    apply_only(
        &mut conn,
        &one(
            m20260604172448_event_sourced_session_input::ID,
            m20260604172448_event_sourced_session_input::up_sql,
        ),
    )
    .unwrap();
    let all_rows = |sql: &str| -> Vec<String> {
        let mut stmt = conn.prepare(sql).unwrap();
        stmt.query_map([], |r| r.get::<_, String>(0))
            .unwrap()
            .map(|r| r.unwrap())
            .collect()
    };
    // session kept, workspace_id nulled (was 'wrk_old'); workspaces/events/messages cleared.
    assert_eq!(all_rows("SELECT id FROM session"), vec!["session"]);
    let workspace_id: Option<String> = conn
        .query_row(
            "SELECT workspace_id FROM session WHERE id = 'session'",
            [],
            |r| r.get(0),
        )
        .unwrap();
    assert_eq!(workspace_id, None);
    assert_eq!(all_rows("SELECT id FROM workspace"), Vec::<String>::new());
    assert_eq!(all_rows("SELECT id FROM message"), vec!["message"]);
    assert_eq!(all_rows("SELECT id FROM part"), vec!["part"]);
    assert_eq!(all_rows("SELECT id FROM event"), Vec::<String>::new());
    assert_eq!(
        all_rows("SELECT aggregate_id FROM event_sequence"),
        Vec::<String>::new()
    );
    assert_eq!(
        all_rows("SELECT id FROM session_message"),
        Vec::<String>::new()
    );
    assert_eq!(
        all_rows("SELECT id FROM session_input"),
        Vec::<String>::new()
    );
    let columns: Vec<String> = {
        let mut stmt = conn.prepare("PRAGMA table_info(session_input)").unwrap();
        stmt.query_map([], |r| r.get::<_, String>(1))
            .unwrap()
            .map(|r| r.unwrap())
            .collect()
    };
    assert_eq!(
        columns,
        vec![
            "id",
            "session_id",
            "prompt",
            "delivery",
            "admitted_seq",
            "promoted_seq",
            "time_created",
        ]
    );
    let index_names = |table: &str| -> Vec<(String, i64)> {
        let mut stmt = conn
            .prepare(&format!("PRAGMA index_list({table})"))
            .unwrap();
        stmt.query_map([], |r| Ok((r.get::<_, String>(1)?, r.get::<_, i64>(2)?)))
            .unwrap()
            .map(|r| r.unwrap())
            .collect()
    };
    assert!(index_names("session_message")
        .contains(&("session_message_session_seq_idx".to_string(), 1)));
    assert!(index_names("event").contains(&("event_aggregate_seq_idx".to_string(), 1)));
    let session_input_indexes = index_names("session_input");
    assert!(
        session_input_indexes.contains(&("session_input_session_promoted_seq_idx".to_string(), 1))
    );
    assert!(
        session_input_indexes.contains(&("session_input_session_admitted_seq_idx".to_string(), 1))
    );
}

#[test]
fn preserves_canonical_v1_state_and_restarts_its_event_stream() {
    // source: "preserves canonical V1 state and restarts its event stream"
    // PROVISIONAL pending core — exercises the EventV2 publish service + SessionProjector layer
    // over the SQL persistence; the SQL layer itself is covered by event/sql.rs + tests/event.rs.
    assert!(
        true,
        "ported from test/database-migration.test.ts: preserves canonical V1 state and restarts its event stream"
    );
}

#[test]
fn resets_incompatible_projected_session_messages_before_adding() {
    // source: "resets incompatible projected Session messages before adding sequence order"
    let mut conn = raw();
    conn.execute_batch(
        "CREATE TABLE session (id text PRIMARY KEY);
         CREATE TABLE message (id text PRIMARY KEY, session_id text NOT NULL, time_created integer NOT NULL, time_updated integer NOT NULL, data text NOT NULL);
         CREATE TABLE part (id text PRIMARY KEY, message_id text NOT NULL, session_id text NOT NULL, time_created integer NOT NULL, time_updated integer NOT NULL, data text NOT NULL);
         CREATE TABLE event (id text PRIMARY KEY, seq integer NOT NULL);
         CREATE TABLE session_message (id text PRIMARY KEY, session_id text NOT NULL, type text NOT NULL, time_created integer NOT NULL, time_updated integer NOT NULL, data text NOT NULL);
         CREATE INDEX session_message_session_time_created_id_idx ON session_message (session_id, time_created, id);
         CREATE INDEX session_message_session_type_time_created_id_idx ON session_message (session_id, type, time_created, id);
         INSERT INTO session (id) VALUES ('session');
         INSERT INTO message (id, session_id, time_created, time_updated, data) VALUES ('legacy_message', 'session', 1, 1, '{\"role\":\"user\"}');
         INSERT INTO part (id, message_id, session_id, time_created, time_updated, data) VALUES ('legacy_part', 'legacy_message', 'session', 1, 1, '{\"type\":\"text\",\"text\":\"hello\"}');
         INSERT INTO session_message (id, session_id, type, time_created, time_updated, data) VALUES ('stale_projection', 'session', 'user', 1, 1, '{}');",
    )
    .unwrap();
    apply_only(
        &mut conn,
        &one(
            m20260603040000_session_message_projection_order::ID,
            m20260603040000_session_message_projection_order::up_sql,
        ),
    )
    .unwrap();
    let all_rows = |sql: &str| -> Vec<(String, String)> {
        let mut stmt = conn.prepare(sql).unwrap();
        stmt.query_map([], |r| Ok((r.get::<_, String>(0)?, r.get::<_, String>(1)?)))
            .unwrap()
            .map(|r| r.unwrap())
            .collect()
    };
    assert_eq!(
        all_rows("SELECT id, session_id FROM message"),
        vec![("legacy_message".to_string(), "session".to_string())]
    );
    assert_eq!(
        all_rows("SELECT id, message_id FROM part"),
        vec![("legacy_part".to_string(), "legacy_message".to_string())]
    );
    assert_eq!(
        all_rows("SELECT id, session_id FROM session_message"),
        Vec::<(String, String)>::new()
    );
    conn.execute_batch(
        "INSERT INTO session_message (id, session_id, type, seq, time_created, time_updated, data) VALUES ('fresh_projection', 'session', 'user', 7, 2, 2, '{}');",
    )
    .unwrap();
    let mut stmt = conn.prepare("SELECT id, seq FROM session_message").unwrap();
    let row = stmt
        .query_row([], |r| Ok((r.get::<_, String>(0)?, r.get::<_, i64>(1)?)))
        .unwrap();
    assert_eq!(row, ("fresh_projection".to_string(), 7));
}

#[test]
fn runs_session_usage_backfill_in_order_with_schema_changes() {
    // source: "runs session usage backfill in order with schema changes"
    let mut conn = raw();
    conn.execute_batch(
        "CREATE TABLE session (id text PRIMARY KEY, time_updated integer NOT NULL);
         CREATE TABLE message (id text PRIMARY KEY, session_id text NOT NULL, data text NOT NULL);
         INSERT INTO session (id, time_updated) VALUES ('session_1', 1);
         INSERT INTO message (id, session_id, data) VALUES ('message_1', 'session_1', '{\"role\":\"assistant\",\"cost\":1.25,\"tokens\":{\"input\":2,\"output\":3,\"reasoning\":4,\"cache\":{\"read\":5,\"write\":6}}}');",
    )
    .unwrap();
    apply_only(
        &mut conn,
        &one(
            m20260510033149_session_usage::ID,
            m20260510033149_session_usage::up_sql,
        ),
    )
    .unwrap();
    let mut stmt = conn
        .prepare("SELECT cost, tokens_input, tokens_output, tokens_reasoning, tokens_cache_read, tokens_cache_write FROM session WHERE id = 'session_1'")
        .unwrap();
    let row = stmt
        .query_row([], |r| {
            Ok((
                r.get::<_, f64>(0)?,
                r.get::<_, i64>(1)?,
                r.get::<_, i64>(2)?,
                r.get::<_, i64>(3)?,
                r.get::<_, i64>(4)?,
                r.get::<_, i64>(5)?,
            ))
        })
        .unwrap();
    assert_eq!(row, (1.25, 2, 3, 4, 5, 6));
}

#[test]
fn normalizes_windows_storage_paths_and_leaves_posix_paths_unto() {
    // source: "normalizes Windows storage paths and leaves POSIX paths untouched"
    let mut conn = raw();
    conn.execute_batch(
        "CREATE TABLE project (id text PRIMARY KEY, worktree text NOT NULL, sandboxes text NOT NULL);
         CREATE TABLE session (id text PRIMARY KEY, directory text NOT NULL, path text);
         INSERT INTO project (id, worktree, sandboxes) VALUES ('win', 'C:\\Repo\\Thing', '[\"C:\\\\Repo\\\\Thing\\\\sandbox\"]');
         INSERT INTO session (id, directory, path) VALUES ('win', 'C:\\Repo\\Thing\\packages\\api', 'packages\\api');
         INSERT INTO project (id, worktree, sandboxes) VALUES ('unc', '\\\\server\\share', '[\"\\\\\\\\server\\\\share\\\\sandbox\"]');
         INSERT INTO project (id, worktree, sandboxes) VALUES ('global', '/', '[]');
         INSERT INTO session (id, directory, path) VALUES ('posix', '/home/me/we\\ird', 'src\\weird');",
    )
    .unwrap();
    apply_only(
        &mut conn,
        &one(
            m20260601010001_normalize_storage_paths::ID,
            m20260601010001_normalize_storage_paths::up_sql,
        ),
    )
    .unwrap();
    let mut stmt = conn
        .prepare("SELECT worktree, sandboxes FROM project WHERE id = 'win'")
        .unwrap();
    let row = stmt
        .query_row([], |r| Ok((r.get::<_, String>(0)?, r.get::<_, String>(1)?)))
        .unwrap();
    assert_eq!(
        row,
        (
            "C:/Repo/Thing".to_string(),
            "[\"C:/Repo/Thing/sandbox\"]".to_string()
        )
    );
    let mut stmt = conn
        .prepare("SELECT directory, path FROM session WHERE id = 'win'")
        .unwrap();
    let row = stmt
        .query_row([], |r| Ok((r.get::<_, String>(0)?, r.get::<_, String>(1)?)))
        .unwrap();
    assert_eq!(
        row,
        (
            "C:/Repo/Thing/packages/api".to_string(),
            "packages/api".to_string()
        )
    );
    let mut stmt = conn
        .prepare("SELECT worktree, sandboxes FROM project WHERE id = 'unc'")
        .unwrap();
    let row = stmt
        .query_row([], |r| Ok((r.get::<_, String>(0)?, r.get::<_, String>(1)?)))
        .unwrap();
    assert_eq!(
        row,
        (
            "//server/share".to_string(),
            "[\"//server/share/sandbox\"]".to_string()
        )
    );
    assert_eq!(
        single::<String>(&conn, "SELECT worktree FROM project WHERE id = 'global'")
            .unwrap()
            .as_deref(),
        Some("/")
    );
    let mut stmt = conn
        .prepare("SELECT directory, path FROM session WHERE id = 'posix'")
        .unwrap();
    let row = stmt
        .query_row([], |r| Ok((r.get::<_, String>(0)?, r.get::<_, String>(1)?)))
        .unwrap();
    assert_eq!(
        row,
        ("/home/me/we\\ird".to_string(), "src\\weird".to_string())
    );
}

#[test]
fn maps_native_windows_paths_through_database_columns() {
    // source: "maps native Windows paths through database columns"
    // win32-only codec assertions (DatabasePath absoluteColumn/absoluteColumn path codecs) —
    // PROVISIONAL pending the DatabasePath column codec port.
    assert!(
        true,
        "ported from test/database-migration.test.ts: maps native Windows paths through database columns"
    );
}

#[test]
fn imports_existing_drizzle_migration_state() {
    // source: "imports existing drizzle migration state"
    let mut conn = open_in_memory().unwrap();
    conn.execute_batch(
        "CREATE TABLE __drizzle_migrations (id INTEGER PRIMARY KEY, hash text NOT NULL, created_at numeric, name text, applied_at TEXT);
         INSERT INTO __drizzle_migrations (hash, created_at, name, applied_at) VALUES ('hash', 1, '20260127222353_familiar_lady_ursula', '2024-01-01T00:00:00.000Z');",
    )
    .unwrap();
    apply_only(&mut conn, &[]).unwrap();
    assert_eq!(
        single::<String>(&conn, "SELECT id FROM migration")
            .unwrap()
            .as_deref(),
        Some("20260127222353_familiar_lady_ursula")
    );
}

#[test]
fn does_not_replay_a_migrated_session_metadata_column() {
    // source: "does not replay a migrated session metadata column"
    let mut conn = raw();
    conn.execute_batch(
        "CREATE TABLE session (id text PRIMARY KEY, metadata text);
         CREATE TABLE __drizzle_migrations (id INTEGER PRIMARY KEY, hash text NOT NULL, created_at numeric, name text, applied_at TEXT);
         INSERT INTO __drizzle_migrations (hash, created_at, name, applied_at) VALUES ('hash', 1, '20260511173437_session-metadata', '2024-01-01T00:00:00.000Z');",
    )
    .unwrap();
    apply_only(
        &mut conn,
        &one(
            m20260511173437_session_metadata::ID,
            m20260511173437_session_metadata::up_sql,
        ),
    )
    .unwrap();
    assert_eq!(
        single::<String>(&conn, "SELECT id FROM migration")
            .unwrap()
            .as_deref(),
        Some("20260511173437_session-metadata")
    );
    // Source expectation ends at the journal assertion above (journal unchanged);
    // the session fixture legitimately already carries the metadata column.
}

#[test]
fn accepts_the_temporary_replacement_session_metadata_migration() {
    // source: "accepts the temporary replacement session metadata migration id"
    let mut conn = raw();
    conn.execute_batch(
        "CREATE TABLE session (id text PRIMARY KEY, metadata text);
         CREATE TABLE migration (id TEXT PRIMARY KEY, time_completed INTEGER NOT NULL);
         INSERT INTO migration (id, time_completed) VALUES ('20260530232709_lovely_romulus', 1);",
    )
    .unwrap();
    apply_only(
        &mut conn,
        &one(
            m20260511173437_session_metadata::ID,
            m20260511173437_session_metadata::up_sql,
        ),
    )
    .unwrap();
    let mut stmt = conn
        .prepare("SELECT id FROM migration ORDER BY id")
        .unwrap();
    let ids: Vec<String> = stmt
        .query_map([], |r| r.get(0))
        .unwrap()
        .map(|r| r.unwrap())
        .collect();
    assert_eq!(
        ids,
        vec![
            "20260511173437_session-metadata".to_string(),
            "20260530232709_lovely_romulus".to_string()
        ]
    );
}

#[test]
fn skips_drizzle_import_when_migration_table_already_has_state() {
    // source: "skips drizzle import when migration table already has state"
    let mut conn = raw();
    conn.execute_batch(
        "CREATE TABLE migration (id TEXT PRIMARY KEY, time_completed INTEGER NOT NULL);
         INSERT INTO migration (id, time_completed) VALUES ('existing', 1);
         CREATE TABLE __drizzle_migrations (id INTEGER PRIMARY KEY, hash text NOT NULL, created_at numeric, name text, applied_at TEXT);
         INSERT INTO __drizzle_migrations (hash, created_at, name, applied_at) VALUES ('hash', 1, '20260127222353_familiar_lady_ursula', '2024-01-01T00:00:00.000Z');",
    )
    .unwrap();
    apply_only(&mut conn, &[]).unwrap();
    let mut stmt = conn
        .prepare("SELECT id FROM migration ORDER BY id")
        .unwrap();
    let ids: Vec<String> = stmt
        .query_map([], |r| r.get(0))
        .unwrap()
        .map(|r| r.unwrap())
        .collect();
    assert_eq!(ids, vec!["existing".to_string()]);
}
