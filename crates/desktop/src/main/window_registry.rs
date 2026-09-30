//! Rust port of `src/main/window-registry.ts` (opencode v1.18.30).
//!
//! The source closes over a JS `Map` (insertion-ordered). The port keeps a
//! `HashMap` for lookups plus an insertion-order `Vec`, reproducing `Map`
//! iteration order for `windows.keys().next().value` exactly.
//!
//! Rust has no default function parameters, so `setQuitting(value = true)`
//! becomes `set_quitting(value: bool)` with the default applied at the call
//! sites (matching the source: `setAppQuitting(quitting = true)` in
//! `windows.ts`, and the bare `registry.setQuitting()` window `session-end`
//! handler passes `true`).
//!
//! Original file: `packages/desktop/src/main/window-registry.ts`

use std::collections::HashMap;

// Tracks open windows and the persisted window id list used to restore
// windows (and their per-window persisted state) across app launches.
pub struct WindowRegistry<W> {
    windows: HashMap<String, W>,
    order: Vec<String>,
    quitting: bool,
    last_focused_id: Option<String>,
    read: Box<dyn Fn() -> serde_json::Value>,
    write: Box<dyn Fn(Vec<String>)>,
    cleanup: Box<dyn Fn(String)>,
}

impl<W> WindowRegistry<W> {
    pub fn new(
        read: impl Fn() -> serde_json::Value + 'static,
        write: impl Fn(Vec<String>) + 'static,
        cleanup: impl Fn(String) + 'static,
    ) -> Self {
        Self {
            windows: HashMap::new(),
            order: Vec::new(),
            quitting: false,
            last_focused_id: None,
            read: Box::new(read),
            write: Box::new(write),
            cleanup: Box::new(cleanup),
        }
    }

    pub fn persisted(&self) -> Vec<String> {
        match (self.read)() {
            serde_json::Value::Array(items) => items
                .into_iter()
                .filter_map(|item| match item {
                    serde_json::Value::String(id) if !id.is_empty() => Some(id),
                    _ => None,
                })
                .collect(),
            _ => Vec::new(),
        }
    }

    pub fn set_quitting(&mut self, value: bool) {
        self.quitting = value;
    }

    pub fn register(&mut self, id: String, window: W) {
        if !self.windows.contains_key(&id) {
            self.order.push(id.clone());
        }
        self.windows.insert(id.clone(), window);
        let ids = self.persisted();
        if !ids.contains(&id) {
            let mut next = ids;
            next.push(id);
            (self.write)(next);
        }
    }

    pub fn focused(&mut self, id: String) {
        self.last_focused_id = Some(id);
    }

    pub fn last_focused(&self) -> Option<&W> {
        self.last_focused_id
            .as_ref()
            .and_then(|id| self.windows.get(id))
    }

    pub fn closed(&mut self, id: &str) {
        self.windows.remove(id);
        self.order.retain(|item| item != id);
        if self.last_focused_id.as_deref() == Some(id) {
            self.last_focused_id = self.order.first().cloned();
        }
        // Only a deliberate close (app keeps running with other windows open)
        // forgets a window. Closing the last window quits the app and fires
        // `closed` before `before-quit`, so treat it as a quit and keep the id
        // for restore on next launch.
        if self.quitting || self.windows.is_empty() {
            return;
        }
        let next: Vec<String> = self
            .persisted()
            .into_iter()
            .filter(|item| item != id)
            .collect();
        (self.write)(next);
        (self.cleanup)(id.to_string());
    }
}

#[cfg(test)]
mod tests {
    // Mirrors `src/main/window-registry.test.ts`
    // (`describe("window registry")`).
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    struct Setup {
        registry: WindowRegistry<String>,
        stored: Rc<RefCell<serde_json::Value>>,
        cleaned: Rc<RefCell<Vec<String>>>,
    }

    fn setup(initial: serde_json::Value) -> Setup {
        let stored = Rc::new(RefCell::new(initial));
        let cleaned = Rc::new(RefCell::new(Vec::new()));
        let read_stored = Rc::clone(&stored);
        let write_stored = Rc::clone(&stored);
        let cleanup_cleaned = Rc::clone(&cleaned);
        Setup {
            registry: WindowRegistry::new(
                move || read_stored.borrow().clone(),
                move |ids| {
                    *write_stored.borrow_mut() = serde_json::Value::Array(
                        ids.into_iter().map(serde_json::Value::String).collect(),
                    );
                },
                move |id| cleanup_cleaned.borrow_mut().push(id),
            ),
            stored,
            cleaned,
        }
    }

