// source: packages/session-ui/src/components/markdown-worker-transport.ts
// 1:1 port — the in-flight/queued request bookkeeping is fully ported as real Rust.
// Insertion order is preserved exactly like the TS `Map`s (`reset` supersedes
// queued requests in insertion order).

/// Synchronous 1:1 port of `createWorkerTransport`.
///
/// The generic payload `T extends { id: number; key: string }` is represented by
/// `(key, id)` pairs; callers keep their payloads while the transport tracks the
/// exact post/supersede/complete/dispose/reset ordering of the TS transport.
#[derive(Debug, Default)]
pub struct WorkerTransport {
    active: Vec<(String, u64)>,
    queued: Vec<(String, u64)>,
}

/// 1:1 port of the `{ post, supersede }` handler record.
pub trait WorkerTransportHandlers {
    fn post(&mut self, key: &str, id: u64);
    fn supersede(&mut self, key: &str, id: u64);
}

fn find(entries: &[(String, u64)], key: &str) -> Option<usize> {
    entries.iter().position(|(entry, _)| entry == key)
}

impl WorkerTransport {
    pub fn new() -> Self {
        Self {
            active: Vec::new(),
            queued: Vec::new(),
        }
    }

    /// 1:1 port of `send(request)`.
    pub fn send(&mut self, key: &str, id: u64, handlers: &mut impl WorkerTransportHandlers) {
        if find(&self.active, key).is_none() {
            self.active.push((key.to_string(), id));
            handlers.post(key, id);
            return;
        }
        match find(&self.queued, key) {
            Some(index) => {
                let previous = self.queued[index].1;
                self.queued[index].1 = id;
                handlers.supersede(key, previous);
            }
            None => self.queued.push((key.to_string(), id)),
        }
    }

    /// 1:1 port of `complete(key, id)`.
    pub fn complete(&mut self, key: &str, id: u64, handlers: &mut impl WorkerTransportHandlers) {
        let Some(index) = find(&self.active, key) else {
            return;
        };
        if self.active[index].1 != id {
            return;
        }
        self.active.remove(index);
        let Some(queued) = find(&self.queued, key) else {
            return;
        };
        let (_, next) = self.queued.remove(queued);
        self.active.push((key.to_string(), next));
        handlers.post(key, next);
    }

    /// 1:1 port of `dispose(key)`.
    pub fn dispose(&mut self, key: &str, handlers: &mut impl WorkerTransportHandlers) {
        if let Some(index) = find(&self.active, key) {
            self.active.remove(index);
        }
        if let Some(index) = find(&self.queued, key) {
            let (_, request) = self.queued.remove(index);
            handlers.supersede(key, request);
        }
    }

    /// 1:1 port of `reset()`.
    pub fn reset(&mut self, handlers: &mut impl WorkerTransportHandlers) {
        for (key, id) in std::mem::take(&mut self.queued) {
            handlers.supersede(&key, id);
        }
        self.active.clear();
    }

    /// 1:1 port of `queued()`.
    pub fn queued(&self) -> usize {
        self.queued.len()
    }

    /// The in-flight request id for `key`, if any.
    pub fn active(&self, key: &str) -> Option<u64> {
        find(&self.active, key).map(|index| self.active[index].1)
    }
}
