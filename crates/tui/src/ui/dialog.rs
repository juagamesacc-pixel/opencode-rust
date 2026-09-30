// source: packages/tui/src/ui/dialog.tsx (231 lines, v1.18.30)
// 1:1 port — the dialog stack is explicit; backdrop geometry (zIndex
// equivalent top layer, quarter-height offset, dim alpha 150, widths
// 60/88/116) and the escape/ctrl+c bindings are verbatim. Contents are
// trait objects so sibling dialog modules create no import cycles.

#![allow(dead_code)]

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::widgets::{Block, Widget};

use super::link::rgba;
use crate::context::clipboard::ClipboardService;
use crate::theme::{Rgba, Theme};
use crate::util::selection::ToastVariant;

/// Dialog widths verbatim (medium 60 / large 88 / xlarge 116).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DialogSize {
    #[default]
    Medium,
    Large,
    Xlarge,
}

impl DialogSize {
    pub fn width(&self) -> u16 {
        match self {
            DialogSize::Medium => 60,
            DialogSize::Large => 88,
            DialogSize::Xlarge => 116,
        }
    }
}

/// Backdrop dim verbatim (`RGBA.fromInts(0, 0, 0, 150)`).
pub const BACKDROP: Rgba = Rgba {
    r: 0.0,
    g: 0.0,
    b: 0.0,
    a: 150.0 / 255.0,
};

/// Control surface dialogs use (mirrors `DialogContext` mutations).
/// Implemented by `DialogStack`; components only see this trait.
pub trait DialogControl {
    fn clear(&mut self);
    fn stack_len(&self) -> usize;
    fn is_modal(&self) -> bool {
        self.stack_len() > 0
    }
}

/// One dialog's contents (implemented by every dialog module).
pub trait DialogContent: Send {
    fn render(&self, theme: &Theme, area: Rect, buf: &mut Buffer);
    /// Returns true when the key was consumed.
    fn handle_key(&mut self, key: &KeyEvent, control: &mut dyn DialogControl) -> bool;
    fn bindings(&self) -> Vec<DialogBinding> {
        Vec::new()
    }
}

/// Mirrors one `useBindings` entry (key/desc/group verbatim).
#[derive(Debug, Clone)]
pub struct DialogBinding {
    pub key: String,
    pub desc: String,
    pub group: String,
}

pub struct DialogEntry {
    pub content: Box<dyn DialogContent>,
    pub on_close: Option<Box<dyn FnMut() + Send>>,
}

/// Mirrors the dialog store (`stack` + `size` + pending refocus).
pub struct DialogStack {
    stack: Vec<DialogEntry>,
    size: DialogSize,
    refocus_pending: bool,
}

impl Default for DialogStack {
    fn default() -> Self {
        Self {
            stack: Vec::new(),
            size: DialogSize::Medium,
            refocus_pending: false,
        }
    }
}

impl DialogStack {
    /// Mirrors `clear` — every `onClose` runs, then stack + size reset.
    pub fn clear(&mut self) {
        for item in &mut self.stack {
            if let Some(on_close) = item.on_close.as_mut() {
                on_close();
            }
        }
        self.size = DialogSize::Medium;
        self.stack.clear();
        self.refocus_pending = true;
    }

    /// Mirrors `replace` — closes everything, pushes one entry.
    pub fn replace(
        &mut self,
        content: Box<dyn DialogContent>,
        on_close: Option<Box<dyn FnMut() + Send>>,
    ) {
        if self.stack.is_empty() {
            // Focus capture (mirrors the pre-blur); restore via poll_refocus.
            self.refocus_pending = true;
        }
        for item in &mut self.stack {
            if let Some(on_close) = item.on_close.as_mut() {
                on_close();
            }
        }
        self.size = DialogSize::Medium;
        self.stack = vec![DialogEntry { content, on_close }];
    }

    pub fn stack_len(&self) -> usize {
        self.stack.len()
    }

    pub fn size(&self) -> DialogSize {
        self.size
    }

