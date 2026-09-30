// source: src/storage/schema.ts — exports: AccountTable, AccountStateTable,
// ControlAccountTable, ProjectTable, SessionTable, MessageTable, PartTable,
// TodoTable, SessionShareTable, WorkspaceTable (all re-exported from
// @opencode-ai/core */sql).
// PROVISIONAL pending crates/core (account/project/session/share/control-plane
// sql tables): table names mirrored as consts.

/// source: re-exported table names — verbatim identifiers.
pub const ACCOUNT_TABLE: &str = "AccountTable";
pub const ACCOUNT_STATE_TABLE: &str = "AccountStateTable";
pub const CONTROL_ACCOUNT_TABLE: &str = "ControlAccountTable";
pub const PROJECT_TABLE: &str = "ProjectTable";
pub const SESSION_TABLE: &str = "SessionTable";
pub const MESSAGE_TABLE: &str = "MessageTable";
pub const PART_TABLE: &str = "PartTable";
pub const TODO_TABLE: &str = "TodoTable";
pub const SESSION_SHARE_TABLE: &str = "SessionShareTable";
pub const WORKSPACE_TABLE: &str = "WorkspaceTable";
