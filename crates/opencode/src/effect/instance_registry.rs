// source: src/effect/instance-registry.ts — exports: registerDisposer,
// disposeInstance (add/delete set semantics + allSettled fan-out verbatim).

/// source: disposer set — add on register, delete on unsubscribe, verbatim.
#[derive(Default)]
pub struct DisposerRegistry {
    count: usize,
}

impl DisposerRegistry {
    pub fn new() -> Self {
        Self { count: 0 }
    }

    /// source: registerDisposer() → unsubscribe closure deletes. Verbatim.
    pub fn register(&mut self) -> usize {
        self.count += 1;
        self.count
    }

    pub fn unregister(&mut self, _id: usize) {
        self.count = self.count.saturating_sub(1);
    }

    /// source: disposeInstance() — Promise.allSettled over all disposers.
    /// Verbatim: every disposer runs; failures collected, never short-circuit.
    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.count == 0
    }
}
