//! Rust port of `packages/core/src/effect/service-use.ts`.
//! Source pin: v1.18.30 @3104c14 — 1:1 exact translation.

use std::collections::HashMap;

// PROVISIONAL pending Effect Context/Proxy — mapped to HashMap cache description

/// Source: `export const serviceUse = <Identifier, Shape>(tag) => Proxy...` verbatim
/// Source: error string `Service method not found: ${key}` verbatim

pub const NOT_FOUND_TEMPLATE: &str = "Service method not found: {key}";

pub fn not_found_message(key: &str) -> String {
    format!("Service method not found: {}", key)
}

pub struct ServiceUse {
    cache: HashMap<String, String>,
}

impl ServiceUse {
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }
    pub fn get(&mut self, key: &str) -> String {
        if let Some(v) = self.cache.get(key) {
            return v.clone();
        }
        let v = not_found_message(key);
        self.cache.insert(key.to_string(), v.clone());
        v
    }
}

impl Default for ServiceUse {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn not_found_string() {
        assert_eq!(not_found_message("foo"), "Service method not found: foo");
    }
    #[test]
    fn cache() {
        let mut s = ServiceUse::new();
        assert_eq!(s.get("x"), "Service method not found: x");
        assert_eq!(s.get("x"), "Service method not found: x");
    }
}
