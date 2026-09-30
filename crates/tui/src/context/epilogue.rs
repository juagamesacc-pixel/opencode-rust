// source: packages/tui/src/context/epilogue.tsx (6 lines, v1.18.30)
// 1:1 port — the context value is the setter itself.

#![allow(dead_code)]

use std::sync::{Arc, Mutex};

/// Epilogue setter seam (`{ set(value?: string) }`).
pub type EpilogueSetter = Arc<Mutex<Box<dyn FnMut(Option<String>) + Send>>>;

/// Mirrors the Epilogue context value (`set(value?: string)`).
#[derive(Clone, Default)]
pub struct Epilogue {
    setter: Option<EpilogueSetter>,
}

impl Epilogue {
    pub fn new(setter: impl FnMut(Option<String>) + Send + 'static) -> Self {
        Self {
            setter: Some(Arc::new(Mutex::new(Box::new(setter)))),
        }
    }

    pub fn set(&self, value: Option<String>) {
        if let Some(setter) = &self.setter {
            if let Ok(mut setter) = setter.lock() {
                setter(value);
            }
        }
    }
}
