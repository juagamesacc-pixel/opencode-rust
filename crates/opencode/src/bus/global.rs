// source: src/bus/global.ts — exports: GlobalEvent, GlobalBus
use serde::{Deserialize, Serialize};
use std::sync::{Mutex, OnceLock};

/// source: GlobalEvent — verbatim fields.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GlobalEvent {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace: Option<String>,
    pub payload: serde_json::Value,
}

type Listener = Box<dyn Fn(&GlobalEvent) + Send + Sync>;

/// source: GlobalBusEmitter — minimal equivalent of the EventEmitter subclass.
/// emit() assigns payload.id exactly like source: payload.syncEvent?.id ??
/// Identifier.create("evt", "ascending"), when payload is an object without id.
pub struct GlobalBusEmitter {
    listeners: Mutex<Vec<Listener>>,
}

impl GlobalBusEmitter {
    pub fn new() -> Self {
        Self {
            listeners: Mutex::new(Vec::new()),
        }
    }

    /// source: emit("event", event) — verbatim id-assignment rule.
    pub fn emit(&self, mut event: GlobalEvent) {
        if let Some(obj) = event.payload.as_object_mut() {
            if !obj.contains_key("id") {
                let id = obj
                    .get("syncEvent")
                    .and_then(|v| v.get("id"))
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string())
                    .unwrap_or_else(|| {
                        crate::id::id::create(
                            crate::id::id::PREFIX_EVENT,
                            crate::id::id::Direction::Ascending,
                            None,
                        )
                    });
                obj.insert("id".to_string(), serde_json::Value::String(id));
            }
        }
        let listeners = self.listeners.lock().unwrap();
        for l in listeners.iter() {
            l(&event);
        }
    }

    pub fn subscribe(&self, listener: Listener) {
        self.listeners.lock().unwrap().push(listener);
    }
}

impl Default for GlobalBusEmitter {
    fn default() -> Self {
        Self::new()
    }
}

fn global_bus() -> &'static GlobalBusEmitter {
    static BUS: OnceLock<GlobalBusEmitter> = OnceLock::new();
    BUS.get_or_init(GlobalBusEmitter::new)
}

/// source: GlobalBus — singleton, verbatim.
pub fn global_bus_emit(event: GlobalEvent) {
    global_bus().emit(event);
}
