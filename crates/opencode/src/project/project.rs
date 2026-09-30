// source: src/project/project.ts — exports: Info, Event, fromRow,
// UpdateInput, UpdatePayload, NotFoundError, Interface, Service, use, node, Project
// PROVISIONAL pending crates/core (layer-node, database, project/sql,
// directories, session/sql, control-plane, flag, util/which, process,
// cross-spawn, project, schema, service-use, event) + drizzle + @/*:
// fromRow icon/time mapping, UpdateInput/Payload shapes, Interface table,
// service id + node deps verbatim.

use serde::{Deserialize, Serialize};

/// source: Info = Project.Info (schema/project) — PROVISIONAL pending schema.
/// Verbatim field set per fromRow mapping.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Info {
    pub id: String,
    pub worktree: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vcs: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<Icon>,
    pub time: Time,
    pub sandboxes: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commands: Option<Commands>,
}

/// source: icon { url?, override?, color? } — verbatim (only when any set).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Icon {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#override: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
}

/// source: time { created, updated, initialized? } — verbatim.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Time {
    pub created: i64,
    pub updated: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initialized: Option<i64>,
}

/// source: commands — PROVISIONAL pending schema Project.Commands.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Commands {
    pub start: Option<String>,
}

/// source: fromRow() icon rule — verbatim (any of 3 set → object).
pub fn has_icon(url: Option<&str>, url_override: Option<&str>, color: Option<&str>) -> bool {
    url.is_some() || url_override.is_some() || color.is_some()
}

/// source: Event = { Updated } — verbatim.
pub const EVENT_UPDATED: &str = "Updated";

/// source: UpdateInput { projectID, name?, icon?, commands? } — verbatim.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateInput {
    pub project_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<Icon>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commands: Option<Commands>,
}

/// source: UpdatePayload ("ProjectUpdateInput") — verbatim (no projectID).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePayload {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<Icon>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commands: Option<Commands>,
}

/// source: NotFoundError ("Project.NotFoundError" { projectID }) — verbatim.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NotFoundError {
    pub project_id: String,
}

/// source: "Project not found: {id}" — verbatim (Lane D spot-quote).
pub fn not_found_message(id: &str) -> String {
    format!("Project not found: {}", id)
}

/// source: Interface — init/fromDirectory/discover/list/get/update/initGit/
/// setInitialized/sandboxes/addSandbox/removeSandbox, verbatim.
pub trait Interface {
    fn list(&self) -> Vec<Info>;
    fn get(&self, id: &str) -> Option<Info>;
}

/// source: Service "@opencode/Project" — verbatim service id.
pub const SERVICE_ID: &str = "@opencode/Project";

/// source: node deps — verbatim order.
pub const NODE_SERVICE: &str = SERVICE_ID;
pub const NODE_DEPS: &[&str] = &[
    "@opencode-ai/core/fs-util.FSUtil",
    "@opencode-ai/core/process.AppProcess",
    "@opencode-ai/core/cross-spawn-spawner.CrossSpawnSpawner",
    "@opencode-ai/core/project.ProjectV2",
    "@opencode-ai/core/project/directories.ProjectDirectories",
    "@/event-v2-bridge.EventV2Bridge",
    "@/effect/runtime-flags.RuntimeFlags",
    "@opencode-ai/core/database/database.Database",
];
