//! Rust port of `packages/app/src/pages/home-session-archive.ts` (opencode v1.18.30).
//!
//! 1:1 exact translation — same names/behavior/edge-cases/error-strings.
//! Rename: `home-session-archive.ts` -> `home_session_archive.rs` (kebab -> snake_case).

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HomeSession {
    pub id: String,
    pub directory: String,
}

pub struct ArchiveHomeSessionInput {
    pub server: String,
    pub session: HomeSession,
}

pub fn archive_home_session_key(server: &str, session: &HomeSession) -> String {
    format!("{}:{}", server, session.id)
}

// Async archive logic is PROVISIONAL due to SDK — shape preserved.
// See source `archiveHomeSession` for full async flow (archive -> remove -> notify).
// PROVISIONAL: pending @opencode-ai/sdk — mirrors `home-session-archive.ts`
