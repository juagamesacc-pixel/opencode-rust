// source: packages/tui/src/component/prompt/autocomplete.tsx (781 lines, v1.18.30)
// 1:1 port — @/slash autocomplete as an explicit state machine. Filter
// extraction, line-range parsing, file-part building, option assembly
// (frecency-boosted fuzzy over commands/agents/references/mcp/files),
// movement, directory expansion, and the hide/show/onInput transitions
// are verbatim. Textarea mutations are returned as `AutocompleteEdit`
// descriptors for the prompt component to apply.

#![allow(dead_code)]

use serde_json::Value;

use crate::ui::dialog_select::fuzzy_score;
use crate::util::locale::truncate_middle;

/// Mirrors the visible union (`false | "@" | "/"`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AutocompleteVisible {
    At,
    Slash,
}

/// Mirrors `AutocompleteOption` (selection callbacks become data).
#[derive(Debug, Clone, Default)]
pub struct AutocompleteOption {
    pub display: String,
    pub value: Option<String>,
    pub aliases: Vec<String>,
    pub disabled: bool,
    pub description: Option<String>,
    pub is_directory: bool,
    pub path: Option<String>,
    pub select: AutocompleteSelect,
}

/// What selecting the option does (mirrors the `onSelect` closures).
#[derive(Debug, Clone)]
pub enum AutocompleteSelect {
    InsertPart { filename: String, part: Value },
    InsertCommand { text: String },
    None,
}

impl Default for AutocompleteSelect {
    fn default() -> Self {
        AutocompleteSelect::None
    }
}

/// Mirrors `removeLineRange` (cut from the last `#`).
pub fn remove_line_range(input: &str) -> String {
    match input.rfind('#') {
        Some(index) => input[..index].to_string(),
        None => input.to_string(),
    }
}

/// Parsed `#start-end` suffix (mirrors `extractLineRange`).
#[derive(Debug, Clone)]
pub struct LineRange {
    pub base_name: String,
    pub start_line: u64,
    pub end_line: Option<u64>,
}

pub fn extract_line_range(input: &str) -> (Option<LineRange>, String) {
    let Some(hash) = input.rfind('#') else {
        return (None, input.to_string());
    };
    let base_name = input[..hash].to_string();
    let line_part = &input[hash + 1..];
    let (start_text, end_text) = match line_part.split_once('-') {
        Some((start, end)) => (start, Some(end)),
        None => (line_part, None),
    };
    if start_text.is_empty() || !start_text.chars().all(|ch| ch.is_ascii_digit()) {
        return (None, base_name);
    }
    if let Some(end) = end_text {
        if !end.chars().all(|ch| ch.is_ascii_digit()) {
            return (None, base_name);
        }
    }
    let start_line: u64 = start_text.parse().unwrap_or(0);
    let end_line = end_text
        .filter(|end| !end.is_empty())
        .and_then(|end| end.parse::<u64>().ok())
        .filter(|end| start_line < *end);
    (
        Some(LineRange {
            base_name: base_name.clone(),
            start_line,
            end_line,
        }),
        base_name,
    )
}

/// Mirrors `normalizeMentionPath` (cwd-relative with `/` separators).
pub fn normalize_mention_path(file_path: &str, base_dir: &str) -> String {
    let absolute = if file_path.starts_with('/') {
        file_path.to_string()
    } else {
        format!("{}/{}", base_dir.trim_end_matches('/'), file_path)
    };
    let relative = crate::runtime::relative_path(base_dir, &absolute);
    if !relative.is_empty()
        && relative != ".."
        && !relative.starts_with("../")
        && !relative.starts_with('/')
    {
        return relative.replace('\\', "/");
    }
    absolute.replace('\\', "/")
}

/// File URL builder (mirrors `pathToFileURL` + line-range params).
pub fn file_url(file_path: &str, line_range: Option<&LineRange>) -> String {
    let mut url = format!("file://{file_path}");
    if let Some(range) = line_range {
        url += &format!("?start={}", range.start_line);
        if let Some(end) = range.end_line {
            url += &format!("&end={end}");
        }
    }
    url
}

/// Mirrors `createFilePart` (filename + part shape).
pub fn create_file_part(
    item_path: &str,
    item_type: &str,
    file_path: &str,
    line_range: Option<&LineRange>,
) -> (String, Value) {
    let filename = match line_range {
        Some(range) if item_type != "directory" => {
            let mut name = format!("{item_path}#{}", range.start_line);
            if let Some(end) = range.end_line {
                name += &format!("-{end}");
            }
            name
        }
        _ => item_path.to_string(),
    };
    let part = serde_json::json!({
        "type": "file",
        "mime": if item_type == "directory" { "application/x-directory" } else { "text/plain" },
        "filename": filename,
        "url": file_url(file_path, line_range),
        "source": {
            "type": "file",
            "text": { "start": 0, "end": 0, "value": "" },
            "path": item_path,
        },
    });
    (filename, part)
}

