// source: src/effect/promise.ts — exports: refineRejection, EffectPromise
// (UnknownError cause-unwrap → refine → fail else die, verbatim).

/// source: refineRejection() cause rule — verbatim as pure decision fn.
pub fn refine_decision(refined: bool) -> &'static str {
    if refined {
        "fail"
    } else {
        "die"
    }
}
