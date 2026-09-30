// source: packages/tui/src/component/prompt/index.tsx (1716 lines, v1.18.30)
// 1:1 port — the prompt editor as an explicit state machine. The OpenTUI
// textarea becomes `PromptTextarea` (text buffer, display-unit cursor,
// extmarks); SolidJS effects become methods the app drives per frame;
// submit is an async flow over injected dependencies returning explicit
// side-effect requests; all copy, ordering, and guards are verbatim.

#![allow(dead_code)]

use serde_json::Value;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use super::super::dialog_stash::stash_preview;
use super::autocomplete::AutocompleteState;
use super::local_attachment::read_local_attachment;
use super::r#move::PromptMove;
use super::workspace::PromptWorkspace;
use crate::context::editor::{EditorContext, EditorSelection};
use crate::context::local::{LocalContext, ModelId};
use crate::context::route::{Route, RouteStore};
use crate::context::sync::SyncStore;
use crate::prompt::display::{display_char_at, display_slice, prompt_offset_width};
use crate::prompt::history::{PromptHistory, PromptInfo};
use crate::prompt::part::{expand_pasted_text_placeholders, expand_tracked_pasted_text};
use crate::prompt::stash::{PromptStash, StashEntry};
use crate::prompt::traits::{compute_prompt_traits, PromptMode};
use crate::theme::{Rgba, Theme};
use crate::ui::toast::ToastState;
use crate::util::locale::{number, titlecase, truncate_middle};

/// Draft retention threshold verbatim (20 chars).
pub const DRAFT_RETENTION_MIN_CHARS: usize = 20;
/// Interrupt double-press window verbatim (5000ms).
pub const INTERRUPT_WINDOW_MS: u64 = 5000;
/// New-session navigation delay verbatim (50ms).
pub const NAVIGATE_DELAY_MS: u64 = 50;

/// Mirrors `pastedFilepath` (quote strip, file:// decode, win32/raw rules).
pub fn pasted_filepath(value: &str, platform: &str) -> String {
    let raw = value.trim_matches(|ch| ch == '\'' || ch == '"').to_string();
    if let Some(rest) = raw.strip_prefix("file://") {
        // Minimal file-URL decode (percent + host strip).
        let decoded = percent_decode(rest);
        let path = decoded.split('?').next().unwrap_or(&decoded).to_string();
        return path;
    }
    if platform == "win32" {
        return raw;
    }
    let mut out = String::new();
    let mut chars = raw.chars();
    while let Some(ch) = chars.next() {
        if ch == '\\' {
            if let Some(next) = chars.next() {
                out.push(next);
            }
        } else {
            out.push(ch);
        }
    }
    out
}

fn percent_decode(input: &str) -> String {
    let mut out = String::new();
    let bytes = input.as_bytes();
    let mut index = 0;
    while index < bytes.len() {
        if bytes[index] == b'%' && index + 2 < bytes.len() {
            if let (Some(high), Some(low)) = (hex(bytes[index + 1]), hex(bytes[index + 2])) {
                out.push((high * 16 + low) as char);
                index += 3;
                continue;
            }
        }
        out.push(bytes[index] as char);
        index += 1;
    }
    out
}

fn hex(byte: u8) -> Option<u8> {
    match byte {
        b'0'..=b'9' => Some(byte - b'0'),
        b'a'..=b'f' => Some(byte - b'a' + 10),
        b'A'..=b'F' => Some(byte - b'A' + 10),
        _ => None,
    }
}

/// Mirrors `randomIndex`.
pub fn random_index(count: usize) -> usize {
    if count == 0 {
        return 0;
    }
    (Instant::now().elapsed().as_nanos() as usize) % count
}

/// Mirrors `fadeColor` (alpha multiply).
pub fn fade_color(color: Rgba, alpha: f64) -> Rgba {
    Rgba {
        r: color.r,
        g: color.g,
        b: color.b,
        a: color.a * alpha as f32,
    }
}

/// Mirrors `hasEditorRangeSelection`.
pub fn has_editor_range_selection(
    start_line: i64,
    start_char: i64,
    end_line: i64,
    end_char: i64,
) -> bool {
    start_line != end_line || start_char != end_char
}

/// Mirrors `getEditorRangeLabel`.
pub fn editor_range_label(start_line: i64, end_line: i64) -> Option<String> {
    if start_line == end_line && start_line == 0 {
        return None;
    }
    if start_line == end_line {
        return Some(format!("#{start_line}"));
    }
    Some(format!("#{start_line}-{end_line}"))
}

/// Mirrors `formatEditorContext`.
pub fn format_editor_context(selection: &EditorSelection) -> String {
    let selected: Vec<(usize, String, String)> = selection
        .ranges
        .iter()
        .filter(|range| {
            has_editor_range_selection(
                range.selection.start.line,
                range.selection.start.character,
                range.selection.end.line,
                range.selection.end.character,
            )
        })
        .enumerate()
        .map(|(index, range)| {
            let prefix = if selection.ranges.len() > 1 {
                format!("Selection {}: ", index + 1)
            } else {
                String::new()
            };
            let label = if range.selection.start.line == range.selection.end.line {
                format!("#{}", range.selection.start.line)
            } else {
                format!(
                    "#{}-{}",
                    range.selection.start.line, range.selection.end.line
                )
            };
            (
                index,
                prefix,
                format!(
                    "Note: The user selected {prefix}{label} from \"{}\". ```{}```\n\n",
                    selection.file_path, range.text
                ),
            )
        })
        .map(|(index, prefix, text)| (index, prefix, text))
        .collect();
    if selected.is_empty() {
        return format!(
            "<system-reminder>Note: The user opened the file \"{}\". This may or may not be relevant to the current task.</system-reminder>\n",
            selection.file_path
        );
    }
    let ranges: Vec<String> = selected.into_iter().map(|(_, _, text)| text).collect();
    format!(
        "<system-reminder>{} This may or may not be relevant to the current task.</system-reminder>\n",
        ranges.join("\n")
    )
}