/// Text edit descriptor for part insertion (mirrors `insertPart`'s
/// textarea ops: delete trigger range, insert `@text `, extmark span).
#[derive(Debug, Clone)]
pub struct PartInsertEdit {
    pub delete_from_offset: usize,
    pub delete_to_offset: usize,
    pub insert_text: String,
    pub extmark_start: usize,
    pub extmark_end: usize,
    pub style_kind: PartStyle,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PartStyle {
    File,
    Agent,
    Other,
}

/// Compute the insertion edit (needs-space rule verbatim).
pub fn part_insert_edit(
    text: &str,
    trigger_offset: usize,
    cursor_offset: usize,
    char_after_cursor: Option<char>,
) -> PartInsertEdit {
    let needs_space = char_after_cursor != Some(' ');
    let insert_text = format!("@{text}{}", if needs_space { " " } else { "" });
    let virtual_text = format!("@{text}");
    PartInsertEdit {
        delete_from_offset: trigger_offset,
        delete_to_offset: cursor_offset,
        insert_text,
        extmark_start: trigger_offset,
        extmark_end: trigger_offset + virtual_text.chars().count(),
        style_kind: PartStyle::Other,
    }
}

/// Agent/reference/mcp/command rows for the option assembly.
#[derive(Debug, Clone, Default)]
pub struct AutocompleteSources {
    pub agents: Vec<(String, bool)>,
    pub references: Vec<ReferenceRow>,
    pub mcp_resources: Vec<McpResourceRow>,
    pub slash_commands: Vec<String>,
    pub server_commands: Vec<ServerCommandRow>,
    pub files: Vec<FileRow>,
}

#[derive(Debug, Clone, Default)]
pub struct ReferenceRow {
    pub name: String,
    pub path: String,
    pub hidden: bool,
    pub source_label: String,
}

#[derive(Debug, Clone, Default)]
pub struct McpResourceRow {
    pub name: String,
    pub description: Option<String>,
    pub mime_type: Option<String>,
    pub uri: String,
    pub client: String,
}

#[derive(Debug, Clone, Default)]
pub struct ServerCommandRow {
    pub name: String,
    pub description: Option<String>,
    pub source: String,
}

#[derive(Debug, Clone)]
pub struct FileRow {
    pub path: String,
    pub entry_type: String,
    pub location_directory: String,
}

/// Reference alias match (mirrors `referenceMatch`).
pub fn reference_match<'a>(
    references: &'a [ReferenceRow],
    visible_at: bool,
    search: &str,
) -> Option<&'a ReferenceRow> {
    if !visible_at {
        return None;
    }
    let (_, base_query) = extract_line_range(search);
    let alias = match base_query.find('/') {
        Some(slash) => base_query[..slash].to_string(),
        None => base_query,
    };
    references
        .iter()
        .find(|item| !item.hidden && item.name == alias)
}

/// Build agent + reference-alias rows (mirrors the memos).
pub fn agent_reference_options(
    sources: &AutocompleteSources,
) -> (Vec<AutocompleteOption>, Vec<AutocompleteOption>) {
    let agents = sources
        .agents
        .iter()
        .filter(|(_, hidden)| !hidden)
        .map(|(name, _)| AutocompleteOption {
            display: format!("@{name}"),
            select: AutocompleteSelect::InsertPart {
                filename: name.clone(),
                part: serde_json::json!({
                    "type": "agent",
                    "name": name,
                    "source": { "start": 0, "end": 0, "value": "" },
                }),
            },
            ..AutocompleteOption::default()
        })
        .collect();
    let references = sources
        .references
        .iter()
        .filter(|reference| !reference.hidden)
        .map(|reference| AutocompleteOption {
            display: format!("@{}", reference.name),
            description: Some(format!(" {}", reference.source_label)),
            select: AutocompleteSelect::InsertPart {
                filename: reference.name.clone(),
                part: serde_json::json!({
                    "type": "file",
                    "mime": "application/x-directory",
                    "filename": reference.name,
                    "url": format!("file://{}", reference.path),
                    "source": { "type": "file", "text": { "start": 0, "end": 0, "value": "" }, "path": reference.name },
                }),
            },
            ..AutocompleteOption::default()
        })
        .collect();
    (agents, references)
}

