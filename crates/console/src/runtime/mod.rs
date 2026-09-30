//! Host-runtime shims for the ECMAScript globals the TypeScript source relies on.
//!
//! 1:1 port helper — NOT a source file. `packages/console` uses the JS `Date`,
//! `URL`, `Number`/`Math` coercions, `TextEncoder`/`TextDecoder` and `crypto`
//! globals. Those are host runtime, not package source, so they are reproduced
//! here with identical observable semantics and no behavior reinterpretation.

pub mod num;
pub mod text;
pub mod time;
pub mod ulid;
pub mod url;