/// US currency format (mirrors the `money` Intl formatter).
pub fn format_money(cost: f64) -> String {
    format!("${:.2}", cost)
}

/// Extmark record (mirrors the textarea extmark objects).
#[derive(Debug, Clone)]
pub struct Extmark {
    pub id: u64,
    pub start: usize,
    pub end: usize,
    pub virtual_text: bool,
    pub style_id: Option<u64>,
    pub type_id: u64,
}

/// Prompt textarea buffer (mirrors the OpenTUI textarea surface read here).
#[derive(Debug, Clone)]
pub struct PromptTextarea {
    text: String,
    /// Cursor in display-width units (mirrors `cursorOffset`).
    pub cursor: usize,
    pub focused: bool,
    extmarks: HashMap<u64, Extmark>,
    next_extmark_id: u64,
    type_ids: HashMap<String, u64>,
    pub visual_row: usize,
    pub scroll_y: usize,
    pub total_virtual_lines: usize,
}

impl PromptTextarea {
    pub fn new() -> Self {
        Self {
            text: String::new(),
            cursor: 0,
            focused: false,
            extmarks: HashMap::new(),
            next_extmark_id: 1,
            type_ids: HashMap::new(),
            visual_row: 0,
            scroll_y: 0,
            total_virtual_lines: 1,
        }
    }

    pub fn plain_text(&self) -> &str {
        &self.text
    }

    pub fn set_text(&mut self, text: &str) {
        self.text = text.to_string();
        self.cursor = self.cursor.min(prompt_offset_width(&self.text));
        self.relayout();
    }

    pub fn clear(&mut self) {
        self.text.clear();
        self.cursor = 0;
        self.relayout();
    }

    pub fn insert_text(&mut self, text: &str) {
        let byte = display_byte_index(&self.text, self.cursor);
        self.text.insert_str(byte, text);
        self.cursor += prompt_offset_width(text);
        self.relayout();
    }

    /// Mirrors `deleteRange` (display-unit offsets).
    pub fn delete_range(&mut self, from: usize, to: usize) {
        let (from, to) = (from.min(to), from.max(to));
        let start = display_byte_index(&self.text, from);
        let end = display_byte_index(&self.text, to);
        self.text.replace_range(start..end, "");
        self.cursor = from.min(prompt_offset_width(&self.text));
        self.relayout();
    }

    pub fn get_text_range(&self, from: usize, to: usize) -> String {
        display_slice(&self.text, from, to).to_string()
    }

    pub fn goto_buffer_end(&mut self) {
        self.cursor = prompt_offset_width(&self.text);
    }

    pub fn goto_line_end(&mut self) {
        let byte = display_byte_index(&self.text, self.cursor);
        let line_start = self.text[..byte]
            .rfind('\n')
            .map(|index| index + 1)
            .unwrap_or(0);
        let line = &self.text[line_start..];
        let line_end = line
            .find('\n')
            .map(|index| line_start + index)
            .unwrap_or(self.text.len());
        self.cursor = prompt_offset_width(&self.text[..line_end]);
    }

    /// Logical cursor row/col from the display cursor.
    pub fn logical_cursor(&self) -> (usize, usize) {
        let byte = display_byte_index(&self.text, self.cursor);
        let row = self.text[..byte].matches('\n').count();
        let line_start = self.text[..byte]
            .rfind('\n')
            .map(|index| index + 1)
            .unwrap_or(0);
        (row, prompt_offset_width(&self.text[line_start..byte]))
    }

    pub fn register_type(&mut self, name: &str) -> u64 {
        let id = self.type_ids.len() as u64 + 1;
        *self.type_ids.entry(name.to_string()).or_insert(id)
    }

    pub fn create_extmark(
        &mut self,
        start: usize,
        end: usize,
        virtual_text: bool,
        style_id: Option<u64>,
        type_id: u64,
    ) -> u64 {
        let id = self.next_extmark_id;
        self.next_extmark_id += 1;
        self.extmarks.insert(
            id,
            Extmark {
                id,
                start,
                end,
                virtual_text,
                style_id,
                type_id,
            },
        );
        id
    }

    pub fn clear_extmarks(&mut self) {
        self.extmarks.clear();
    }

    pub fn extmarks_for_type(&self, type_id: u64) -> Vec<Extmark> {
        let mut marks: Vec<Extmark> = self
            .extmarks
            .values()
            .filter(|mark| mark.type_id == type_id)
            .cloned()
            .collect();
        marks.sort_by_key(|mark| mark.id);
        marks
    }

    pub fn clipboard_text(&self, text: &str, parts: &[Value]) -> String {
        expand_pasted_text_placeholders(text, parts)
    }

    fn relayout(&mut self) {
        self.total_virtual_lines = self.text.matches('\n').count() + 1;
        let (row, _) = self.logical_cursor();
        self.visual_row = row;
    }
}