    fn stored_ids(setup: &Setup) -> Vec<String> {
        match &*setup.stored.borrow() {
            serde_json::Value::Array(items) => items
                .iter()
                .filter_map(|item| item.as_str().map(str::to_string))
                .collect(),
            _ => Vec::new(),
        }
    }

    #[test]
    fn restores_persisted_ids_and_ignores_malformed_entries() {
        assert_eq!(
            setup(serde_json::json!(["a", "", 42, "b"]))
                .registry
                .persisted(),
            vec!["a".to_string(), "b".to_string()]
        );
        assert_eq!(
            setup(serde_json::json!("junk")).registry.persisted(),
            Vec::<String>::new()
        );
        assert_eq!(
            setup(serde_json::Value::Null).registry.persisted(),
            Vec::<String>::new()
        );
    }

    #[test]
    fn registers_windows_and_persists_each_id_once() {
        let mut app = setup(serde_json::json!([]));
        app.registry.register("a".to_string(), "a".to_string());
        app.registry.register("a".to_string(), "a".to_string());
        app.registry.register("b".to_string(), "b".to_string());
        assert_eq!(stored_ids(&app), vec!["a".to_string(), "b".to_string()]);
    }

    #[test]
    fn forgets_a_deliberately_closed_window_while_others_remain_open() {
        let mut app = setup(serde_json::json!([]));
        app.registry.register("a".to_string(), "a".to_string());
        app.registry.register("b".to_string(), "b".to_string());
        app.registry.closed("a");
        assert_eq!(stored_ids(&app), vec!["b".to_string()]);
        assert_eq!(*app.cleaned.borrow(), vec!["a".to_string()]);
    }

    #[test]
    fn keeps_the_id_when_the_last_window_closes_so_relaunch_restores_it() {
        let mut app = setup(serde_json::json!([]));
        app.registry.register("a".to_string(), "a".to_string());
        app.registry.closed("a");
        assert_eq!(stored_ids(&app), vec!["a".to_string()]);
        assert!(app.cleaned.borrow().is_empty());

        let restarted_stored = Rc::clone(&app.stored);
        let restarted_write = Rc::clone(&app.stored);
        let mut restarted: WindowRegistry<String> = WindowRegistry::new(
            move || restarted_stored.borrow().clone(),
            move |ids| {
                *restarted_write.borrow_mut() = serde_json::Value::Array(
                    ids.into_iter().map(serde_json::Value::String).collect(),
                );
            },
            |_| {},
        );
        assert_eq!(restarted.persisted(), vec!["a".to_string()]);
    }

    #[test]
    fn keeps_every_id_when_windows_close_during_quit() {
        let mut app = setup(serde_json::json!([]));
        app.registry.register("a".to_string(), "a".to_string());
        app.registry.register("b".to_string(), "b".to_string());
        app.registry.set_quitting(true);
        app.registry.closed("a");
        app.registry.closed("b");
        assert_eq!(stored_ids(&app), vec!["a".to_string(), "b".to_string()]);
        assert!(app.cleaned.borrow().is_empty());
    }

    #[test]
    fn tracks_the_last_focused_window_and_falls_back_on_close() {
        let mut app = setup(serde_json::json!([]));
        app.registry.register("a".to_string(), "a".to_string());
        app.registry.register("b".to_string(), "b".to_string());
        app.registry.focused("a".to_string());
        assert_eq!(app.registry.last_focused(), Some(&"a".to_string()));
        app.registry.closed("a");
        assert_eq!(app.registry.last_focused(), Some(&"b".to_string()));
        app.registry.closed("b");
        assert_eq!(app.registry.last_focused(), None);
    }

    #[test]
    fn resumes_forgetting_closed_windows_after_the_quit_flag_resets() {
        let mut app = setup(serde_json::json!([]));
        app.registry.register("a".to_string(), "a".to_string());
        app.registry.register("b".to_string(), "b".to_string());
        app.registry.set_quitting(true);
        app.registry.set_quitting(false);
        app.registry.closed("a");
        assert_eq!(stored_ids(&app), vec!["b".to_string()]);
        assert_eq!(*app.cleaned.borrow(), vec!["a".to_string()]);
    }
}
