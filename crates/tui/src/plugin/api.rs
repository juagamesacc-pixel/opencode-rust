// source: packages/tui/src/plugin/api.ts (52 lines, v1.18.30)
// 1:1 port — SolidJS reactivity (createSignal revision) → explicit revision
// counter; symbol key → incrementing u64; AbortSignal → Arc<AtomicBool>;
// onDispose → Box<dyn FnOnce() + Send>. Behavior, ordering, and unregister
// semantics verbatim.

#![allow(dead_code)]

use std::collections::HashMap;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
};

use crate::ui::dialog::DialogStack;

/// Mirrors `TuiRouteDefinition["render"]` — every real use mounts dialogs.
pub type RenderFn = Box<dyn FnMut(&mut DialogStack) + Send>;

/// Shared render handle so `get` can hand out the last-registered render
/// (mirrors `.at(-1)`) while `register` keeps key-based unregister.
pub type SharedRender = Arc<Mutex<RenderFn>>;

/// Mirrors `RouteEntry` — `key: symbol` becomes an incrementing `u64`
/// (recorded rename: Rust has no Symbol; unregister semantics identical).
pub struct RouteEntry {
    pub key: u64,
    pub render: SharedRender,
}

/// Mirrors `RouteMap = Map<string, RouteEntry[]>`.
pub type RouteMap = HashMap<String, Vec<RouteEntry>>;

/// Mirrors the `TuiRouteDefinition` input to `register`.
pub struct RouteDefinition {
    pub name: String,
    pub render: RenderFn,
}

impl RouteDefinition {
    pub fn new(name: impl Into<String>, render: RenderFn) -> Self {
        Self {
            name: name.into(),
            render,
        }
    }
}

/// Mirrors the return of `createPluginRoutes()` — routes + revision counter
/// (explicit `revision: u64` replaces `createSignal(0)`).
pub struct PluginRoutes {
    routes: RouteMap,
    revision: u64,
    next_key: u64,
}

impl PluginRoutes {
    pub fn new() -> Self {
        Self {
            routes: HashMap::new(),
            revision: 0,
            next_key: 0,
        }
    }

    /// Mirrors `register(list)` — appends `{key, render}` per name, bumps
    /// revision, returns the unregister token. Applying it via
    /// `unregister(token)` removes only matching-key entries, deletes the
    /// name when empty, and bumps revision (mirrors the returned closure).
    /// (Recorded shape note: the TS closure is modeled as token + method
    /// because a returned `FnOnce(&mut Self)` closure cannot borrow safely.)
    pub fn register(&mut self, list: Vec<RouteDefinition>) -> Unregister {
        let key = self.next_key;
        self.next_key += 1;
        let mut names: Vec<String> = Vec::with_capacity(list.len());
        for item in list {
            names.push(item.name.clone());
            let entry = RouteEntry {
                key,
                render: Arc::new(Mutex::new(item.render)),
            };
            self.routes.entry(item.name).or_default().push(entry);
        }
        self.revision += 1;
        Unregister { key, names }
    }

    /// Mirrors `get(name)` — touches revision, returns the LAST registered
    /// render (`.at(-1)`), cloned as shared handle.
    pub fn get(&mut self, name: &str) -> Option<SharedRender> {
        let _ = self.revision;
        self.routes.get(name)?.last().map(|e| Arc::clone(&e.render))
    }

    /// Revision counter (mirrors the `revision` signal value).
    pub fn revision(&self) -> u64 {
        self.revision
    }

    /// Unregister helper (mirrors the closure returned by `register`).
    pub fn unregister(&mut self, un: Unregister) {
        for name in un.names {
            let remove_name = match self.routes.get_mut(&name) {
                Some(list) => {
                    list.retain(|entry| entry.key != un.key);
                    list.is_empty()
                }
                None => false,
            };
            if remove_name {
                self.routes.remove(&name);
            }
        }
        self.revision += 1;
    }

    #[cfg(test)]
    fn len(&self, name: &str) -> usize {
        self.routes.get(name).map(|v| v.len()).unwrap_or(0)
    }
}

impl Default for PluginRoutes {
    fn default() -> Self {
        Self::new()
    }
}

/// Mirrors the unregister closure returned by `register`.
pub struct Unregister {
    key: u64,
    names: Vec<String>,
}

/// Mirrors `PluginRoutes = ReturnType<typeof createPluginRoutes>`.
pub fn create_plugin_routes() -> PluginRoutes {
    PluginRoutes::new()
}

/// Mirrors the `lifecycle` field built by `createTuiApi` — signal starts
/// un-aborted; disposers run at dispose.
pub struct TuiLifecycle {
    pub signal: Arc<AtomicBool>,
    disposers: Vec<Box<dyn FnOnce() + Send>>,
}

impl TuiLifecycle {
    pub fn new() -> Self {
        Self {
            signal: Arc::new(AtomicBool::new(false)),
            disposers: Vec::new(),
        }
    }

    /// Mirrors `onDispose` — stores the cleanup, returns a no-op remover
    /// (source default returns `() => {}`).
    pub fn on_dispose(&mut self, f: Box<dyn FnOnce() + Send>) -> Box<dyn FnOnce() + Send> {
        self.disposers.push(f);
        Box::new(|| {})
    }

    pub fn dispose(self) {
        for f in self.disposers {
            f();
        }
        self.signal.store(true, Ordering::SeqCst);
    }

    pub fn aborted(&self) -> bool {
        self.signal.load(Ordering::SeqCst)
    }
}

impl Default for TuiLifecycle {
    fn default() -> Self {
        Self::new()
    }
}

/// Mirrors `createTuiApi(input)` lifecycle half — spreads `input` at the
/// adapters layer; here only the `lifecycle` is built (signal + onDispose).
pub fn create_tui_lifecycle() -> TuiLifecycle {
    TuiLifecycle::new()
}
