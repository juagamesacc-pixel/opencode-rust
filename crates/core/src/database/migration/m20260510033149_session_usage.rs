//! Rust port of `packages/core/src/database/migration/20260510033149_session_usage.ts`.
//! Source pin: v1.18.30 @3104c14 — 1:1 SQL verbatim extraction.
//! NOTE: `up_sql(conn)` accepts the connection because the TypeScript migration inspects the
//! live schema at migration time (see `packages/core/src/database/migration.ts` apply/applyOnly).

use rusqlite::Connection;

pub const ID: &str = "20260510033149_session_usage";

pub fn up_sql(_conn: &Connection) -> Result<Vec<String>, String> {
    Ok(vec![
        r#"ALTER TABLE `session` ADD `cost` real DEFAULT 0 NOT NULL;"#.to_string(),
        r#"ALTER TABLE `session` ADD `tokens_input` integer DEFAULT 0 NOT NULL;"#.to_string(),
        r#"ALTER TABLE `session` ADD `tokens_output` integer DEFAULT 0 NOT NULL;"#.to_string(),
        r#"ALTER TABLE `session` ADD `tokens_reasoning` integer DEFAULT 0 NOT NULL;"#.to_string(),
        r#"ALTER TABLE `session` ADD `tokens_cache_read` integer DEFAULT 0 NOT NULL;"#.to_string(),
        r#"ALTER TABLE `session` ADD `tokens_cache_write` integer DEFAULT 0 NOT NULL;"#.to_string(),
        r#"UPDATE session
        SET
          cost = coalesce((
            SELECT sum(coalesce(json_extract(message.data, '$.cost'), 0))
            FROM message
            WHERE message.session_id = session.id
              AND json_extract(message.data, '$.role') = 'assistant'
          ), 0),
          tokens_input = coalesce((
            SELECT sum(coalesce(json_extract(message.data, '$.tokens.input'), 0))
            FROM message
            WHERE message.session_id = session.id
              AND json_extract(message.data, '$.role') = 'assistant'
          ), 0),
          tokens_output = coalesce((
            SELECT sum(coalesce(json_extract(message.data, '$.tokens.output'), 0))
            FROM message
            WHERE message.session_id = session.id
              AND json_extract(message.data, '$.role') = 'assistant'
          ), 0),
          tokens_reasoning = coalesce((
            SELECT sum(coalesce(json_extract(message.data, '$.tokens.reasoning'), 0))
            FROM message
            WHERE message.session_id = session.id
              AND json_extract(message.data, '$.role') = 'assistant'
          ), 0),
          tokens_cache_read = coalesce((
            SELECT sum(coalesce(json_extract(message.data, '$.tokens.cache.read'), 0))
            FROM message
            WHERE message.session_id = session.id
              AND json_extract(message.data, '$.role') = 'assistant'
          ), 0),
          tokens_cache_write = coalesce((
            SELECT sum(coalesce(json_extract(message.data, '$.tokens.cache.write'), 0))
            FROM message
            WHERE message.session_id = session.id
              AND json_extract(message.data, '$.role') = 'assistant'
          ), 0)"#
            .to_string(),
    ])
}
