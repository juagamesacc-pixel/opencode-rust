//! Rust port of `packages/app/src/utils/menu-dismiss-controller.ts` (opencode v1.18.30).
//!
//! Source 30 lines: `createMenuDismissController` focus-restore state machine.
//! `requestAnimationFrame`/DOM connection checks are modelled as explicit
//! `update`/`transition` fns.
//! Original file: `packages/app/src/utils/menu-dismiss-controller.ts`

#![allow(dead_code)]

/// Mirrors the `createMenuDismissController` state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MenuDismissController {
    pub restore_trigger: bool,
}

impl MenuDismissController {
    pub fn new() -> Self {
        Self {
            restore_trigger: true,
        }
    }

    /// Mirrors `allowTriggerRestore()`.
    pub fn update_allow_trigger_restore(&mut self) {
        self.restore_trigger = true;
    }

    /// Mirrors `preventTriggerRestore()`.
    pub fn update_prevent_trigger_restore(&mut self) {
        self.restore_trigger = false;
    }

    /// Mirrors `onCloseAutoFocus(event)` — returns `true` when the event
    /// default must be prevented.
    pub fn transition_close_auto_focus(&self) -> bool {
        !self.restore_trigger
    }

    /// Mirrors `afterClose(callback)` readiness: `true` when content is
    /// already disconnected and the callback may run.
    pub fn transition_after_close_ready(&self, content_connected: bool) -> bool {
        !content_connected
    }
}

impl Default for MenuDismissController {
    fn default() -> Self {
        Self::new()
    }
}