    pub fn set_size(&mut self, size: DialogSize) {
        self.size = size;
    }

    pub fn top_mut(&mut self) -> Option<&mut DialogEntry> {
        self.stack.last_mut()
    }

    pub fn top(&self) -> Option<&DialogEntry> {
        self.stack.last()
    }

    /// Consume a pending refocus request (mirrors `refocus()`).
    pub fn poll_refocus(&mut self) -> bool {
        std::mem::replace(&mut self.refocus_pending, false)
    }

    /// Mirrors the escape / ctrl+c stack bindings. Returns true when the
    /// key closed the top dialog (caller clears an active selection first,
    /// mirroring the `cmd` bodies).
    pub fn handle_stack_key(&mut self, key: &KeyEvent, selection_active: bool) -> bool {
        if self.stack.is_empty() {
            return false;
        }
        let escape = key.code == KeyCode::Esc;
        let ctrl_c =
            key.code == KeyCode::Char('c') && key.modifiers.contains(KeyModifiers::CONTROL);
        if !escape && !ctrl_c {
            return false;
        }
        if selection_active && stack_bindings_enabled(selection_has_text(selection_active)) {
            return false;
        }
        let _ = selection_active;
        if let Some(mut entry) = self.stack.pop() {
            if let Some(on_close) = entry.on_close.as_mut() {
                on_close();
            }
        }
        self.refocus_pending = true;
        true
    }
}

fn selection_has_text(active: bool) -> bool {
    active
}

/// Mirrors the `useBindings` gate (`stack > 0 && !selectedText`).
pub fn stack_bindings_enabled(selected_text: bool) -> bool {
    !selected_text
}

/// Mirrors the `DialogProvider` copy-on-select (returns the copied flag).
pub fn copy_selection(
    selected_text: Option<&str>,
    clipboard: &ClipboardService,
    toast: &mut dyn FnMut(&str, ToastVariant),
    clear_selection: &mut dyn FnMut(),
) -> bool {
    let Some(text) = selected_text else {
        return false;
    };
    if text.is_empty() || clipboard.write.is_none() {
        return false;
    }
    clipboard.write(text);
    toast("Copied to clipboard", ToastVariant::Info);
    clear_selection();
    true
}

/// Backdrop click (mirrors `onMouseDown`/`onMouseUp` on the backdrop box):
/// a press that started on a selection never dismisses; a clean release
/// closes everything.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BackdropClick {
    Keep,
    CloseAll,
}

pub fn backdrop_release(press_started_on_selection: bool) -> BackdropClick {
    if press_started_on_selection {
        BackdropClick::Keep
    } else {
        BackdropClick::CloseAll
    }
}

/// Inner-box mouse-up (mirrors the inner `onMouseUp`): a selection release
/// bubbles to the provider's copy handler; otherwise the event is stopped.
pub fn inner_release(selected_text: Option<&str>) -> bool {
    selected_text.map(|text| !text.is_empty()).unwrap_or(false)
}

/// Centered dialog area (mirrors the backdrop padding + inner width).
pub fn dialog_area(area: Rect, size: DialogSize) -> Rect {
    let width = size.width().min(area.width.saturating_sub(2));
    let x = area.x + area.width.saturating_sub(width) / 2;
    let y = area.y + area.height / 4;
    Rect {
        x,
        y: area.y + area.height.saturating_sub(y - area.y).min(area.height),
        width,
        height: area.height.saturating_sub(y - area.y),
    }
}

/// Render the backdrop + top dialog content.
pub fn render_dialog(stack: &DialogStack, theme: &Theme, area: Rect, buf: &mut Buffer) {
    if stack.stack.is_empty() {
        return;
    }
    Block::default()
        .style(Style::default().bg(rgba(BACKDROP)))
        .render(area, buf);
    let inner = dialog_area(area, stack.size);
    // Panel background + top padding (mirrors `backgroundPanel` + paddingTop 1).
    Block::default()
        .style(Style::default().bg(rgba(theme.background_panel)))
        .render(inner, buf);
    let content_area = Rect {
        x: inner.x,
        y: inner.y + 1,
        width: inner.width,
        height: inner.height.saturating_sub(1),
    };
    if let Some(top) = stack.top() {
        top.content.render(theme, content_area, buf);
    }
}

