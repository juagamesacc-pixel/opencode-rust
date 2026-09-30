// source: packages/tui/src/context/event.ts (36 lines, v1.18.30)
// 1:1 port — the SDK event emitter is an explicit `EventSource` seam;
// `sync` payloads are filtered before reaching handlers, verbatim.

#![allow(dead_code)]

use serde_json::Value;
use std::collections::HashMap;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

/// Mirrors `EventMetadata`.
#[derive(Debug, Clone, Default)]
pub struct EventMetadata {
    pub directory: String,
    pub workspace: Option<String>,
}

/// Mirrors one SDK `"event"` emission (payload + routing metadata).
#[derive(Debug, Clone, Default)]
pub struct SdkEventEnvelope {
    pub payload: Value,
    pub directory: String,
    pub workspace: Option<String>,
}

/// Minimal SDK event-emitter surface (`sdk.event.on("event", …)`).
/// Implemented by the real sdk module when it lands; tests use a fake.
pub trait EventSource {
    fn add_listener(&mut self, handler: Box<dyn FnMut(SdkEventEnvelope) + Send>) -> u64;
    fn remove_listener(&mut self, id: u64);
}

/// Handle that detaches the listener on `unsubscribe` (mirrors the
/// emitter's off-function return).
pub struct Subscription {
    active: Arc<AtomicBool>,
    id: u64,
    detach: Option<Box<dyn FnOnce(u64) + Send>>,
}

impl Subscription {
    pub fn unsubscribe(mut self, source: &mut dyn EventSource) {
        self.active.store(false, Ordering::Relaxed);
        if let Some(detach) = self.detach.take() {
            detach(self.id);
        }
        source.remove_listener(self.id);
    }

    pub fn is_active(&self) -> bool {
        self.active.load(Ordering::Relaxed)
    }
}

/// Mirrors `subscribe` — drops `sync` payloads, forwards the rest with metadata.
pub fn subscribe(
    source: &mut dyn EventSource,
    mut handler: impl FnMut(Value, EventMetadata) + Send + 'static,
) -> Subscription {
    let active = Arc::new(AtomicBool::new(true));
    let flag = active.clone();
    let id = source.add_listener(Box::new(move |event: SdkEventEnvelope| {
        if !flag.load(Ordering::Relaxed) {
            return;
        }
        if event.payload.get("type").and_then(|t| t.as_str()) == Some("sync") {
            return;
        }
        handler(
            event.payload,
            EventMetadata {
                directory: event.directory,
                workspace: event.workspace,
            },
        );
    }));
    Subscription {
        active,
        id,
        detach: None,
    }
}

/// Mirrors `on` — type-filtered subscription.
pub fn on(
    source: &mut dyn EventSource,
    event_type: &str,
    handler: impl FnMut(Value, EventMetadata) + Send + 'static,
) -> Subscription {
    let wanted = event_type.to_string();
    let mut handler = handler;
    subscribe(source, move |event: Value, metadata: EventMetadata| {
        if event.get("type").and_then(|t| t.as_str()) != Some(wanted.as_str()) {
            return;
        }
        handler(event, metadata);
    })
}

/// In-memory `EventSource` for tests and headless use.
#[derive(Default)]
pub struct MemoryEventSource {
    next_id: u64,
    listeners: HashMap<u64, Box<dyn FnMut(SdkEventEnvelope) + Send>>,
}

impl MemoryEventSource {
    pub fn emit(&mut self, event: SdkEventEnvelope) {
        for handler in self.listeners.values_mut() {
            handler(event.clone());
        }
    }
}

impl EventSource for MemoryEventSource {
    fn add_listener(&mut self, handler: Box<dyn FnMut(SdkEventEnvelope) + Send>) -> u64 {
        self.next_id += 1;
        let id = self.next_id;
        self.listeners.insert(id, handler);
        id
    }

    fn remove_listener(&mut self, id: u64) {
        self.listeners.remove(&id);
    }
}
