// source: src/patch/index.ts — exports: PatchSchema, PatchParams,
// ApplyPatchArgs, Hunk, UpdateFileChunk, ApplyPatchAction,
// ApplyPatchFileChange, AffectedPaths, ApplyPatchError, MaybeApplyPatch,
// MaybeApplyPatchVerified, parsePatch, maybeParseApplyPatch,
// deriveNewContentsFromChunks, applyHunksToFiles, applyPatch,
// maybeParseApplyPatchVerified, Patch
// Pure parser + content-derivation ported verbatim; BOM helpers mirror
// src/util/bom.ts (split/join); fs-touching fns as trait w/ verbatim messages.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// source: PatchSchema { patchText } — verbatim.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PatchParams {
    pub patch_text: String,
}

/// source: Hunk — verbatim variants/fields.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Hunk {
    Add {
        path: String,
        contents: String,
    },
    Delete {
        path: String,
    },
    Update {
        path: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        move_path: Option<String>,
        chunks: Vec<UpdateFileChunk>,
    },
}

/// source: UpdateFileChunk — verbatim snake_case fields.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateFileChunk {
    pub old_lines: Vec<String>,
    pub new_lines: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_context: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_end_of_file: Option<bool>,
}

/// source: ApplyPatchArgs — verbatim.
#[derive(Debug, Clone)]
pub struct ApplyPatchArgs {
    pub patch: String,
    pub hunks: Vec<Hunk>,
    pub workdir: Option<String>,
}

/// source: ApplyPatchFileChange — verbatim.
#[derive(Debug, Clone)]
pub enum ApplyPatchFileChange {
    Add {
        content: String,
    },
    Delete {
        content: String,
    },
    Update {
        unified_diff: String,
        move_path: Option<String>,
        new_content: String,
    },
}

/// source: ApplyPatchAction — verbatim.
#[derive(Debug, Clone)]
pub struct ApplyPatchAction {
    pub changes: HashMap<String, ApplyPatchFileChange>,
    pub patch: String,
    pub cwd: String,
}

/// source: AffectedPaths — verbatim.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AffectedPaths {
    pub added: Vec<String>,
    pub modified: Vec<String>,
    pub deleted: Vec<String>,
}

/// source: ApplyPatchError — verbatim string values.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ApplyPatchError {
    ParseError,
    IoError,
    ComputeReplacements,
    ImplicitInvocation,
}

impl ApplyPatchError {
    pub fn as_str(&self) -> &'static str {
        match self {
            ApplyPatchError::ParseError => "ParseError",
            ApplyPatchError::IoError => "IoError",
            ApplyPatchError::ComputeReplacements => "ComputeReplacements",
            ApplyPatchError::ImplicitInvocation => "ImplicitInvocation",
        }
    }
}

/// source: MaybeApplyPatch — verbatim.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MaybeApplyPatch {
    Body,
    ShellParseError,
    PatchParseError,
    NotApplyPatch,
}

/// source: MaybeApplyPatchVerified — verbatim.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MaybeApplyPatchVerified {
    Body,
    ShellParseError,
    CorrectnessError,
    NotApplyPatch,
}

/// source: markers — "*** Begin Patch" / "*** End Patch", verbatim.
pub const BEGIN_MARKER: &str = "*** Begin Patch";
pub const END_MARKER: &str = "*** End Patch";
/// source: "Invalid patch format: missing Begin/End markers" — verbatim.
pub const INVALID_FORMAT_MESSAGE: &str = "Invalid patch format: missing Begin/End markers";
/// source: "No files were modified." — verbatim.
pub const NO_FILES_MESSAGE: &str = "No files were modified.";
/// source: APPLY_PATCH_COMMANDS — verbatim.
pub const APPLY_PATCH_COMMANDS: &[&str] = &["apply_patch", "applypatch"];

fn parse_patch_header(lines: &[&str], start: usize) -> Option<(String, Option<String>, usize)> {
    let line = lines.get(start)?;
    if let Some(rest) = line.strip_prefix("*** Add File:") {
        let p = rest.trim();
        return if p.is_empty() {
            None
        } else {
            Some((p.to_string(), None, start + 1))
        };
    }
    if let Some(rest) = line.strip_prefix("*** Delete File:") {
        let p = rest.trim();
        return if p.is_empty() {
            None
        } else {
            Some((p.to_string(), None, start + 1))
        };
    }
    if let Some(rest) = line.strip_prefix("*** Update File:") {
        let p = rest.trim();
        if p.is_empty() {
            return None;
        }
        let mut move_path = None;
        let mut next = start + 1;
        if next < lines.len() {
            if let Some(m) = lines[next].strip_prefix("*** Move to:") {
                move_path = Some(m.trim().to_string());
                next += 1;
            }
        }
        return Some((p.to_string(), move_path, next));
    }
    None
}

