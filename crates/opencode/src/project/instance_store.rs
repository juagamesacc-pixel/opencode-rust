// source: src/project/instance-store.ts — exports: LoadInput, Interface,
// Service, use, bootstrapNode, node, InstanceStore
// PROVISIONAL: fiber/deferred/scope orchestration as descriptors; cache
// identity rules (removeEntry/disposeEntry compare-and-delete), disposed
// event { type: "server.instance.disposed", properties: { directory } },
// log strings, spans verbatim.

/// source: LoadInput { directory, worktree?, project? } — verbatim.
#[derive(Debug, Clone)]
pub struct LoadInput {
    pub directory: String,
    pub worktree: Option<String>,
}

/// source: Interface — load/reload/dispose/disposeDirectory/disposeAll/provide, verbatim.
pub trait Interface {
    fn dispose_all(&self);
}

/// source: Service "@opencode/InstanceStore" — verbatim service id.
pub const SERVICE_ID: &str = "@opencode/InstanceStore";

/// source: node deps [Project.node, bootstrapNode] — verbatim.
pub const NODE_DEPS: &[&str] = &["@/project/project.Project", "bootstrapNode"];

/// source: disposed event — verbatim type + properties.
pub const DISPOSED_TYPE: &str = "server.instance.disposed";

/// source: log strings — verbatim.
pub const LOG_CREATING: &str = "creating instance";
pub const LOG_RELOADING: &str = "reloading instance";
pub const LOG_DISPOSING: &str = "disposing instance";
pub const LOG_DISPOSING_ALL: &str = "disposing all instances";
pub const LOG_DISPOSE_FAILED: &str = "instance dispose failed";

/// source: spans — verbatim.
pub const SPAN_BOOT: &str = "InstanceStore.boot";
pub const SPAN_LOAD: &str = "InstanceStore.load";
pub const SPAN_RELOAD: &str = "InstanceStore.reload";
