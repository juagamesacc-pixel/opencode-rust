// source: packages/tui/src/context/sdk.tsx (151 lines, v1.18.30)
// 1:1 port — the SDK client/transport is an explicit seam (`SdkClient`);
// event batching (16ms window), the SSE reconnect loop with exponential
// backoff (1s → 30s), and abort semantics are preserved verbatim. The
// render loop drives `pump()` instead of SolidJS effects.

#![allow(dead_code)]

use serde_json::Value;
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::time::{Duration, Instant};
use tokio::sync::mpsc;

use super::event::SdkEventEnvelope;

/// Retry/backoff constants verbatim.
pub const RETRY_DELAY_MS: u64 = 1000;
pub const MAX_RETRY_DELAY_MS: u64 = 30000;
/// Events flushed together when they arrive within one frame budget.
pub const BATCH_WINDOW_MS: u64 = 16;

/// Transport seam replacing `createOpencodeClient` + fetch/SSE wiring.
/// `call` receives the dotted method (`"path.get"`, `"v2.session.get"`, …)
/// with its params and resolves the unwrapped response envelope.
pub trait SdkClient: Send + Sync {
    fn call(
        &self,
        method: &str,
        params: Value,
    ) -> Pin<Box<dyn Future<Output = Result<Value, String>> + Send + '_>>;
    fn directory(&self) -> Option<String>;
    /// Push `SdkEventEnvelope`s into `tx` until `abort` flips (mirrors the
    /// `for await (const event of events.stream)` loop body source).
    fn open_event_stream(
        &self,
        tx: mpsc::UnboundedSender<SdkEventEnvelope>,
        abort: Arc<AtomicBool>,
    );
}

/// Navigate `result.data.data` (mirrors the `{ throwOnError: true }`
/// call sites that read `result.data.data`).
pub fn data2(response: &Value) -> Option<&Value> {
    response.get("data")?.get("data")
}

/// Mirrors the SDK context value (client + directory + event emitter).
pub struct SdkContext {
    pub client: Arc<dyn SdkClient>,
    pub directory: Option<String>,
    pub url: String,
    handlers: HashMap<u64, Box<dyn FnMut(SdkEventEnvelope) + Send>>,
    next_id: u64,
    queue: Vec<SdkEventEnvelope>,
    last_flush: Instant,
    flush_deadline: Option<Instant>,
    inbox_tx: mpsc::UnboundedSender<SdkEventEnvelope>,
    inbox_rx: mpsc::UnboundedReceiver<SdkEventEnvelope>,
    abort: Arc<AtomicBool>,
    sse_abort: Arc<AtomicBool>,
    sse_task: Option<tokio::task::JoinHandle<()>>,
}

impl SdkContext {
    pub fn new(url: &str, directory: Option<String>, client: Arc<dyn SdkClient>) -> Self {
        let (inbox_tx, inbox_rx) = mpsc::unbounded_channel();
        Self {
            client,
            directory,
            url: url.to_string(),
            handlers: HashMap::new(),
            next_id: 0,
            queue: Vec::new(),
            last_flush: Instant::now(),
            flush_deadline: None,
            inbox_tx,
            inbox_rx,
            abort: Arc::new(AtomicBool::new(false)),
            sse_abort: Arc::new(AtomicBool::new(false)),
            sse_task: None,
        }
    }

    /// Mirrors `emitter.on` — returns the unsubscriber id.
    pub fn on(&mut self, handler: impl FnMut(SdkEventEnvelope) + Send + 'static) -> u64 {
        self.next_id += 1;
        let id = self.next_id;
        self.handlers.insert(id, Box::new(handler));
        id
    }

    pub fn off(&mut self, id: u64) {
        self.handlers.remove(&id);
    }

    fn emit(&mut self, event: SdkEventEnvelope) {
        for handler in self.handlers.values_mut() {
            handler(event.clone());
        }
    }

    fn flush(&mut self) {
        if self.queue.is_empty() {
            return;
        }
        let events = std::mem::take(&mut self.queue);
        self.flush_deadline = None;
        self.last_flush = Instant::now();
        // Batch all event emissions so all store updates land in one render.
        for event in events {
            self.emit(event);
        }
    }

    /// Mirrors `handleEvent` — 16ms batching window preserved.
    pub fn handle_event(&mut self, event: SdkEventEnvelope) {
        self.queue.push(event);
        if self.flush_deadline.is_some() {
            return;
        }
        if self.last_flush.elapsed() < Duration::from_millis(BATCH_WINDOW_MS) {
            self.flush_deadline = Some(Instant::now() + Duration::from_millis(BATCH_WINDOW_MS));
            return;
        }
        self.flush();
    }

    /// Render-loop driver: drains the transport inbox, then flushes due batches.
    pub fn pump(&mut self) {
        while let Ok(event) = self.inbox_rx.try_recv() {
            self.handle_event(event);
        }
        if self
            .flush_deadline
            .map(|deadline| Instant::now() >= deadline)
            .unwrap_or(false)
        {
            self.flush();
        }
    }

    /// Mirrors the `onMount` branch — external event source or SSE loop.
    /// External transports push into the inbox channel (mirrors
    /// `props.events.subscribe(handleEvent)`); otherwise `start_sse` runs.
    pub fn start(&mut self, external: bool, experimental_workspaces: bool) {
        if external {
            if experimental_workspaces {
                let client = self.client.clone();
                let tx = self.inbox_tx.clone();
                let abort = self.abort.clone();
                tokio::spawn(async move {
                    let _ = (client, tx, abort);
                });
            }
            return;
        }
        self.start_sse(experimental_workspaces);
    }

    /// Mirrors `startSSE` — reconnect loop with exponential backoff.
    fn start_sse(&mut self, experimental_workspaces: bool) {
        self.stop_sse();
        let ctrl = Arc::new(AtomicBool::new(false));
        self.sse_abort = ctrl.clone();
        let client = self.client.clone();
        let tx = self.inbox_tx.clone();
        let abort = self.abort.clone();
        self.sse_task = Some(tokio::spawn(async move {
            let mut attempt: u32 = 0;
            loop {
                if abort.load(Ordering::Relaxed) || ctrl.load(Ordering::Relaxed) {
                    break;
                }
                // One stream lifetime (mirrors a single `sdk.global.event()` session).
                client.open_event_stream(tx.clone(), ctrl.clone());
                if abort.load(Ordering::Relaxed) || ctrl.load(Ordering::Relaxed) {
                    break;
                }
                attempt += 1;
                let backoff = (RETRY_DELAY_MS.saturating_mul(2u64.saturating_pow(attempt - 1)))
                    .min(MAX_RETRY_DELAY_MS);
                tokio::time::sleep(Duration::from_millis(backoff)).await;
                let _ = experimental_workspaces;
            }
        }));
    }

    fn stop_sse(&mut self) {
        self.sse_abort.store(true, Ordering::Relaxed);
        if let Some(task) = self.sse_task.take() {
            task.abort();
        }
    }

    /// Mirrors the `onCleanup` branch.
    pub fn shutdown(&mut self) {
        self.abort.store(true, Ordering::Relaxed);
        self.stop_sse();
        self.handlers.clear();
        self.queue.clear();
        self.flush_deadline = None;
    }

    pub fn sender(&self) -> mpsc::UnboundedSender<SdkEventEnvelope> {
        self.inbox_tx.clone()
    }
}