fn parse_update_chunks(lines: &[&str], start: usize) -> (Vec<UpdateFileChunk>, usize) {
    let mut chunks = Vec::new();
    let mut i = start;
    while i < lines.len() && !lines[i].starts_with("***") {
        if lines[i].starts_with("@@") {
            let context = lines[i][2..].trim().to_string();
            i += 1;
            let mut old_lines = Vec::new();
            let mut new_lines = Vec::new();
            let mut eof = false;
            while i < lines.len() && !lines[i].starts_with("@@") && !lines[i].starts_with("***") {
                let l = lines[i];
                if l == "*** End of File" {
                    eof = true;
                    i += 1;
                    break;
                } else if let Some(c) = l.strip_prefix(' ') {
                    old_lines.push(c.to_string());
                    new_lines.push(c.to_string());
                } else if let Some(c) = l.strip_prefix('-') {
                    old_lines.push(c.to_string());
                } else if let Some(c) = l.strip_prefix('+') {
                    new_lines.push(c.to_string());
                }
                i += 1;
            }
            chunks.push(UpdateFileChunk {
                old_lines,
                new_lines,
                change_context: if context.is_empty() {
                    None
                } else {
                    Some(context)
                },
                is_end_of_file: if eof { Some(true) } else { None },
            });
        } else {
            i += 1;
        }
    }
    (chunks, i)
}

fn parse_add_content(lines: &[&str], start: usize) -> (String, usize) {
    let mut content = String::new();
    let mut i = start;
    while i < lines.len() && !lines[i].starts_with("***") {
        if let Some(c) = lines[i].strip_prefix('+') {
            content.push_str(c);
            content.push('\n');
        }
        i += 1;
    }
    if content.ends_with('\n') {
        content.pop();
    }
    (content, i)
}

fn strip_heredoc(input: &str) -> &str {
    // source regex: /^(?:cat\s+)?<<['"]?(\w+)['"]?\s*\n([\s\S]*?)\n\1\s*$/ — verbatim logic.
    let mut rest = input;
    if let Some(s) = rest.strip_prefix("cat ") {
        rest = s.trim_start();
        if rest.starts_with(' ') {
            rest = rest.trim_start();
        }
    } else if rest.starts_with("cat\t") {
        rest = rest[4..].trim_start();
    }
    if !rest.starts_with("<<") {
        return input;
    }
    let mut r = &rest[2..];
    r = r.trim_start_matches(|c| c == '\'' || c == '"');
    let delim_end = r
        .find(|c: char| !(c.is_alphanumeric() || c == '_'))
        .unwrap_or(r.len());
    let delim = &r[..delim_end];
    if delim.is_empty() {
        return input;
    }
    let mut r2 = r[delim_end..].trim_start_matches(|c| c == '\'' || c == '"');
    if !r2.starts_with('\n') {
        // allow trailing spaces before newline
        let t = r2.trim_start_matches(' ');
        if !t.starts_with('\n') {
            return input;
        }
        r2 = t;
    }
    let body = &r2[1..];
    let closing = format!("\n{}", delim);
    if let Some(pos) = body.rfind(&closing) {
        let after = &body[pos + closing.len()..];
        if after.trim().is_empty() {
            return &body[..pos];
        }
    }
    input
}

/// source: parsePatch — verbatim.
pub fn parse_patch(patch_text: &str) -> Result<Vec<Hunk>, String> {
    let cleaned = strip_heredoc(patch_text.trim());
    let lines: Vec<&str> = cleaned.split('\n').collect();
    let begin = lines.iter().position(|l| l.trim() == BEGIN_MARKER);
    let end = lines.iter().position(|l| l.trim() == END_MARKER);
    match (begin, end) {
        Some(b, e) if b < e => {}
        _ => return Err(INVALID_FORMAT_MESSAGE.to_string()),
    }
    let (b, e) = (begin.unwrap(), end.unwrap());
    let mut hunks = Vec::new();
    let mut i = b + 1;
    while i < e {
        match parse_patch_header(&lines, i) {
            None => i += 1,
            Some((path, move_path, next)) => {
                if lines[i].starts_with("*** Add File:") {
                    let (content, ni) = parse_add_content(&lines, next);
                    hunks.push(Hunk::Add {
                        path,
                        contents: content,
                    });
                    i = ni;
                } else if lines[i].starts_with("*** Delete File:") {
                    hunks.push(Hunk::Delete { path });
                    i = next;
                } else if lines[i].starts_with("*** Update File:") {
                    let (chunks, ni) = parse_update_chunks(&lines, next);
                    hunks.push(Hunk::Update {
                        path,
                        move_path,
                        chunks,
                    });
                    i = ni;
                } else {
                    i += 1;
                }
            }
        }
    }
    Ok(hunks)
}

