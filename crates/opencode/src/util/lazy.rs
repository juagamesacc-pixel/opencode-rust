// source: src/util/lazy.ts — exports: lazy (verbatim: loaded flag, reset, loaded()).
use std::sync::Mutex;

/// source: lazy(fn) — run-once cell with reset + loaded introspection, verbatim.
pub struct Lazy<T: Clone> {
    inner: Mutex<LazyInner<T>>,
    f: Mutex<Option<Box<dyn Fn() -> T + Send>>>,
}

struct LazyInner<T> {
    value: Option<T>,
    loaded: bool,
}

impl<T: Clone> Lazy<T> {
    pub fn new(f: impl Fn() -> T + Send + 'static) -> Self {
        Self {
            inner: Mutex::new(LazyInner {
                value: None,
                loaded: false,
            }),
            f: Mutex::new(Some(Box::new(f))),
        }
    }

    /// source: result() — cached after first call, verbatim.
    pub fn get(&self) -> T {
        let mut inner = self.inner.lock().unwrap();
        if inner.loaded {
            return inner.value.clone().unwrap();
        }
        let f = self.f.lock().unwrap().take().expect("lazy fn consumed");
        let v = f();
        inner.value = Some(v.clone());
        inner.loaded = true;
        v
    }

    /// source: result.reset() — verbatim.
    pub fn reset(&self) {
        let mut inner = self.inner.lock().unwrap();
        inner.loaded = false;
        inner.value = None;
    }

    /// source: result.loaded() — verbatim.
    pub fn loaded(&self) -> bool {
        self.inner.lock().unwrap().loaded
    }
}

/// source: lazy() — verbatim constructor.
pub fn lazy<T: Clone>(f: impl Fn() -> T + Send + 'static) -> Lazy<T> {
    Lazy::new(f)
}
