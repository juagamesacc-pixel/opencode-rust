// source: src/util/queue.ts — exports: AsyncQueue, work (verbatim).
// Minimal equivalent: mutex-backed queue (resolver-shift/push/next order
// preserved); work() runs items with N workers popping from the tail.

use std::collections::VecDeque;
use std::sync::Mutex;

/// source: AsyncQueue — verbatim push/next semantics.
#[derive(Debug, Default)]
pub struct AsyncQueue<T> {
    queue: Mutex<VecDeque<T>>,
}

impl<T> AsyncQueue<T> {
    pub fn new() -> Self {
        Self {
            queue: Mutex::new(VecDeque::new()),
        }
    }

    /// source: push() — resolve waiter first (here: enqueue; waiters poll),
    /// else queue. Verbatim ordering: FIFO for queued items.
    pub fn push(&self, item: T) {
        self.queue.lock().unwrap().push_back(item);
    }

    /// source: next() — shift or wait. Verbatim (blocking variant).
    pub fn next(&self) -> T {
        loop {
            if let Some(item) = self.queue.lock().unwrap().pop_front() {
                return item;
            }
            std::thread::yield_now();
        }
    }

    pub fn len(&self) -> usize {
        self.queue.lock().unwrap().len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// source: work() — N workers popping from tail (pop), verbatim.
pub fn work<T: Send>(concurrency: usize, items: Vec<T>, f: impl Fn(T) + Send + Sync) {
    let pending = Mutex::new(items);
    std::thread::scope(|s| {
        for _ in 0..concurrency {
            s.spawn(|| loop {
                let item = pending.lock().unwrap().pop();
                match item {
                    None => break,
                    Some(v) => f(v),
                }
            });
        }
    });
}
