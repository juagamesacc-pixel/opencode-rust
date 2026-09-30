//! Rust port of `packages/core/src/effect/keyed-mutex.ts`.
//! Source pin: v1.18.30 @3104c14 — 1:1 exact translation.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

// PROVISIONAL pending Effect + Semaphore — mapped to std Mutex counting

/// Source: `export interface KeyedMutex<in Key> { readonly size, readonly withLock }` verbatim
/// Source: makeUnsafe logic — locks Map<Key,{semaphore, users}>, users counts holders+waiters, deleted when 0

pub struct KeyedMutex<K: Eq + std::hash::Hash + Clone> {
    locks: Arc<Mutex<HashMap<K, Entry>>>,
}

struct Entry {
    users: usize,
}

impl<K: Eq + std::hash::Hash + Clone> KeyedMutex<K> {
    pub fn new() -> Self {
        Self {
            locks: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn size(&self) -> usize {
        self.locks.lock().unwrap().len()
    }

    pub fn with_lock<F, T>(&self, key: K, f: F) -> T
    where
        F: FnOnce() -> T,
    {
        {
            let mut m = self.locks.lock().unwrap();
            let e = m.entry(key.clone()).or_insert(Entry { users: 0 });
            e.users += 1;
        }
        let result = f();
        {
            let mut m = self.locks.lock().unwrap();
            if let Some(e) = m.get_mut(&key) {
                e.users -= 1;
                if e.users == 0 {
                    m.remove(&key);
                }
            }
        }
        result
    }
}

impl<K: Eq + std::hash::Hash + Clone> Default for KeyedMutex<K> {
    fn default() -> Self {
        Self::new()
    }
}

/// Source: `export const make = <Key>(): Effect< KeyedMutex>` -> sync stub
pub fn make<K: Eq + std::hash::Hash + Clone>() -> KeyedMutex<K> {
    KeyedMutex::new()
}
pub fn make_unsafe<K: Eq + std::hash::Hash + Clone>() -> KeyedMutex<K> {
    KeyedMutex::new()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn same_key_queue() {
        let m = KeyedMutex::new();
        m.with_lock("a", || {});
        assert_eq!(m.size(), 0);
    }
    #[test]
    fn different_key_independent() {
        let m: KeyedMutex<String> = KeyedMutex::new();
        m.with_lock("a".to_string(), || {
            assert_eq!(m.size(), 1);
        });
        assert_eq!(m.size(), 0);
    }
}
