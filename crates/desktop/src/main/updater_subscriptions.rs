//! Rust port of `src/main/updater-subscriptions.ts` (opencode v1.18.30).
//!
//! Original file: `packages/desktop/src/main/updater-subscriptions.ts`

use std::collections::HashMap;

pub struct UpdaterSubscriptions {
    subscriptions: HashMap<u64, Box<dyn FnMut()>>,
}

impl UpdaterSubscriptions {
    pub fn new() -> Self {
        Self {
            subscriptions: HashMap::new(),
        }
    }

    fn remove(&mut self, id: u64) {
        if let Some(mut unsubscribe) = self.subscriptions.remove(&id) {
            unsubscribe();
        }
    }

    pub fn set(&mut self, id: u64, unsubscribe: impl FnMut() + 'static) {
        self.remove(id);
        self.subscriptions.insert(id, Box::new(unsubscribe));
    }

    pub fn delete(&mut self, id: u64) {
        self.remove(id);
    }

    pub fn clear(&mut self) {
        for (_, mut unsubscribe) in self.subscriptions.drain() {
            unsubscribe();
        }
    }
}

impl Default for UpdaterSubscriptions {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    // Mirrors `src/main/updater-subscriptions.test.ts`
    // (`describe("updater subscriptions")`).
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn replaces_the_previous_renderer_subscription_on_reload() {
        let mut subscriptions = UpdaterSubscriptions::new();
        let disposed = Rc::new(RefCell::new(Vec::<&'static str>::new()));

        let first_disposed = Rc::clone(&disposed);
        subscriptions.set(1, move || first_disposed.borrow_mut().push("first"));
        let second_disposed = Rc::clone(&disposed);
        subscriptions.set(1, move || second_disposed.borrow_mut().push("second"));

        assert_eq!(*disposed.borrow(), vec!["first"]);
        subscriptions.delete(1);
        assert_eq!(*disposed.borrow(), vec!["first", "second"]);
    }
}
