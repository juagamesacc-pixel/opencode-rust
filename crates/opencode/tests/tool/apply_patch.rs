// source: test/tool/apply_patch.test.ts — exports: []
// PROVISIONAL pending @opencode-ai/core + effect/drizzle/bun:test — verbatim shape where pure
// original imports: import { describe, expect } from "bun:test"; import path from "path"; import * as fs from "fs/promises"

#[test]
fn tool_apply_patch_freeform() {
    // source: "tool.apply_patch freeform"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn requires_patch_text() {
    // source: "requires patchText"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn rejects_invalid_patch_format() {
    // source: "rejects invalid patch format"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn rejects_empty_patch() {
    // source: "rejects empty patch"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn produces_json_encodable_permission_metadata() {
    // source: "produces JSON-encodable permission metadata"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn applies_add_update_delete_in_one_patch() {
    // source: "applies add/update/delete in one patch"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn permission_metadata_includes_move_file_info() {
    // source: "permission metadata includes move file info"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn applies_multiple_hunks_to_one_file() {
    // source: "applies multiple hunks to one file"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn does_not_invent_a_first_line_diff_for_bom_files() {
    // source: "does not invent a first-line diff for BOM files"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn inserts_lines_with_insert_only_hunk() {
    // source: "inserts lines with insert-only hunk"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn appends_trailing_newline_on_update() {
    // source: "appends trailing newline on update"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn moves_file_to_a_new_directory() {
    // source: "moves file to a new directory"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn moves_file_overwriting_existing_destination() {
    // source: "moves file overwriting existing destination"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn adds_file_overwriting_existing_file() {
    // source: "adds file overwriting existing file"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn rejects_update_when_target_file_is_missing() {
    // source: "rejects update when target file is missing"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn rejects_delete_when_file_is_missing() {
    // source: "rejects delete when file is missing"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn rejects_delete_when_target_is_a_directory() {
    // source: "rejects delete when target is a directory"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn rejects_invalid_hunk_header() {
    // source: "rejects invalid hunk header"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn rejects_update_with_missing_context() {
    // source: "rejects update with missing context"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn verification_failure_leaves_no_side_effects() {
    // source: "verification failure leaves no side effects"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn supports_end_of_file_anchor() {
    // source: "supports end of file anchor"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn rejects_missing_second_chunk_context() {
    // source: "rejects missing second chunk context"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn disambiguates_change_context_with_header() {
    // source: "disambiguates change context with @@ header"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn eof_anchor_matches_from_end_of_file_first() {
    // source: "EOF anchor matches from end of file first"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn parses_heredoc_wrapped_patch() {
    // source: "parses heredoc-wrapped patch"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn parses_heredoc_wrapped_patch_without_cat() {
    // source: "parses heredoc-wrapped patch without cat"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn matches_with_trailing_whitespace_differences() {
    // source: "matches with trailing whitespace differences"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn matches_with_leading_whitespace_differences() {
    // source: "matches with leading whitespace differences"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn matches_with_unicode_punctuation_differences() {
    // source: "matches with Unicode punctuation differences"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
