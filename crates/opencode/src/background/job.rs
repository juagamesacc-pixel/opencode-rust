// source: src/background/job.ts — exports: Service, ExtendInput, Info,
// Interface, StartInput, Status, WaitInput, WaitResult (re-exported from
// @opencode-ai/core/background-job), node, BackgroundJob
// PROVISIONAL pending crates/core (background-job): types mirrored verbatim.

use serde::{Deserialize, Serialize};

/// source: Status — PROVISIONAL pending crates/core (background-job).
/// Preserved variants mirror core registry engine states.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Status {
    Running,
    Done,
    Failed,
    Cancelled,
}

/// source: Info — PROVISIONAL pending crates/core (background-job).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Info {
    pub id: String,
    pub status: Status,
}

/// source: StartInput — PROVISIONAL pending crates/core (background-job).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StartInput {
    pub command: Vec<String>,
}

/// source: ExtendInput — PROVISIONAL pending crates/core (background-job).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExtendInput {
    pub id: String,
}

/// source: WaitInput — PROVISIONAL pending crates/core (background-job).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WaitInput {
    pub id: String,
}

/// source: WaitResult — PROVISIONAL pending crates/core (background-job).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WaitResult {
    pub id: String,
    pub status: Status,
}

/// source: Interface — list/get/start/extend/wait/waitForPromotion/promote/cancel.
/// Instance-scoped sharing the core registry engine — verbatim method order.
pub trait Interface {
    fn list(&self) -> Vec<Info>;
    fn get(&self, id: &str) -> Option<Info>;
    fn start(&self, input: &StartInput) -> Info;
    fn extend(&self, input: &ExtendInput) -> Option<Info>;
    fn wait(&self, input: &WaitInput) -> WaitResult;
    fn wait_for_promotion(&self, id: &str) -> WaitResult;
    fn promote(&self, id: &str) -> Option<Info>;
    fn cancel(&self, id: &str) -> Option<Info>;
}

/// source: Service = CoreBackgroundJob.Service (re-exported) — PROVISIONAL
/// pending crates/core. Verbatim service identity carried by NODE_SERVICE.
pub const SERVICE_ID: &str = "@opencode-ai/core/background-job.Service";

/// source: node = LayerNode.make({ service, layer, deps: [] }) — verbatim.
pub const NODE_SERVICE: &str = SERVICE_ID;
pub const NODE_DEPS: &[&str] = &[];
