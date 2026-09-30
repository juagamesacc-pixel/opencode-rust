// source: src/util/local-context.ts — exports: NotFound, create, LocalContext
// Minimal equivalent: thread-local context stack (AsyncLocalStorage has no
// direct equivalent without async runtime); use/provide + `No context found
// for ${name}` verbatim.

use std::cell::RefCell;

/// source: NotFound — `No context found for ${name}`, verbatim.
#[derive(Debug, Clone)]
pub struct NotFound {
    pub name: String,
}

impl std::fmt::Display for NotFound {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "No context found for {}", self.name)
    }
}

impl std::error::Error for NotFound {}

/// source: create(name) — use()/provide() pair, verbatim semantics.
pub struct LocalContext<T: Clone + 'static> {
    name: String,
    stack: RefCell<Vec<T>>,
}

impl<T: Clone + 'static> LocalContext<T> {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            stack: RefCell::new(Vec::new()),
        }
    }

    /// source: use() — throws NotFound when empty, verbatim.
    pub fn use_(&self) -> Result<T, NotFound> {
        self.stack.borrow().last().cloned().ok_or_else(|| NotFound {
            name: self.name.clone(),
        })
    }

    /// source: provide(value, fn) — verbatim scoping.
    pub fn provide<R>(&self, value: T, f: impl FnOnce() -> R) -> R {
        self.stack.borrow_mut().push(value);
        let out = f();
        self.stack.borrow_mut().pop();
        out
    }
}

/// source: create() — verbatim constructor.
pub fn create<T: Clone + 'static>(name: impl Into<String>) -> LocalContext<T> {
    LocalContext::new(name)
}
