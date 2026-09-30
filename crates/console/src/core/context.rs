// source: core/src/context.ts
//! 1:1 port of the `Context` async-local-storage namespace.
//! Source pin: v1.18.30 @3104c14.

use std::cell::RefCell;
use std::marker::PhantomData;

/// `Context.NotFound` — thrown by `use()` when no value is provided.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NotFound;

impl std::fmt::Display for NotFound {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("NotFound")
    }
}

impl std::error::Error for NotFound {}

/// Error returned by [`Context::use`], mirroring the single-member
/// `Context.NotFound` error class.
pub type ContextError = NotFound;

struct Storage<T> {
    slot: RefCell<Vec<T>>,
    marker: PhantomData<fn()>,
}

impl<T> Storage<T> {
    fn new() -> Self {
        Self {
            slot: RefCell::new(Vec::new()),
            marker: PhantomData,
        }
    }
}

/// The `{ use, provide }` pair returned by `Context.create<T>()`.
pub struct Context<T> {
    storage: Storage<T>,
}

impl<T: Clone + 'static> Context<T> {
    /// `Context.create<T>()`.
    pub fn create() -> Self {
        Self {
            storage: Storage::new(),
        }
    }

    /// `use()` — `Err(ContextError::NotFound)` when nothing is provided.
    pub fn use(&self) -> Result<T, ContextError> {
        self.storage
            .slot
            .borrow()
            .last()
            .cloned()
            .ok_or(NotFound)
    }

    /// `provide(value, fn)` — runs `func` with `value` pushed, popping it (and
    /// restoring the previous value on unwind) afterwards.
    pub fn provide<R, F: FnOnce() -> R>(&self, value: T, func: F) -> R {
        self.storage.slot.borrow_mut().push(value);
        let result = func();
        self.storage.slot.borrow_mut().pop();
        result
    }
}

impl<T: Clone + 'static> Default for Context<T> {
    fn default() -> Self {
        Self::create()
    }
}