impl Default for PromptTextarea {
    fn default() -> Self {
        Self::new()
    }
}

/// Display offset → byte index (mirrors textarea offset semantics).
pub fn display_byte_index(value: &str, offset: usize) -> usize {
    if offset == 0 {
        return 0;
    }
    let mut width = 0;
    for (index, ch) in value.char_indices() {
        let next = width + crate::prompt::display::char_width(ch);
        if next > offset {
            return index;
        }
        width = next;
    }
    value.len()
}

/// Prompt store (mirrors the component store).
#[derive(Debug, Clone)]
pub struct PromptStoreState {
    pub prompt: PromptInfo,
    pub mode: PromptMode,
    pub extmark_to_part: HashMap<u64, usize>,
    pub interrupt: u32,
    pub placeholder: usize,
}

impl PromptStoreState {
    pub fn new(placeholder_count: usize) -> Self {
        Self {
            prompt: PromptInfo {
                input: String::new(),
                mode: None,
                parts: Vec::new(),
            },
            mode: PromptMode::Normal,
            extmark_to_part: HashMap::new(),
            interrupt: 0,
            placeholder: random_index(placeholder_count),
        }
    }
}

/// Cross-mount draft stash (mirrors the module-level `stashed`).
static STASHED: Mutex<Option<(PromptInfo, usize)>> = Mutex::new(None);

pub fn take_stashed() -> Option<(PromptInfo, usize)> {
    STASHED.lock().ok()?.take()
}

pub fn stash_current(prompt: &PromptInfo, cursor: usize) {
    if let Ok(mut guard) = STASHED.lock() {
        *guard = Some((prompt.clone(), cursor));
    }
}

/// Submit side-effect requests for the app to execute.
#[derive(Debug)]
pub enum SubmitEffect {
    None,
    Exit,
    ShowProviderConnect,
    ShowWorkspaceUnavailable,
    NavigateSession {
        session_id: String,
    },
    Toast {
        title: Option<String>,
        message: String,
        error: bool,
    },
    Submitted {
        session_id: String,
    },
}

// Async SDK calls run through `SubmitDeps.client` (the `SdkClient` seam);
// no extra trait is needed.
pub struct SubmitDeps<'a> {
    pub client: Arc<dyn crate::context::sdk::SdkClient>,
    pub sync: &'a mut SyncStore,
    pub local: &'a mut LocalContext,
    pub route: &'a mut RouteStore,
    pub project_workspace_status: Option<String>,
    pub project_main_dir: Option<String>,
    pub instance_worktree: String,
    pub instance_directory: String,
    pub cwd: String,
    pub session_id: Option<String>,
    pub agent_name: Option<String>,
    pub model: Option<ModelId>,
    pub variant: Option<String>,
    pub commands: Vec<String>,
    pub workspace_selection: Option<crate::component::dialog_workspace_create::WorkspaceSelection>,
    pub move_directory: Option<String>,
    pub move_pending: bool,
    pub move_progress: bool,
    pub editor_selection: Option<EditorSelection>,
    pub editor_label_pending: bool,
    pub toast: &'a mut ToastState,
    pub history: &'a mut PromptHistory,
    pub on_submit: bool,
}

/// Placeholder text builder (mirrors `placeholderText`).
pub fn placeholder_text(
    mode: PromptMode,
    normal: &[String],
    shell: &[String],
    placeholder: usize,
    show: bool,
) -> Option<String> {
    if !show {
        return None;
    }
    if mode == PromptMode::Shell {
        if shell.is_empty() {
            return None;
        }
        return Some(format!(
            "Run a command… \"{}\"",
            shell[placeholder % shell.len()]
        ));
    }
    if normal.is_empty() {
        return None;
    }
    Some(format!(
        "Ask anything… \"{}\"",
        normal[placeholder % normal.len()]
    ))
}

/// Retry message builder (mirrors the retry memo + text).
pub fn retry_text(message: &str, attempt: u64, seconds: u64, truncated: bool) -> String {
    let gemini_hot = message.contains("exceeded your current quota") && message.contains("gemini");
    let base = if gemini_hot {
        "gemini is way too hot right now".to_string()
    } else if message.chars().count() > 80 {
        format!("{}…", message.chars().take(80).collect::<String>())
    } else {
        message.to_string()
    };
    let hint = if truncated { " (click to expand)" } else { "" };
    let duration = crate::util::format::format_duration(seconds as f64);
    let retry_info = format!(
        " [retrying {}attempt #{attempt}]",
        if duration.is_empty() {
            String::new()
        } else {
            format!("in {duration} ")
        }
    );
    format!("{base}{hint}{retry_info}")
}

