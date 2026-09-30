// source: src/util/lazy.ts — exports: lazy
//
// TS `lazy(fn)` returns a zero-arg closure that computes `fn()` once and
// caches it. Rust returns a `Lazy<T>` handle — `get()` mirrors the cached
// closure call (same value on repeat calls).

use std::cell::OnceCell;

/// Port of the `lazy(fn)` memoizing closure.
pub struct Lazy<T> {
    value: OnceCell<T>,
    init: Option<Box<dyn FnOnce() -> T>>,
}

impl<T> Lazy<T> {
    pub fn new(init: impl FnOnce() -> T + 'static) -> Self {
        Lazy {
            value: OnceCell::new(),
            init: Some(Box::new(init)),
        }
    }

    /// Mirrors calling the lazy closure: returns the cached value, computing
    /// `fn()` on the first call. UNSAFE-free; panics only if `init` was
    /// already consumed (impossible through `get` alone).
    pub fn get(&self) -> &T {
        if self.value.get().is_none() {
            // Take the init closure once; get_or_init is single-writer.
            let init = self.init.take().expect("lazy init already consumed");
            let _ = self.value.set(init());
        }
        self.value.get().expect("lazy value not initialized")
    }
}

/// source: `lazy(fn)` — memoized thunk.
pub fn lazy<T>(f: impl FnOnce() -> T + 'static) -> Lazy<T> {
    Lazy::new(f)
}
