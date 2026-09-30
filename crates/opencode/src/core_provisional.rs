//! PROVISIONAL stubs for `@opencode-ai/core/*` (unported `crates/core`)
//! and pending intra-crate modules. Each item flagged; remove as upstreams land.
//!
//! source: @opencode-ai/core/* (multiple) — exports: stub shells below.

/// PROVISIONAL pending crates/core (effect/layer-node): service-identity helper.
pub mod layer_node {
    /// PROVISIONAL pending crates/core: node descriptor.
    #[derive(Debug, Clone)]
    pub struct Node {
        pub service: &'static str,
        pub deps: Vec<&'static str>,
    }

    impl Node {
        /// PROVISIONAL pending crates/core.
        pub fn make(service: &'static str, deps: Vec<&'static str>) -> Self {
            Self { service, deps }
        }
    }
}

/// PROVISIONAL pending crates/core (event): v2 event service surface used by event_v2_bridge.
pub mod event_v2 {
    /// PROVISIONAL pending crates/core.
    #[derive(Debug, Clone)]
    pub struct Info {
        pub directory: String,
        pub workspace_id: Option<String>,
        pub project_id: Option<String>,
    }
}

/// PROVISIONAL pending crates/core (location/schema/project): newtypes.
pub mod ids {
    /// PROVISIONAL pending crates/core.
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct AbsolutePath(pub String);
    /// PROVISIONAL pending crates/core.
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct ProjectId(pub String);
}

/// PROVISIONAL pending bus/control-plane/effect modules: instance routing + bus.
pub mod instance {
    /// PROVISIONAL pending effect/bus modules.
    #[derive(Debug, Clone, Default)]
    pub struct Ctx {
        pub directory: String,
        pub project_id: String,
        pub worktree: String,
        pub workspace_id: Option<String>,
    }
}

/// PROVISIONAL pending crates/core: generic service error preserving message.
#[derive(Debug, Clone)]
pub struct CoreError {
    pub message: String,
}

impl CoreError {
    /// PROVISIONAL pending crates/core.
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl std::fmt::Display for CoreError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for CoreError {}
