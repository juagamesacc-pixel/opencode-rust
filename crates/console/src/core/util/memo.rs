// source: core/src/util/memo.ts
//! 1:1 port of the lazy `memo` wrapper. Source pin: v1.18.30 @3104c14.

use std::cell::RefCell;
use std::rc::Rc;

struct MemoState<T> {
    value: Option<T>,
    loaded: bool,
}

/// `memo(fn, cleanup?)` — `Memo::call()` runs `func` at most once, `reset()` runs
/// the optional cleanup on the cached value and re-arms the thunk (as in JS,
/// where the closure re-invokes `fn` after every reset).
pub struct Memo<T> {
    state: RefCell<MemoState<T>>,
    func: Rc<dyn Fn() -> T>,
    cleanup: Option<Rc<dyn Fn(&T)>>,
}

impl<T: 'static> Memo<T> {
    fn build(func: Rc<dyn Fn() -> T>, cleanup: Option<Rc<dyn Fn(&T)>>) -> Self {
        Self {
            state: RefCell::new(MemoState {
                value: None,
                loaded: false,
            }),
            func,
            cleanup,
        }
    }

    /// `result()`.
    pub fn call(&self) -> T
    where
        T: Clone,
    {
        if self.state.borrow().loaded {
            return self
                .state
                .borrow()
                .value
                .clone()
                .expect("memo loaded without a value");
        }
        let value = (self.func)();
        let mut state = self.state.borrow_mut();
        state.loaded = true;
        state.value = Some(value.clone());
        value
    }

    /// Whether the thunk has already produced a value.
    pub fn is_loaded(&self) -> bool {
        self.state.borrow().loaded
    }

    /// `result.reset()`.
    pub fn reset(&self) {
        if let Some(cleanup) = self.cleanup.as_ref() {
            if let Some(value) = self.state.borrow().value.clone() {
                cleanup(&value);
            }
        }
        let mut state = self.state.borrow_mut();
        state.loaded = false;
        state.value = None;
    }
}

/// `memo(fn)`.
pub fn memo<T, F: Fn() -> T + 'static>(func: F) -> Memo<T> {
    Memo::build(Rc::new(func), None)
}

/// `memo(fn, cleanup)`.
pub fn memo_with_cleanup<T, F: Fn() -> T + 'static, C: Fn(&T) + 'static>(
    func: F,
    cleanup: C,
) -> Memo<T> {
    Memo::build(Rc::new(func), Some(Rc::new(cleanup)))
}
