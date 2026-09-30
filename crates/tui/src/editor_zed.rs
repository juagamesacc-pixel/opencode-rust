//! Rust port of `src/editor-zed.ts` (opencode v1.18.30).
//!
//! Zed editor selection via its SQLite database (read-only opens, like
//! `bun:sqlite` in the source). `EditorSelection` is defined here — it is the
//! canonical shape from `context/editor.ts`, which re-exports this module's
//! type when that context lands, so the shape exists exactly once.
//!
//! Text offsets mirror the source precisely: JavaScript strings are UTF-16
//! sequences, so byte-to-index conversions count UTF-16 code units (an emoji
//! advances the index by two).
//!
//! Original file: `packages/tui/src/editor-zed.ts`

use rusqlite::{types::Value, Connection, OpenFlags, OptionalExtension};

/// A 1-based line/character position (`PositionSchema`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EditorPosition {
    pub line: i64,
    pub character: i64,
}

/// One selected span with its text (`EditorSelectionRangeSchema`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EditorSelectionRange {
    pub text: String,
    pub selection: EditorSelectionSpan,
}

/// Start/end positions of a range.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EditorSelectionSpan {
    pub start: EditorPosition,
    pub end: EditorPosition,
}

/// The canonical editor selection shape (`EditorSelection` in
/// `context/editor.ts`: `{ filePath, source?, ranges }` with at least one
/// range; `source` is `"websocket"` or `"zed"` when present).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EditorSelection {
    pub file_path: String,
    pub source: Option<String>,
    pub ranges: Vec<EditorSelectionRange>,
}

/// Mirrors `ZedSelectionResult`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ZedSelectionResult {
    Selection { selection: EditorSelection },
    Empty,
    Unavailable,
}

struct ZedEditorRow {
    item_kind: String,
    editor_id: Option<i64>,
    workspace_id: i64,
    workspace_paths: Option<String>,
    timestamp: String,
    buffer_path: Option<String>,
}

fn as_i64(value: &Value) -> Option<i64> {
    match value {
        Value::Integer(value) => Some(*value),
        // Schema.Number accepts floats; offsets are integral in practice.
        Value::Real(value) => Some(*value as i64),
        _ => None,
    }
}

fn as_string(value: &Value) -> Option<String> {
    match value {
        Value::Text(value) => Some(value.clone()),
        _ => None,
    }
}

fn open_readonly(db_path: &str) -> Result<Connection, String> {
    Connection::open_with_flags(db_path, OpenFlags::SQLITE_OPEN_READ_ONLY)
        .map_err(|error| error.to_string())
}

const ACTIVE_EDITOR_SQL: &str = "select
          i.kind as item_kind,
          e.item_id as editor_id,
          i.workspace_id as workspace_id,
          w.paths as workspace_paths,
          w.timestamp as timestamp,
          e.buffer_path as buffer_path
        from items i
        join panes p on p.pane_id = i.pane_id and p.workspace_id = i.workspace_id
        join workspaces w on w.workspace_id = i.workspace_id
        left join editors e on e.item_id = i.item_id and e.workspace_id = i.workspace_id
        where i.active = 1 and p.active = 1
        order by w.timestamp desc";

const SELECTIONS_SQL: &str = "select
          start as selection_start,
          end as selection_end
        from editor_selections
        where editor_id = $editorID and workspace_id = $workspaceID";

const CONTENTS_SQL: &str = "select contents
        from editors
        where item_id = $editorID and workspace_id = $workspaceID";

enum ActiveEditor {
    Row(ZedEditorRow),
    Empty,
    Unavailable,
}

