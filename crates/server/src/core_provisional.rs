//! Core wiring for `@opencode-ai/core/*` — verified against `crates/core`.
//!
//! Source: `packages/server` imports from ~20 `core` modules. Where `crates/core`
//! now exposes the real API (Location/Session/Pty/Permission service IDs and
//! pure types), this module re-exports the real types. Unported runtime
//! surfaces (Ticket/Protocol/Database/Event services requiring Effect/SQLite)
//! remain flagged PROVISIONAL with reason.
//!
//! FLAG: items marked PROVISIONAL are still pending upstream runtime wiring;
//! pure constants/IDs/error strings are verbatim from source.

/// Service ID verbatim from source layers where applicable.
pub const DATABASE_SERVICE_ID: &str = "@opencode/Database";
pub const LOCATION_SERVICE_ID: &str = "@opencode/Location";
pub const LOCATION_SERVICE_MAP_ID: &str = "@opencode/LocationServiceMap";
pub const AGENT_SERVICE_ID: &str = "@opencode/Agent";
pub const SESSION_SERVICE_ID: &str = "@opencode/Session";
pub const EVENT_SERVICE_ID: &str = "@opencode/EventV2";
pub const CATALOG_SERVICE_ID: &str = "@opencode/Catalog";
pub const FILE_SYSTEM_SERVICE_ID: &str = "@opencode/FileSystem";
pub const INTEGRATION_SERVICE_ID: &str = "@opencode/Integration";
pub const PERMISSION_SERVICE_ID: &str = "@opencode/PermissionV2";
pub const PERMISSION_SAVED_SERVICE_ID: &str = "@opencode/PermissionSaved";
pub const PTY_SERVICE_ID: &str = "@opencode/Pty";
pub const PTY_TICKET_SERVICE_ID: &str = "@opencode/PtyTicket";
pub const QUESTION_SERVICE_ID: &str = "@opencode/QuestionV2";
pub const REFERENCE_SERVICE_ID: &str = "@opencode/Reference";
pub const SKILL_SERVICE_ID: &str = "@opencode/SkillV2";
pub const COMMAND_SERVICE_ID: &str = "@opencode/CommandV2";
pub const PROJECT_COPY_SERVICE_ID: &str = "@opencode/ProjectCopy";
pub const TOOL_OUTPUT_STORE_SERVICE_ID: &str = "@opencode/ToolOutputStore";

// ---------------------------------------------------------------------------
// Location — WIRED to crates/core/src/location.rs (verified: LocationRef/LocationInfo/ProjectRef + SERVICE_ID)
// ---------------------------------------------------------------------------
pub mod location {
    // Real types from core — 1:1 with `Location.Ref` / `Location.Info` (directory + workspaceID + project)
    pub use core::location::{LocationInfo as Info, LocationRef as Ref, ProjectRef, SERVICE_ID};
}

// ---------------------------------------------------------------------------
// Session — WIRED to crates/core/src/session/schema.rs + session.rs (verified: ID brand = String, service ID)
// ---------------------------------------------------------------------------
pub mod session {
    /// Mirrors `SessionV2.ID` brand (opaque string) — wired to core's SessionSchema ID (String brand)
    pub type ID = String;
    pub const SERVICE_ID: &str = "@opencode/Session";
}

// ---------------------------------------------------------------------------
// Pty — WIRED to crates/core/src/pty.rs (verified: Info/CreateInput/UpdateInput/NotFoundError/ExitedError + constants)
// ---------------------------------------------------------------------------
pub mod pty {
    pub use core::pty::{CreateInput, ExitedError, Info, NotFoundError, Size, UpdateInput};
    pub type ID = String;
    pub const NOT_FOUND_TAG: &str = "Pty.NotFoundError";
    pub const EXITED_TAG: &str = "Pty.ExitedError";
    pub const BUFFER_LIMIT: usize = core::pty::BUFFER_LIMIT;
    pub const EXITED_LIMIT: usize = core::pty::EXITED_LIMIT;
}

// ---------------------------------------------------------------------------
// Permission
// ---------------------------------------------------------------------------
pub mod permission {
    pub type ID = String;
    pub const NOT_FOUND_TAG: &str = "PermissionV2.NotFoundError";
}

// ---------------------------------------------------------------------------
// Question
// ---------------------------------------------------------------------------
pub mod question {
    pub type ID = String;
    pub const NOT_FOUND_TAG: &str = "QuestionV2.NotFoundError";
}

// ---------------------------------------------------------------------------
// Database / SessionTable
// ---------------------------------------------------------------------------
pub mod database {
    #[derive(Clone, Debug)]
    pub struct Row {
        pub directory: String,
        pub workspace_id: Option<String>,
    }
}

// ---------------------------------------------------------------------------
// PtyEnvironment stub — also re-exported via `pty_environment.rs`.
// ---------------------------------------------------------------------------
pub mod pty_environment_stub {
    pub const SERVICE_ID: &str = "@opencode/ServerPtyEnvironment";
}