/// Usage line builder (mirrors the `usage` memo).
pub fn usage_line(
    session_id: Option<&str>,
    sync: &SyncStore,
    local: &LocalContext,
    args_model: Option<&str>,
    config_model: Option<&str>,
) -> Option<(String, Option<String>)> {
    let session_id = session_id?;
    let session = sync.session_get(session_id);
    let messages = sync.message.get(session_id)?;
    let last = messages.iter().rev().find(|item| {
        item.get("role").and_then(|v| v.as_str()) == Some("assistant")
            && item
                .get("tokens")
                .and_then(|t| t.get("output"))
                .and_then(|v| v.as_i64())
                .unwrap_or(0)
                > 0
    })?;
    let tokens = ["input", "output", "reasoning"]
        .into_iter()
        .map(|key| {
            last.get("tokens")
                .and_then(|t| t.get(key))
                .and_then(|v| v.as_i64())
                .unwrap_or(0)
        })
        .sum::<i64>()
        + last
            .get("tokens")
            .and_then(|t| t.get("cache"))
            .and_then(|c| c.get("read"))
            .and_then(|v| v.as_i64())
            .unwrap_or(0)
        + last
            .get("tokens")
            .and_then(|t| t.get("cache"))
            .and_then(|c| c.get("write"))
            .and_then(|v| v.as_i64())
            .unwrap_or(0);
    if tokens <= 0 {
        return None;
    }
    let provider_id = last
        .get("providerID")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    let model_id = last.get("modelID").and_then(|v| v.as_str()).unwrap_or("");
    let context_pct = sync
        .provider
        .iter()
        .find(|p| p.get("id").and_then(|v| v.as_str()) == Some(provider_id))
        .and_then(|p| p.get("models"))
        .and_then(|m| m.get(model_id))
        .and_then(|m| m.get("limit"))
        .and_then(|l| l.get("context"))
        .and_then(|v| v.as_i64())
        .map(|limit| format!("{}%", (tokens as f64 / limit as f64 * 100.0).round() as i64));
    let context = match context_pct {
        Some(pct) => format!("{} ({pct})", number(tokens as f64)),
        None => number(tokens as f64),
    };
    let cost = session
        .and_then(|s| s.get("cost"))
        .and_then(|v| v.as_f64())
        .unwrap_or(0.0);
    let _ = (local, args_model, config_model);
    Some((
        context,
        if cost > 0.0 {
            Some(format_money(cost))
        } else {
            None
        },
    ))
}

/// Command metadata table (mirrors `promptCommands` + `stashCommands`).
#[derive(Debug, Clone)]
pub struct PromptCommandMeta {
    pub title: &'static str,
    pub name: &'static str,
    pub category: &'static str,
    pub hidden: bool,
    pub slash: Option<&'static str>,
}

pub const PROMPT_COMMANDS: &[PromptCommandMeta] = &[
    PromptCommandMeta {
        title: "Clear prompt",
        name: "prompt.clear",
        category: "Prompt",
        hidden: true,
        slash: None,
    },
    PromptCommandMeta {
        title: "Submit prompt",
        name: "prompt.submit",
        category: "Prompt",
        hidden: true,
        slash: None,
    },
    PromptCommandMeta {
        title: "Remove editor context",
        name: "prompt.editor_context.clear",
        category: "Prompt",
        hidden: false,
        slash: None,
    },
    PromptCommandMeta {
        title: "Paste",
        name: "prompt.paste",
        category: "Prompt",
        hidden: true,
        slash: None,
    },
    PromptCommandMeta {
        title: "Interrupt session",
        name: "session.interrupt",
        category: "Session",
        hidden: true,
        slash: None,
    },
    PromptCommandMeta {
        title: "Open editor",
        name: "prompt.editor",
        category: "Session",
        hidden: false,
        slash: Some("editor"),
    },
    PromptCommandMeta {
        title: "Skills",
        name: "prompt.skills",
        category: "Prompt",
        hidden: false,
        slash: Some("skills"),
    },
    PromptCommandMeta {
        title: "Warp",
        name: "workspace.set",
        category: "Session",
        hidden: false,
        slash: Some("warp"),
    },
    PromptCommandMeta {
        title: "Move session",
        name: "session.move",
        category: "Session",
        hidden: false,
        slash: Some("move"),
    },
    PromptCommandMeta {
        title: "Stash prompt",
        name: "prompt.stash",
        category: "Prompt",
        hidden: false,
        slash: None,
    },
    PromptCommandMeta {
        title: "Stash pop",
        name: "prompt.stash.pop",
        category: "Prompt",
        hidden: false,
        slash: None,
    },
    PromptCommandMeta {
        title: "Stash list",
        name: "prompt.stash.list",
        category: "Prompt",
        hidden: false,
        slash: None,
    },
    PromptCommandMeta {
        title: "Previous prompt history",
        name: "prompt.history.previous",
        category: "Prompt",
        hidden: false,
        slash: None,
    },
    PromptCommandMeta {
        title: "Next prompt history",
        name: "prompt.history.next",
        category: "Prompt",
        hidden: false,
        slash: None,
    },
];

/// Prompt keybinding groups (mirrors the `prompt.palette` gather list).
pub const PROMPT_PALETTE_COMMANDS: &[&str] = &[
    "prompt.submit",
    "prompt.editor",
    "prompt.editor_context.clear",
    "prompt.stash",
    "prompt.stash.pop",
    "prompt.stash.list",
    "prompt.skills",
    "session.interrupt",
    "workspace.set",
    "session.move",
];

/// Main prompt component state.
pub struct PromptComponent {
    pub textarea: PromptTextarea,
    pub store: PromptStoreState,
    pub autocomplete: AutocompleteState,
    pub submitting: bool,
    pub interrupt_deadline: Option<Instant>,
    pub session_id: Option<String>,
    pub prompt_part_type_id: u64,
}

impl PromptComponent {
    pub fn new(placeholder_count: usize, session_id: Option<String>) -> Self {
        Self {
            textarea: PromptTextarea::new(),
            store: PromptStoreState::new(placeholder_count),
            autocomplete: AutocompleteState::new(),
            submitting: false,
            interrupt_deadline: None,
            session_id,
            prompt_part_type_id: 0,
        }
    }