/// Build MCP resource rows.
pub fn mcp_resource_options(
    sources: &AutocompleteSources,
    width: usize,
) -> Vec<AutocompleteOption> {
    sources
        .mcp_resources
        .iter()
        .map(|res| {
            let display = crate::util::locale::truncate_middle(&res.name, width.max(1));
            AutocompleteOption {
                display,
                value: Some(res.name.clone()),
                description: res.description.clone(),
                select: AutocompleteSelect::InsertPart {
                    filename: res.name.clone(),
                    part: serde_json::json!({
                        "type": "file",
                        "mime": res.mime_type.clone().unwrap_or_else(|| "text/plain".to_string()),
                        "filename": res.name,
                        "url": res.uri,
                        "source": {
                            "type": "resource",
                            "text": { "start": 0, "end": 0, "value": "" },
                            "clientName": res.client,
                            "uri": res.uri,
                        },
                    }),
                },
                ..AutocompleteOption::default()
            }
        })
        .collect()
}

/// Build slash + server command rows (sorted, padded, verbatim).
pub fn command_options(sources: &AutocompleteSources) -> Vec<AutocompleteOption> {
    let mut results: Vec<AutocompleteOption> = sources
        .slash_commands
        .iter()
        .map(|display| AutocompleteOption {
            display: display.clone(),
            ..AutocompleteOption::default()
        })
        .collect();
    for server in &sources.server_commands {
        if server.source == "skill" {
            continue;
        }
        let label = if server.source == "mcp" { ":mcp" } else { "" };
        results.push(AutocompleteOption {
            display: format!("/{}{label}", server.name),
            description: server.description.clone(),
            select: AutocompleteSelect::InsertCommand {
                text: format!("/{} ", server.name),
            },
            ..AutocompleteOption::default()
        });
    }
    results.sort_by(|a, b| a.display.cmp(&b.display));
    let max = results
        .iter()
        .map(|item| item.display.chars().count())
        .max()
        .unwrap_or(0);
    if max == 0 {
        return results;
    }
    for item in &mut results {
        let pad = max + 2 - item.display.chars().count();
        item.display += &" ".repeat(pad);
    }
    results
}

/// Build file rows (order trusted from the finder — no re-sort).
pub fn file_options(rows: &[FileRow], width: usize) -> Vec<AutocompleteOption> {
    rows.iter()
        .map(|row| {
            let (filename, part) = create_file_part(
                &row.path,
                &row.entry_type,
                &format!(
                    "{}/{}",
                    row.location_directory.trim_end_matches('/'),
                    row.path
                ),
                None,
            );
            AutocompleteOption {
                display: truncate_middle(&filename, width.max(1)),
                value: Some(filename.clone()),
                is_directory: row.entry_type == "directory",
                path: Some(row.path.clone()),
                select: AutocompleteSelect::InsertPart { filename, part },
                ..AutocompleteOption::default()
            }
        })
        .collect()
}

/// Assemble the option list (mirrors the `options` memo: reference-alias
/// short-circuit, file passthrough, fuzzy non-files capped at 10).
pub fn assemble_options(
    visible: AutocompleteVisible,
    search: &str,
    files: Vec<AutocompleteOption>,
    mut non_files: Vec<AutocompleteOption>,
    matched_reference: Option<&ReferenceRow>,
    frecency_of: &dyn Fn(Option<&str>) -> f64,
) -> Vec<AutocompleteOption> {
    if visible == AutocompleteVisible::At {
        if let Some(reference) = matched_reference {
            return non_files
                .into_iter()
                .filter(|item| item.display == format!("@{}", reference.name))
                .collect();
        }
    }
    let file_options = if visible == AutocompleteVisible::At {
        files
    } else {
        Vec::new()
    };
    if search.is_empty() {
        non_files.extend(file_options);
        return non_files;
    }
    let needle = remove_line_range(search);
    let prefix = format!(
        "{}{}",
        if visible == AutocompleteVisible::At {
            "@"
        } else {
            "/"
        },
        search
    );
    let mut scored: Vec<(i64, AutocompleteOption)> = Vec::new();
    for option in non_files {
        let target = remove_line_range(
            option
                .value
                .as_deref()
                .unwrap_or(&option.display)
                .trim_end()
                .to_string()
                .as_str(),
        );
        let Some(mut score) = fuzzy_score(&needle.to_lowercase(), &target.to_lowercase(), None)
        else {
            continue;
        };
        if option.display.starts_with(&prefix) {
            score *= 2;
        }
        let frecency = frecency_of(option.path.as_deref());
        score = ((score as f64) * (1.0 + frecency)) as i64;
        scored.push((score, option));
    }
    scored.sort_by_key(|(score, _)| std::cmp::Reverse(*score));
    let mut out: Vec<AutocompleteOption> = scored
        .into_iter()
        .take(10)
        .map(|(_, option)| option)
        .collect();
    out.extend(file_options);
    out.into_iter().take(10).collect()
}