impl DialogControl for DialogStack {
    fn clear(&mut self) {
        DialogStack::clear(self);
    }

    fn stack_len(&self) -> usize {
        self.stack.len()
    }
}

/// Minimal multiline text area (mirrors the OpenTUI textarea bits read
/// here: value, placeholder, cursor, line-end, suspend-when-busy).
#[derive(Debug, Clone)]
pub struct TextAreaState {
    lines: Vec<String>,
    cursor_row: usize,
    cursor_col: usize,
    pub placeholder: String,
}

impl TextAreaState {
    pub fn new(value: Option<&str>, placeholder: &str) -> Self {
        let mut state = Self {
            lines: vec![String::new()],
            cursor_row: 0,
            cursor_col: 0,
            placeholder: placeholder.to_string(),
        };
        if let Some(value) = value {
            state.lines = value.split('\n').map(str::to_string).collect();
            if state.lines.is_empty() {
                state.lines.push(String::new());
            }
            state.goto_line_end();
        }
        state
    }

    pub fn plain_text(&self) -> String {
        self.lines.join("\n")
    }

    pub fn goto_line_end(&mut self) {
        self.cursor_row = self.lines.len().saturating_sub(1);
        self.cursor_col = self.lines[self.cursor_row].chars().count();
    }

    pub fn insert(&mut self, ch: char) {
        if ch == '\n' {
            let tail: String = self.lines[self.cursor_row]
                .chars()
                .skip(self.cursor_col)
                .collect();
            self.lines[self.cursor_row] = self.lines[self.cursor_row]
                .chars()
                .take(self.cursor_col)
                .collect();
            self.lines.insert(self.cursor_row + 1, tail);
            self.cursor_row += 1;
            self.cursor_col = 0;
            return;
        }
        let mut line: Vec<char> = self.lines[self.cursor_row].chars().collect();
        line.insert(self.cursor_col.min(line.len()), ch);
        self.lines[self.cursor_row] = line.into_iter().collect();
        self.cursor_col += 1;
    }

    pub fn backspace(&mut self) {
        if self.cursor_col > 0 {
            let mut line: Vec<char> = self.lines[self.cursor_row].chars().collect();
            line.remove(self.cursor_col - 1);
            self.lines[self.cursor_row] = line.into_iter().collect();
            self.cursor_col -= 1;
        } else if self.cursor_row > 0 {
            let tail = self.lines.remove(self.cursor_row);
            self.cursor_row -= 1;
            self.cursor_col = self.lines[self.cursor_row].chars().count();
            self.lines[self.cursor_row].push_str(&tail);
        }
    }

    pub fn move_left(&mut self) {
        if self.cursor_col > 0 {
            self.cursor_col -= 1;
        } else if self.cursor_row > 0 {
            self.cursor_row -= 1;
            self.cursor_col = self.lines[self.cursor_row].chars().count();
        }
    }

    pub fn move_right(&mut self) {
        let len = self.lines[self.cursor_row].chars().count();
        if self.cursor_col < len {
            self.cursor_col += 1;
        } else if self.cursor_row + 1 < self.lines.len() {
            self.cursor_row += 1;
            self.cursor_col = 0;
        }
    }

    /// Viewport slice + cursor position for a fixed-height render.
    pub fn viewport(&self, height: usize) -> (Vec<String>, usize, usize) {
        let start = self.cursor_row.saturating_sub(height.saturating_sub(1));
        let lines = self
            .lines
            .iter()
            .skip(start)
            .take(height.max(1))
            .cloned()
            .collect();
        (lines, self.cursor_row - start, self.cursor_col)
    }
}
