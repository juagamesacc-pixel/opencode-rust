// source: packages/tui/src/util/selection.ts (79 lines, v1.18.30)
// 1:1 port — renderer/toast/clipboard are explicit traits; SolidJS
// reactivity is replaced by direct calls from the render loop.

#![allow(dead_code)]

/// Mirrors the toast variant union.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToastVariant {
    Info,
    Success,
    Warning,
    Error,
}

/// Mirrors the `Toast` structural type.
pub trait SelectionToast {
    fn show(&mut self, message: &str, variant: ToastVariant);
    fn error(&mut self, err: &str);
}

/// Mirrors `ClipboardService.write` (async in TS; the result is delivered
/// through the toast synchronously here once the write resolves).
pub trait SelectionClipboard {
    fn write_text(&mut self, text: String) -> Result<(), String>;
}

/// Mirrors the `Renderer` structural type: current selection plus the
/// focused target when it participates in the selection.
pub trait SelectionRenderer {
    /// `getSelection()?.getSelectedText()`
    fn selected_text(&self) -> Option<String>;
    /// Focused renderable is inside `selectedRenderables`.
    fn focus_in_selection(&self) -> bool;
    /// `focus.getClipboardText(text)` when the focus participates.
    fn focus_clipboard_text(&self, text: &str) -> Option<String>;
    /// `focus.hasSelection()` for the escape/retain check.
    fn focus_has_selection(&self) -> bool;
    fn clear_selection(&mut self);
}

/// Mirrors the key event fields read by `handleSelectionKey`.
#[derive(Debug, Clone, Default)]
pub struct SelectionKeyEvent {
    pub ctrl: bool,
    pub name: String,
    pub default_prevented: bool,
    pub propagation_stopped: bool,
}

impl SelectionKeyEvent {
    pub fn prevent_default(&mut self) {
        self.default_prevented = true;
    }
    pub fn stop_propagation(&mut self) {
        self.propagation_stopped = true;
    }
}

/// Mirrors `copy`.
pub fn copy(
    renderer: &mut dyn SelectionRenderer,
    toast: &mut dyn SelectionToast,
    clipboard: &mut dyn SelectionClipboard,
) -> bool {
    let text = match renderer.selected_text() {
        Some(text) => text,
        None => return false,
    };
    if text.is_empty() {
        return false;
    }
    let clipboard_text = match renderer.focus_clipboard_text(&text) {
        Some(mapped) if renderer.focus_in_selection() => mapped,
        _ => text,
    };
    match clipboard.write_text(clipboard_text) {
        Ok(()) => toast.show("Copied to clipboard", ToastVariant::Info),
        Err(err) => toast.error(&err),
    }
    renderer.clear_selection();
    true
}

/// Mirrors `handleSelectionKey`.
pub fn handle_selection_key(
    renderer: &mut dyn SelectionRenderer,
    toast: &mut dyn SelectionToast,
    event: &mut SelectionKeyEvent,
    clipboard: &mut dyn SelectionClipboard,
) {
    if renderer.selected_text().is_none() {
        return;
    }
    if event.ctrl && event.name == "c" {
        if !copy(renderer, toast, clipboard) {
            renderer.clear_selection();
            return;
        }
        event.prevent_default();
        event.stop_propagation();
        return;
    }
    if event.name == "escape" {
        renderer.clear_selection();
        event.prevent_default();
        event.stop_propagation();
        return;
    }
    if renderer.focus_in_selection() && renderer.focus_has_selection() {
        return;
    }
    renderer.clear_selection();
}
