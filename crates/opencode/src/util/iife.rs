// source: src/util/iife.ts — exports: iife (verbatim).
/// source: iife(fn) — immediately-invoked function expression, verbatim.
pub fn iife<T>(f: impl FnOnce() -> T) -> T {
    f()
}