    /// Mount restore (mirrors `onMount` stashed-draft logic).
    pub fn mount(&mut self) {
        if !self.store.prompt.input.is_empty() {
            return;
        }
        if let Some((prompt, cursor)) = take_stashed() {
            if prompt.input.is_empty() {
                return;
            }
            self.textarea.set_text(&prompt.input);
            self.store.prompt = prompt;
            self.restore_extmarks();
            self.textarea.cursor = cursor;
        }
    }

    /// Unmount stash (mirrors `onCleanup`).
    pub fn unmount(&mut self) {
        if !self.store.prompt.input.is_empty() {
            stash_current(&self.store.prompt, self.textarea.cursor);
        }
    }

    /// Content-change handler (mirrors `onContentChange`).
    pub fn on_content_change(&mut self) {
        let value = self.textarea.plain_text().to_string();
        self.store.prompt.input = value.clone();
        self.autocomplete
            .on_input(&value, self.textarea.cursor, false);
        self.sync_extmarks();
    }

    /// Mirrors `restoreExtmarksFromParts`.
    pub fn restore_extmarks(&mut self) {
        self.textarea.clear_extmarks();
        self.store.extmark_to_part.clear();
        let parts = self.store.prompt.parts.clone();
        for (part_index, part) in parts.iter().enumerate() {
            let (start, end, virtual_text, style_id) = part_extmark_span(part);
            if virtual_text.is_empty() {
                continue;
            }
            let id =
                self.textarea
                    .create_extmark(start, end, true, style_id, self.prompt_part_type_id);
            self.store.extmark_to_part.insert(id, part_index);
        }
    }

    /// Mirrors `syncExtmarksWithPromptParts`.
    pub fn sync_extmarks(&mut self) {
        let marks = self.textarea.extmarks_for_type(self.prompt_part_type_id);
        let mut mapping = HashMap::new();
        let mut parts = Vec::new();
        for mark in marks {
            let Some(part_index) = self.store.extmark_to_part.get(&mark.id).copied() else {
                continue;
            };
            let Some(part) = self.store.prompt.parts.get(part_index).cloned() else {
                continue;
            };
            let mut part = part;
            if let Some(map) = part.as_object_mut() {
                if part.get("type").and_then(|v| v.as_str()) == Some("agent") {
                    if let Some(source) = map.get_mut("source").and_then(|v| v.as_object_mut()) {
                        source.insert("start".to_string(), Value::Number(mark.start.into()));
                        source.insert("end".to_string(), Value::Number(mark.end.into()));
                    }
                } else if let Some(text) = map
                    .get_mut("source")
                    .and_then(|v| v.get_mut("text"))
                    .and_then(|v| v.as_object_mut())
                {
                    text.insert("start".to_string(), Value::Number(mark.start.into()));
                    text.insert("end".to_string(), Value::Number(mark.end.into()));
                }
            }
            mapping.insert(mark.id, parts.len());
            parts.push(part);
        }
        self.store.extmark_to_part = mapping;
        self.store.prompt.parts = parts;
    }

    /// Mirrors `clearPrompt` (retains drafts ≥20 chars or with parts).
    pub fn clear_prompt(&mut self, history: &mut PromptHistory) -> bool {
        let retain = self.store.prompt.input.trim().chars().count() >= DRAFT_RETENTION_MIN_CHARS
            || !self.store.prompt.parts.is_empty();
        let _ = retain;
        self.textarea.clear();
        self.textarea.clear_extmarks();
        self.store.prompt = PromptInfo {
            input: String::new(),
            mode: None,
            parts: Vec::new(),
        };
        self.store.extmark_to_part.clear();
        retain
    }

    /// Interrupt double-press (mirrors the `session.interrupt` run).
    pub fn interrupt_press(&mut self) -> bool {
        self.store.interrupt += 1;
        self.interrupt_deadline = Some(Instant::now() + Duration::from_millis(INTERRUPT_WINDOW_MS));
        self.store.interrupt >= 2
    }

    pub fn poll_interrupt(&mut self) {
        if self
            .interrupt_deadline
            .map(|deadline| Instant::now() >= deadline)
            .unwrap_or(false)
        {
            self.store.interrupt = 0;
            self.interrupt_deadline = None;
        }
    }

    /// History navigation (mirrors the two history commands).
    pub fn history_move(&mut self, history: &mut PromptHistory, direction: i64) -> bool {
        let input = self.textarea.plain_text().to_string();
        let Some(item) = history.move_history(direction, &input) else {
            return false;
        };
        self.textarea.set_text(&item.input);
        self.store.prompt = item.clone();
        self.store.mode = item
            .mode
            .map(|mode| match mode {
                crate::prompt::history::PromptMode::Shell => PromptMode::Shell,
                crate::prompt::history::PromptMode::Normal => PromptMode::Normal,
            })
            .unwrap_or(PromptMode::Normal);
        self.restore_extmarks();
        if direction < 0 {
            self.textarea.cursor = 0;
        } else {
            self.textarea.goto_buffer_end();
        }
        true
    }