/// source: maybeParseApplyPatch — verbatim (direct + bash-heredoc forms).
pub enum MaybeParse {
    Body(ApplyPatchArgs),
    PatchParseError(String),
    NotApplyPatch,
}

pub fn maybe_parse_apply_patch(argv: &[&str]) -> MaybeParse {
    if argv.len() == 2 && APPLY_PATCH_COMMANDS.contains(&argv[0]) {
        return match parse_patch(argv[1]) {
            Ok(hunks) => MaybeParse::Body(ApplyPatchArgs {
                patch: argv[1].to_string(),
                hunks,
                workdir: None,
            }),
            Err(e) => MaybeParse::PatchParseError(e),
        };
    }
    if argv.len() == 3 && argv[0] == "bash" && argv[1] == "-lc" {
        let script = argv[2];
        if let Some(pos) = script.find("apply_patch") {
            let rest = &script[pos + "apply_patch".len()..];
            let rest = rest.trim_start();
            if rest.starts_with("<<") {
                let mut r = rest[2..].trim_start_matches(|c| c == '\'' || c == '"');
                let de = r
                    .find(|c: char| !(c.is_alphanumeric() || c == '_'))
                    .unwrap_or(r.len());
                let delim = r[..de].to_string();
                r = r[de..].trim_start_matches(|c| c == '\'' || c == '"');
                if let Some(nl) = r.find('\n') {
                    let body = &r[nl + 1..];
                    let closing = format!("\n{}", delim);
                    if let Some(ep) = body.find(&closing) {
                        let content = &body[..ep];
                        return match parse_patch(content) {
                            Ok(hunks) => MaybeParse::Body(ApplyPatchArgs {
                                patch: content.to_string(),
                                hunks,
                                workdir: None,
                            }),
                            Err(e) => MaybeParse::PatchParseError(e),
                        };
                    }
                }
            }
        }
    }
    MaybeParse::NotApplyPatch
}

/// source: normalizeUnicode — verbatim replacement sets.
pub fn normalize_unicode(s: &str) -> String {
    s.replace(['‘', '’', '‚', '‛'], "'")
        .replace(['“', '”', '„', '‟'], "\"")
        .replace(['‐', '‑', '‒', '–', '—', '―'], "-")
        .replace('…', "...")
        .replace(' ', " ")
}

fn try_match(
    lines: &[String],
    pattern: &[String],
    start: usize,
    eof: bool,
    cmp: &dyn Fn(&str, &str) -> bool,
) -> i64 {
    if eof && pattern.len() <= lines.len() {
        let from_end = lines.len() - pattern.len();
        if from_end >= start
            && pattern
                .iter()
                .enumerate()
                .all(|(j, p)| cmp(&lines[from_end + j], p))
        {
            return from_end as i64;
        }
    }
    if pattern.len() > lines.len() {
        return -1;
    }
    for i in start..=(lines.len() - pattern.len()) {
        if pattern
            .iter()
            .enumerate()
            .all(|(j, p)| cmp(&lines[i + j], p))
        {
            return i as i64;
        }
    }
    -1
}

/// source: seekSequence — 4 passes (exact, rstrip, trim, normalized), verbatim.
pub fn seek_sequence(lines: &[String], pattern: &[String], start: usize, eof: bool) -> i64 {
    if pattern.is_empty() {
        return -1;
    }
    let r = try_match(lines, pattern, start, eof, &|a, b| a == b);
    if r != -1 {
        return r;
    }
    let r = try_match(lines, pattern, start, eof, &|a, b| {
        a.trim_end() == b.trim_end()
    });
    if r != -1 {
        return r;
    }
    let r = try_match(lines, pattern, start, eof, &|a, b| a.trim() == b.trim());
    if r != -1 {
        return r;
    }
    try_match(lines, pattern, start, eof, &|a, b| {
        normalize_unicode(a.trim()) == normalize_unicode(b.trim())
    })
}

