// source: src/effect/bridge.ts — exports: Shape, bind, fromPromise, make,
// EffectBridge (workspace-restore + ref-capture rules verbatim).
// PROVISIONAL: fiber/ALS capture modelled as descriptors.

/// source: Shape { promise, fork, run, bind } — verbatim keys.
pub const SHAPE_KEYS: &[&str] = &["promise", "fork", "run", "bind"];