/// Autocomplete state machine.
pub struct AutocompleteState {
    pub index: usize,
    pub selected: usize,
    pub visible: Option<AutocompleteVisible>,
    pub keyboard: bool,
    pub search: String,
    pub scroll_offset: usize,
}

impl AutocompleteState {
    pub fn new() -> Self {
        Self {
            index: 0,
            selected: 0,
            visible: None,
            keyboard: true,
            search: String::new(),
            scroll_offset: 0,
        }
    }

    /// Mirrors `show` (trigger offset captured by the caller).
    pub fn show(&mut self, mode: AutocompleteVisible, trigger_offset: usize) {
        self.visible = Some(mode);
        self.index = trigger_offset;
        self.selected = 0;
    }

    pub fn hide(&mut self) {
        self.visible = None;
    }

    /// Mirrors `move` (wrap-around).
    pub fn move_selection(&mut self, option_count: usize, direction: i64) {
        if self.visible.is_none() || option_count == 0 {
            return;
        }
        let mut next = self.selected as i64 + direction;
        if next < 0 {
            next = option_count as i64 - 1;
        }
        if next >= option_count as i64 {
            next = 0;
        }
        self.move_to(next as usize, option_count);
    }

    /// Mirrors `moveTo` (viewport follow).
    pub fn move_to(&mut self, next: usize, option_count: usize) {
        self.selected = next;
        let viewport = option_count.min(10).max(1);
        let bottom = self.scroll_offset + viewport;
        if next < self.scroll_offset {
            self.scroll_offset = next;
        } else if next + 1 > bottom {
            self.scroll_offset = next + 1 - viewport;
        }
    }

    /// Hide-with-`/`-cleanup (mirrors `hide()`): returns the cleanup edit
    /// when `/cmd` text needs resetting to plain input.
    pub fn hide_cleanup(&mut self, prompt_text: &str) -> Option<SlashCleanup> {
        let slash = self.visible == Some(AutocompleteVisible::Slash);
        self.visible = None;
        if slash && !prompt_text.ends_with(' ') && prompt_text.starts_with('/') {
            return Some(SlashCleanup);
        }
        None
    }

    /// Mirrors the `onInput` transition (hide / reopen checks verbatim).
    pub fn on_input(
        &mut self,
        value: &str,
        cursor_offset: usize,
        text_range_has_space: bool,
    ) -> Option<AutocompleteVisible> {
        if self.visible.is_some() {
            if cursor_offset <= self.index
                || text_range_has_space
                || (self.visible == Some(AutocompleteVisible::Slash)
                    && value.matches(' ').count() >= 2
                    && value.starts_with('/'))
            {
                // Note: the `/^\S+\s+\S+\s*$/` test is approximated by the
                // two-space count above (single-space `/cmd ` stays open).
                self.visible = None;
            }
            return None;
        }
        if cursor_offset == 0 {
            return None;
        }
        if value.starts_with('/') && !value[..cursor_offset.min(value.len())].contains(' ') {
            self.show(AutocompleteVisible::Slash, 0);
            return Some(AutocompleteVisible::Slash);
        }
        if let Some(idx) = crate::prompt::display::mention_trigger_index(value, cursor_offset) {
            self.show(AutocompleteVisible::At, idx);
            return Some(AutocompleteVisible::At);
        }
        None
    }

    /// List height (mirrors `height()`: min(10, count), capped by anchor y).
    pub fn list_height(option_count: usize, visible: bool, anchor_y: u16) -> usize {
        let count = option_count.max(1);
        if !visible {
            return count.min(10);
        }
        count.min(10).min((anchor_y as usize).max(1))
    }

    pub fn mark_keyboard(&mut self) {
        self.keyboard = true;
    }

    pub fn mark_mouse(&mut self) {
        self.keyboard = false;
    }
}

impl Default for AutocompleteState {
    fn default() -> Self {
        Self::new()
    }
}

/// Slash cleanup marker (caller resets the prompt input to plain text).
pub struct SlashCleanup;

/// Directory expansion edit (mirrors `expandDirectory`).
pub fn expand_directory_edit(
    selected: &AutocompleteOption,
    trigger_offset: usize,
) -> (usize, usize, String) {
    let display = selected
        .value
        .as_deref()
        .unwrap_or(&selected.display)
        .trim_end()
        .to_string();
    let path = display.strip_prefix('@').unwrap_or(&display).to_string();
    (trigger_offset, usize::MAX, format!("@{path}/"))
}

/// Empty-list copy verbatim.
pub const NO_MATCHING_ITEMS: &str = "No matching items";
