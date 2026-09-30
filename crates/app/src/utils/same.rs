//! Rust port of `packages/app/src/utils/same.ts` (opencode v1.18.30).
//!
//! Source 6 lines: `same(a, b)` — reference equality first, then both-undefined,
//! then length, then elementwise equality.
//!
//! 1:1 notes: the `a === b` reference-identity fast path is subsumed by the
//! value comparison (identical arrays compare equal elementwise).
//! Original file: `packages/app/src/utils/same.ts`

#![allow(dead_code)]

/// Mirrors `same<T>(a: readonly T[] | undefined, b: readonly T[] | undefined)`.
pub fn same<T: PartialEq>(a: Option<&[T]>, b: Option<&[T]>) -> bool {
    match (a, b) {
        (None, None) => true,
        (None, Some(_)) => false,
        (Some(_), None) => false,
        (Some(a), Some(b)) => a.len() == b.len() && a.iter().zip(b.iter()).all(|(x, y)| x == y),
    }
}
