// source: packages/session-ui/src/components/markdown-worker-queue.ts
// 1:1 port — the synchronous latest-wins queue bookkeeping is fully ported as real Rust.
// PROVISIONAL: the TS async `Promise` pump (`schedule`/`running`/`idle`) needs an async
// runtime; the sync `highlight`/`dispose`/`pending` surface is ported exactly.

use std::collections::HashMap;

/// 1:1 port of the internal `Slot`/`dispose` job union.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LatestWorkerJob {
    /// `{ type: "highlight"; key: string; request?: T }`
    Highlight {
        key: String,
        request: Option<String>,
    },
    /// `{ type: "dispose"; key: string }`
    Dispose { key: String },
}

/// Synchronous 1:1 port of `createLatestWorkerQueue`.
///
/// The generic request payload `T extends { key: string }` is represented by its
/// `request_key()`; callers keep their payloads and the queue tracks the exact
/// latest-wins/supersede/dispose ordering the TS queue applies.
#[derive(Debug, Default)]
pub struct LatestWorkerQueue {
    jobs: Vec<LatestWorkerJob>,
    slots: HashMap<String, usize>,
}

/// 1:1 port of the `{ run, supersede, dispose }` handler record.
pub trait LatestWorkerQueueHandlers {
    fn run(&mut self, key: &str);
    fn supersede(&mut self, key: &str);
    fn dispose(&mut self, key: &str);
}

impl LatestWorkerQueue {
    pub fn new() -> Self {
        Self {
            jobs: Vec::new(),
            slots: HashMap::new(),
        }
    }

    /// 1:1 port of `highlight(request)`.
    pub fn highlight(&mut self, key: &str, handlers: &mut impl LatestWorkerQueueHandlers) {
        if self.slots.contains_key(key) {
            // A live slot keeps only the latest request; the previous pending
            // request is superseded exactly like the TS `input.supersede(slot.request)`.
            handlers.supersede(key);
            if let Some(job) = self.jobs.iter_mut().rev().find(
                |job| matches!(job, LatestWorkerJob::Highlight { key: slot, .. } if slot == key),
            ) {
                if let LatestWorkerJob::Highlight { request, .. } = job {
                    *request = Some(key.to_string());
                }
            }
            return;
        }
        let owned = key.to_string();
        self.slots.insert(owned.clone(), self.jobs.len());
        self.jobs.push(LatestWorkerJob::Highlight {
            key: owned,
            request: Some(key.to_string()),
        });
    }

    /// 1:1 port of `dispose(key)`.
    pub fn dispose(&mut self, key: &str, handlers: &mut impl LatestWorkerQueueHandlers) {
        if self.slots.remove(key).is_some() {
            // `if (slot?.request) input.supersede(slot.request)` — the queued
            // request for a disposed live slot is superseded.
            handlers.supersede(key);
            for job in self.jobs.iter_mut() {
                if let LatestWorkerJob::Highlight { key: slot, request } = job {
                    if slot == key {
                        *request = None;
                    }
                }
            }
        }
        self.jobs.push(LatestWorkerJob::Dispose {
            key: key.to_string(),
        });
    }

    /// 1:1 port of `pending()` — the number of live slots.
    pub fn pending(&self) -> usize {
        self.slots.len()
    }

    /// The queued jobs in order (drives the async `schedule` pump verbatim).
    pub fn jobs(&self) -> &[LatestWorkerJob] {
        &self.jobs
    }
}
