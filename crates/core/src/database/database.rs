//! Rust port of `packages/core/src/database/database.ts`.
//! Source pin: v1.18.30 @3104c14 — 1:1 exact translation.

use rusqlite::Connection;

// PROVISIONAL pending Global.Flag wiring now via path_for_flag; Effect Layer mapped to sync open

pub const SERVICE_ID: &str = "@opencode/v2/storage/Database";

pub const PRAGMAS: &[&str] = &[
    "PRAGMA journal_mode = WAL",
    "PRAGMA synchronous = NORMAL",
    "PRAGMA busy_timeout = 5000",
    "PRAGMA cache_size = -64000",
    "PRAGMA foreign_keys = ON",
    "PRAGMA wal_checkpoint(PASSIVE)",
];

pub const CHANNELS: &[&str] = &["latest", "beta", "prod"];

pub fn path_for_flag(
    flag: Option<&str>,
    global_data: &str,
    channel: &str,
    disable_channel_db: bool,
) -> String {
    if let Some(f) = flag {
        if f == ":memory:" || f.starts_with('/') {
            return f.to_string();
        }
        return format!("{}/{}", global_data, f);
    }
    if CHANNELS.contains(&channel) || disable_channel_db {
        return format!("{}/opencode.db", global_data);
    }
    let safe = channel.replace(
        |c: char| !c.is_ascii_alphanumeric() && c != '.' && c != '_' && c != '-',
        "-",
    );
    format!("{}/opencode-{}.db", global_data, safe)
}

pub fn open(path: &str) -> Result<Connection, String> {
    let mut conn = if path == ":memory:" {
        Connection::open_in_memory().map_err(|e| e.to_string())?
    } else {
        if let Some(parent) = std::path::Path::new(path).parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        Connection::open(path).map_err(|e| e.to_string())?
    };
    // Set the busy timeout before any other pragma. A concurrent `apply` on
    // another connection to the same file may hold a write lock while this
    // connection runs the pragmas below (the source relies on cooperative
    // scheduling here; OS threads need the timeout up front or the first
    // lock-taking pragma fails with "database is locked").
    conn.execute_batch("PRAGMA busy_timeout = 5000")
        .map_err(|e| e.to_string())?;
    for pragma in PRAGMAS {
        conn.execute_batch(pragma).map_err(|e| e.to_string())?;
    }
    crate::database::migration::apply(&mut conn)?;
    Ok(conn)
}

pub fn open_in_memory() -> Result<Connection, String> {
    open(":memory:")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn memory_flag() {
        assert_eq!(
            path_for_flag(Some(":memory:"), "/data", "dev", false),
            ":memory:"
        );
    }
    #[test]
    fn channel_latest() {
        assert_eq!(
            path_for_flag(None, "/data", "latest", false),
            "/data/opencode.db"
        );
    }
    #[test]
    fn custom_channel() {
        assert_eq!(
            path_for_flag(None, "/data", "my/channel", false),
            "/data/opencode-my-channel.db"
        );
    }
}