/// source: computeReplacements — verbatim (context seek, pure-addition branch,
/// trailing-empty retry, sort by index).
pub fn compute_replacements(
    original: &[String],
    file_path: &str,
    chunks: &[UpdateFileChunk],
) -> Result<Vec<(usize, usize, Vec<String>)>, String> {
    let mut replacements: Vec<(usize, usize, Vec<String>)> = Vec::new();
    let mut line_index = 0;
    for chunk in chunks {
        if let Some(ctx) = &chunk.change_context {
            let c = seek_sequence(original, &[ctx.clone()], line_index, false);
            if c == -1 {
                return Err(format!("Failed to find context '{}' in {}", ctx, file_path));
            }
            line_index = (c as usize) + 1;
        }
        if chunk.old_lines.is_empty() {
            let idx = if !original.is_empty() && original[original.len() - 1].is_empty() {
                original.len() - 1
            } else {
                original.len()
            };
            replacements.push((idx, 0, chunk.new_lines.clone()));
            continue;
        }
        let mut pattern = chunk.old_lines.clone();
        let mut new_slice = chunk.new_lines.clone();
        let mut found = seek_sequence(
            original,
            &pattern,
            line_index,
            chunk.is_end_of_file.unwrap_or(false),
        );
        if found == -1 && !pattern.is_empty() && pattern[pattern.len() - 1].is_empty() {
            pattern.pop();
            if !new_slice.is_empty() && new_slice[new_slice.len() - 1].is_empty() {
                new_slice.pop();
            }
            found = seek_sequence(
                original,
                &pattern,
                line_index,
                chunk.is_end_of_file.unwrap_or(false),
            );
        }
        if found != -1 {
            line_index = (found as usize) + pattern.len();
            replacements.push((found as usize, pattern.len(), new_slice));
        } else {
            return Err(format!(
                "Failed to find expected lines in {}:\n{}",
                file_path,
                chunk.old_lines.join("\n")
            ));
        }
    }
    replacements.sort_by(|a, b| a.0.cmp(&b.0));
    Ok(replacements)
}

/// source: applyReplacements — reverse-order splice, verbatim.
pub fn apply_replacements(
    lines: &[String],
    replacements: &[(usize, usize, Vec<String>)],
) -> Vec<String> {
    let mut result = lines.to_vec();
    for (start, old_len, seg) in replacements.iter().rev() {
        result.splice(*start..start + old_len, seg.clone());
    }
    result
}

/// source: deriveNewContentsFromChunks — verbatim incl. trailing-newline rule.
pub struct FileUpdate {
    pub unified_diff: String,
    pub content: String,
    pub bom: bool,
}

pub fn derive_new_contents_from_chunks(
    file_path: &str,
    chunks: &[UpdateFileChunk],
    original_text: &str,
) -> Result<FileUpdate, String> {
    let (bom, text) = crate::util::bom::split(original_text);
    let mut original_lines: Vec<String> = text.split('\n').map(|s| s.to_string()).collect();
    if !original_lines.is_empty() && original_lines[original_lines.len() - 1].is_empty() {
        original_lines.pop();
    }
    let replacements = compute_replacements(&original_lines, file_path, chunks)?;
    let mut new_lines = apply_replacements(&original_lines, &replacements);
    if new_lines.is_empty() || !new_lines[new_lines.len() - 1].is_empty() {
        new_lines.push(String::new());
    }
    let joined = new_lines.join("\n");
    let (next_bom, new_content) = crate::util::bom::split(&joined);
    let unified_diff = generate_unified_diff(&text, &new_content);
    Ok(FileUpdate {
        unified_diff,
        content: new_content,
        bom: bom || next_bom,
    })
}

/// source: generateUnifiedDiff — verbatim simplified algorithm.
pub fn generate_unified_diff(old_content: &str, new_content: &str) -> String {
    let old_lines: Vec<&str> = old_content.split('\n').collect();
    let new_lines: Vec<&str> = new_content.split('\n').collect();
    let mut diff = String::from("@@ -1 +1 @@\n");
    let mut changed = false;
    let max = old_lines.len().max(new_lines.len());
    for i in 0..max {
        let o = old_lines.get(i).copied().unwrap_or("");
        let n = new_lines.get(i).copied().unwrap_or("");
        if o != n {
            if !o.is_empty() {
                diff.push('-');
                diff.push_str(o);
                diff.push('\n');
            }
            if !n.is_empty() {
                diff.push('+');
                diff.push_str(n);
                diff.push('\n');
            }
            changed = true;
        } else if !o.is_empty() {
            diff.push(' ');
            diff.push_str(o);
            diff.push('\n');
        }
    }
    if changed {
        diff
    } else {
        String::new()
    }
}

/// source: applyHunksToFiles / applyPatch / maybeParseApplyPatchVerified —
/// fs-touching; modelled as trait with verbatim messages.
/// PROVISIONAL pending crates/core (fs-util) — bodies run on CI-wired fs.
pub trait Fs {
    fn apply_hunks(&self, hunks: &[Hunk]) -> Result<AffectedPaths, String>;
}

/// source: "Failed to read file for deletion: {path}" — verbatim.
pub fn deletion_read_message(path: &str) -> String {
    format!("Failed to read file for deletion: {}", path)
}

/// source: "Failed to read file {path}: {cause}" — verbatim.
pub fn update_read_message(path: &str, cause: &str) -> String {
    format!("Failed to read file {}: {}", path, cause)
}
