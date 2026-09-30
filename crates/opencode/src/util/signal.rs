// source: src/util/signal.ts — exports: signal (verbatim: trigger/wait pair).
use std::sync::{Arc, Condvar, Mutex};

/// source: signal() — trigger()/wait() pair, verbatim semantics.
#[derive(Debug, Clone)]
pub struct Signal {
    inner: Arc<(Mutex<bool>, Condvar)>,
}

impl Signal {
    pub fn new() -> Self {
        Self {
            inner: Arc::new((Mutex::new(false), Condvar::new())),
        }
    }

    /// source: trigger() — resolves the waiter, verbatim.
    pub fn trigger(&self) {
        let (lock, cvar) = &*self.inner;
        *lock.lock().unwrap() = true;
        cvar.notify_all();
    }

    /// source: wait() — blocks until triggered, verbatim.
    pub fn wait(&self) {
        let (lock, cvar) = &*self.inner;
        let mut fired = lock.lock().unwrap();
        while !*fired {
            fired = cvar.wait(fired).unwrap();
        }
    }
}

impl Default for Signal {
    fn default() -> Self {
        Self::new()
    }
}

/// source: signal() — verbatim constructor.
pub fn signal() -> Signal {
    Signal::new()
}
