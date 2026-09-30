// source: core/src/drizzle/index.ts
//! 1:1 port of the `Database` namespace. Source pin: v1.18.30 @3104c14.
//!
//! PROVISIONAL: the source wires `drizzle-orm/planetscale-serverless` over a
//! `@planetscale/database` `Client` built from `Resource.Database`. Neither
//! driver has a Rust equivalent, so the connection is a descriptor carrying the
//! verbatim keys. The *control flow* — the `Context`-backed transaction reuse,
//! the deferred `effect()` queue and the nesting rules — is fully ported so it
//! stays exercisable.

use crate::core::context::{Context, NotFound};
use std::cell::RefCell;
use std::rc::Rc;

/// `Resource.Database` — the PlanetScale connection descriptor. Key names are
/// verbatim from the source's `new Client({ host, username, password })`.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct DatabaseResource {
    pub host: String,
    pub username: String,
    pub password: String,
}

/// A deferred `Database.effect` closure, flushed after the enclosing transaction.
pub type Effect = Box<dyn Fn()>;

#[derive(Clone)]
struct TransactionState {
    effects: Rc<RefCell<Vec<Effect>>>,
}

thread_local! {
    static TRANSACTION_CONTEXT: Context<TransactionState> = Context::create();
    static CLIENT: RefCell<Option<DatabaseResource>> = const { RefCell::new(None) };
}

/// The `Database` namespace.
pub struct Database;

impl Database {
    /// The module-private `memo(() => new Client({ ... }))`.
    fn client() -> DatabaseResource {
        let existing = CLIENT.with(|slot| slot.borrow().clone());
        if let Some(existing) = existing {
            return existing;
        }
        let result = DatabaseResource {
            host: crate::resource::node::Sst::get("Database", "host"),
            username: crate::resource::node::Sst::get("Database", "username"),
            password: crate::resource::node::Sst::get("Database", "password"),
        };
        CLIENT.with(|slot| *slot.borrow_mut() = Some(result.clone()));
        result
    }

    /// Seeds the memoized `client`. The source's `client` is module-private, so
    /// this is the only seam the ported control flow needs to be testable.
    pub fn set_client(resource: DatabaseResource) {
        CLIENT.with(|slot| *slot.borrow_mut() = Some(resource));
    }

    /// Drops the memoized `client` so the next access rebuilds it.
    pub fn reset_client() {
        CLIENT.with(|slot| *slot.borrow_mut() = None);
    }

    /// `Database.use(callback)`.
    ///
    /// Nested inside an ambient transaction the callback joins it; at the top
    /// level a fresh `effects` queue is provided, the callback runs against
    /// `client()`, and every queued effect is flushed afterwards.
    pub fn use_tx<T, F: FnOnce(&DatabaseResource) -> T>(callback: F) -> T {
        if TRANSACTION_CONTEXT.with(|ctx| ctx.use()).is_ok() {
            return callback(&Database::client());
        }
        let effects: Rc<RefCell<Vec<Effect>>> = Rc::new(RefCell::new(Vec::new()));
        let result = TRANSACTION_CONTEXT.with(|ctx| {
            ctx.provide(TransactionState { effects: effects.clone() }, || {
                callback(&Database::client())
            })
        });
        flush(&effects);
        result
    }

    /// `Database.fn(callback)` — `input => use(tx => callback(input, tx))`.
    pub fn fn_with<Input, T, F>(callback: F) -> impl Fn(Input) -> T
    where
        F: Fn(Input, &DatabaseResource) -> T,
    {
        move |input| Database::use_tx(|tx| callback(input, tx))
    }

    /// `Database.effect(effect)` — queued while a transaction is ambient, run
    /// immediately otherwise.
    pub fn effect<F: FnOnce() + 'static>(effect: F) {
        let slot: RefCell<Option<F>> = RefCell::new(Some(effect));
        let queued = TRANSACTION_CONTEXT.with(|ctx| match ctx.use() {
            Ok(state) => {
                state.effects.borrow_mut().push(Box::new(move || {
                    if let Some(effect) = slot.borrow_mut().take() {
                        effect();
                    }
                }));
                true
            }
            Err(NotFound) => false,
        });
        if !queued {
            if let Some(effect) = slot.borrow_mut().take() {
                effect();
            }
        }
    }

    /// `Database.transaction(callback, config?)`.
    ///
    /// The ambient-transaction branch returns `callback(tx)` directly; the
    /// top-level branch opens a transaction, provides the `effects` queue to the
    /// callback, then flushes the queue.
    pub fn transaction<T, F: FnOnce(&DatabaseResource) -> T>(callback: F) -> T {
        if TRANSACTION_CONTEXT.with(|ctx| ctx.use()).is_ok() {
            return callback(&Database::client());
        }
        let effects: Rc<RefCell<Vec<Effect>>> = Rc::new(RefCell::new(Vec::new()));
        let result = TRANSACTION_CONTEXT.with(|ctx| {
            ctx.provide(TransactionState { effects: effects.clone() }, || {
                callback(&Database::client())
            })
        });
        flush(&effects);
        result
    }
}

/// `await Promise.all(effects.map((x) => x()))`.
fn flush(effects: &Rc<RefCell<Vec<Effect>>>) {
    for effect in effects.borrow_mut().drain(..) {
        effect();
    }
}
