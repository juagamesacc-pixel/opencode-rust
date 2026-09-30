//! Rust port of `packages/app/src/utils/scoped-cache.ts` (opencode v1.18.30).
//!
//! Source 104 lines: `createScopedCache` (maxEntries/TTL/dispose/now,
//! `get`/`peek`/`delete`/`clear`, LRU touch + sweep + prune). Ported
//! verbatim as an explicit state machine.
//! Original file: `packages/app/src/utils/scoped-cache.ts`

#![allow(dead_code)]

use std::collections::HashMap;

/// Mirrors `ScopedCacheOptions`.
#[derive(Debug, Clone, Default)]
pub struct ScopedCacheOptions {
    pub max_entries: Option<usize>,
    pub ttl_ms: Option<u64>,
}

/// Mirrors one cache entry.
#[derive(Debug, Clone)]
struct Entry<T: Clone> {
    value: T,
    touched_at: u64,
}

/// Mirrors `createScopedCache(createValue, options?)`.
#[derive(Debug, Clone)]
pub struct ScopedCache<T: Clone> {
    store: HashMap<String, Entry<T>>,
    order: Vec<String>,
    options: ScopedCacheOptions,
    pub disposed: Vec<String>,
    clock: u64,
}

impl<T: Clone> ScopedCache<T> {
    pub fn new(options: ScopedCacheOptions) -> Self {
        Self {
            store: HashMap::new(),
            order: vec![],
            options,
            disposed: vec![],
            clock: 0,
        }
    }

    /// Mirrors the `now` override (explicit clock for tests).
    pub fn update_clock(&mut self, now: u64) {
        self.clock = now;
    }

    fn expired(&self, entry: &Entry<T>) -> bool {
        match self.options.ttl_ms {
            None => false,
            Some(ttl) => self.clock.saturating_sub(entry.touched_at) >= ttl,
        }
    }

    fn touch(&mut self, key: &str) {
        if let Some(entry) = self.store.get_mut(key) {
            entry.touched_at = self.clock;
        }
        self.order.retain(|k| k != key);
        self.order.push(key.to_string());
    }

    fn sweep(&mut self) {
        if self.options.ttl_ms.is_none() {
            return;
        }
        let clock = self.clock;
        let ttl = self.options.ttl_ms.unwrap_or(u64::MAX);
        let stale: Vec<String> = self
            .store
            .iter()
            .filter(|(_, entry)| clock.saturating_sub(entry.touched_at) >= ttl)
            .map(|(key, _)| key.clone())
            .collect();
        for key in stale {
            self.store.remove(&key);
            self.order.retain(|k| k != &key);
            self.disposed.push(key);
        }
    }

    fn prune(&mut self) {
        let Some(max) = self.options.max_entries else {
            return;
        };
        while self.store.len() > max {
            let Some(oldest) = self.order.first().cloned() else {
                return;
            };
            self.order.remove(0);
            if self.store.remove(&oldest).is_some() {
                self.disposed.push(oldest);
            }
        }
    }

    /// Mirrors `peek(key)`.
    pub fn peek(&mut self, key: &str) -> Option<T> {
        self.sweep();
        let entry = self.store.get(key)?.clone();
        if self.expired(&entry) {
            self.store.remove(key);
            self.order.retain(|k| k != key);
            self.disposed.push(key.to_string());
            return None;
        }
        Some(entry.value)
    }

    /// Mirrors `get(key)` with `createValue` callback.
    pub fn get(&mut self, key: &str, create_value: impl FnOnce(&str) -> T) -> T {
        self.sweep();
        if let Some(entry) = self.store.get(key).cloned() {
            if !self.expired(&entry) {
                self.touch(key);
                return entry.value;
            }
            self.store.remove(key);
            self.order.retain(|k| k != key);
            self.disposed.push(key.to_string());
        }
        let value = create_value(key);
        self.store.insert(
            key.to_string(),
            Entry {
                value: value.clone(),
                touched_at: self.clock,
            },
        );
        self.order.retain(|k| k != key);
        self.order.push(key.to_string());
        self.prune();
        value
    }

    /// Mirrors `delete(key)`.
    pub fn transition_delete(&mut self, key: &str) -> Option<T> {
        let entry = self.store.remove(key)?;
        self.order.retain(|k| k != key);
        self.disposed.push(key.to_string());
        Some(entry.value)
    }

    /// Mirrors `clear()`.
    pub fn transition_clear(&mut self) {
        let keys: Vec<String> = self.store.keys().cloned().collect();
        self.store.clear();
        self.order.clear();
        self.disposed.extend(keys);
    }
}
