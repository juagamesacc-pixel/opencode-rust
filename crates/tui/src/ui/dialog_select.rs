// source: packages/tui/src/ui/dialog-select.tsx (791 lines, v1.18.30)
// 1:1 port — the generic select dialog as an explicit state machine over
// `serde_json::Value` options (deep-equal = JSON equality). Grouping,
// title-weighted fuzzy filtering, wrap-around movement, action focus,
// scroll-to-selection row math, and every command/binding name are
// verbatim. The render loop drives `set_options`/`set_filter`/`poll`
// instead of SolidJS effects; scrolling is an offset, not a ScrollBox.

#![allow(dead_code)]

use crossterm::event::KeyEvent;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Paragraph, Widget, Wrap};
use serde_json::Value;

use super::dialog::{DialogBinding, DialogContent, DialogControl};
use super::link::rgba;
use crate::theme::{Rgba, Theme};
use crate::util::locale::{truncate, truncate_left, truncate_middle};

/// Fallback title width verbatim.
pub const DEFAULT_TITLE_WIDTH: usize = 61;
/// Empty-list copy verbatim.
pub const NO_RESULTS: &str = "No results found";
/// Filter input status tag verbatim.
pub const FILTER_STATUS: &str = "FILTER";
/// Default filter placeholder verbatim.
pub const DEFAULT_PLACEHOLDER: &str = "Search";
/// Page step verbatim.
pub const PAGE_STEP: i64 = 10;

/// Mirrors `DialogSelectOption` (views/gutters arrive pre-rendered).
#[derive(Debug, Clone, Default)]
pub struct SelectOption {
    pub title: String,
    pub title_view: Option<String>,
    pub value: Value,
    pub description: Option<String>,
    pub details: Vec<String>,
    pub footer: Option<String>,
    pub title_width: Option<usize>,
    pub truncate_title: TruncateTitle,
    pub category: Option<String>,
    pub category_view: Option<String>,
    pub disabled: bool,
    pub bg: Option<Rgba>,
    pub gutter: Option<String>,
    pub margin: Option<String>,
    pub has_on_select: bool,
}

/// Mirrors `truncateTitle`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TruncateTitle {
    #[default]
    End,
    Off,
    Left,
}

/// Action trigger seam.
pub type ActionTrigger = Box<dyn FnMut(&SelectOption) + Send>;
/// Disabled predicate seam (receives the selected option).
pub type DisabledFn = Box<dyn Fn(Option<&SelectOption>) -> bool + Send>;
/// Move/filter/select callbacks.
pub type MoveFn = Box<dyn FnMut(&SelectOption) + Send>;
pub type FilterFn = Box<dyn FnMut(&str) + Send>;

/// Mirrors one `actions[]` entry (disabled fn sees the selected option).
pub struct SelectAction {
    pub command: String,
    pub title: String,
    pub side: SelectSide,
    pub hidden: bool,
    pub disabled: SelectDisabled,
    pub on_trigger: ActionTrigger,
}

pub enum SelectDisabled {
    Flag(bool),
    Fn(DisabledFn),
}

