// source: packages/tui/src/component/command-palette.tsx (79 lines, v1.18.30)
// 1:1 port — palette entries become select options (footer = formatted
// bindings); the unfiltered list prefixes `suggested:*` entries under a
// `Suggested` category; selection clears the dialog and dispatches.

#![allow(dead_code)]

use crate::keymap::{is_visible_palette_command, PaletteCommandRef, COMMAND_PALETTE_COMMAND};
use crate::ui::dialog_select::{SelectOption, SelectState};

/// One palette command entry (mirrors the keymap entry fields read here).
#[derive(Debug, Clone)]
pub struct PaletteEntry {
    pub name: String,
    pub title: Option<String>,
    pub desc: Option<String>,
    pub category: Option<String>,
    pub suggested: bool,
    pub bindings_label: String,
}

/// Mirrors `isSuggestedPaletteCommand` (boolean form; function form is
/// resolved by the keymap before reaching this module).
pub fn is_suggested(suggested: bool) -> bool {
    suggested
}

/// Build the palette option list (mirrors the `options` memo + the
/// suggested-prefix `list()` branch).
pub fn palette_options(entries: &[PaletteEntry], filter_active: bool) -> Vec<SelectOption> {
    let visible: Vec<&PaletteEntry> = entries
        .iter()
        .filter(|entry| {
            is_visible_palette_command(&PaletteCommandRef {
                name: entry.name.clone(),
                hidden: false,
            }) && entry.name != COMMAND_PALETTE_COMMAND
        })
        .collect();
    let mut options: Vec<SelectOption> = visible
        .iter()
        .map(|entry| SelectOption {
            title: entry.title.clone().unwrap_or_else(|| entry.name.clone()),
            description: entry.desc.clone(),
            category: entry.category.clone(),
            footer: Some(entry.bindings_label.clone()),
            value: serde_json::Value::String(entry.name.clone()),
            ..SelectOption::default()
        })
        .collect();
    if filter_active {
        return options;
    }
    let mut suggested: Vec<SelectOption> = visible
        .iter()
        .filter(|entry| entry.suggested)
        .map(|entry| SelectOption {
            title: entry.title.clone().unwrap_or_else(|| entry.name.clone()),
            description: entry.desc.clone(),
            category: Some("Suggested".to_string()),
            footer: Some(entry.bindings_label.clone()),
            value: serde_json::Value::String(format!("suggested:{}", entry.name)),
            ..SelectOption::default()
        })
        .collect();
    suggested.append(&mut options);
    suggested
}

/// Build the palette select state (title `Commands`, verbatim).
pub fn palette_state(entries: &[PaletteEntry]) -> SelectState {
    SelectState::new("Commands", palette_options(entries, false))
}

/// Strip the `suggested:` prefix before dispatch (mirrors the value
/// encoding in the unfiltered list).
pub fn dispatch_name(value: &serde_json::Value) -> Option<String> {
    let name = value.as_str()?;
    Some(name.strip_prefix("suggested:").unwrap_or(name).to_string())
}
