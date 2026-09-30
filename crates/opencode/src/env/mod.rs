// source: src/env/index.ts — exports: Interface, Service, use, node, Env
// PROVISIONAL pending crates/core (@opencode-ai/core/effect/layer-node, service-use)
// and intra-crate @/effect/instance-state: Effect runtime modelled as descriptors;
// state semantics (seed from process env, per-instance map) preserved verbatim.

use std::collections::HashMap;

/// source: State = Record<string, string | undefined> — verbatim.
pub type State = HashMap<String, Option<String>>;

/// source: Interface — get/all/set/remove, verbatim signatures.
pub trait Interface {
    fn get(&self, key: &str) -> Option<String>;
    fn all(&self) -> State;
    fn set(&mut self, key: &str, value: String);
    fn remove(&mut self, key: &str);
}

/// source: Service "@opencode/Env" — verbatim service id.
pub const SERVICE_ID: &str = "@opencode/Env";

/// source: use = serviceUse(Service) — descriptor, verbatim.
pub const USE_SERVICE: &str = SERVICE_ID;

/// source: node = LayerNode.make({ service, layer, deps: [] }) — verbatim.
pub const NODE_SERVICE: &str = SERVICE_ID;
pub const NODE_DEPS: &[&str] = &[];

/// In-memory Env state seeded from process env — mirrors InstanceState.make
/// seeding `{ ...process.env }` verbatim.
#[derive(Debug, Clone, Default)]
pub struct EnvState {
    inner: HashMap<String, String>,
}

impl EnvState {
    pub fn seeded_from_process() -> Self {
        Self {
            inner: std::env::vars().collect(),
        }
    }
}

impl Interface for EnvState {
    fn get(&self, key: &str) -> Option<String> {
        self.inner.get(key).cloned()
    }
    fn all(&self) -> State {
        self.inner
            .iter()
            .map(|(k, v)| (k.clone(), Some(v.clone())))
            .collect()
    }
    fn set(&mut self, key: &str, value: String) {
        self.inner.insert(key.to_string(), value);
    }
    fn remove(&mut self, key: &str) {
        self.inner.remove(key);
    }
}
