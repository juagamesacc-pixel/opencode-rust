// source: packages/tui/src/context/prompt.tsx (18 lines, v1.18.30)
// 1:1 port — the prompt component handle is an explicit trait (the real
// component implements it in the prompt batch); this module owns the slot.

#![allow(dead_code)]

use crate::prompt::history::PromptInfo;

/// Mirrors `PromptRef` — the live prompt component handle.
pub trait PromptRefHandle: Send {
    fn focused(&self) -> bool;
    fn current(&self) -> PromptInfo;
    fn set(&mut self, prompt: PromptInfo);
    fn reset(&mut self);
    fn blur(&mut self);
    fn focus(&mut self);
    fn submit(&mut self);
}

/// Mirrors the PromptRef context value (a settable slot).
#[derive(Default)]
pub struct PromptRefSlot {
    current: Option<Box<dyn PromptRefHandle>>,
}

impl PromptRefSlot {
    pub fn get(&self) -> Option<&dyn PromptRefHandle> {
        self.current.as_deref()
    }

    pub fn get_mut(&mut self) -> Option<&mut (dyn PromptRefHandle + 'static)> {
        self.current.as_deref_mut()
    }

    pub fn set(&mut self, handle: Option<Box<dyn PromptRefHandle>>) {
        self.current = handle;
    }
}