fn query_zed_active_editor(db_path: &str, cwd: &str) -> ActiveEditor {
    let conn = match open_readonly(db_path) {
        Ok(conn) => conn,
        Err(_) => return ActiveEditor::Unavailable,
    };
    let mut statement = match conn.prepare(ACTIVE_EDITOR_SQL) {
        Ok(statement) => statement,
        Err(_) => return ActiveEditor::Unavailable,
    };
    let raw = match statement.query_map([], |row| {
        Ok((
            row.get::<_, Value>(0)?,
            row.get::<_, Value>(1)?,
            row.get::<_, Value>(2)?,
            row.get::<_, Value>(3)?,
            row.get::<_, Value>(4)?,
            row.get::<_, Value>(5)?,
        ))
    }) {
        Ok(raw) => raw,
        Err(_) => return ActiveEditor::Unavailable,
    };
    let mut rows: Vec<ZedEditorRow> = Vec::new();
    let mut raw_count = 0;
    for entry in raw {
        raw_count += 1;
        let Ok((item_kind, editor_id, workspace_id, workspace_paths, timestamp, buffer_path)) =
            entry
        else {
            continue;
        };
        let (Some(item_kind), Some(workspace_id), Some(timestamp)) = (
            as_string(&item_kind),
            as_i64(&workspace_id),
            as_string(&timestamp),
        ) else {
            continue;
        };
        rows.push(ZedEditorRow {
            item_kind,
            editor_id: as_i64(&editor_id),
            workspace_id,
            workspace_paths: as_string(&workspace_paths),
            timestamp,
            buffer_path: as_string(&buffer_path),
        });
    }
    if raw_count > 0 && rows.is_empty() {
        return ActiveEditor::Unavailable;
    }
    let mut scored: Vec<(ZedEditorRow, usize)> = rows
        .into_iter()
        .map(|row| {
            let score = score_zed_workspace(row.workspace_paths.as_deref(), cwd);
            (row, score)
        })
        .filter(|(_, score)| *score > 0)
        .collect();
    // Timestamp comparison is byte-lexicographic, like `localeCompare` on the
    // ISO strings Zed stores.
    scored.sort_by(|left, right| {
        right
            .1
            .cmp(&left.1)
            .then_with(|| right.0.timestamp.cmp(&left.0.timestamp))
    });
    let Some((row, _)) = scored.into_iter().next() else {
        return ActiveEditor::Empty;
    };
    if row.item_kind != "Editor" {
        return ActiveEditor::Unavailable;
    }
    if row.editor_id.is_none() {
        return ActiveEditor::Empty;
    }
    ActiveEditor::Row(row)
}

fn query_zed_editor_selections(
    db_path: &str,
    editor_id: i64,
    workspace_id: i64,
) -> Result<Vec<(i64, i64)>, ()> {
    let conn = open_readonly(db_path).map_err(|_| ())?;
    let mut statement = conn.prepare(SELECTIONS_SQL).map_err(|_| ())?;
    let raw = statement
        .query_map(
            rusqlite::named_params!["$editorID": editor_id, "$workspaceID": workspace_id],
            |row| Ok((row.get::<_, Value>(0)?, row.get::<_, Value>(1)?)),
        )
        .map_err(|_| ())?;
    let mut selections = Vec::new();
    let mut raw_count = 0;
    for entry in raw {
        raw_count += 1;
        let Ok((start, end)) = entry else {
            continue;
        };
        let (Some(start), Some(end)) = (as_i64(&start), as_i64(&end)) else {
            continue;
        };
        selections.push((start, end));
    }
    if raw_count > 0 && selections.is_empty() {
        return Err(());
    }
    Ok(selections)
}

fn query_zed_editor_contents(
    db_path: &str,
    editor_id: i64,
    workspace_id: i64,
) -> Option<Option<String>> {
    let conn = open_readonly(db_path).ok()?;
    let contents: Option<Value> = conn
        .query_row(
            CONTENTS_SQL,
            rusqlite::named_params!["$editorID": editor_id, "$workspaceID": workspace_id],
            |row| row.get::<_, Value>(0),
        )
        .optional()
        .ok()
        .flatten();
    Some(as_string(&contents?))
}