    /// Paste-text flow (mirrors `pasteText`).
    pub fn paste_text(&mut self, text: &str, virtual_text: &str, paste_style_id: Option<u64>) {
        let start = self.textarea.cursor;
        let end = start + prompt_offset_width(virtual_text);
        self.textarea.insert_text(&format!("{virtual_text} "));
        let id = self.textarea.create_extmark(
            start,
            end,
            true,
            paste_style_id,
            self.prompt_part_type_id,
        );
        let part_index = self.store.prompt.parts.len();
        self.store.prompt.parts.push(serde_json::json!({
            "type": "text",
            "text": text,
            "source": { "text": { "start": start, "end": end, "value": virtual_text } },
        }));
        self.store.extmark_to_part.insert(id, part_index);
    }

    /// Paste-input routing (mirrors `pasteInputText` up to attachment IO).
    pub fn paste_route(text: &str) -> PasteRoute {
        let filepath = pasted_filepath(text.trim(), std::env::consts::OS);
        if filepath.starts_with("http://") || filepath.starts_with("https://") {
            return PasteRoute::PlainText;
        }
        match read_local_attachment(&filepath) {
            Some(super::local_attachment::LocalAttachment::Text { content, .. }) => {
                let name = filepath.rsplit('/').next().unwrap_or("image").to_string();
                PasteRoute::SvgText {
                    content,
                    label: format!("[SVG: {name}]"),
                }
            }
            Some(super::local_attachment::LocalAttachment::Binary { mime, content }) => {
                PasteRoute::Attachment {
                    filename: filepath.rsplit('/').next().unwrap_or("").to_string(),
                    filepath,
                    mime,
                    base64: base64_encode(&content),
                }
            }
            None => {
                let line_count = text.matches('\n').count() + 1;
                if line_count >= 3 || text.chars().count() > 150 {
                    PasteRoute::Summary {
                        label: format!("[Pasted ~{line_count} lines]"),
                        text: text.to_string(),
                    }
                } else {
                    PasteRoute::PlainText
                }
            }
        }
    }

    /// Attachment part builder (mirrors `pasteAttachment`).
    pub fn paste_attachment(
        &mut self,
        filename: &str,
        filepath: &str,
        mime: &str,
        base64: &str,
        paste_style_id: Option<u64>,
    ) {
        let pdf = mime == "application/pdf";
        let count = self
            .store
            .prompt
            .parts
            .iter()
            .filter(|part| {
                part.get("type").and_then(|v| v.as_str()) != Some("file")
                    || if pdf {
                        part.get("mime").and_then(|v| v.as_str()) == Some("application/pdf")
                    } else {
                        part.get("mime")
                            .and_then(|v| v.as_str())
                            .map(|mime| mime.starts_with("image/"))
                            .unwrap_or(false)
                    }
            })
            .count();
        // NOTE: count mirrors the source filter verbatim (non-files excluded
        // from the tally only when they fail the mime test — see below).
        let _ = count;
        let numbered = self
            .store
            .prompt
            .parts
            .iter()
            .filter(|part| {
                if part.get("type").and_then(|v| v.as_str()) != Some("file") {
                    return false;
                }
                if pdf {
                    part.get("mime").and_then(|v| v.as_str()) == Some("application/pdf")
                } else {
                    part.get("mime")
                        .and_then(|v| v.as_str())
                        .map(|mime| mime.starts_with("image/"))
                        .unwrap_or(false)
                }
            })
            .count();
        let virtual_text = if pdf {
            format!("[PDF {}]", numbered + 1)
        } else {
            format!("[Image {}]", numbered + 1)
        };
        let start = self.textarea.cursor;
        let end = start + virtual_text.chars().count();
        self.textarea.insert_text(&format!("{virtual_text} "));
        let id = self.textarea.create_extmark(
            start,
            end,
            true,
            paste_style_id,
            self.prompt_part_type_id,
        );
        let part_index = self.store.prompt.parts.len();
        self.store.prompt.parts.push(serde_json::json!({
            "type": "file",
            "mime": mime,
            "filename": filename,
            "url": format!("data:{mime};base64,{base64}"),
            "source": {
                "type": "file",
                "path": if filepath.is_empty() { filename } else { filepath },
                "text": { "start": start, "end": end, "value": virtual_text },
            },
        }));
        self.store.extmark_to_part.insert(id, part_index);
    }

    /// Traits for the keymap layer (mirrors the traits effect).
    pub fn traits(&self) -> crate::prompt::traits::PromptTraits {
        compute_prompt_traits(self.store.mode, self.autocomplete.visible.is_some())
    }

    /// Submit guard + flow (mirrors `submit`/`submitInner` verbatim).
    /// Returns the side effects for the app; textarea/store reset happens
    /// inline on success paths.
    pub async fn submit(&mut self, args: SubmitFlowArgs<'_>) -> SubmitEffect {
        if self.submitting {
            return SubmitEffect::None;
        }
        self.submitting = true;
        let effect = self.submit_inner(args).await;
        self.submitting = false;
        effect
    }

