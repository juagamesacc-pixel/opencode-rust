//! Rust port of `packages/core/src/database/migration/20260622202450_simplify_session_input.ts`.
//! Source pin: v1.18.30 @3104c14 — 1:1 SQL verbatim extraction.
//! NOTE: `up_sql(conn)` accepts the connection because the TypeScript migration inspects the
//! live schema at migration time (see `packages/core/src/database/migration.ts` apply/applyOnly).

use rusqlite::Connection;

pub const ID: &str = "20260622202450_simplify_session_input";

pub fn up_sql(_conn: &Connection) -> Result<Vec<String>, String> {
    Ok(vec![
        r#"DELETE FROM `session_context_epoch`;"#.to_string(),
        r#"DELETE FROM `session_input`;"#.to_string(),
        r#"DELETE FROM `session_message`;"#.to_string(),
        r#"DELETE FROM `event`;"#.to_string(),
        r#"DELETE FROM `event_sequence`;"#.to_string(),
        r#"UPDATE `session` SET `workspace_id` = NULL WHERE `workspace_id` IS NOT NULL;"#
            .to_string(),
        r#"DELETE FROM `workspace`;"#.to_string(),
    ])
}
