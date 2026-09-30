// source: packages/tui/src/component/dialog-theme-list.tsx (50 lines, v1.18.30)
// 1:1 port — sorted theme options with live preview on move/filter and
// restore-on-cancel via `cleanup()` (mirrors `onCleanup`).

#![allow(dead_code)]

use crate::context::theme::ThemeContext;
use crate::ui::dialog_select::{SelectOption, SelectState};

/// Theme list dialog (live preview; cancel restores the initial theme).
pub struct ThemeListDialog {
    pub state: SelectState,
    initial: String,
    confirmed: bool,
}

impl ThemeListDialog {
    pub fn new(theme: &ThemeContext) -> Self {
        let mut names: Vec<String> = theme.theme_names();
        names.sort_by_key(|name| name.to_lowercase());
        let options = names
            .into_iter()
            .map(|name| SelectOption {
                title: name.clone(),
                value: serde_json::Value::String(name),
                ..SelectOption::default()
            })
            .collect();
        let mut state = SelectState::new("Themes", options);
        let initial = theme.selected().to_string();
        state.current = Some(serde_json::Value::String(initial.clone()));
        Self {
            state,
            initial,
            confirmed: false,
        }
    }

    /// Mirrors `onMove` — live-apply the hovered theme.
    pub fn on_move(&mut self, theme: &mut ThemeContext, value: &str) {
        theme.set(value);
    }

    /// Mirrors `onFilter` — empty query restores, otherwise previews the
    /// first match.
    pub fn on_filter(&mut self, theme: &mut ThemeContext, query: &str) {
        self.state.set_filter(query);
        if query.is_empty() {
            theme.set(&self.initial.clone());
            return;
        }
        if let Some(first) = self.state.filtered().first() {
            if let Some(value) = first.value.as_str() {
                theme.set(value);
            }
        }
    }

    /// Mirrors `onSelect` — apply, confirm, close (closed by the caller).
    pub fn on_select(&mut self, theme: &mut ThemeContext, value: &str) {
        theme.set(value);
        self.confirmed = true;
    }

    /// Mirrors `onCleanup` — restore the initial theme unless confirmed.
    pub fn cleanup(&self, theme: &mut ThemeContext) {
        if !self.confirmed {
            theme.set(&self.initial.clone());
        }
    }
}
