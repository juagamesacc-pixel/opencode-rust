// source: src/util/defer.ts — exports: defer (verbatim).
// Minimal equivalent: Rust has no Symbol.dispose; disposal runs the closure
// synchronously on drop, async-dispose awaits it. Verbatim ordering.

/// source: defer(fn) — returns a guard running fn on drop.
pub struct Deferred<F: FnOnce()> {
    f: Option<F>,
}

impl<F: FnOnce()> Deferred<F> {
    pub fn new(f: F) -> Self {
        Self { f: Some(f) }
    }
}

/// source: [Symbol.dispose]() { void fn() } — verbatim (fire-and-forget on drop).
impl<F: FnOnce()> Drop for Deferred<F> {
    fn drop(&mut self) {
        if let Some(f) = self.f.take() {
            f();
        }
    }
}

/// source: defer() — verbatim constructor.
pub fn defer<F: FnOnce()>(f: F) -> Deferred<F> {
    Deferred::new(f)
}
