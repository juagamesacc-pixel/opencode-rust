//! Rust port of `packages/app/src/utils/refcount.ts` (opencode v1.18.30).
//!
//! Source 32 lines: `createRefCountMap(create, remove?, identity?)` returns an
//! acquire function. Each call registers a Solid `onCleanup`; when the owning
//! scope is disposed the refcount decrements, and at zero the item is removed
//! (after calling `remove(id)`).
//!
//! 1:1 notes:
//! - Solid's `createRoot`/`onCleanup` lifecycle has no Rust equivalent; the
//!   acquire call returns an RAII `RefCountToken` whose `Drop` plays the role
//!   of the disposed owner (decrement, and remove at zero). The 1:1 test
//!   (`refcount.test.ts`) keeps tokens alive and drops them instead of calling
//!   `dispose()`.
//! - `identity` defaults to the identity function; `remove` is optional.
//! - Original file: `packages/app/src/utils/refcount.ts`

#![allow(dead_code)]

use std::cell::RefCell;
use std::collections::HashMap;

/// Mirrors `createRefCountMap`'s returned acquire function.
#[allow(clippy::type_complexity)] // 1:1 source uses Box<dyn Fn> complexity
pub struct RefCountMap<T: 'static> {
    create: Box<dyn Fn(String) -> T>,
    items: RefCell<HashMap<String, T>>,
    ref_counts: RefCell<HashMap<String, u32>>,
    remove: Option<Box<dyn Fn(&str)>>,
    identity: Box<dyn Fn(&str) -> String>,
}

impl<T: 'static> RefCountMap<T> {
    /// Mirrors `createRefCountMap(create, remove?, identity?)`.
    #[allow(clippy::type_complexity)] // 1:1 preserves exact signatures
    pub fn new(
        create: Box<dyn Fn(String) -> T>,
        remove: Option<Box<dyn Fn(&str)>>,
        identity: Box<dyn Fn(&str) -> String>,
    ) -> Self {
        RefCountMap {
            create,
            items: RefCell::new(HashMap::new()),
            ref_counts: RefCell::new(HashMap::new()),
            remove,
            identity,
        }
    }

    /// Mirrors one call of the returned acquire function: registers ownership
    /// of `key` and returns the token that releases it when dropped.
    pub fn acquire(&self, key: &str) -> RefCountToken<'_, T> {
        let id = (self.identity)(key);
        let cached = self.items.borrow().get(&id).is_some();
        if cached {
            let mut ref_counts = self.ref_counts.borrow_mut();
            *ref_counts.entry(id.clone()).or_insert(0) += 1;
        } else {
            let item = (self.create)(key.to_string());
            self.items.borrow_mut().insert(id.clone(), item);
            self.ref_counts.borrow_mut().insert(id.clone(), 1);
        }
        RefCountToken {
            owner: self,
            id,
            active: true,
        }
    }
}

impl<T> std::fmt::Debug for RefCountMap<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RefCountMap").finish_non_exhaustive()
    }
}

/// RAII mirror of one Solid `onCleanup` owner registration.
pub struct RefCountToken<'a, T: 'static> {
    owner: &'a RefCountMap<T>,
    id: String,
    active: bool,
}

impl<T> Drop for RefCountToken<'_, T> {
    fn drop(&mut self) {
        if !self.active {
            return;
        }
        self.active = false;
        let mut ref_counts = self.owner.ref_counts.borrow_mut();
        let next = ref_counts
            .get(&self.id)
            .copied()
            .unwrap_or(0)
            .saturating_sub(1);
        if next == 0 {
            drop(ref_counts);
            if let Some(remove) = &self.owner.remove {
                remove(&self.id);
            }
            self.owner.items.borrow_mut().remove(&self.id);
            self.owner.ref_counts.borrow_mut().remove(&self.id);
        } else {
            ref_counts.insert(self.id.clone(), next);
        }
    }
}

impl<T> std::fmt::Debug for RefCountToken<'_, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RefCountToken")
            .field("id", &self.id)
            .finish_non_exhaustive()
    }
}