/// Mirrors `resolveZedSelection`.
pub fn resolve_zed_selection(db_path: &str, cwd: &str) -> ZedSelectionResult {
    let row = match query_zed_active_editor(db_path, cwd) {
        ActiveEditor::Row(row) => row,
        ActiveEditor::Empty => return ZedSelectionResult::Empty,
        ActiveEditor::Unavailable => return ZedSelectionResult::Unavailable,
    };
    let editor_id = row.editor_id.unwrap_or(0);
    let Some(buffer_path) = row.buffer_path.clone() else {
        return ZedSelectionResult::Empty;
    };
    let Ok(selections) = query_zed_editor_selections(db_path, editor_id, row.workspace_id) else {
        return ZedSelectionResult::Unavailable;
    };
    let mut byte_ranges: Vec<(i64, i64)> = selections
        .into_iter()
        .map(|(start, end)| (start.min(end), start.max(end)))
        .collect();
    byte_ranges.sort_by(|left, right| left.0.cmp(&right.0).then(left.1.cmp(&right.1)));
    if byte_ranges.is_empty() {
        return ZedSelectionResult::Unavailable;
    }
    let text = match query_zed_editor_contents(db_path, editor_id, row.workspace_id) {
        Some(Some(contents)) => contents,
        _ => match std::fs::read_to_string(&buffer_path) {
            Ok(text) => text,
            Err(_) => return ZedSelectionResult::Unavailable,
        },
    };
    let ranges = byte_ranges
        .into_iter()
        .map(|(start, end)| {
            let start_offset = utf8_byte_offset_to_string_index(&text, start);
            let end_offset = utf8_byte_offset_to_string_index(&text, end);
            EditorSelectionRange {
                text: utf16_slice(&text, start_offset, end_offset),
                selection: offsets_to_selection(&text, start_offset as i64, end_offset as i64),
            }
        })
        .collect();
    ZedSelectionResult::Selection {
        selection: EditorSelection {
            file_path: buffer_path,
            source: Some("zed".to_string()),
            ranges,
        },
    }
}

/// Mirrors `resolveZedDbPath`.
pub fn resolve_zed_db_path() -> Option<String> {
    let mut candidates = Vec::new();
    if let Ok(value) = std::env::var("OPENCODE_ZED_DB") {
        if !value.is_empty() {
            candidates.push(value);
        }
    }
    if let Some(home) = crate::editor::home_dir() {
        candidates.push(format!(
            "{home}/Library/Application Support/Zed/db/0-stable/db.sqlite"
        ));
        candidates.push(format!("{home}/.local/share/zed/db/0-stable/db.sqlite"));
    }
    candidates
        .into_iter()
        .find(|item| crate::editor::is_file(item))
}

/// Mirrors `isZedTerminal`.
pub fn is_zed_terminal() -> bool {
    if std::env::var("ZED_TERM").as_deref() == Ok("true") {
        return true;
    }
    std::env::var("TERM_PROGRAM")
        .map(|program| program.to_lowercase() == "zed")
        .unwrap_or(false)
}

/// Mirrors `offsetToPosition`.
pub fn offset_to_position(text: &str, offset: i64) -> EditorPosition {
    let string_offset = utf8_byte_offset_to_string_index(text, offset);
    offsets_to_selection(text, string_offset as i64, string_offset as i64).start
}

/// UTF-16 code-unit length (what JavaScript calls `text.length`).
fn utf16_len(text: &str) -> usize {
    text.encode_utf16().count()
}

/// Mirrors `utf8ByteOffsetToStringIndex`: walk UTF-16 units, accumulating
/// UTF-8 bytes, and return the first unit index at or past `byte_offset`.
fn utf8_byte_offset_to_string_index(text: &str, byte_offset: i64) -> usize {
    if byte_offset <= 0 {
        return 0;
    }
    let mut bytes: i64 = 0;
    let mut units = 0;
    for ch in text.chars() {
        bytes += ch.len_utf8() as i64;
        units += ch.len_utf16();
        if bytes >= byte_offset {
            return units;
        }
    }
    units
}

/// Slice `text` by UTF-16 unit indices (mirrors `String.prototype.slice`).
fn utf16_slice(text: &str, start: usize, end: usize) -> String {
    fn byte_index(text: &str, units: usize) -> usize {
        let mut seen = 0;
        for (byte, ch) in text.char_indices() {
            if seen >= units {
                return byte;
            }
            seen += ch.len_utf16();
        }
        text.len()
    }
    let total = utf16_len(text);
    let start = start.min(total);
    let end = end.min(total).max(start);
    text[byte_index(text, start)..byte_index(text, end)].to_string()
}

