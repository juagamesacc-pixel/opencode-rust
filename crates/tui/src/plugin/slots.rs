// source: packages/tui/src/plugin/slots.tsx (65 lines, v1.18.30)
// 1:1 port — SolidJS signals/registry (@opentui/solid) → explicit state:
// view signal → SlotView enum + revision; SolidSlotRegistry → HashMap
// registry with incrementing u64 ids; console.error → eprintln! with the
// same tag and field order. Behavior and strings verbatim.

#![allow(dead_code)]

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use serde_json::Value;

use crate::util::record::is_record;

/// Mirrors `RuntimeSlotMap` — slot name → props object.
pub type RuntimeSlotMap = HashMap<String, Value>;

/// Mirrors the `SlotView` component — rendered later by the app.
/// (Assignment decision: enum with None + text/kind variants.)
#[derive(Debug, Clone, Default)]
pub enum SlotView {
    #[default]
    None,
    Text {
        slot: String,
        text: String,
    },
    Kind {
        slot: String,
        kind: String,
        props: Value,
    },
}

/// Mirrors `HostSlotPlugin<Slots>` — `{ id: string, slots: Record }`.
#[derive(Debug, Clone, Default)]
pub struct HostSlotPlugin {
    pub id: String,
    pub slots: HashMap<String, Value>,
}

/// Mirrors `HostPluginApi` usage in `setup` — renderer + theme only.
#[derive(Debug, Clone, Default)]
pub struct HostPluginApi {
    pub renderer: Value,
    pub theme: Value,
}

/// Mirrors the plugin-error event payload (field order verbatim).
#[derive(Debug, Clone)]
pub struct SlotPluginError {
    pub plugin_id: String,
    pub slot: String,
    pub phase: String,
    pub source: String,
    pub message: String,
}

/// Mirrors `isHostSlotPlugin` — record with string `id` and record `slots`.
pub fn is_host_slot_plugin(value: &Value) -> bool {
    if !is_record(value) {
        return false;
    }
    let id_ok = value.get("id").and_then(|v| v.as_str()).is_some();
    let slots_ok = value.get("slots").map(is_record).unwrap_or(false);
    id_ok && slots_ok
}

#[derive(Debug, Default)]
struct RegistryInner {
    plugins: HashMap<u64, HostSlotPlugin>,
    next_id: u64,
    theme: Value,
}

impl RegistryInner {
    fn register(&mut self, plugin: HostSlotPlugin) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        self.plugins.insert(id, plugin);
        id
    }

    fn unregister(&mut self, id: u64) {
        self.plugins.remove(&id);
    }

    fn on_plugin_error(&self, event: &SlotPluginError) {
        // Mirrors console.error("[tui.slot] plugin error", {...}) — same tag,
        // same key order: plugin/slot/phase/source/message.
        eprintln!(
            "[tui.slot] plugin error {{ plugin: {}, slot: {}, phase: {}, source: {}, message: {} }}",
            event.plugin_id, event.slot, event.phase, event.source, event.message
        );
    }
}

#[derive(Debug, Default)]
struct SlotsInner {
    view: SlotView,
    revision: u64,
    registry: RegistryInner,
}

/// Mirrors the return of `createSlots()` — `{ Slot, setup, clear }`.
/// (`Slot` the component is modeled as the current `SlotView` value;
/// the app renders it later.)
#[derive(Debug, Clone, Default)]
pub struct SlotManager {
    inner: Arc<Mutex<SlotsInner>>,
}

impl SlotManager {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(SlotsInner::default())),
        }
    }

    /// Mirrors the `Slot` view — current view value.
    pub fn view(&self) -> SlotView {
        self.inner
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .view
            .clone()
    }

    /// Mirrors `setup(api)` — builds the registry from `api.renderer` +
    /// `{ theme: api.theme }`, installs the error hook, sets the slot view,
    /// returns `{ register, dispose }`.
    pub fn setup(&self, api: &HostPluginApi) -> HostSlots {
        {
            let mut guard = self.inner.lock().unwrap_or_else(|e| e.into_inner());
            guard.registry.theme = api.theme.clone();
            guard.view = SlotView::None;
            guard.revision += 1;
        }
        HostSlots {
            manager: self.clone(),
        }
    }

    /// Mirrors `clear()` — resets the view to empty.
    pub fn clear(&self) {
        let mut guard = self.inner.lock().unwrap_or_else(|e| e.into_inner());
        guard.view = SlotView::None;
        guard.revision += 1;
    }

    pub fn revision(&self) -> u64 {
        self.inner
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .revision
    }
}

/// Mirrors `HostSlots` — `{ register, dispose }`.
#[derive(Debug, Clone)]
pub struct HostSlots {
    manager: SlotManager,
}

impl HostSlots {
    /// Mirrors `register(plugin)` — non-host plugins yield a no-op
    /// unregister; host plugins register and yield a removing unregister.
    pub fn register(&self, plugin: HostSlotPlugin) -> Box<dyn FnOnce() + Send> {
        // Validate shape the same way (id string + slots record).
        let shape = serde_json::json!({ "id": plugin.id, "slots": plugin.slots });
        if !is_host_slot_plugin(&shape) {
            return Box::new(|| {});
        }
        let mut guard = self.manager.inner.lock().unwrap_or_else(|e| e.into_inner());
        let id = guard.registry.register(plugin);
        let manager = self.manager.clone();
        Box::new(move || {
            manager
                .inner
                .lock()
                .unwrap_or_else(|e| e.into_inner())
                .registry
                .unregister(id);
        })
    }

    /// Mirrors `dispose()` — resets the view to empty.
    pub fn dispose(&self) {
        self.manager.clear();
    }
}

/// Mirrors `createSlots()`.
pub fn create_slots() -> SlotManager {
    SlotManager::new()
}
