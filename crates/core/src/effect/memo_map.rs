//! Rust port of `packages/core/src/effect/memo-map.ts`.
//! Source pin: v1.18.30 @3104c14 — 1:1 exact translation.

//! Source (3 lines):
//! ```ts
//! import { Layer } from "effect"
//! export const memoMap = Layer.makeMemoMapUnsafe()
//! ```

// PROVISIONAL pending Effect Layer — mapped to opaque

pub struct MemoMap;

pub fn memo_map() -> MemoMap {
    MemoMap
}
