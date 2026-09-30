//! Rust port of `src/main/initialization.ts` (opencode v1.18.30).
//!
//! The source is a six-line Effect combinator: run `effect`, and on failure
//! copy its `Cause` into `initialization` via `Deferred.failCause`, so a
//! server that never became ready learns why. The observable behaviour is
//! ported 1:1 as a [`Deferred`] plus a fallible closure — the effect's own
//! result is returned unchanged in both arms.
//!
//! PROVISIONAL(packages/desktop/src/main/initialization.ts): `Deferred`,
//! `Effect`, `Effect.tapCause` and `Cause` are from the `effect` package,
//! which has no in-workspace Rust binding. [`Deferred`] is the local stand-in
//! and models only what this combinator observes: a single failure slot that
//! is written at most once (`Deferred.failCause` on an already-completed
//! deferred is a no-op in Effect).
//!
//! Original file: `packages/desktop/src/main/initialization.ts`

use std::cell::RefCell;
use std::rc::Rc;

/// `Deferred.Deferred<A, unknown>` — a single-slot completion cell.
#[derive(Debug)]
pub struct Deferred<A> {
    value: Rc<RefCell<Option<A>>>,
}

impl<A> Deferred<A> {
    pub fn new() -> Self {
        Self {
            value: Rc::new(RefCell::new(None)),
        }
    }

    /// `Deferred.failCause(deferred, cause)` / `Deferred.succeed(...)`: the
    /// first completion wins, as in Effect.
    pub fn complete(&self, value: A) -> bool {
        let mut slot = self.value.borrow_mut();
        if slot.is_some() {
            return false;
        }
        *slot = Some(value);
        true
    }

    /// `Deferred.isDone`-like read: has the deferred been completed?
    pub fn is_completed(&self) -> bool {
        self.value.borrow().is_some()
    }

    /// The completed value, if any.
    pub fn get(&self) -> Option<A>
    where
        A: Clone,
    {
        self.value.borrow().clone()
    }
}

impl<A> Default for Deferred<A> {
    fn default() -> Self {
        Self::new()
    }
}

/// `forwardInitializationFailure(initialization)(effect)`
///
/// Returns the effect's result unchanged, mirroring the cause into
/// `initialization` when it fails.
pub fn forward_initialization_failure<A, B, E>(
    initialization: &Deferred<E>,
    effect: impl FnOnce() -> Result<B, E>,
) -> Result<B, E>
where
    E: Clone,
{
    let result = effect();
    if let Err(cause) = &result {
        initialization.complete(cause.clone());
    }
    result
}

#[cfg(test)]
mod tests {
    // The source ships no `initialization.test.ts`; these cover the pass-through
    // and the first-completion-wins rule.
    use super::*;

    #[test]
    fn a_successful_effect_leaves_the_deferred_alone() {
        let deferred: Deferred<&str> = Deferred::new();
        let result = forward_initialization_failure(&deferred, || Ok::<_, &str>(7));
        assert_eq!(result, Ok(7));
        assert!(!deferred.is_completed());
    }

    #[test]
    fn a_failed_effect_mirrors_its_cause() {
        let deferred: Deferred<&str> = Deferred::new();
        let result = forward_initialization_failure(&deferred, || Err::<u8, _>("boom"));
        assert_eq!(result, Err("boom"));
        assert_eq!(deferred.get(), Some("boom"));
    }

    #[test]
    fn the_first_completion_wins() {
        let deferred: Deferred<&str> = Deferred::new();
        assert!(deferred.complete("first"));
        assert!(!deferred.complete("second"));
        assert_eq!(deferred.get(), Some("first"));

        // A later failure must not overwrite the recorded cause.
        let result = forward_initialization_failure(&deferred, || Err::<u8, _>("late"));
        assert_eq!(result, Err("late"));
        assert_eq!(deferred.get(), Some("first"));
    }
}
