// source: src/util/iife.ts — exports: iife

/// source: `iife(fn)` — immediately-invoked function expression; returns the
/// call's result. Verbatim.
pub fn iife<T>(f: impl FnOnce() -> T) -> T {
    f()
}
