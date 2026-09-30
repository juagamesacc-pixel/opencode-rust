// source: core/src/util/log.ts
//! 1:1 port of the `Log` namespace. Source pin: v1.18.30 @3104c14.

use crate::core::context::Context;
use std::cell::RefCell;
use std::rc::Rc;

/// The `Log` namespace's private context store: `{ tags: Record<string, any> }`.
pub struct LogTags {
    pub tags: Vec<(String, String)>,
}

impl LogTags {
    fn new() -> Self {
        Self { tags: Vec::new() }
    }
}

impl Default for LogTags {
    fn default() -> Self {
        Self::new()
    }
}

thread_local! {
    static CTX: Context<LogTags> = Context::create();
}

/// The object returned by `Log.create()`.
#[derive(Clone)]
pub struct Logger {
    /// `tags` is captured by value and mutated by `tag()`, exactly like the
    /// source's closure-local `tags` object.
    tags: Rc<RefCell<Vec<(String, String)>>>,
}

/// Every line emitted while the port runs, captured in place of `console.log`
/// so ordering and formatting stay assertable.
thread_local! {
    static EMITTED: RefCell<Vec<String>> = const { RefCell::new(Vec::new()) };
}

/// The `Log` namespace.
pub struct Log;

impl Log {
    /// `Log.create(tags?)`.
    pub fn create(tags: Option<Vec<(String, String)>>) -> Logger {
        Logger {
            tags: Rc::new(RefCell::new(tags.unwrap_or_default())),
        }
    }

    /// `Log.provide(tags, cb)` — merges `tags` over the ambient ones for the
    /// duration of `cb`.
    pub fn provide<R, F: FnOnce() -> R>(tags: Vec<(String, String)>, cb: F) -> R {
        let existing = Self::use_or_default();
        let mut merged = existing;
        for (key, value) in tags {
            match merged.iter_mut().find(|(existing_key, _)| *existing_key == key) {
                Some(slot) => slot.1 = value,
                None => merged.push((key, value)),
            }
        }
        CTX.with(|ctx| ctx.provide(LogTags { tags: merged }, cb))
    }

    /// `use()` — the private helper behind `info`, falling back to `{ tags: {} }`
    /// when nothing is provided.
    fn use_or_default() -> Vec<(String, String)> {
        CTX
            .with(|ctx| ctx.use().map(|value| value.tags))
            .unwrap_or_default()
    }

    /// Captured `console.log` output, in emission order.
    pub fn emitted() -> Vec<String> {
        EMITTED.with(|lines| lines.borrow().clone())
    }

    /// Clear the captured output.
    pub fn clear() {
        EMITTED.with(|lines| lines.borrow_mut().clear());
    }
}

impl Logger {
    /// `result.info(message?, extra?)` — emits
    /// `"<k>=<v> …<message>"` (the `prefix` argument and the message are joined by
    /// a single space, and `console.log` stringifies the array as
    /// `prefix message`).
    pub fn info(&self, message: Option<&str>, extra: Option<Vec<(String, String)>>) -> Logger {
        let mut entries = Log::use_or_default();
        for (key, value) in self.tags.borrow().iter() {
            match entries.iter_mut().find(|(existing, _)| existing == key) {
                Some(slot) => slot.1 = value.clone(),
                None => entries.push((key.clone(), value.clone())),
            }
        }
        if let Some(extra) = extra {
            for (key, value) in extra {
                match entries.iter_mut().find(|(existing, _)| *existing == key) {
                    Some(slot) => slot.1 = value,
                    None => entries.push((key, value)),
                }
            }
        }
        let prefix = entries
            .iter()
            .map(|(key, value)| format!("{}={}", key, value))
            .collect::<Vec<_>>()
            .join(" ");
        let line = match message {
            Some(message) => format!("{} {}", prefix, message),
            None => prefix,
        };
        EMITTED.with(|lines| lines.borrow_mut().push(line));
        self.clone()
    }

    /// `result.tag(key, value)` — mutates and returns the same logger.
    pub fn tag(&self, key: &str, value: &str) -> Logger {
        let mut tags = self.tags.borrow_mut();
        match tags.iter_mut().find(|(existing, _)| existing == key) {
            Some(slot) => slot.1 = value.to_string(),
            None => tags.push((key.to_string(), value.to_string())),
        }
        drop(tags);
        self.clone()
    }

    /// `result.clone()` — a fresh logger seeded with a shallow copy of the tags.
    pub fn clone_logger(&self) -> Logger {
        Log::create(Some(self.tags.borrow().clone()))
    }
}
