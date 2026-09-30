// source: src/format/index.ts — exports: Status, Interface, Service, use,
// node, Format
// PROVISIONAL pending crates/core + @/config/config + @/effect/* +
// @/util/error: Status shape + log strings verbatim.

use serde::{Deserialize, Serialize};

/// source: Status { name, extensions, enabled } ("FormatterStatus") — verbatim.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Status {
    pub name: String,
    pub extensions: Vec<String>,
    pub enabled: bool,
}

/// source: Interface — init/status/file, verbatim.
pub trait Interface {
    fn init(&self);
    fn status(&self) -> Vec<Status>;
    fn file(&self, filepath: &str) -> bool;
}

/// source: Service "@opencode/Format" — verbatim service id.
pub const SERVICE_ID: &str = "@opencode/Format";

/// source: node deps [Config.node, AppProcess.node, RuntimeFlags.node] — verbatim.
pub const NODE_SERVICE: &str = SERVICE_ID;
pub const NODE_DEPS: &[&str] = &[
    "@/config/config.Config",
    "@opencode-ai/core/process.AppProcess",
    "@/effect/runtime-flags.RuntimeFlags",
];

/// source: log strings — verbatim.
pub const LOG_FORMATTING: &str = "formatting";
pub const LOG_RUNNING: &str = "running";
pub const LOG_FORMAT_FAILED: &str = "failed to format file";
pub const LOG_SPAWN_FAILED: &str = "spawn failed";
pub const LOG_FAILED: &str = "failed";
pub const LOG_ALL_DISABLED: &str = "all formatters are disabled";
pub const LOG_INIT: &str = "init";

pub mod formatter;