/// Mirrors `offsetsToSelection`: UTF-16 offsets to 1-based line/character
/// positions (`character` is `offset - lineStart + 1`).
fn offsets_to_selection(text: &str, start_offset: i64, end_offset: i64) -> EditorSelectionSpan {
    let total = utf16_len(text) as i64;
    let start = start_offset.clamp(0, total) as usize;
    let end = end_offset.clamp(0, total).max(start_offset.clamp(0, total)) as usize;
    let units: Vec<u16> = text.encode_utf16().collect();
    let position = |line: i64, line_start: usize, offset: usize| EditorPosition {
        line,
        character: offset as i64 - line_start as i64 + 1,
    };
    let (mut line, mut line_start) = (1i64, 0usize);
    let mut start_position = None;
    let mut end_position = None;
    for index in 0..=end {
        if index == start {
            start_position = Some(position(line, line_start, index));
        }
        if index == end {
            end_position = Some(position(line, line_start, index));
            break;
        }
        if units.get(index) == Some(&(b'\n' as u16)) {
            line += 1;
            line_start = index + 1;
        }
    }
    EditorSelectionSpan {
        start: start_position.expect("start offset is inside the text"),
        end: end_position.expect("end offset is inside the text"),
    }
}

/// Mirrors `scoreZedWorkspace`.
fn score_zed_workspace(workspace_paths: Option<&str>, cwd: &str) -> usize {
    zed_workspace_paths(workspace_paths)
        .iter()
        .map(|item| {
            if crate::editor::path_contains(item, cwd) {
                item.chars().count()
            } else {
                0
            }
        })
        .max()
        .unwrap_or(0)
}

/// Mirrors `zedWorkspacePaths`: JSON string array, else newline-split lines.
fn zed_workspace_paths(value: Option<&str>) -> Vec<String> {
    let Some(value) = value else {
        return Vec::new();
    };
    if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(value) {
        if let Some(items) = parsed.as_array() {
            return items
                .iter()
                .filter_map(|item| item.as_str().map(str::to_string))
                .collect();
        }
    }
    value
        .split('\n')
        .map(|line| line.strip_suffix('\r').unwrap_or(line).to_string())
        .filter(|line| !line.is_empty())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn byte_offsets_count_utf16_units() {
        // a(1B,1u) é(2B,1u) 😀(4B,2u) b(1B,1u)
        let text = "aé😀b";
        assert_eq!(utf8_byte_offset_to_string_index(text, 0), 0);
        assert_eq!(utf8_byte_offset_to_string_index(text, 1), 1);
        assert_eq!(utf8_byte_offset_to_string_index(text, 3), 2);
        assert_eq!(utf8_byte_offset_to_string_index(text, 4), 4);
        assert_eq!(utf8_byte_offset_to_string_index(text, 100), 5);
        assert_eq!(utf16_slice(text, 2, 4), "😀");
    }

    #[test]
    fn offsets_become_one_based_positions() {
        let span = offsets_to_selection("ab\ncde", 0, 5);
        assert_eq!(
            span.start,
            EditorPosition {
                line: 1,
                character: 1
            }
        );
        assert_eq!(
            span.end,
            EditorPosition {
                line: 2,
                character: 3
            }
        );
        // `offsetToPosition` returns the start of a zero-width range at the
        // offset (`offsetsToSelection(text, offset, offset).start`).
        assert_eq!(
            offset_to_position("ab\ncde", 4),
            EditorPosition {
                line: 2,
                character: 2
            }
        );
    }

    #[test]
    fn workspace_paths_accept_json_or_newlines() {
        assert_eq!(zed_workspace_paths(None), Vec::<String>::new());
        assert_eq!(
            zed_workspace_paths(Some("[\"/a\", 1, \"/b\"]")),
            vec!["/a".to_string(), "/b".to_string()]
        );
        assert_eq!(
            zed_workspace_paths(Some("/a\r\n\r\n/b\n")),
            vec!["/a".to_string(), "/b".to_string()]
        );
    }

    #[test]
    fn missing_databases_are_unavailable() {
        assert_eq!(
            resolve_zed_selection("/definitely/not/zed.sqlite", "/tmp"),
            ZedSelectionResult::Unavailable
        );
    }

    #[test]
    fn db_path_prefers_the_env_override() {
        let path = std::env::temp_dir().join(format!("zed-db-test-{}.sqlite", std::process::id()));
        std::fs::write(&path, b"").expect("write temp db");
        let name = path.to_string_lossy().into_owned();
        let previous = std::env::var("OPENCODE_ZED_DB").ok();
        std::env::set_var("OPENCODE_ZED_DB", &name);
        let resolved = resolve_zed_db_path();
        match previous {
            Some(value) => std::env::set_var("OPENCODE_ZED_DB", value),
            None => std::env::remove_var("OPENCODE_ZED_DB"),
        }
        assert_eq!(resolved, Some(name));
        let _ = std::fs::remove_file(&path);
    }
}
