//! Rust port of `packages/core/src/database/migration/20260309230000_move_org_to_state.ts`.
//! Source pin: v1.18.30 @3104c14 — 1:1 SQL verbatim extraction.
//! NOTE: `up_sql(conn)` accepts the connection because the TypeScript migration inspects the
//! live schema at migration time (see `packages/core/src/database/migration.ts` apply/applyOnly).

use rusqlite::Connection;

pub const ID: &str = "20260309230000_move_org_to_state";

pub fn up_sql(_conn: &Connection) -> Result<Vec<String>, String> {
    Ok(vec![
        r#"ALTER TABLE `account_state` ADD `active_org_id` text;"#.to_string(),
        r#"UPDATE `account_state` SET `active_org_id` = (SELECT `selected_org_id` FROM `account` WHERE `account`.`id` = `account_state`.`active_account_id`);"#.to_string(),
        r#"ALTER TABLE `account` DROP COLUMN `selected_org_id`;"#.to_string(),
    ])
}