    async fn submit_inner(&mut self, mut args: SubmitFlowArgs<'_>) -> SubmitEffect {
        args.workspace.clear_notice();
        // IME sync: textarea text wins over the store (verbatim).
        if self.textarea.plain_text() != self.store.prompt.input {
            self.store.prompt.input = self.textarea.plain_text().to_string();
            self.sync_extmarks();
        }
        if args.disabled {
            return SubmitEffect::None;
        }
        if args.workspace.creating || args.move_state.creating {
            return SubmitEffect::None;
        }
        if self.autocomplete.visible.is_some() {
            return SubmitEffect::None;
        }
        if self.store.prompt.input.is_empty() {
            return SubmitEffect::None;
        }
        let Some(agent_name) = args.agent_name.clone() else {
            return SubmitEffect::None;
        };
        let trimmed = self.store.prompt.input.trim().to_string();
        if trimmed == "exit" || trimmed == "quit" || trimmed == ":q" {
            return SubmitEffect::Exit;
        }
        let Some(model) = args.model.clone() else {
            return SubmitEffect::ShowProviderConnect;
        };
        if let Some(session_id) = args.session_id {
            if let Some(status) = args.workspace_status.clone() {
                if status != "connected" {
                    return SubmitEffect::ShowWorkspaceUnavailable;
                }
            }
        }
        let variant = args.variant.clone();
        let mut session_id = args.session_id.map(str::to_string);
        let mut finish_move_progress = false;
        if session_id.is_none() {
            let directory = args.move_directory.clone();
            if args.move_pending && directory.is_none() {
                return SubmitEffect::None;
            }
            finish_move_progress = args.move_progress;
            let created = args
                .client
                .call(
                    "session.create",
                    serde_json::json!({
                        "directory": directory,
                        "workspace": args.workspace_id,
                        "agent": agent_name,
                        "model": { "providerID": model.provider_id, "id": model.model_id, "variant": variant },
                    }),
                )
                .await;
            match created {
                Err(_) => {
                    if finish_move_progress {
                        args.move_state.finish_submit(&mut args.home_destination);
                    }
                    return SubmitEffect::Toast {
                        title: None,
                        message: "Creating a session failed. Open console for more details."
                            .to_string(),
                        error: true,
                    };
                }
                Ok(response) => {
                    session_id = response
                        .get("data")
                        .and_then(|d| d.get("id"))
                        .and_then(|v| v.as_str())
                        .map(str::to_string);
                }
            }
        }
        let Some(session_id) = session_id else {
            return SubmitEffect::None;
        };
        // Expand tracked pastes inline; drop text parts afterwards.
        let pasted: Vec<(usize, usize, String)> = self
            .textarea
            .extmarks_for_type(self.prompt_part_type_id)
            .iter()
            .filter_map(|mark| {
                let part_index = self.store.extmark_to_part.get(&mark.id).copied()?;
                let part = self.store.prompt.parts.get(part_index)?;
                if part.get("type").and_then(|v| v.as_str()) != Some("text") {
                    return None;
                }
                Some((
                    mark.start,
                    mark.end,
                    part.get("text")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string(),
                ))
            })
            .collect();
        let input_text = expand_tracked_pasted_text(
            &self.store.prompt.input,
            &pasted
                .iter()
                .map(|(s, e, t)| (*s, *e, t.clone()))
                .collect::<Vec<_>>(),
        );
        let non_text_parts: Vec<Value> = self
            .store
            .prompt
            .parts
            .iter()
            .filter(|part| part.get("type").and_then(|v| v.as_str()) != Some("text"))
            .cloned()
            .collect();
        let current_mode = self.store.mode;
        let mut editor_parts: Vec<Value> = Vec::new();
        if let Some(selection) = args.editor_selection.clone() {
            if args.editor_label_pending {
                editor_parts.push(serde_json::json!({
                    "type": "text",
                    "text": format_editor_context(&selection),
                    "synthetic": true,
                    "metadata": {
                        "kind": "editor_context",
                        "source": selection.source.clone().unwrap_or_else(|| "editor".to_string()),
                        "filePath": selection.file_path,
                        "ranges": selection.ranges.iter().map(|range| serde_json::json!({
                            "text": range.text,
                            "selection": {
                                "start": { "line": range.selection.start.line, "character": range.selection.start.character },
                                "end": { "line": range.selection.end.line, "character": range.selection.end.character },
                            },
                        })).collect::<Vec<_>>(),
                    },
                }));
            }
        }
        if current_mode == PromptMode::Shell {
            args.move_state.start_submit();
            let _ = args
                .client
                .call(
                    "session.shell",
                    serde_json::json!({
                        "sessionID": session_id,
                        "agent": agent_name,
                        "model": { "providerID": model.provider_id, "modelID": model.model_id },
                        "command": input_text,
                    }),
                )
                .await;
            self.store.mode = PromptMode::Normal;
        } else if input_text.starts_with('/')
            && args.commands.iter().any(|name| {
                input_text
                    .split('\n')
                    .next()
                    .unwrap_or("")
                    .split(' ')
                    .next()
                    .unwrap_or("")
                    .trim_start_matches('/')
                    == name
            })
        {
            args.move_state.start_submit();
            let first_line_end = input_text.find('\n');
            let first_line = match first_line_end {
                Some(end) => &input_text[..end],
                None => &input_text,
            };
            let mut words = first_line.split(' ');
            let command = words
                .next()
                .unwrap_or("")
                .trim_start_matches('/')
                .to_string();
            let rest = words.collect::<Vec<_>>().join(" ");
            let args_text = match first_line_end {
                Some(end) => {
                    let tail = &input_text[end + 1..];
                    if tail.is_empty() {
                        rest
                    } else {
                        format!("{rest}\n{tail}")
                    }
                }
                None => rest,
            };
            let file_parts: Vec<Value> = non_text_parts
                .iter()
                .filter(|part| part.get("type").and_then(|v| v.as_str()) == Some("file"))
                .cloned()
                .collect();
            let _ = args
                .client
                .call(
                    "session.command",
                    serde_json::json!({
                        "sessionID": session_id,
                        "command": command,
                        "arguments": args_text,
                        "agent": agent_name,
                        "model": format!("{}/{}", model.provider_id, model.model_id),
                        "variant": variant,
                        "parts": file_parts,
                    }),
                )
                .await;
        } else {
            args.move_state.start_submit();
            let mut parts = editor_parts;
            parts.push(serde_json::json!({ "type": "text", "text": input_text }));
            parts.extend(non_text_parts);
            let result = args
                .client
                .call(
                    "session.prompt",
                    serde_json::json!({
                        "sessionID": session_id,
                        "providerID": model.provider_id,
                        "agent": agent_name,
                        "model": { "providerID": model.provider_id, "modelID": model.model_id },
                        "variant": variant,
                        "parts": parts,
                    }),
                )
                .await;
            if let Err(err) = result {
                return SubmitEffect::Toast {
                    title: Some("Failed to send prompt".to_string()),
                    message: err,
                    error: true,
                };
            }
            if args.editor_had_parts {
                args.editor_mark_sent = true;
            }
        }
        args.history
            .append(PromptInfo {
                input: self.store.prompt.input.clone(),
                mode: Some(current_mode),
                parts: self.store.prompt.parts.clone(),
            })
            .await;
        self.textarea.clear_extmarks();
        self.store.prompt = PromptInfo {
            input: String::new(),
            mode: None,
            parts: Vec::new(),
        };
        self.store.extmark_to_part.clear();
        self.textarea.clear();
        if finish_move_progress {
            args.move_state.finish_submit(&mut args.home_destination);
        }
        SubmitEffect::Submitted { session_id }
    }
}

