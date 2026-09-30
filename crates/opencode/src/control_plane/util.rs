// source: src/control-plane/util.ts — exports: waitEvent
// (pre-aborted check, abort/timeout cleanup, predicate-match resolve,
// "Request aborted" + "Timed out waiting for global event" verbatim).

/// source: "Request aborted" — verbatim.
pub const ABORTED_MESSAGE: &str = "Request aborted";
/// source: "Timed out waiting for global event" — verbatim.
pub const TIMEOUT_MESSAGE: &str = "Timed out waiting for global event";