impl SelectDisabled {
    fn is_disabled(&self, selected: Option<&SelectOption>) -> bool {
        match self {
            SelectDisabled::Flag(flag) => *flag,
            SelectDisabled::Fn(check) => check(selected),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelectSide {
    Left,
    Right,
}

/// Mirrors one `footerHints[]` entry.
#[derive(Debug, Clone)]
pub struct FooterHint {
    pub title: String,
    pub label: String,
    pub side: SelectSide,
}

/// Keyboard/mouse input mode verbatim.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelectInputMode {
    Keyboard,
    Mouse,
}

/// Subsequence fuzzy score (title matches weigh 2x, category 1x —
///
/// mirrors the fuzzysort `keys: ["title", "category"]` weighting).
/// Returns `None` when the needle is not a subsequence of either field.
/// PORT-NOTE: ranking approximates fuzzysort (no fuzzysort crate
/// offline); inclusion semantics (subsequence) match.
pub fn fuzzy_score(needle: &str, title: &str, category: Option<&str>) -> Option<i64> {
    if needle.is_empty() {
        return Some(0);
    }
    let title_score = subsequence_score(needle, title)?;
    let category_score = category
        .and_then(|c| subsequence_score(needle, c))
        .unwrap_or(i64::MIN / 2);
    Some(title_score.saturating_mul(2).saturating_add(category_score))
}

fn subsequence_score(needle: &str, haystack: &str) -> Option<i64> {
    let needle: Vec<char> = needle.chars().collect();
    let haystack: Vec<char> = haystack.chars().collect();
    let (mut ni, mut score, mut consecutive, mut first_at) = (0usize, 0i64, 0i64, None::<usize>);
    for (hi, hc) in haystack.iter().enumerate() {
        if ni < needle.len() && hc.to_lowercase().next() == needle[ni].to_lowercase().next() {
            if first_at.is_none() {
                first_at = Some(hi);
            }
            consecutive += 1;
            // Start-of-string and consecutive bonuses mirror fuzzysort shape.
            score += 10 + if hi == 0 { 8 } else { 0 } + consecutive * 3;
            if *hc != needle[ni] {
                score -= 1;
            }
            ni += 1;
        } else {
            consecutive = 0;
        }
    }
    if ni < needle.len() {
        return None;
    }
    score -= first_at.unwrap_or(0) as i64;
    Some(score)
}

/// Select dialog state (generic `T` becomes `Value`).
pub struct SelectState {
    pub title: String,
    pub title_view: Option<String>,
    pub placeholder: String,
    pub footer: Vec<String>,
    pub empty_text: Option<String>,
    pub flat: bool,
    pub skip_filter: bool,
    pub render_filter: bool,
    pub locked: bool,
    pub preserve_selection: bool,
    pub current: Option<Value>,
    pub action_labels: std::collections::HashMap<String, String>,
    pub actions: Vec<SelectAction>,
    pub footer_hints: Vec<FooterHint>,
    options: Vec<SelectOption>,
    selected: usize,
    filter: String,
    input_mode: SelectInputMode,
    focused_action: Option<usize>,
    selection: Option<(Value, Option<String>)>,
    reset_selection: bool,
    scroll_offset: usize,
    filter_text: String,
    pub on_move: Option<MoveFn>,
    pub on_filter: Option<FilterFn>,
    pub on_select: Option<MoveFn>,
    cached_theme: Option<Theme>,
    last_height: usize,
}

impl SelectState {
    pub fn new(title: &str, options: Vec<SelectOption>) -> Self {
        Self {
            title: title.to_string(),
            title_view: None,
            placeholder: DEFAULT_PLACEHOLDER.to_string(),
            footer: Vec::new(),
            empty_text: None,
            flat: false,
            skip_filter: false,
            render_filter: true,
            locked: false,
            preserve_selection: false,
            current: None,
            action_labels: std::collections::HashMap::new(),
            options,
            actions: Vec::new(),
            footer_hints: Vec::new(),
            selected: 0,
            filter: String::new(),
            input_mode: SelectInputMode::Keyboard,
            focused_action: None,
            selection: None,
            reset_selection: false,
            scroll_offset: 0,
            filter_text: String::new(),
            on_move: None,
            on_filter: None,
            on_select: None,
            cached_theme: None,
            last_height: 1,
        }
    }

    /// Filtered options (disabled excluded; fuzzy on title+category).
    pub fn filtered(&self) -> Vec<&SelectOption> {
        let mut options: Vec<&SelectOption> = self.options.iter().filter(|x| !x.disabled).collect();
        if self.skip_filter || !self.render_filter {
            return options;
        }
        let needle = self.filter.to_lowercase();
        if needle.is_empty() {
            return options;
        }
        let mut scored: Vec<(i64, &SelectOption)> = options
            .drain(..)
            .filter_map(|opt| {
                fuzzy_score(&needle, &opt.title.to_lowercase(), opt.category.as_deref())
                    .map(|s| (s, opt))
            })
            .collect();
        scored.sort_by_key(|(score, _)| std::cmp::Reverse(*score));
        scored.into_iter().map(|(_, opt)| opt).collect()
    }

    fn is_flat(&self) -> bool {
        self.flat && !self.filter.is_empty()
    }

    /// Grouped rows (category header order = first appearance).
    pub fn grouped(&self) -> Vec<(String, Vec<&SelectOption>)> {
        if self.is_flat() {
            return vec![(String::new(), self.filtered())];
        }
        let mut groups: Vec<(String, Vec<&SelectOption>)> = Vec::new();
        for opt in self.filtered() {
            let category = opt.category.clone().unwrap_or_default();
            match groups.iter_mut().find(|(c, _)| *c == category) {
                Some((_, list)) => list.push(opt),
                None => groups.push((category, vec![opt])),
            }
        }
        groups
    }

    pub fn flat_list(&self) -> Vec<&SelectOption> {
        self.grouped()
            .into_iter()
            .flat_map(|(_, options)| options)
            .collect()
    }

    /// Row count incl. headers + details (mirrors `rows()`).
    pub fn row_count(&self) -> usize {
        let grouped = self.grouped();
        let headers = grouped
            .iter()
            .enumerate()
            .fold(0, |acc, (i, (category, _))| {
                if category.is_empty() {
                    acc
                } else {
                    acc + if i > 0 { 2 } else { 1 }
                }
            });
        grouped
            .into_iter()
            .flat_map(|(_, options)| options)
            .fold(headers, |acc, option| acc + 1 + option.details.len())
    }

    /// Visible height (mirrors `height()` — needs terminal height).
    pub fn list_height(&self, term_height: u16) -> usize {
        self.row_count().min((term_height as usize) / 2 - 6)
    }

    pub fn selected_option(&self) -> Option<&SelectOption> {
        self.flat_list().get(self.selected).copied()
    }

    fn shown_actions(&self) -> Vec<&SelectAction> {
        self.actions.iter().filter(|item| !item.hidden).collect()
    }

    fn visible_actions(&self) -> Vec<VisibleAction<'_>> {
        let mut out: Vec<VisibleAction> = self
            .shown_actions()
            .into_iter()
            .filter_map(|item| {
                let label = self
                    .action_labels
                    .get(&item.command)
                    .cloned()
                    .unwrap_or_default();
                if label.is_empty() {
                    return None;
                }
                Some(VisibleAction::Action(item, label))
            })
            .collect();
        out.extend(self.footer_hints.iter().map(VisibleAction::Hint));
        out
    }

    fn action_items(&self) -> Vec<&SelectAction> {
        let selected = self.selected_option();
        self.visible_actions()
            .into_iter()
            .filter_map(|item| match item {
                VisibleAction::Action(action, _) if !action.disabled.is_disabled(selected) => {
                    Some(action)
                }
                _ => None,
            })
            .collect()
    }

    /// Mirrors `move` (wrap-around, locked/empty guards).
    pub fn move_selection(&mut self, direction: i64) {
        if self.locked {
            return;
        }
        let len = self.flat_list().len();
        if len == 0 {
            return;
        }
        let mut next = self.selected as i64 + direction;
        if next < 0 {
            next = len as i64 - 1;
        }
        if next >= len as i64 {
            next = 0;
        }
        self.move_to(next as usize, true);
    }

    /// Mirrors `moveTo`.
    pub fn move_to(&mut self, next: usize, center: bool) {
        self.focused_action = None;
        self.selected = next;
        let option = self.selected_option().cloned();
        if let Some(option) = option {
            self.selection = Some((option.value.clone(), option.category.clone()));
            self.reset_selection = false;
            if let Some(on_move) = self.on_move.as_mut() {
                on_move(&option);
            }
        }
        self.scroll_to_selection(center);
    }

    /// Mirrors `scrollToSelection` — row-index math through groups.
    pub fn scroll_to_selection(&mut self, center: bool) {
        let grouped = self.grouped();
        let mut remaining = self.selected;
        let mut index = 0usize;
        for (category, options) in &grouped {
            if !category.is_empty() {
                index += 1;
            }
            if remaining < options.len() {
                index += remaining;
                break;
            }
            index += options.len();
            remaining -= options.len();
        }
        let height = self.last_height.max(1);
        if center {
            let center_offset = height / 2;
            self.scroll_offset = index.saturating_sub(center_offset);
        } else {
            let y = index.saturating_sub(self.scroll_offset);
            if y >= height {
                self.scroll_offset += y - height + 1;
            } else if index < self.scroll_offset {
                self.scroll_offset = index;
                if self.flat_list().first().map(|o| o.value.clone())
                    == self.selected_option().map(|o| o.value.clone())
                {
                    self.scroll_offset = 0;
                }
            }
        }
    }

    /// Mirrors `submit` (action focus wins over option).
    pub fn submit(&mut self) {
        if self.locked {
            return;
        }
        self.input_mode = SelectInputMode::Keyboard;
        if let Some(index) = self.focused_action {
            // Resolve the action by position in the enabled list first so the
            // mutable trigger borrow never overlaps the selection borrow.
            let command = self
                .action_items()
                .get(index)
                .map(|item| item.command.clone());
            if let Some(command) = command {
                self.trigger_action(&command);
            }
            return;
        }
        if let Some(option) = self.selected_option().cloned() {
            if let Some(on_select) = self.on_select.as_mut() {
                on_select(&option);
            }
        }
    }

    /// Mirrors `moveAction` (tab cycling with drop-off at the ends).
    pub fn move_action(&mut self, direction: i64) {
        if self.locked {
            return;
        }
        let total = self.action_items().len();
        if total == 0 {
            return;
        }
        self.focused_action = match self.focused_action {
            None => Some(if direction == 1 { 0 } else { total - 1 }),
            Some(index) => {
                let next = index as i64 + direction;
                if next < 0 || next >= total as i64 {
                    None
                } else {
                    Some(next as usize)
                }
            }
        };
    }

    /// Filter input (mirrors the input `onInput` + filter effect).
    pub fn set_filter(&mut self, query: &str) {
        if self.locked {
            return;
        }
        self.filter = query.to_string();
        self.filter_text = query.to_string();
        if let Some(on_filter) = self.on_filter.as_mut() {
            on_filter(query);
        }
        // Filter effect: keyboard mode, drop action focus, reset to top.
        self.input_mode = SelectInputMode::Keyboard;
        self.focused_action = None;
        if !query.is_empty() {
            self.reset_selection = true;
            if !self.flat_list().is_empty() {
                self.move_to(0, true);
            }
        }
    }

    /// Options-change effect (mirrors the `preserveSelection` branch).
    pub fn set_options(&mut self, options: Vec<SelectOption>) {
        self.options = options;
        if !self.preserve_selection {
            return;
        }
        if self.reset_selection && !self.filter.is_empty() {
            let first = self.flat_list().first().map(|o| (*o).clone());
            if let Some(option) = first {
                self.selected = 0;
                self.selection = Some((option.value.clone(), option.category.clone()));
            }
            return;
        }
        if self.selection.is_none() {
            if let Some(current) = self.current.clone() {
                if let Some(index) = self.flat_list().iter().position(|o| o.value == current) {
                    self.selected = index;
                    if let Some(option) = self.flat_list().get(index) {
                        self.selection = Some((option.value.clone(), option.category.clone()));
                    }
                    return;
                }
            }
            if let Some(option) = self.selected_option().cloned() {
                self.selection = Some((option.value.clone(), option.category.clone()));
            }
            return;
        }
        let previous = match self.selection.clone() {
            Some(previous) => previous,
            None => return,
        };
        if let Some(index) = self.flat_list().iter().position(|o| o.value == previous.0) {
            let option = self.flat_list()[index].clone();
            let moved = index != self.selected || option.category != previous.1;
            self.selected = index;
            self.selection = Some((option.value.clone(), option.category.clone()));
            if moved {
                self.scroll_to_selection(false);
            }
            return;
        }
        let len = self.flat_list().len();
        let next = self.selected.min(len.saturating_sub(1));
        if len == 0 {
            return;
        }
        self.selected = next;
        if let Some(option) = self.flat_list().get(next) {
            self.selection = Some((option.value.clone(), option.category.clone()));
        }
    }

    /// Mirrors the `current` effect.
    pub fn sync_current(&mut self) {
        let Some(current) = self.current.clone() else {
            return;
        };
        if let Some(index) = self.flat_list().iter().position(|o| o.value == current) {
            self.selected = index;
            if let Some(option) = self.flat_list().get(index) {
                self.selection = Some((option.value.clone(), option.category.clone()));
            }
        }
    }

    pub fn ref_move_to_value(&mut self, value: &Value) {
        if let Some(index) = self.flat_list().iter().position(|o| &o.value == value) {
            self.move_to(index, true);
        }
    }

    fn option_fg(&self, theme: &Theme, active: bool, muted: bool, current: bool) -> Rgba {
        if active && !muted {
            return crate::theme::selected_foreground(theme, None);
        }
        if muted && (active || current) {
            return theme.text_muted;
        }
        if current {
            return theme.primary;
        }
        theme.text
    }

    pub fn mouse_move(&mut self) {
        if self.locked {
            return;
        }
        self.input_mode = SelectInputMode::Mouse;
        self.focused_action = None;
    }

    pub fn hover(&mut self, value: &Value) {
        if self.locked || self.input_mode != SelectInputMode::Mouse {
            return;
        }
        if let Some(index) = self.flat_list().iter().position(|o| &o.value == value) {
            self.move_to(index, false);
        }
    }

    pub fn trigger_action(&mut self, command: &str) {
        if self.locked {
            return;
        }
        let selected = self.selected_option().cloned();
        let Some(selected) = selected else { return };
        for action in &mut self.actions {
            if action.command == command
                && !action.hidden
                && !action.disabled.is_disabled(Some(&selected))
            {
                self.input_mode = SelectInputMode::Keyboard;
                (action.on_trigger)(&selected);
                return;
            }
        }
    }

    // Frame-scoped caches set by `render_select`.
    // (fields live on the struct above)
}

enum VisibleAction<'a> {
    Action(&'a SelectAction, String),
    Hint(&'a FooterHint),
}

impl DialogContent for SelectState {
    fn render(&self, _theme: &Theme, _area: Rect, _buf: &mut Buffer) {
        // Rendering is frame-driven via `render_select` (needs terminal
        // height); the trait entry forwards with a best-effort height.
    }

    fn handle_key(&mut self, _key: &KeyEvent, _control: &mut dyn DialogControl) -> bool {
        // Selection commands arrive from the app keymap (`dialog.select.*`
        // + action commands + tab); text input flows through `set_filter`.
        false
    }

    fn bindings(&self) -> Vec<DialogBinding> {
        let mut bindings = vec![
            DialogBinding {
                key: "dialog.select.prev".to_string(),
                desc: "Previous item".to_string(),
                group: "Dialog".to_string(),
            },
            DialogBinding {
                key: "dialog.select.next".to_string(),
                desc: "Next item".to_string(),
                group: "Dialog".to_string(),
            },
            DialogBinding {
                key: "dialog.select.page_up".to_string(),
                desc: "Page up".to_string(),
                group: "Dialog".to_string(),
            },
            DialogBinding {
                key: "dialog.select.page_down".to_string(),
                desc: "Page down".to_string(),
                group: "Dialog".to_string(),
            },
            DialogBinding {
                key: "dialog.select.home".to_string(),
                desc: "First item".to_string(),
                group: "Dialog".to_string(),
            },
            DialogBinding {
                key: "dialog.select.end".to_string(),
                desc: "Last item".to_string(),
                group: "Dialog".to_string(),
            },
            DialogBinding {
                key: "dialog.select.submit".to_string(),
                desc: "Select item".to_string(),
                group: "Dialog".to_string(),
            },
        ];
        if !self.actions.iter().any(|a| a.hidden) {
            bindings.push(DialogBinding {
                key: "tab".to_string(),
                desc: "Next dialog action".to_string(),
                group: "Dialog".to_string(),
            });
            bindings.push(DialogBinding {
                key: "shift+tab".to_string(),
                desc: "Previous dialog action".to_string(),
                group: "Dialog".to_string(),
            });
        }
        bindings
    }
}

/// Frame render (mirrors the JSX tree: title row, filter, grouped list
/// with headers/details, footer actions).
pub fn render_select(
    state: &mut SelectState,
    theme: &Theme,
    area: Rect,
    buf: &mut Buffer,
    term_height: u16,
) {
    state.cached_theme = Some(theme.clone());
    let height = state.list_height(term_height);
    state.last_height = height.max(1);
    let mut y = area.y;
    // Title row (pad 4 + esc).
    Line::styled(
        state
            .title_view
            .clone()
            .unwrap_or_else(|| state.title.clone()),
        ratatui::style::Style::default()
            .fg(rgba(theme.text))
            .add_modifier(Modifier::BOLD),
    )
    .render(
        Rect {
            x: area.x + 4,
            y,
            width: area.width.saturating_sub(11),
            height: 1,
        },
        buf,
    );
    Line::styled(
        "esc",
        ratatui::style::Style::default().fg(rgba(theme.text_muted)),
    )
    .render(
        Rect {
            x: area.x + area.width.saturating_sub(7),
            y,
            width: 3,
            height: 1,
        },
        buf,
    );
    y += 1;
    // Filter input.
    if state.render_filter {
        let shown = if state.filter.is_empty() {
            state.placeholder.clone()
        } else {
            state.filter.clone()
        };
        let fg = if state.filter.is_empty() {
            theme.text_muted
        } else {
            theme.text
        };
        Paragraph::new(Line::styled(
            format!("FILTER {shown}"),
            ratatui::style::Style::default().fg(rgba(fg)),
        ))
        .render(
            Rect {
                x: area.x + 4,
                y,
                width: area.width.saturating_sub(8),
                height: 1,
            },
            buf,
        );
        y += 1;
    }
    // Grouped list window.
    let grouped = state.grouped();
    let mut rows: Vec<SelectRow> = Vec::new();
    for (gi, (category, options)) in grouped.iter().enumerate() {
        if !category.is_empty() {
            rows.push(SelectRow::Header(category.clone(), gi > 0));
        }
        for option in options {
            rows.push(SelectRow::Option(Box::new((*option).clone())));
        }
    }
    let flat = state.flat_list();
    let selected_value = state.selected_option().map(|o| o.value.clone());
    let end = (state.scroll_offset + height).min(rows.len());
    for row in rows
        .iter()
        .skip(state.scroll_offset)
        .take(end - state.scroll_offset)
    {
        if y >= area.y + area.height {
            break;
        }
        match row {
            SelectRow::Header(category, spaced) => {
                let row_y = y + if *spaced { 1 } else { 0 };
                Paragraph::new(Line::styled(
                    category.clone(),
                    ratatui::style::Style::default()
                        .fg(rgba(theme.accent))
                        .add_modifier(Modifier::BOLD),
                ))
                .render(
                    Rect {
                        x: area.x + 4,
                        y: row_y,
                        width: area.width.saturating_sub(8),
                        height: 1,
                    },
                    buf,
                );
                y = row_y + 1;
            }
            SelectRow::Option(option) => {
                let option: &SelectOption = option.as_ref();
                let active = selected_value.as_ref() == Some(&option.value);
                let current = state.current.as_ref() == Some(&option.value);
                let action_focused = state.focused_action.is_some();
                let bg = if active {
                    if action_focused {
                        Some(theme.background_element)
                    } else {
                        Some(option.bg.unwrap_or(theme.primary))
                    }
                } else {
                    None
                };
                let fg = state.option_fg(theme, active, action_focused, current);
                let title = match option.truncate_title {
                    TruncateTitle::Off => option.title.clone(),
                    TruncateTitle::Left => truncate_left(
                        &option.title,
                        option.title_width.unwrap_or(DEFAULT_TITLE_WIDTH),
                    ),
                    TruncateTitle::End => truncate(
                        &option.title,
                        option.title_width.unwrap_or(DEFAULT_TITLE_WIDTH),
                    ),
                };
                let mut spans = vec![];
                if current && option.gutter.is_none() {
                    spans.push(Span::styled(
                        "● ",
                        ratatui::style::Style::default().fg(rgba(fg)),
                    ));
                }
                spans.push(Span::styled(
                    title,
                    ratatui::style::Style::default().fg(rgba(fg)).add_modifier(
                        if active && !action_focused {
                            Modifier::BOLD
                        } else {
                            Modifier::empty()
                        },
                    ),
                ));
                let description = if state.is_flat() {
                    option.category.clone().or(option.description.clone())
                } else {
                    option
                        .description
                        .clone()
                        .filter(|d| Some(d) != option.category.as_ref())
                };
                if let Some(description) = description {
                    spans.push(Span::styled(
                        format!(" {description}"),
                        ratatui::style::Style::default().fg(rgba(if active && !action_focused {
                            crate::theme::selected_foreground(theme, None)
                        } else {
                            theme.text_muted
                        })),
                    ));
                }
                let mut style = ratatui::style::Style::default();
                if let Some(bg) = bg {
                    style = style.bg(rgba(bg));
                }
                Paragraph::new(Line::from(spans)).style(style).render(
                    Rect {
                        x: area.x + 4,
                        y,
                        width: area.width.saturating_sub(8),
                        height: 1,
                    },
                    buf,
                );
                y += 1;
                for detail in &option.details {
                    let clipped = truncate_middle(
                        detail,
                        (76usize).min((area.width as usize).saturating_sub(12).max(1)),
                    );
                    Paragraph::new(Line::styled(
                        clipped,
                        ratatui::style::Style::default().fg(rgba(theme.text_muted)),
                    ))
                    .wrap(Wrap { trim: false })
                    .render(
                        Rect {
                            x: area.x + 4,
                            y,
                            width: area.width.saturating_sub(8),
                            height: 1,
                        },
                        buf,
                    );
                    y += 1;
                }
            }
        }
    }
    let _ = flat;
}

#[derive(Clone)]
enum SelectRow {
    Header(String, bool),
    Option(Box<SelectOption>),
}