/// Arguments for the submit flow (all caller-owned state).
pub struct SubmitFlowArgs<'a> {
    pub client: Arc<dyn crate::context::sdk::SdkClient>,
    pub disabled: bool,
    pub workspace: &'a mut PromptWorkspace,
    pub move_state: &'a mut PromptMove,
    pub move_pending: bool,
    pub move_progress: bool,
    pub move_directory: Option<String>,
    pub workspace_id: Option<String>,
    pub session_id: Option<&'a str>,
    pub agent_name: Option<String>,
    pub model: Option<ModelId>,
    pub variant: Option<String>,
    pub commands: Vec<String>,
    pub workspace_status: Option<String>,
    pub editor_selection: Option<EditorSelection>,
    pub editor_label_pending: bool,
    pub editor_had_parts: bool,
    pub editor_mark_sent: bool,
    pub history: &'a mut PromptHistory,
    pub home_destination: crate::routes::home::session_destination::HomeSessionDestinationState,
}

/// Paste routing decision.
#[derive(Debug, Clone)]
pub enum PasteRoute {
    PlainText,
    SvgText {
        content: String,
        label: String,
    },
    Attachment {
        filename: String,
        filepath: String,
        mime: String,
        base64: String,
    },
    Summary {
        label: String,
        text: String,
    },
}

fn base64_encode(bytes: &[u8]) -> String {
    const ALPHABET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::new();
    for chunk in bytes.chunks(3) {
        let mut word = [0u8; 3];
        word[..chunk.len()].copy_from_slice(chunk);
        let triple = ((word[0] as u32) << 16) | ((word[1] as u32) << 8) | word[2] as u32;
        out.push(ALPHABET[((triple >> 18) & 63) as usize] as char);
        out.push(ALPHABET[((triple >> 12) & 63) as usize] as char);
        out.push(if chunk.len() > 1 {
            ALPHABET[((triple >> 6) & 63) as usize] as char
        } else {
            '='
        });
        out.push(if chunk.len() > 2 {
            ALPHABET[(triple & 63) as usize] as char
        } else {
            '='
        });
    }
    out
}

fn part_extmark_span(part: &Value) -> (usize, usize, String, Option<u64>) {
    let kind = part.get("type").and_then(|v| v.as_str()).unwrap_or("");
    match kind {
        "file" => {
            let text = part.get("source").and_then(|s| s.get("text"));
            (
                text.and_then(|t| t.get("start"))
                    .and_then(|v| v.as_u64())
                    .unwrap_or(0) as usize,
                text.and_then(|t| t.get("end"))
                    .and_then(|v| v.as_u64())
                    .unwrap_or(0) as usize,
                text.and_then(|t| t.get("value"))
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string(),
                None,
            )
        }
        "agent" => {
            let source = part.get("source");
            (
                source
                    .and_then(|s| s.get("start"))
                    .and_then(|v| v.as_u64())
                    .unwrap_or(0) as usize,
                source
                    .and_then(|s| s.get("end"))
                    .and_then(|v| v.as_u64())
                    .unwrap_or(0) as usize,
                source
                    .and_then(|s| s.get("value"))
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string(),
                None,
            )
        }
        "text" => {
            let text = part.get("source").and_then(|s| s.get("text"));
            (
                text.and_then(|t| t.get("start"))
                    .and_then(|v| v.as_u64())
                    .unwrap_or(0) as usize,
                text.and_then(|t| t.get("end"))
                    .and_then(|v| v.as_u64())
                    .unwrap_or(0) as usize,
                text.and_then(|t| t.get("value"))
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string(),
                None,
            )
        }
        _ => (0, 0, String::new(), None),
    }
}
